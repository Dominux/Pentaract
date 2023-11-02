use std::{collections::HashMap, sync::Arc};

use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use uuid::Uuid;

use crate::{
    common::{helpers::not_ok, jwt_manager::AuthUser, routing::app_state::AppState},
    schemas::files::{InFileSchema, IN_FILE_SCHEMA_FIELDS_AMOUNT},
    services::storages::StoragesService,
    templates::{files::upload_form::UploadFormTemplate, storages::id::StorageTemplate},
};

pub struct FilesRouter;

impl FilesRouter {
    pub async fn index(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(storage_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match StoragesService::new(&state.db).get(storage_id, &user).await {
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
            Ok(storage) => Html(
                StorageTemplate::new(storage_id, &storage.name)
                    .render()
                    .unwrap(),
            )
            .into_response(),
        }
    }

    pub async fn get_upload_form(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(storage_id): Path<Uuid>,
    ) -> impl IntoResponse {
        UploadFormTemplate::new(storage_id, None, None)
            .render()
            .unwrap()
    }

    pub async fn upload(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path(storage_id): Path<Uuid>,
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
                .map(|path| String::from_utf8(path.to_vec()).map_err(|_| "Path cannot be parsed"))
                .unwrap_or(Err("Path is required"));

            let file = body_parts.get("file").ok_or("File is required");

            if path.is_err() || file.is_err() {
                // returning form with errors
                let form_with_errors =
                    UploadFormTemplate::new(storage_id, not_ok(path), not_ok(file));
                return Html(form_with_errors.render().unwrap()).into_response();
            }

            // now we have ensured that values are cleared
            InFileSchema {
                storage_id,
                file: file.unwrap().clone(),
                path: path.unwrap(),
            }
        };

        todo!()
    }
}
