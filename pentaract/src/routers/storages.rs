use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    errors::PentaractResult,
    schemas::storages::InStorageSchema,
    services::storages::StoragesService,
    templates::storages::{
        create_form::StoragesCreateFormTemplate,
        index::{StoragesIndexTemplate, StoragesListTemplate},
    },
};

use super::files::FilesRouter;

pub struct StoragesRouter;

impl StoragesRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        let files_router = FilesRouter::get_router(state.clone());
        Router::new()
            .route("/", get(Self::index).post(Self::create))
            .route("/list", get(Self::list))
            .route("/create", get(Self::get_create_form))
            .nest("/:storage_id/files", files_router)
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn index(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let list_result = StoragesService::new(&state.db).list(&user).await;
        match list_result {
            Ok(s) => Html(StoragesIndexTemplate::new(s).render().unwrap()).into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn get_create_form() -> impl IntoResponse {
        Html(StoragesCreateFormTemplate::default().render().unwrap())
    }

    async fn create(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Json(in_schema): Json<InStorageSchema>,
    ) -> impl IntoResponse {
        let storage = StoragesService::new(&state.db)
            .create(in_schema, &user)
            .await?;
        Ok::<_, (StatusCode, String)>((StatusCode::OK, Json(storage)))
    }

    async fn list(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let service = StoragesService::new(&state.db);
        match Self::_list(service, &user).await {
            Ok(page) => page.into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn _list<'a>(
        service: StoragesService<'a>,
        user: &AuthUser,
    ) -> PentaractResult<Html<String>> {
        service
            .list(&user)
            .await
            .map(|s| Html(StoragesListTemplate::new(s).render().unwrap()))
    }
}
