use std::sync::Arc;

use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::{Redirect, Response},
};

use crate::{
    common::{
        jwt_manager::{AuthUser, JWTManager},
        routing::app_state::AppState,
    },
    errors::{PentaractError, PentaractResult},
};

/// Middleware that requires to be loggen in
pub async fn logged_in_required<B>(
    State(state): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Redirect> {
    let auth_user = authenticate(req.headers(), &state.config.secret_key)
        .map_err(|_| Redirect::to("/auth/login"))?;

    req.extensions_mut().insert(auth_user);
    Ok(next.run(req).await)
}

#[inline]
fn authenticate(headers: &HeaderMap<HeaderValue>, secret_key: &str) -> PentaractResult<AuthUser> {
    let auth_header = headers
        .typed_get::<Authorization<Bearer>>()
        .ok_or(PentaractError::NotAuthenticated)?;

    JWTManager::validate(auth_header.token(), secret_key)
}
