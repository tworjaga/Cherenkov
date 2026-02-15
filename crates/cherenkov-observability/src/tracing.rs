use opentelemetry::trace::TracerProvider;
use opentelemetry_jaeger::Config;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;

pub fn init_jaeger_tracing(service_name: &str) {
    let provider = Config::new()
        .with_service_name(service_name)
        .init_tracer()
        .expect("Failed to initialize Jaeger tracer");
    
    let tracer = provider.tracer(service_name);
    let telemetry = OpenTelemetryLayer::new(tracer);
    
    tracing_subscriber::registry().with(telemetry).init();
}

#[derive(Debug)]
pub struct SpanContext {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
}

impl SpanContext {
    pub fn new() -> Self {
        Self {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
        }
    }
    
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: Some(self.span_id.clone()),
        }
    }
}
