use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::HeaderMap,
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Form, Router,
};

use crate::{
    common::{
        constants::ACCESS_TOKEN_NAME,
        routing::{app_state::AppState, middlewares::auth::logged_out_required},
    },
    schemas::auth::LoginSchema,
    services::auth::AuthService,
    templates::login::LoginTemplate,
};

pub struct AuthRouter;

impl AuthRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        let login_router = Router::new()
            .route("/", get(Self::get_login_page).post(Self::login))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_out_required,
            ))
            .with_state(state);
        let logout_router = Router::new().route("/", get(Self::logout));

        Router::new()
            .nest("/login", login_router)
            .nest("/logout", logout_router)
    }

    async fn get_login_page() -> impl IntoResponse {
        Html(LoginTemplate::default().render().unwrap())
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
                    return Html(
                        LoginTemplate::new(Some("Invalid credentials"))
                            .render()
                            .unwrap(),
                    )
                    .into_response();
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

    async fn logout() -> impl IntoResponse {
        // setting deleting token in a cookie
        let headers = {
            let mut headers = HeaderMap::with_capacity(1);
            let max_age = 0;
            let cookie_header = format!("{ACCESS_TOKEN_NAME}=deleted; Path=/; Max-Age={max_age}");
            headers.insert("Set-Cookie", cookie_header.parse().unwrap());
            headers
        };

        // redirecting to home page
        (headers, Redirect::to("/auth/login"))
    }
}
