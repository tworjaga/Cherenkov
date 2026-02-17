use tracing_subscriber::fmt::format::FmtSpan;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TracingService {
    active_spans: Arc<RwLock<HashMap<String, SpanContext>>>,
    service_name: String,
    service_version: String,
    deployment_environment: String,
}

#[derive(Debug, Clone)]
pub struct SpanContext {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub service_name: String,
    pub operation: String,
    pub start_time: std::time::Instant,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TraceConfig {
    pub jaeger_endpoint: String,
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
    pub sample_rate: f64,
    pub max_spans_per_trace: usize,
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            jaeger_endpoint: "http://localhost:14268/api/traces".to_string(),
            service_name: "cherenkov".to_string(),
            service_version: "0.1.0".to_string(),
            environment: "development".to_string(),
            sample_rate: 1.0,
            max_spans_per_trace: 1000,
        }
    }
}

impl TracingService {
    pub fn new(config: TraceConfig) -> Self {
        Self {
            active_spans: Arc::new(RwLock::new(HashMap::new())),
            service_name: config.service_name.clone(),
            service_version: config.service_version.clone(),
            deployment_environment: config.environment.clone(),
        }
    }
    
    pub fn init_subscriber(&self) {
        let _ = tracing_subscriber::fmt()
            .with_span_events(FmtSpan::CLOSE)
            .json()
            .try_init();
    }
    
    pub async fn start_span(
        &self,
        operation: impl Into<String>,
        parent_context: Option<SpanContext>,
        attributes: HashMap<String, String>,
    ) -> SpanContext {
        let operation = operation.into();
        let trace_id = parent_context.as_ref()
            .map(|p| p.trace_id.clone())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        
        let span_id = Uuid::new_v4().to_string();
        let parent_span_id = parent_context.map(|p| p.span_id);
        
        let span = SpanContext {
            trace_id: trace_id.clone(),
            span_id: span_id.clone(),
            parent_span_id: parent_span_id.clone(),
            service_name: self.service_name.clone(),
            operation: operation.to_string(),
            start_time: std::time::Instant::now(),
            attributes: attributes.clone(),
        };
        
        let mut spans = self.active_spans.write().await;
        spans.insert(span_id.clone(), span.clone());
        
        tracing::info!(
            trace_id = %trace_id,
            span_id = %span_id,
            operation = %operation,
            "Span started"
        );
        
        span
    }
    
    pub async fn end_span(&self, span_id: &str) -> Option<u64> {
        let mut spans = self.active_spans.write().await;
        
        if let Some(span) = spans.remove(span_id) {
            let duration = span.start_time.elapsed().as_millis() as u64;
            return Some(duration);
        }
        
        None
    }
    
    pub async fn add_event(&self, span_id: &str, event_name: &str, attributes: HashMap<String, String>) {
        let spans = self.active_spans.read().await;
        
        if let Some(_span) = spans.get(span_id) {
            tracing::info!(
                trace_id = %_span.trace_id,
                span_id = %span_id,
                event = %event_name,
                ?attributes,
                "Trace event"
            );
        }
    }
    
    pub async fn get_trace_context(&self, span_id: &str) -> Option<SpanContext> {
        let spans = self.active_spans.read().await;
        spans.get(span_id).cloned()
    }
    
    pub fn extract_context_from_headers(headers: &HashMap<String, String>) -> Option<SpanContext> {
        let trace_id = headers.get("x-trace-id")?;
        let span_id = headers.get("x-span-id")?;
        let parent_span_id = headers.get("x-parent-span-id").cloned();
        
        Some(SpanContext {
            trace_id: trace_id.clone(),
            span_id: span_id.clone(),
            parent_span_id,
            service_name: "external".to_string(),
            operation: "incoming".to_string(),
            start_time: std::time::Instant::now(),
            attributes: HashMap::new(),
        })
    }
    
    pub fn inject_context_to_headers(context: &SpanContext) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("x-trace-id".to_string(), context.trace_id.clone());
        headers.insert("x-span-id".to_string(), context.span_id.clone());
        
        if let Some(parent) = &context.parent_span_id {
            headers.insert("x-parent-span-id".to_string(), parent.clone());
        }
        
        headers
    }
}

#[macro_export]
macro_rules! trace_span {
    ($tracing:expr, $operation:expr, $($key:expr => $value:expr),*) => {
        {
            let mut attrs = std::collections::HashMap::new();
            $(attrs.insert($key.to_string(), $value.to_string());)*
            $tracing.start_span($operation, None, attrs).await
        }
    };
}

pub struct DistributedTracer {
    tracing_service: Arc<TracingService>,
}

impl DistributedTracer {
    pub fn new(tracing_service: Arc<TracingService>) -> Self {
        Self { tracing_service }
    }
    
    pub async fn trace_request<F, Fut, R>(
        &self,
        operation: &str,
        request_id: &str,
        f: F,
    ) -> R
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let mut attrs = HashMap::new();
        attrs.insert("request_id".to_string(), request_id.to_string());
        
        let span = self.tracing_service.start_span(operation, None, attrs).await;
        
        let result = f().await;
        
        self.tracing_service.end_span(&span.span_id).await;
        
        result
    }
    
    pub async fn trace_db_query<F, Fut, R>(
        &self,
        query: &str,
        table: &str,
        f: F,
    ) -> R
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let mut attrs = HashMap::new();
        attrs.insert("db.query".to_string(), query.to_string());
        attrs.insert("db.table".to_string(), table.to_string());
        attrs.insert("db.system".to_string(), "scylladb".to_string());
        
        let span = self.tracing_service.start_span("db.query", None, attrs).await;
        
        let result = f().await;
        
        let duration = self.tracing_service.end_span(&span.span_id).await;
        if let Some(d) = duration {
            tracing::debug!(query = %query, duration_ms = d, "Database query completed");
        }
        
        result
    }
    
    pub async fn trace_ml_inference<F, Fut, R>(
        &self,
        model: &str,
        batch_size: usize,
        f: F,
    ) -> R
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let mut attrs = HashMap::new();
        attrs.insert("ml.model".to_string(), model.to_string());
        attrs.insert("ml.batch_size".to_string(), batch_size.to_string());
        attrs.insert("ml.framework".to_string(), "candle".to_string());
        
        let span = self.tracing_service.start_span("ml.inference", None, attrs).await;
        
        let result = f().await;
        
        let duration = self.tracing_service.end_span(&span.span_id).await;
        if let Some(d) = duration {
            tracing::info!(model = %model, duration_ms = d, "ML inference completed");
        }
        
        result
    }
}

pub fn init_jaeger_tracing(service_name: impl Into<String>) {
    let service_name = service_name.into();
    let _ = tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .json()
        .try_init();
    
    tracing::info!(service = %service_name, "Tracing initialized");
}
