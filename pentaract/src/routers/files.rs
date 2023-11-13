use std::{collections::HashMap, sync::Arc};

use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension,
};
use uuid::Uuid;

use crate::{
    common::{helpers::not_ok, jwt_manager::AuthUser, routing::app_state::AppState},
    errors::{PentaractError, PentaractResult},
    schemas::files::{InFileSchema, IN_FILE_SCHEMA_FIELDS_AMOUNT},
    services::{files::FilesService, storages::StoragesService},
    templates::{files::upload_form::UploadFormTemplate, storages::id::StorageTemplate},
};

pub struct FilesRouter;

impl FilesRouter {
    pub async fn index(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Path((storage_id, path)): Path<(Uuid, String)>,
    ) -> impl IntoResponse {
        // dynamic path resolution
        let (root_path, path) = path.split_once("/").unwrap_or((&path, ""));
        if root_path != "files" {
            return (StatusCode::NOT_FOUND, "Not found").into_response();
        };

        match Self::list(state, user, storage_id, path).await {
            Ok(o) => o,
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn list(
        state: Arc<AppState>,
        user: AuthUser,
        storage_id: Uuid,
        path: &str,
    ) -> PentaractResult<Response> {
        let storage = StoragesService::new(&state.db)
            .get(storage_id, &user)
            .await?;
        let fs_layer = FilesService::new(&state.db, state.tx.clone())
            .list_dir(storage_id, path)
            .await?;

        let res = Html(
            StorageTemplate::new(storage_id, &storage.name, fs_layer)
                .render()
                .unwrap(),
        )
        .into_response();
        Ok(res)
    }

    pub async fn get_upload_form(Path(storage_id): Path<Uuid>) -> impl IntoResponse {
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

        // do all other stuff
        if let Err(e) = FilesService::new(&state.db, state.tx.clone())
            .upload(in_schema, &user)
            .await
        {
            return match e {
                PentaractError::AlreadyExists(_) | PentaractError::InvalidPath => {
                    return Html(
                        UploadFormTemplate::new(storage_id, Some(&e.to_string()), None)
                            .render()
                            .unwrap(),
                    )
                    .into_response()
                }
                _ => <(StatusCode, String)>::from(e).into_response(),
            };
        };

        (StatusCode::CREATED).into_response()
    }
}
