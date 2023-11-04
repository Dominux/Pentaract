use std::{net::SocketAddr, sync::Arc};

use axum::{middleware, Router};
use tower::limit::ConcurrencyLimitLayer;

use crate::{
    common::routing::{app_state::AppState, middlewares::auth::logged_in_required},
    routers::{auth::AuthRouter, storage_workers::StorageWorkersRouter, storages::StoragesRouter},
};

pub struct Server {
    router: Router,
}

impl Server {
    pub fn build_server(workers: usize, app_state: Arc<AppState>) -> Self {
        let router = Router::new()
            .nest("/auth", AuthRouter::get_router(app_state.clone()))
            .nest("/storages", StoragesRouter::get_router(app_state.clone()))
            .nest(
                "/storage_workers",
                StorageWorkersRouter::get_router(app_state.clone()),
            )
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                logged_in_required,
            ))
            .layer(ConcurrencyLimitLayer::new(workers.into()));

        Self { router }
    }

    pub async fn run(self, addr: &SocketAddr) {
        tracing::debug!("listening on http://{addr}");
        axum::Server::bind(addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap();
    }
}
