use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Extension,
    Json, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    schemas::storages::{InStorageSchema, StoragesListSchema},
    services::storages::StoragesService,
};

use super::files::FilesRouter;

pub struct StoragesRouter;

impl StoragesRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        let files_router = FilesRouter::get_router(state.clone());
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .nest("/:storage_id/files", files_router)
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn create(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Json(in_schema): Json<InStorageSchema>,
    ) -> impl IntoResponse {
        let storage = StoragesService::new(&state.db)
            .create(in_schema, &user)
            .await?;
        Ok::<_, (StatusCode, String)>((StatusCode::CREATED, Json(storage)))
    }

    async fn list(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let storages = StoragesService::new(&state.db)
            .list(&user)
            .await
            .map(|s| StoragesListSchema::new(s))?;
        Ok::<_, (StatusCode, String)>((StatusCode::OK, Json(storages)))
    }
}
