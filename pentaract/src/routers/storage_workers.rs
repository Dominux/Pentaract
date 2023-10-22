use std::sync::Arc;

use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    common::routing::app_state::AppState,
    templates::storage_workers::list::StorageWorkersListTemplate,
};

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new().route("/", get(Self::list)).with_state(state)
    }

    async fn list() -> impl IntoResponse {
        Html(
            StorageWorkersListTemplate::new(Some("Kirill"))
                .render()
                .unwrap(),
        )
    }
}
