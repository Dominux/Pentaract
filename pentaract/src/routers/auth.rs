use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Form, Router,
};

use crate::{
    common::{constants::ACCESS_TOKEN_NAME, routing::app_state::AppState},
    schemas::auth::LoginSchema,
    services::auth::AuthService,
    templates::login::LoginTemplate,
};

pub struct AuthRouter;

impl AuthRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/login", get(Self::get_login_page).post(Self::login))
            .with_state(state)
    }

    async fn get_login_page() -> impl IntoResponse {
        Html(LoginTemplate::new().render().unwrap())
    }

    async fn login(
        State(state): State<Arc<AppState>>,
        Form(login_data): Form<LoginSchema>,
    ) -> impl IntoResponse {
        let (token, expire_in) = {
            let login_result = AuthService::new(&state.db)
                .login(login_data, &state.config)
                .await;

            match login_result {
                Ok(o) => o,
                Err(_) => {
                    // TODO: return login page with error message displayed via some sort of a toast
                    return Redirect::to("/auth/login").into_response();
                }
            }
        };

        // setting token in a cookie
        let headers = {
            let mut headers = HeaderMap::with_capacity(1);
            let max_age = expire_in.as_secs();
            let cookie_header = format!(
                "{ACCESS_TOKEN_NAME}={token}; Path=/; HttpOnly; SameSite=Strict; Max-Age={max_age}"
            );
            headers.insert("Set-Cookie", cookie_header.parse().unwrap());
            headers
        };

        // redirecting to home page
        (headers, Redirect::to("/")).into_response()
    }
}
