use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, middleware, response::IntoResponse, routing::get, Extension,
    Form, Json, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    schemas::storage_workers::InStorageWorkerSchema,
    services::storage_workers::StorageWorkersService,
};

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn create(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Form(in_schema): Form<InStorageWorkerSchema>,
    ) -> impl IntoResponse {
        let sw = StorageWorkersService::new(&state.db)
            .create(in_schema, &user)
            .await?;
        Ok::<_, (StatusCode, String)>((StatusCode::CREATED, Json(sw)))
    }

    async fn list(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let sws = StorageWorkersService::new(&state.db).list(&user).await?;
        Ok::<_, (StatusCode, String)>((StatusCode::OK, Json(sws)))
    }
}
