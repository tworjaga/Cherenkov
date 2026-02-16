use std::time::Duration;
use tokio::time::timeout;
use cherenkov_core::{EventBus, CherenkovEvent, NormalizedReading, Anomaly, Alert, Severity, SensorStatus};
use cherenkov_db::{RadiationReading, QualityFlag as DbQualityFlag};
use uuid::Uuid;
use chrono::Utc;

/// Integration test: EventBus publish/subscribe
#[tokio::test]
async fn test_eventbus_publish_subscribe() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let reading = NormalizedReading {
        sensor_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        latitude: 40.7128,
        longitude: -74.0060,
        dose_rate_microsieverts: 0.15,
        uncertainty: 0.02,
        source: "test".to_string(),
        quality_flag: cherenkov_core::QualityFlag::Valid,
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
        anomaly_id: Uuid::new_v4().to_string(),
        sensor_id: Uuid::new_v4(),
        severity: Severity::Critical,
        z_score: 4.5,
        detected_at: Utc::now(),
        dose_rate: 2.5,
        baseline: 0.15,
        algorithm: "welford".to_string(),
    };
    
    let event = CherenkovEvent::AnomalyDetected(anomaly.clone());
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::AnomalyDetected(a) => {
            assert_eq!(a.anomaly_id, anomaly.anomaly_id);
            assert_eq!(a.severity as i32, anomaly.severity as i32);
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
        anomaly_ids: vec![Uuid::new_v4().to_string()],
        message: "Test alert".to_string(),
        severity: Severity::Warning,
        created_at: Utc::now(),
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
    
    let reading = NormalizedReading {
        sensor_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        latitude: 51.5074,
        longitude: -0.1278,
        dose_rate_microsieverts: 0.12,
        uncertainty: 0.01,
        source: "test".to_string(),
        quality_flag: cherenkov_core::QualityFlag::Valid,
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
        let reading = NormalizedReading {
            sensor_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            latitude: 35.6762,
            longitude: 139.6503,
            dose_rate_microsieverts: 0.1 * (i as f64),
            uncertainty: 0.01,
            source: "test".to_string(),
            quality_flag: cherenkov_core::QualityFlag::Valid,
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
        quality_flag: DbQualityFlag::Valid,
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
    let valid = DbQualityFlag::Valid;
    let suspect = DbQualityFlag::Suspect;
    let invalid = DbQualityFlag::Invalid;
    
    let valid_json = serde_json::to_string(&valid).unwrap();
    let suspect_json = serde_json::to_string(&suspect).unwrap();
    let invalid_json = serde_json::to_string(&invalid).unwrap();
    
    assert!(valid_json.contains("Valid"));
    assert!(suspect_json.contains("Suspect"));
    assert!(invalid_json.contains("Invalid"));
}


/// Integration test: Sensor status change event
#[tokio::test]
async fn test_sensor_status_change() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let sensor_id = Uuid::new_v4();
    
    let event = CherenkovEvent::SensorStatusChange {
        sensor_id,
        status: SensorStatus::Offline,
        timestamp: Utc::now(),
    };
    
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::SensorStatusChange { sensor_id: id, status, .. } => {
            assert_eq!(id, sensor_id);
            assert!(matches!(status, SensorStatus::Offline));
        }
        _ => panic!("Expected SensorStatusChange"),
    }
}


/// Integration test: Health update event
#[tokio::test]
async fn test_health_update_event() {
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    
    let event = CherenkovEvent::HealthUpdate {
        component: "ingest".to_string(),
        healthy: true,
        message: Some("All systems operational".to_string()),
    };
    
    event_bus.publish(event).await.unwrap();
    
    let received = timeout(Duration::from_secs(1), rx.recv()).await;
    assert!(received.is_ok());
    
    match received.unwrap().unwrap() {
        CherenkovEvent::HealthUpdate { component, healthy, .. } => {
            assert_eq!(component, "ingest");
            assert!(healthy);
        }
        _ => panic!("Expected HealthUpdate event"),
    }
}
