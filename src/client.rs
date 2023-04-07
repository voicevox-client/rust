use crate::{audio_query::AudioQuery, restapi::RestAPI, types::audio_query::AudioQueryType};
use reqwest::Result;

pub struct Client {
    restapi: RestAPI,
}

impl Client {
    pub fn new(base_path: String) -> Self {
        Self {
            restapi: RestAPI::new(base_path),
        }
    }

    pub async fn create_audio_query(
        &self,
        text: &str,
        core_version: Option<&str>,
    ) -> Result<AudioQuery> {
        let data: AudioQueryType = self.restapi.create_audio_query(text, core_version).await?;
        let audio_query = AudioQuery::new(self.restapi.clone(), data);
        Ok(audio_query)
    }
}
