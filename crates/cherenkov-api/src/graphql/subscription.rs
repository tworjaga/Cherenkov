use async_graphql::{Subscription, ID};
use futures_util::stream::Stream;
use std::time::Duration;
use tokio_stream::StreamExt;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn reading_updates(&self, sensor_id: ID) -> impl Stream<Item = ReadingUpdate> {
        let interval = tokio::time::interval(Duration::from_secs(1));
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(move |_| ReadingUpdate {
                sensor_id: sensor_id.clone(),
                timestamp: chrono::Utc::now(),
                dose_rate: 0.15,
            })
    }
    
    async fn global_alerts(&self) -> impl Stream<Item = Alert> {
        let interval = tokio::time::interval(Duration::from_secs(5));
        tokio_stream::wrappers::IntervalStream::new(interval)
            .map(|_| Alert {
                id: ID::from(uuid::Uuid::new_v4().to_string()),
                severity: "info".to_string(),
                message: "System operational".to_string(),
            })
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct ReadingUpdate {
    pub sensor_id: ID,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub dose_rate: f64,
}

#[derive(async_graphql::SimpleObject)]
pub struct Alert {
    pub id: ID,
    pub severity: String,
    pub message: String,
}
