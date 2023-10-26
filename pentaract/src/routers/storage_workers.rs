use std::sync::Arc;

use askama::Template;
use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    common::routing::{app_state::AppState, middlewares::auth::logged_in_required},
    templates::storage_workers::list::StorageWorkersListTemplate,
};

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(Self::list))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn list() -> impl IntoResponse {
        Html(
            StorageWorkersListTemplate::new(Some("Kirill"))
                .render()
                .unwrap(),
        )
    }
}
