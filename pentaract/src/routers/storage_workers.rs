use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    schemas::storage_workers::{
        HasStorageWorkers, InStorageWorkerSchema, StorageWorkersStorageIDQuery,
    },
    services::storage_workers::StorageWorkersService,
};

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/has_workers", get(Self::has_storages_workers))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn create(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Json(in_schema): Json<InStorageWorkerSchema>,
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

    pub async fn has_storages_workers(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        query: Query<StorageWorkersStorageIDQuery>,
    ) -> Result<Response, (StatusCode, String)> {
        let has = StorageWorkersService::new(&state.db)
            .has_storage_workers(query.0.storage_id, &user)
            .await?;
        Ok(Json(HasStorageWorkers { has }).into_response())
    }
}
