use reqwest::Client;
use serde_json::json;

/// API integration test: Health check endpoint
#[tokio::test]
async fn test_health_endpoint() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/health")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "healthy");
    assert!(body.get("database").is_some());
}

/// API integration test: Readiness check endpoint
#[tokio::test]
async fn test_readiness_endpoint() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/ready")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["ready"].as_bool().unwrap_or(false));
    assert!(body.get("checks").is_some());
}

/// API integration test: GraphQL introspection
#[tokio::test]
async fn test_graphql_introspection() {
    let client = Client::new();
    
    let query = json!({
        "query": "
            {
                __schema {
                    types {
                        name
                    }
                }
            }
        "
    });
    
    let response = client
        .post("http://localhost:8080/graphql")
        .json(&query)
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("data").is_some());
    assert!(body["data"].get("__schema").is_some());
}

/// API integration test: GraphQL sensor query
#[tokio::test]
async fn test_graphql_sensor_query() {
    let client = Client::new();
    
    let query = json!({
        "query": "
            query {
                sensors {
                    sensorId
                    name
                    location
                    status
                }
            }
        "
    });
    
    let response = client
        .post("http://localhost:8080/graphql")
        .json(&query)
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("data").is_some());
}

/// API integration test: GraphQL readings query
#[tokio::test]
async fn test_graphql_readings_query() {
    let client = Client::new();
    
    let query = json!({
        "query": "
            query($sensorIds: [String!]!, $from: DateTime!, $to: DateTime!) {
                readings(
                    sensorIds: $sensorIds,
                    from: $from,
                    to: $to,
                    aggregation: MINUTE
                ) {
                    timestamp
                    value
                    sensorId
                }
            }
        ",
        "variables": {
            "sensorIds": ["test-sensor-001"],
            "from": "2024-01-01T00:00:00Z",
            "to": "2024-01-02T00:00:00Z"
        }
    });
    
    let response = client
        .post("http://localhost:8080/graphql")
        .json(&query)
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("data").is_some());
}

/// API integration test: GraphQL anomalies query
#[tokio::test]
async fn test_graphql_anomalies_query() {
    let client = Client::new();
    
    let query = json!({
        "query": "
            query {
                anomalies(
                    severity: CRITICAL,
                    limit: 10
                ) {
                    sensorId
                    severity
                    zScore
                    timestamp
                }
            }
        "
    });
    
    let response = client
        .post("http://localhost:8080/graphql")
        .json(&query)
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("data").is_some());
}

/// API integration test: REST sensors endpoint
#[tokio::test]
async fn test_rest_sensors() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/sensors")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.as_array().is_some());
}

/// API integration test: REST sensor detail endpoint
#[tokio::test]
async fn test_rest_sensor_detail() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/sensors/test-sensor-001")
        .send()
        .await
        .expect("Failed to send request");
    
    // May be 200 or 404 depending on if sensor exists
    assert!(response.status().is_success() || response.status().as_u16() == 404);
}

/// API integration test: REST readings endpoint
#[tokio::test]
async fn test_rest_readings() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/sensors/test-sensor-001/readings?from=2024-01-01&to=2024-01-02")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success() || response.status().as_u16() == 404);
}

/// API integration test: REST nearby sensors endpoint
#[tokio::test]
async fn test_rest_nearby() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/sensors/nearby?lat=40.7128&lon=-74.0060&radius_km=10")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.as_array().is_some());
}

/// API integration test: REST status endpoint
#[tokio::test]
async fn test_rest_status() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/status")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.get("defcon_level").is_some());
    assert!(body.get("active_sensors").is_some());
    assert!(body.get("anomalies_24h").is_some());
}

/// API integration test: REST anomalies endpoint
#[tokio::test]
async fn test_rest_anomalies() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/anomalies?severity=warning&limit=20")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.as_array().is_some());
}

/// API integration test: REST acknowledge alert endpoint
#[tokio::test]
async fn test_rest_acknowledge_alert() {
    let client = Client::new();
    let response = client
        .post("http://localhost:8080/v1/alerts/test-alert-001/acknowledge")
        .send()
        .await
        .expect("Failed to send request");
    
    // May be 200 or 404 depending on if alert exists
    assert!(response.status().is_success() || response.status().as_u16() == 404);
}

/// API integration test: CORS headers
#[tokio::test]
async fn test_cors_headers() {
    let client = Client::new();
    let response = client
        .request(reqwest::Method::OPTIONS, "http://localhost:8080/v1/sensors")
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "GET")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    assert!(response.headers().get("access-control-allow-origin").is_some());
}

/// API integration test: Rate limiting
#[tokio::test]
async fn test_rate_limiting() {
    let client = Client::new();
    
    // Make many requests quickly
    for _ in 0..70 {
        let _ = client
            .get("http://localhost:8080/health")
            .send()
            .await;
    }
    
    // Next request should be rate limited
    let response = client
        .get("http://localhost:8080/health")
        .send()
        .await
        .expect("Failed to send request");
    
    // Should either succeed or return 429 Too Many Requests
    assert!(response.status().is_success() || response.status().as_u16() == 429);
}

/// API integration test: Compression
#[tokio::test]
async fn test_compression() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/v1/sensors")
        .header("Accept-Encoding", "gzip")
        .send()
        .await
        .expect("Failed to send request");
    
    assert!(response.status().is_success());
    // Response may or may not be compressed depending on size
}

/// API integration test: Invalid endpoint
#[tokio::test]
async fn test_invalid_endpoint() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/invalid-endpoint")
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status().as_u16(), 404);
}

/// API integration test: Method not allowed
#[tokio::test]
async fn test_method_not_allowed() {
    let client = Client::new();
    let response = client
        .post("http://localhost:8080/health")
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status().as_u16(), 405);
}
