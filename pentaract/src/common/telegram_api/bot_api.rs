use crate::errors::PentaractResult;

use super::schemas::{InUploadSchema, UploadBodySchema, UploadSchema};

pub struct TelegramBotApi<'t> {
    base_url: &'t str,
    token: &'t str,
}

impl<'t> TelegramBotApi<'t> {
    pub fn new(base_url: &'t str, token: &'t str) -> Self {
        Self { base_url, token }
    }

    pub async fn upload(&self, in_schema: &'t InUploadSchema<'t>) -> PentaractResult<UploadSchema> {
        let url = self.build_url("", "sendDocument");

        let body: UploadBodySchema = reqwest::Client::new()
            .post(url)
            .form(in_schema)
            .send()
            .await?
            .json()
            .await?;
        Ok(body.result.document)
    }

    pub async fn download(&self, file_id: i64) -> PentaractResult<()> {
        // getting file path
        let url = self.build_url("", "getFile");

        // downloading the file itself
        let url = self.build_url("/file", "");
        todo!()
    }

    // https://stackoverflow.com/a/32679930/12255756

    #[inline]
    fn build_url(&self, pre: &str, relative: &str) -> String {
        format!("{pre}/{}{}/{relative}", self.base_url, self.token)
    }
}
