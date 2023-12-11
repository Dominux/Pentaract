use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;

use crate::{
    common::routing::app_state::AppState, schemas::users::InUser, services::users::UsersService,
};

pub struct UsersRouter;

impl UsersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", post(Self::register))
            .with_state(state)
    }

    async fn register(
        State(state): State<Arc<AppState>>,
        Json(in_user): Json<InUser>,
    ) -> impl IntoResponse {
        UsersService::new(&state.db).create(in_user).await?;
        Ok::<_, (StatusCode, String)>(StatusCode::OK)
    }
}
