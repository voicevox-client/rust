use crate::{restapi::RestAPI, types::audio_query::AudioQueryType};
use bytes::Bytes;
use reqwest::Result;

pub struct AudioQuery {
    restapi: RestAPI,
    audio_query: AudioQueryType,
}

impl AudioQuery {
    pub fn new(restapi: RestAPI, audio_query: AudioQueryType) -> Self {
        Self {
            restapi,
            audio_query,
        }
    }

    pub async fn synthesis(
        &self,
        speaker: i32,
        enable_interrogative_upspeak: bool,
    ) -> Result<Bytes> {
        let data = self
            .restapi
            .synthesis(
                &self.audio_query,
                speaker.to_string().as_str(),
                enable_interrogative_upspeak,
            )
            .await?;
        Ok(data)
    }
}
