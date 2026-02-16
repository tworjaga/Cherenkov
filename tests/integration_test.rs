use std::time::Duration;
use tokio::time::timeout;
use cherenkov_core::{EventBus, CherenkovEvent, NewReading, Anomaly, Alert};
use cherenkov_db::{RadiationReading, QualityFlag};
use uuid::Uuid;
use chrono::Utc;

/// Integration test: EventBus publish/subscribe
#[tokio::test]
async fn test_eventbus_publish_subscribe() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let reading = NewReading {
        sensor_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        latitude: 40.7128,
        longitude: -74.0060,
        dose_rate_microsieverts: 0.15,
        source: "test".to_string(),
    };
    
    let event = CherenkovEvent::NewReading(reading.clone());
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::NewReading(r) => {
            assert_eq!(r.sensor_id, reading.sensor_id);
            assert_eq!(r.dose_rate_microsieverts, reading.dose_rate_microsieverts);
        }
        _ => panic!("Expected NewReading event"),
    }
}

/// Integration test: Anomaly detection event flow
#[tokio::test]
async fn test_anomaly_event_flow() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let anomaly = Anomaly {
        sensor_id: Uuid::new_v4().to_string(),
        severity: cherenkov_core::Severity::Critical,
        z_score: 4.5,
        timestamp: Utc::now(),
        dose_rate: 2.5,
        baseline: 0.15,
        message: "Test anomaly".to_string(),
    };
    
    let event = CherenkovEvent::AnomalyDetected(anomaly.clone());
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::AnomalyDetected(a) => {
            assert_eq!(a.sensor_id, anomaly.sensor_id);
            assert_eq!(a.severity, anomaly.severity);
        }
        _ => panic!("Expected AnomalyDetected event"),
    }
}

/// Integration test: Alert event flow
#[tokio::test]
async fn test_alert_event_flow() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let alert = Alert {
        alert_id: Uuid::new_v4().to_string(),
        alert_type: cherenkov_core::AlertType::RadiationSpike,
        severity: cherenkov_core::Severity::Warning,
        message: "Test alert".to_string(),
        affected_sensors: vec![Uuid::new_v4().to_string()],
        timestamp: Utc::now(),
        acknowledged: false,
    };
    
    let event = CherenkovEvent::AlertTriggered(alert.clone());
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::AlertTriggered(a) => {
            assert_eq!(a.alert_id, alert.alert_id);
            assert_eq!(a.message, alert.message);
        }
        _ => panic!("Expected AlertTriggered event"),
    }
}

/// Integration test: Multiple subscribers receive events
#[tokio::test]
async fn test_multiple_subscribers() {
    let event_bus = EventBus::new(1000);
    let mut rx1 = event_bus.subscribe();
    let mut rx2 = event_bus.subscribe();
    
    let reading = NewReading {
        sensor_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        latitude: 51.5074,
        longitude: -0.1278,
        dose_rate_microsieverts: 0.12,
        source: "test".to_string(),
    };
    
    let event = CherenkovEvent::NewReading(reading);
    event_bus.publish(event).await.unwrap();
    
    let received1 = timeout(Duration::from_secs(1), rx1.recv()).await;
    let received2 = timeout(Duration::from_secs(1), rx2.recv()).await;
    
    assert!(received1.is_ok());
    assert!(received2.is_ok());
}

/// Integration test: EventBus lag handling
#[tokio::test]
async fn test_eventbus_lag() {
    let event_bus = EventBus::new(10); // Small buffer
    let rx = event_bus.subscribe();
    
    // Publish more events than buffer size
    for i in 0..20 {
        let reading = NewReading {
            sensor_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            latitude: 35.6762,
            longitude: 139.6503,
            dose_rate_microsieverts: 0.1 * (i as f64),
            source: "test".to_string(),
        };
        let event = CherenkovEvent::NewReading(reading);
        event_bus.publish(event).await.unwrap();
    }
    
    // Subscriber should detect lag
    drop(rx); // Just verify no panic
}

/// Integration test: Database reading serialization
#[test]
fn test_radiation_reading_serialization() {
    let reading = RadiationReading {
        sensor_id: Uuid::new_v4(),
        bucket: 1234567890,
        timestamp: 1234567890123,
        latitude: 48.8566,
        longitude: 2.3522,
        dose_rate_microsieverts: 0.18,
        uncertainty: 0.05,
        quality_flag: QualityFlag::Valid,
        source: "safecast".to_string(),
        cell_id: "u09".to_string(),
    };
    
    let json = serde_json::to_string(&reading).unwrap();
    let deserialized: RadiationReading = serde_json::from_str(&json).unwrap();
    
    assert_eq!(reading.sensor_id, deserialized.sensor_id);
    assert_eq!(reading.dose_rate_microsieverts, deserialized.dose_rate_microsieverts);
    assert_eq!(reading.quality_flag as i32, deserialized.quality_flag as i32);
}

/// Integration test: QualityFlag serialization
#[test]
fn test_quality_flag_serialization() {
    let valid = QualityFlag::Valid;
    let suspect = QualityFlag::Suspect;
    let invalid = QualityFlag::Invalid;
    
    let valid_json = serde_json::to_string(&valid).unwrap();
    let suspect_json = serde_json::to_string(&suspect).unwrap();
    let invalid_json = serde_json::to_string(&invalid).unwrap();
    
    assert!(valid_json.contains("Valid"));
    assert!(suspect_json.contains("Suspect"));
    assert!(invalid_json.contains("Invalid"));
}

/// Integration test: Correlated event detection
#[tokio::test]
async fn test_correlated_event() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let primary = Anomaly {
        sensor_id: Uuid::new_v4().to_string(),
        severity: cherenkov_core::Severity::Critical,
        z_score: 5.0,
        timestamp: Utc::now(),
        dose_rate: 3.0,
        baseline: 0.15,
        message: "Primary anomaly".to_string(),
    };
    
    let event = CherenkovEvent::CorrelatedEventDetected {
        primary,
        correlated_count: 5,
        correlation_score: 0.85,
    };
    
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::CorrelatedEventDetected { correlated_count, .. } => {
            assert_eq!(correlated_count, 5);
        }
        _ => panic!("Expected CorrelatedEventDetected"),
    }
}

/// Integration test: Sensor status events
#[tokio::test]
async fn test_sensor_status_events() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let sensor_id = Uuid::new_v4();
    
    // Test offline event
    let offline_event = CherenkovEvent::SensorOffline { 
        sensor_id,
        last_seen: Utc::now(),
    };
    event_bus.publish(offline_event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::SensorOffline { id, .. } => {
            assert_eq!(id, sensor_id);
        }
        _ => panic!("Expected SensorOffline event"),
    }
    
    // Test online event
    let online_event = CherenkovEvent::SensorOnline {
        sensor_id,
        first_reading: Utc::now(),
    };
    event_bus.publish(online_event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::SensorOnline { id, .. } => {
            assert_eq!(id, sensor_id);
        }
        _ => panic!("Expected SensorOnline event"),
    }
}
