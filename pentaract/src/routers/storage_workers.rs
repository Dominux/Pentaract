use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Extension, Form, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    errors::PentaractError,
    schemas::storage_workers::InStorageWorkerSchema,
    services::storage_workers::StorageWorkersService,
    templates::storage_workers::{
        create_form::StorageWorkersCreateFormTemplate,
        index::{StorageWorkersIndexTemplate, StorageWorkersListTemplate},
    },
};

pub struct StorageWorkersRouter;

impl StorageWorkersRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(Self::index).post(Self::create))
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

    async fn get_create_form() -> impl IntoResponse {
        Html(
            StorageWorkersCreateFormTemplate::default()
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
                PentaractError::StorageWorkerNameConflict => (
                    StatusCode::CONFLICT,
                    Html(
                        StorageWorkersCreateFormTemplate::new(Some("This name isn't unique"), None)
                            .render()
                            .unwrap(),
                    ),
                )
                    .into_response(),
                PentaractError::StorageWorkerTokenConflict => (
                    StatusCode::CONFLICT,
                    Html(
                        StorageWorkersCreateFormTemplate::new(
                            None,
                            Some("This token isn't unique"),
                        )
                        .render()
                        .unwrap(),
                    ),
                )
                    .into_response(),
                PentaractError::UserWasRemoved => Redirect::to("/auth/logout").into_response(),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
            };
        };

        let storage_workers = match service.list(&user).await {
            Ok(sw) => sw,
            Err(e) => return <(StatusCode, String)>::from(e).into_response(),
        };
        let page = Html(
            StorageWorkersListTemplate::new(storage_workers)
                .render()
                .unwrap(),
        );
        (StatusCode::CREATED, page).into_response()
    }
}
