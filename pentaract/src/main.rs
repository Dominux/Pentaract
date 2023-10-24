use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use common::channels::StorageManagerSender;
use sqlx::postgres::PgPoolOptions;
use tokio::{sync::mpsc, time};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::common::routing::app_state::AppState;
use crate::config::Config;
use crate::server::Server;

mod common;
mod config;
mod errors;
mod routers;
mod server;
mod templates;
mod repositories;
mod models;

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

    let (tx, mut rx) = mpsc::channel::<StorageManagerSender>(config.channel_capacity.into());

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

    let server = {
        let workers = config.workers;
        let app_state = AppState::new(db, config);
        let shared_state = Arc::new(app_state);
        Server::build_server(workers.into(), shared_state, tx)
    };

    server.run(&addr).await
}
