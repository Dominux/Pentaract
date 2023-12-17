use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use tokio::{sync::mpsc, time};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    common::{channels::ClientMessage, db::pool::get_pool, routing::app_state::AppState},
    config::Config,
    server::Server,
    startup::{create_db, create_superuser, init_db},
    storage_manager::StorageManager,
};

mod common;
mod config;
mod errors;
mod models;
mod repositories;
mod routers;
mod schemas;
mod server;
mod services;
mod startup;
mod storage_manager;

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

    let (tx, rx) = mpsc::channel::<ClientMessage>(config.channel_capacity.into());

    // creating db
    create_db(
        &config.db_uri_without_dbname,
        &config.db_name,
        config.workers.into(),
        time::Duration::from_secs(30),
    )
    .await;

    // set up connection pool
    let db = get_pool(
        &config.db_uri,
        config.workers.into(),
        time::Duration::from_secs(30),
    )
    .await;

    // initing db
    init_db(&db).await;

    // creating a superuser
    create_superuser(&db, &config).await;

    // running manager
    let config_copy = config.clone();
    tokio::spawn(async move {
        let db = get_pool(
            &config_copy.db_uri,
            config.workers.into(),
            time::Duration::from_secs(30),
        )
        .await;
        let mut manager = StorageManager::new(rx, db, config_copy);

        tracing::debug!("running manager");
        manager.run().await;
    });

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.port);

    let server = {
        let workers = config.workers;
        let app_state = AppState::new(db, config, tx);
        let shared_state = Arc::new(app_state);
        Server::build_server(workers.into(), shared_state)
    };

    server.run(&addr).await
}
