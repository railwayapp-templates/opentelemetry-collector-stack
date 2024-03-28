use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use rand::Rng;
use std::{convert::Infallible, env, net::SocketAddr};
use tracing::instrument;

mod instrumentation;
use instrumentation::*;

#[tracing::instrument(
    name = "roll the dice",
)]
async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/rolldice") => {
            let random_number = rand::thread_rng().gen_range(1..7);
            *response.body_mut() = Body::from(random_number.to_string());
            tracing::info!("Rolled a dice with result: {}", random_number);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            tracing::error!("Path not found: {}", req.uri().path());
        }
    };

    Ok(response)
}


#[tokio::main]
async fn main() {
    setup_telemetry().expect("Failed to set up telemetry");

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