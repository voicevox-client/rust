use crate::{audio_query::AudioQuery, restapi::RestAPI, types::audio_query::AudioQueryType};
use reqwest::{header::HeaderMap, Result};

pub struct Client {
    restapi: RestAPI,
}

impl Client {
    pub fn new(base_path: String, headers: Option<HeaderMap>) -> Self {
        Self {
            restapi: RestAPI::new(base_path, headers),
        }
    }

    pub async fn create_audio_query(
        &self,
        text: &str,
        speaker: i32,
        core_version: Option<&str>,
    ) -> Result<AudioQuery> {
        let data: AudioQueryType = self
            .restapi
            .create_audio_query(text, speaker.to_string().as_str(), core_version)
            .await?;
        let audio_query = AudioQuery::new(self.restapi.clone(), data);
        Ok(audio_query)
    }
}
