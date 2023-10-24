use std::sync::Arc;

use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{common::routing::app_state::AppState, templates::login::LoginTemplate};

pub struct AuthRouter;

impl AuthRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/login", get(Self::get_login_page))
            .with_state(state)
    }

    async fn get_login_page() -> impl IntoResponse {
        Html(LoginTemplate::new().render().unwrap())
    }
}
