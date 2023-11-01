use serde::{Deserialize, Serialize};

use crate::common::types::ChatId;

#[derive(Serialize)]
pub struct InUploadSchema<'f> {
    file: &'f [u8],
    chat_id: ChatId,
}

impl<'f> InUploadSchema<'f> {
    pub fn new(file: &'f [u8], chat_id: ChatId) -> Self {
        // inserting 100 between minus sign and chat id
        // cause telegram devs are complete retards and it works this way only
        //
        // https://stackoverflow.com/a/65965402/12255756
        let chat_id = {
            let n = chat_id.abs().checked_ilog10().unwrap_or(0) + 1;
            chat_id - (100 * ChatId::from(10).pow(n))
        };

        Self { file, chat_id }
    }
}

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

pub struct DownloadSchema {}
