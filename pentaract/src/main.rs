use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::{
    sync::{mpsc, oneshot},
    time,
};
use tower::limit::ConcurrencyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use errors::PentaractError;
use routing::app_state::AppState;

mod config;
mod errors;
mod routing;

#[tokio::main]
async fn main() {
    let config = Config::new().unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "pentaract=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, mut rx) = mpsc::channel::<Responder<String>>(config.channel_capacity.into());

    // set up connection pool
    let db = PgPoolOptions::new()
        .max_connections(config.workers.into())
        .acquire_timeout(time::Duration::from_secs(3))
        .connect(&config.db_uri)
        .await
        .expect("can't establish database connection");

    // running manager
    tokio::spawn(async move {
        tracing::debug!("manager ran");

        let mut counter = 0;

        // Start receiving messages
        while let Some(resp_rx) = rx.recv().await {
            counter += 1;

            tracing::debug!("stopped at {counter}");

            let _ = resp_rx.send(counter.to_string());
        }
    });

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.port);

    // build our application with a single route
    let app = {
        let workers = config.workers;
        let app_state = AppState::new(db, config);
        let shared_state = Arc::new(app_state);
        Router::new()
            .route(
                "/",
                get(|| async move {
                    let (resp_tx, resp_rx) = oneshot::channel();

                    tracing::debug!("started");
                    let _ = tx.send(resp_tx).await;

                    // simulating some io operations
                    time::sleep(time::Duration::from_secs(5)).await;

                    resp_rx.await.unwrap()
                }),
            )
            .layer(ConcurrencyLimitLayer::new(workers.into()))
            .with_state(shared_state)
    };

    // run it
    tracing::debug!("listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = oneshot::Sender<T>;
