use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Form, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    errors::{PentaractError, PentaractResult},
    models::storages::Storage,
    schemas::storage_workers::InStorageWorkerSchema,
    services::{storage_workers::StorageWorkersService, storages::StoragesService},
    templates::storage_workers::{
        create_form::StorageWorkersCreateFormTemplate,
        index::{StorageWorkersIndexTemplate, StorageWorkersListTemplate},
    },
};

use super::auth::AuthRouter;

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(Self::index).post(Self::create))
            .route("/list", get(Self::list))
            .route("/create", get(Self::get_create_form))
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
        let list_result = StorageWorkersService::new(&state.db).list(&user).await;
        match list_result {
            Ok(sw) => Html(StorageWorkersIndexTemplate::new(sw).render().unwrap()).into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn get_create_form(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let service = StoragesService::new(&state.db);
        let storages = Self::_list_storages(service, &user).await;
        Html(
            StorageWorkersCreateFormTemplate::new(None, None, storages)
                .render()
                .unwrap(),
        )
    }

    async fn create(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Form(in_schema): Form<InStorageWorkerSchema>,
    ) -> impl IntoResponse {
        let service = StorageWorkersService::new(&state.db);
        let sw_creating_result = service.create(in_schema, &user).await;

        if let Err(e) = sw_creating_result {
            return match e {
                PentaractError::StorageWorkerNameConflict
                | PentaractError::StorageWorkerTokenConflict => {
                    let service = StoragesService::new(&state.db);
                    let storages = Self::_list_storages(service, &user).await;

                    let (name_err, token_err) = match e {
                        PentaractError::StorageWorkerNameConflict => {
                            (Some("This name is not unique"), None)
                        }
                        _ => (None, Some("This token is not unique")),
                    };
                    (
                        StatusCode::CONFLICT,
                        Html(
                            StorageWorkersCreateFormTemplate::new(name_err, token_err, storages)
                                .render()
                                .unwrap(),
                        ),
                    )
                        .into_response()
                }
                PentaractError::UserWasRemoved => {
                    AuthRouter::logout_for_htmx().await.into_response()
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
            };
        };

        match Self::_list(service, &user).await {
            Ok(page) => (StatusCode::CREATED, page).into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn list(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let service = StorageWorkersService::new(&state.db);
        match Self::_list(service, &user).await {
            Ok(page) => page.into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn _list<'a>(
        service: StorageWorkersService<'a>,
        user: &AuthUser,
    ) -> PentaractResult<Html<String>> {
        service
            .list(&user)
            .await
            .map(|sw| Html(StorageWorkersListTemplate::new(sw).render().unwrap()))
    }

    async fn _list_storages<'a>(service: StoragesService<'a>, user: &AuthUser) -> Vec<Storage> {
        service.list(&user).await.map_or(vec![], |v| v)
    }
}
