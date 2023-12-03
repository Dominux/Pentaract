use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;

use crate::{
    common::routing::app_state::AppState,
    schemas::auth::{LoginSchema, TokenSchema},
    services::auth::AuthService,
};

pub struct AuthRouter;

impl AuthRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/login", post(Self::login))
            .with_state(state)
    }

    async fn login(
        State(state): State<Arc<AppState>>,
        Json(login_data): Json<LoginSchema>,
    ) -> impl IntoResponse {
        let (token, _expire_in) = AuthService::new(&state.db)
            .login(login_data, &state.config)
            .await?;

        let schema = TokenSchema::new(token);
        Ok::<_, (StatusCode, String)>((StatusCode::OK, Json(schema)))
    }
}
