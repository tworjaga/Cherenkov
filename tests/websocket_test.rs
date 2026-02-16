use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::json;
use url::Url;

/// WebSocket integration test: Connect and receive messages
#[tokio::test]
async fn test_websocket_connection() {
    // Note: This test requires a running API server
    // Run with: cargo test --test websocket_test -- --ignored
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Send ping
    let ping = json!({"action": "ping"});
    write.send(Message::Text(ping.to_string())).await.unwrap();
    
    // Receive pong
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "pong");
    }
    
    // Close connection
    write.close().await.unwrap();
}

/// WebSocket integration test: Subscribe to sensor
#[tokio::test]
async fn test_websocket_subscribe_sensor() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Subscribe to sensor
    let subscribe = json!({
        "action": "subscribe_sensor",
        "sensor_id": "test-sensor-001"
    });
    write.send(Message::Text(subscribe.to_string())).await.unwrap();
    
    // Receive confirmation
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "subscribed");
        assert_eq!(response["sensor_id"], "test-sensor-001");
    }
    
    write.close().await.unwrap();
}

/// WebSocket integration test: Subscribe to region
#[tokio::test]
async fn test_websocket_subscribe_region() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Subscribe to region
    let subscribe = json!({
        "action": "subscribe_region",
        "region": {
            "lat_min": 40.0,
            "lat_max": 41.0,
            "lon_min": -75.0,
            "lon_max": -74.0
        }
    });
    write.send(Message::Text(subscribe.to_string())).await.unwrap();
    
    // Receive confirmation
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "subscribed");
        assert_eq!(response["region"], "active");
    }
    
    write.close().await.unwrap();
}

/// WebSocket integration test: Get subscriptions
#[tokio::test]
async fn test_websocket_get_subscriptions() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Subscribe to multiple sensors
    let subscribe = json!({
        "action": "subscribe_sensors",
        "sensor_ids": ["sensor-001", "sensor-002", "sensor-003"]
    });
    write.send(Message::Text(subscribe.to_string())).await.unwrap();
    
    // Skip confirmation
    let _ = read.next().await;
    
    // Get subscriptions
    let get_subs = json!({"action": "get_subscriptions"});
    write.send(Message::Text(get_subs.to_string())).await.unwrap();
    
    // Receive subscription list
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "subscriptions");
        assert_eq!(response["sensor_count"], 3);
    }
    
    write.close().await.unwrap();
}

/// WebSocket integration test: Unsubscribe
#[tokio::test]
async fn test_websocket_unsubscribe() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Subscribe
    let subscribe = json!({
        "action": "subscribe_sensor",
        "sensor_id": "test-sensor"
    });
    write.send(Message::Text(subscribe.to_string())).await.unwrap();
    let _ = read.next().await; // Skip confirmation
    
    // Unsubscribe
    let unsubscribe = json!({
        "action": "unsubscribe_sensor",
        "sensor_id": "test-sensor"
    });
    write.send(Message::Text(unsubscribe.to_string())).await.unwrap();
    
    // Receive confirmation
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "unsubscribed");
        assert_eq!(response["sensor_id"], "test-sensor");
    }
    
    write.close().await.unwrap();
}

/// WebSocket integration test: Heartbeat
#[tokio::test]
async fn test_websocket_heartbeat() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Wait for server ping (30 second interval)
    // For test, we'll send a client ping and expect pong
    write.send(Message::Ping(vec![])).await.unwrap();
    
    let msg = read.next().await.unwrap().unwrap();
    assert!(matches!(msg, Message::Pong(_)));
    
    write.close().await.unwrap();
}

/// WebSocket integration test: Rate limiting
#[tokio::test]
async fn test_websocket_rate_limit() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Send many messages quickly
    for i in 0..110 {
        let msg = json!({
            "action": "ping",
            "seq": i
        });
        write.send(Message::Text(msg.to_string())).await.unwrap();
    }
    
    // Should receive rate limit error
    let mut found_rate_limit = false;
    for _ in 0..110 {
        if let Some(Ok(Message::Text(text))) = read.next().await {
            if text.contains("Rate limit exceeded") {
                found_rate_limit = true;
                break;
            }
        }
    }
    
    assert!(found_rate_limit, "Should have received rate limit error");
    write.close().await.unwrap();
}

/// WebSocket integration test: Invalid command
#[tokio::test]
async fn test_websocket_invalid_command() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Send invalid command
    let invalid = json!({
        "action": "invalid_command"
    });
    write.send(Message::Text(invalid.to_string())).await.unwrap();
    
    // Receive error
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "error");
        assert!(response["message"].as_str().unwrap().contains("Unknown command"));
    }
    
    write.close().await.unwrap();
}

/// WebSocket integration test: Batch updates
#[tokio::test]
async fn test_websocket_batch_updates() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut _write, mut read) = ws_stream.split();
    
    // Subscribe to all sensors
    let subscribe = json!({
        "action": "subscribe_sensors",
        "sensor_ids": ["*"]
    });
    _write.send(Message::Text(subscribe.to_string())).await.unwrap();
    let _ = read.next().await; // Skip confirmation
    
    // Wait for potential batch updates
    sleep(Duration::from_millis(200)).await;
    
    // Connection should still be alive
    _write.close().await.unwrap();
}

/// WebSocket integration test: Connection status
#[tokio::test]
async fn test_websocket_status() {
    let url = Url::parse("ws://localhost:8080/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Get status
    let status = json!({"action": "get_status"});
    write.send(Message::Text(status.to_string())).await.unwrap();
    
    // Receive status
    let msg = read.next().await.unwrap().unwrap();
    if let Message::Text(text) = msg {
        let response: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(response["type"], "status");
        assert!(response.get("connections").is_some());
        assert!(response.get("server_time").is_some());
    }
    
    write.close().await.unwrap();
}
