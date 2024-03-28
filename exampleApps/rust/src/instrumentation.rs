use opentelemetry::{global, KeyValue, trace::TraceError};
use opentelemetry_sdk::{runtime::Tokio, trace as sdktrace, Resource};
use opentelemetry_otlp::WithExportConfig;
use std::env;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt, layer::SubscriberExt, Registry};
use tracing_opentelemetry::OpenTelemetryLayer;


pub fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    let collector_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "none".to_string());

    println!("{}", collector_endpoint);

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint(&collector_endpoint),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "dice_server_rust"
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
}

pub fn setup_telemetry() -> Result<(), Box<dyn std::error::Error>> {
    let otlp_tracer = init_tracer().expect("failed to init tracer");
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(otlp_tracer);
    let subscriber = Registry::default()
        .with(telemetry_layer)
        .with(fmt::layer());

    set_global_default(subscriber)?;
    Ok(())
}