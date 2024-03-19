// helpful docs:
// https://opentelemetry.io/docs/languages/rust/getting-started/
// https://github.com/open-telemetry/opentelemetry-rust/tree/main/opentelemetry-otlp/examples/basic-otlp-http

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use rand::Rng;
use std::{convert::Infallible, env, net::SocketAddr};
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{SpanKind, Status};
use opentelemetry::{global, trace::{TraceError, Tracer}, KeyValue};
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_stdout::SpanExporter;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    let tracer = global::tracer("dice_server_rust");

    let mut span = tracer
        .span_builder(format!("{} {}", req.method(), req.uri().path()))
        .with_kind(SpanKind::Server)
        .start(&tracer);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/rolldice") => {
            let random_number = rand::thread_rng().gen_range(1..7);
            *response.body_mut() = Body::from(random_number.to_string());
            span.set_status(Status::Ok);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            span.set_status(Status::error("Not Found"));
        }
    };

    Ok(response)
}

fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
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
                "dice_server_rust",
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
}


#[tokio::main]
async fn main() {
    init_tracer();

    // Read the PORT environment variable, defaulting to 8080 if not found
    let port: u16 = env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("Invalid PORT environment variable");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server =
        Server::bind(&addr).serve(make_svc);

    println!("Listening on {addr}");
    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}
