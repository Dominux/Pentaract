use serde::Deserialize;

#[derive(Deserialize)]
pub struct UploadBodySchema {
    pub result: UploadResultSchema,
}

#[derive(Deserialize)]
pub struct UploadResultSchema {
    pub document: UploadSchema,
}

#[derive(Deserialize)]
pub struct UploadSchema {
    pub file_id: String,
}

#[derive(Deserialize)]
pub struct DownloadBodySchema {
    pub result: DownloadSchema,
}

#[derive(Deserialize)]
pub struct DownloadSchema {
    pub file_path: String,
}
