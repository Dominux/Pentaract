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
    templates::storage_workers::{
        create_form::StorageWorkersCreateFormTemplate, index::StorageWorkersListTemplate,
    },
};

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(Self::index))
            .route("/create", get(Self::get_create_form))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn index() -> impl IntoResponse {
        Html(StorageWorkersListTemplate::new(vec![]).render().unwrap())
    }

    async fn get_create_form() -> impl IntoResponse {
        Html(
            StorageWorkersCreateFormTemplate::default()
                .render()
                .unwrap(),
        )
    }
}
