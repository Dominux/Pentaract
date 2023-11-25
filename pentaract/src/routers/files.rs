use std::{collections::HashMap, path::Path, sync::Arc};

use askama::Template;
use axum::{
    body::Full,
    extract::{DefaultBodyLimit, Multipart, Path as RoutePath, Query, State},
    http::{HeaderMap, StatusCode},
    middleware,
    response::{AppendHeaders, Html, IntoResponse, Response},
    routing::{get, post},
    Extension, Router,
};
use reqwest::header;
use tokio_util::bytes::Bytes;
use uuid::Uuid;

use crate::{
    common::{
        helpers::not_ok,
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    errors::{PentaractError, PentaractResult},
    models::{
        files::{FSElement, InFile},
        storages::Storage,
    },
    schemas::files::{InFileSchema, InFolderSchema, UploadParams, IN_FILE_SCHEMA_FIELDS_AMOUNT},
    services::{files::FilesService, storages::StoragesService},
    templates::{
        files::{list::FilesListTemplate, upload_to_form::UploadToFormTemplate},
        storages::id::StorageTemplate,
    },
};

const HX_PROMPT: &str = "HX-PROMPT";

pub struct FilesRouter;

impl FilesRouter {
    pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>, axum::body::Body> {
        Router::new()
            .route("/create_folder", post(Self::create_folder))
            .route("/upload", post(Self::upload))
            .route("/upload_to", post(Self::upload_to))
            .route("/upload_to_form", get(Self::get_upload_to_form))
            .route("/*path", get(Self::dynamic_get).delete(Self::delete))
            .layer(DefaultBodyLimit::disable())
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn dynamic_get(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        RoutePath((storage_id, path)): RoutePath<(Uuid, String)>,
    ) -> Result<Response, (StatusCode, String)> {
        let (root_path, path) = path.split_once("/").unwrap_or((&path, ""));
        match root_path {
            "tree" => Self::tree(state, user, storage_id, path).await,
            "download" => Self::download(state, user, storage_id, path).await,
            _ => Err((StatusCode::NOT_FOUND, "Not found".to_owned())),
        }
    }

    async fn tree(
        state: Arc<AppState>,
        user: AuthUser,
        storage_id: Uuid,
        path: &str,
    ) -> Result<Response, (StatusCode, String)> {
        let (storage, fs_layer) = Self::_list(state, user, storage_id, path)
            .await
            .map_err(|e| <(StatusCode, String)>::from(e))?;

        let res = Html(
            StorageTemplate::new(storage_id, path, &storage.name, fs_layer)
                .render()
                .unwrap(),
        )
        .into_response();
        Ok(res)
    }

    async fn _list(
        state: Arc<AppState>,
        user: AuthUser,
        storage_id: Uuid,
        path: &str,
    ) -> PentaractResult<(Storage, Vec<FSElement>)> {
        let storage = StoragesService::new(&state.db)
            .get(storage_id, &user)
            .await?;
        let fs_layer = {
            let mut fs_layer = FilesService::new(&state.db, state.tx.clone())
                .list_dir(storage_id, path)
                .await?;

            // inserting `back` option into UX
            if !path.is_empty() {
                let path = Path::new(path)
                    .parent()
                    .unwrap_or(Path::new(""))
                    .to_str()
                    .unwrap();

                fs_layer.insert(
                    0,
                    FSElement {
                        path: path.to_string(),
                        name: "..".to_string(),
                        is_file: false,
                    },
                );
            }

            fs_layer
        };

        Ok((storage, fs_layer))
    }

    async fn get_upload_to_form(RoutePath(storage_id): RoutePath<Uuid>) -> impl IntoResponse {
        UploadToFormTemplate::new(storage_id, None, None)
            .render()
            .unwrap()
    }

    async fn upload(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Query(params): Query<UploadParams>,
        RoutePath(storage_id): RoutePath<Uuid>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        // parsing
        let (filename, file) = match multipart.next_field().await.unwrap() {
            Some(field) => (
                field.file_name().unwrap_or("unnamed").to_owned(),
                field.bytes().await.unwrap(),
            ),
            None => {
                return (StatusCode::BAD_REQUEST, "file field is not presented").into_response()
            }
        };
        let path = match Self::construct_path(&params.path, &filename) {
            Ok(p) => p,
            Err(e) => return <(StatusCode, String)>::from(e).into_response(),
        };
        let size = file.len() as i64;
        let in_file = InFile::new(path, size, storage_id);

        // do all other stuff
        if let Err(e) = FilesService::new(&state.db, state.tx.clone())
            .upload_anyway(in_file, file, &user)
            .await
        {
            return <(StatusCode, String)>::from(e).into_response();
        };

        Self::render_list(state, user, storage_id, &params.path).await
    }

    async fn upload_to(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        RoutePath(storage_id): RoutePath<Uuid>,
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
                    UploadToFormTemplate::new(storage_id, not_ok(path), not_ok(file));
                return Html(form_with_errors.render().unwrap()).into_response();
            }

            // now we have ensured that values are cleared
            InFileSchema::new(storage_id, path.unwrap(), file.unwrap().clone())
        };
        let path = in_schema.path.clone();

        // do all other stuff
        if let Err(e) = FilesService::new(&state.db, state.tx.clone())
            .upload_to(in_schema, &user)
            .await
        {
            return match e {
                PentaractError::AlreadyExists(_) | PentaractError::InvalidPath => {
                    return Html(
                        UploadToFormTemplate::new(storage_id, Some(&e.to_string()), None)
                            .render()
                            .unwrap(),
                    )
                    .into_response()
                }
                _ => <(StatusCode, String)>::from(e).into_response(),
            };
        };

        Self::render_list(state, user, storage_id, &path).await
    }

    async fn create_folder(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Query(params): Query<UploadParams>,
        RoutePath(storage_id): RoutePath<Uuid>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let in_schema = match || -> PentaractResult<InFolderSchema> {
            // parsing folder name
            let folder_name = headers
                .get(HX_PROMPT)
                .ok_or(PentaractError::HeaderMissed(HX_PROMPT.to_owned()))?
                .to_str()
                .map_err(|_| {
                    PentaractError::HeaderIsInvalid(HX_PROMPT.to_owned(), "UTF-8 string".to_owned())
                })?
                .to_owned();

            let schema = InFolderSchema::new(storage_id, params.path, folder_name);
            Ok(schema)
        }() {
            Ok(s) => s,
            Err(e) => return <(StatusCode, String)>::from(e).into_response(),
        };

        let path = in_schema.parent_path.clone();

        // do all other stuff
        match FilesService::new(&state.db, state.tx.clone())
            .create_folder(in_schema, &user)
            .await
        {
            Ok(_) => Self::render_list(state, user, storage_id, &path).await,
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    #[inline]
    fn construct_path(path: &str, filename: &str) -> PentaractResult<String> {
        Path::new(path)
            .join(filename)
            .to_str()
            .ok_or(PentaractError::InvalidPath)
            .map(|p| p.to_string())
    }

    #[inline]
    async fn render_list(
        state: Arc<AppState>,
        user: AuthUser,
        storage_id: Uuid,
        path: &str,
    ) -> Response {
        let list = Self::_list(state, user, storage_id, path).await;
        match list {
            Ok((storage, fs_layer)) => Html(
                FilesListTemplate::new(storage.id, path, fs_layer)
                    .render()
                    .unwrap(),
            )
            .into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn download(
        state: Arc<AppState>,
        user: AuthUser,
        storage_id: Uuid,
        path: &str,
    ) -> Result<Response, (StatusCode, String)> {
        FilesService::new(&state.db, state.tx.clone())
            .download(path, storage_id, &user)
            .await
            .map(|data| {
                let filename = Path::new(&path)
                    .file_name()
                    .map(|name| name.to_str().unwrap_or_default())
                    .unwrap_or("unnamed.bin");
                let content_type = mime_guess::from_path(filename)
                    .first_or_octet_stream()
                    .to_string();
                let bytes = Bytes::from(data);
                let body = Full::new(bytes);

                let headers = AppendHeaders([
                    (header::CONTENT_TYPE, content_type),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{filename}\""),
                    ),
                ]);

                (headers, body).into_response()
            })
            .map_err(|e| <(StatusCode, String)>::from(e))
    }

    async fn delete(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        RoutePath((storage_id, path)): RoutePath<(Uuid, String)>,
    ) -> Result<Response, (StatusCode, String)> {
        FilesService::new(&state.db, state.tx.clone())
            .delete(&path, storage_id, &user)
            .await
            .map_err(|e| <(StatusCode, String)>::from(e))?;

        // since we deleted the path, we take a parent one
        let path = Path::new(&path)
            .parent()
            .map(|path| path.to_str().unwrap())
            .unwrap_or("");

        let mut response = Self::render_list(state, user, storage_id, &path).await;
        *response.status_mut() = StatusCode::NO_CONTENT;
        Ok(response)
    }
}
