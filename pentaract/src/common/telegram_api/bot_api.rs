use reqwest::multipart;

use crate::{common::types::ChatId, errors::PentaractResult};

use super::schemas::{UploadBodySchema, UploadSchema};

pub struct TelegramBotApi<'t> {
    base_url: &'t str,
    token: &'t str,
}

impl<'t> TelegramBotApi<'t> {
    pub fn new(base_url: &'t str, token: &'t str) -> Self {
        Self { base_url, token }
    }

    pub async fn upload(&self, file: &[u8], chat_id: ChatId) -> PentaractResult<UploadSchema> {
        // inserting 100 between minus sign and chat id
        // cause telegram devs are complete retards and it works this way only
        //
        // https://stackoverflow.com/a/65965402/12255756
        let chat_id = {
            let n = chat_id.abs().checked_ilog10().unwrap_or(0) + 1;
            chat_id - (100 * ChatId::from(10).pow(n))
        };

        let url = self.build_url("bot", "sendDocument");

        let file_part = multipart::Part::bytes(file.to_vec()).file_name("who_cares.bin");
        let form = multipart::Form::new()
            .text("chat_id", chat_id.to_string())
            .part("document", file_part);
        let body: UploadBodySchema = reqwest::Client::new()
            .post(url)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;
        Ok(body.result.document)
    }

    pub async fn download(&self, file_id: i64) -> PentaractResult<()> {
        // getting file path
        let url = self.build_url("bot", "getFile");

        // downloading the file itself
        let url = self.build_url("file/bot", "");
        todo!()
    }

    // https://stackoverflow.com/a/32679930/12255756

    #[inline]
    fn build_url(&self, pre: &str, relative: &str) -> String {
        format!("{}/{pre}{}/{relative}", self.base_url, self.token)
    }
}
