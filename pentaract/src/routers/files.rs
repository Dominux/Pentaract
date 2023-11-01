use std::{collections::HashMap, sync::Arc};

use askama::Template;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Form, Router,
};
use uuid::Uuid;

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    schemas::files::{InFileSchema, InFileValidationSchema, IN_FILE_SCHEMA_FIELDS_AMOUNT},
};

pub struct FilesRouter;

impl FilesRouter {
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

    async fn upload(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        // parsing and validating schema
        let in_schema = {
            let mut body_parts = HashMap::with_capacity(IN_FILE_SCHEMA_FIELDS_AMOUNT);

            // parsing
            while let Some(field) = multipart.next_field().await.unwrap() {
                let name = field.name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                body_parts.insert(name, data);
            }

            // validating
            let path = body_parts
                .get("path")
                .map(|path| {
                    String::from_utf8(path.to_vec())
                        .map_err(|_| "Path cannot be parsed".to_string())
                })
                .unwrap_or(Err("Path is required".to_string()));

            let storage_id = body_parts
                .get("storage_id")
                .map(|storage_id| {
                    let storage_id = String::from_utf8(storage_id.to_vec())
                        .map_err(|_| "Storage id cannot be parsed".to_string())?;
                    Uuid::parse_str(&storage_id)
                        .map_err(|_| "Storage id cannot be parse".to_string())
                })
                .unwrap_or(Err("Storage id is required".to_string()));

            let file = body_parts.get("file").ok_or("File is required".to_string());

            if path.is_err() || storage_id.is_err() || file.is_err() {
                // returning form with errors
                let validation_schema = InFileValidationSchema {
                    path_err: not_ok(path),
                    storage_id_err: not_ok(storage_id),
                    file_err: not_ok(file),
                };
                todo!("return template of upload form with errors");
                return;
            }

            // now we have ensured that values are cleared
            InFileSchema {
                file: file.unwrap().clone(),
                path: path.unwrap(),
                storage_id: storage_id.unwrap(),
            }
        };
    }
}

fn not_ok<T, E>(res: Result<T, E>) -> Option<E> {
    match res {
        Err(e) => Some(e),
        _ => None,
    }
}
