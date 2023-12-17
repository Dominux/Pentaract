use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use uuid::Uuid;

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    models::storages::Storage,
    schemas::{
        access::{GrantAccess, RestrictAccess},
        storages::{InStorageSchema, StoragesListSchema},
    },
    services::storages::StoragesService,
};

use super::files::FilesRouter;

pub struct StoragesRouter;

impl StoragesRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        let files_router = FilesRouter::get_router(state.clone());
        Router::new()
            .route("/", get(Self::list).post(Self::create))
            .route("/:storage_id", get(Self::get).delete(Self::delete))
            .route(
                "/:storage_id/access",
                get(Self::list_users_with_access)
                    .post(Self::grant_access)
                    .delete(Self::restrict_access),
            )
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
        Ok::<_, (StatusCode, String)>(Json(storages))
    }

    async fn get(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<Storage>, (StatusCode, String)> {
        let storage = StoragesService::new(&state.db).get(id, &user).await?;
        Ok(Json(storage))
    }

    async fn delete(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(id): Path<Uuid>,
    ) -> Result<StatusCode, (StatusCode, String)> {
        StoragesService::new(&state.db).delete(id, &user).await?;
        Ok(StatusCode::NO_CONTENT)
    }

    async fn grant_access(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(id): Path<Uuid>,
        Json(in_schema): Json<GrantAccess>,
    ) -> Result<StatusCode, (StatusCode, String)> {
        StoragesService::new(&state.db)
            .grant_access(id, in_schema, &user)
            .await?;
        Ok(StatusCode::NO_CONTENT)
    }

    async fn list_users_with_access(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(id): Path<Uuid>,
    ) -> impl IntoResponse {
        let users = StoragesService::new(&state.db)
            .list_users_with_access(id, &user)
            .await?;
        Ok::<_, (StatusCode, String)>(Json(users))
    }

    async fn restrict_access(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(id): Path<Uuid>,
        Json(in_schema): Json<RestrictAccess>,
    ) -> Result<StatusCode, (StatusCode, String)> {
        StoragesService::new(&state.db)
            .restrict_access(id, in_schema, &user)
            .await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
