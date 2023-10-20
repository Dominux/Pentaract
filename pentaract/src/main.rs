use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::sync::{mpsc, oneshot};
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

    let (tx, mut rx) = mpsc::channel::<Responder<String>>(32);

    // running manager
    tokio::spawn(async move {
        tracing::debug!("manager ran");

        // Start receiving messages
        while let Some(resp_rx) = rx.recv().await {
            tracing::debug!("got task");

            let _ = resp_rx.send("lmao".to_string());
        }
    });

    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|| async move {
            let (resp_tx, resp_rx) = oneshot::channel();

            tracing::debug!("sending task to manager");
            let _ = tx.send(resp_tx).await;

            resp_rx.await.unwrap()
        }),
    );

    // run it
    tracing::debug!("listening on http://{ADDR}");
    axum::Server::bind(&ADDR)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = oneshot::Sender<T>;
