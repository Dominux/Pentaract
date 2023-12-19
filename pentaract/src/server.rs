use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{
    cors,
    services::{ServeDir, ServeFile},
};

use crate::{
    common::routing::app_state::AppState,
    routers::{
        auth::AuthRouter, storage_workers::StorageWorkersRouter, storages::StoragesRouter,
        users::UsersRouter,
    },
};

pub struct Server {
    router: Router,
}

impl Server {
    pub fn build_server(workers: usize, app_state: Arc<AppState>) -> Self {
        let serve_ui = ServeFile::new("ui/index.html");
        let serve_assets = ServeDir::new("ui/assets");

        let router = Router::new()
            .nest("/api", Self::build_api_router(workers, app_state))
            .nest_service("/assets", serve_assets)
            .fallback_service(serve_ui);

        Self { router }
    }

    #[inline]
    fn build_api_router(workers: usize, app_state: Arc<AppState>) -> Router {
        let app_cors = cors::CorsLayer::new()
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_origin(cors::Any);

        Router::new()
            .nest("/users", UsersRouter::get_router(app_state.clone()))
            .nest("/auth", AuthRouter::get_router(app_state.clone()))
            .nest("/storages", StoragesRouter::get_router(app_state.clone()))
            .nest(
                "/storage_workers",
                StorageWorkersRouter::get_router(app_state.clone()),
            )
            .layer(ConcurrencyLimitLayer::new(workers.into()))
            .layer(app_cors)
    }

    pub async fn run(self, addr: &SocketAddr) {
        tracing::info!("listening on http://{addr}");
        axum::Server::bind(addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap();
    }
}
