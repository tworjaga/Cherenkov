use tokio::sync::broadcast;
use tracing::{debug, error, info};
use std::sync::Arc;

use crate::events::CherenkovEvent;

/// Event bus for inter-crate communication
/// 
/// Provides publish/subscribe messaging between all Cherenkov services:
/// - ingest publishes NewReading events
/// - stream publishes AnomalyDetected events  
/// - api subscribes to both for WebSocket broadcasting
#[derive(Debug, Clone)]
pub struct EventBus {
    tx: broadcast::Sender<CherenkovEvent>,
}

impl EventBus {
    /// Create new event bus with specified capacity
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        info!("EventBus initialized with capacity {}", capacity);
        Self { tx }
    }
    
    /// Publish event to all subscribers
    pub async fn publish(&self, event: CherenkovEvent) -> anyhow::Result<()> {
        match self.tx.send(event.clone()) {
            Ok(count) => {
                debug!("Event published to {} subscribers: {:?}", count, event);
                Ok(())
            }
            Err(_) => {
                error!("No subscribers for event: {:?}", event);
                Err(anyhow::anyhow!("No active subscribers"))
            }
        }
    }
    
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<CherenkovEvent> {
        self.tx.subscribe()
    }
    
    /// Get number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Shared event bus handle for dependency injection
pub type SharedEventBus = Arc<EventBus>;
