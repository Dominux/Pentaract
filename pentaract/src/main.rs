use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "pentaract=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it
    tracing::debug!("listening on {ADDR}");
    axum::Server::bind(&ADDR)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
