use crate::types::audio_query::AudioQueryType;
use bytes::Bytes;
use reqwest::{Client, RequestBuilder, Result};

#[derive(Clone)]
pub struct RestAPI {
    base_path: String,
    client: Client,
}

impl RestAPI {
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            client: Client::new(),
        }
    }

    pub fn request(&self, method: &str, path: &str) -> RequestBuilder {
        self.client.request(
            method.parse().unwrap(),
            format!("{}{}", self.base_path, path),
        )
    }

    pub async fn create_audio_query(
        &self, speaker: i32,
        text: &str,
        core_version: Option<&str>,
    ) -> Result<AudioQueryType> {
        let mut params = vec![("text", text), ("speaker", speaker)];
        if let Some(core_version) = core_version {
            params.push(("core_version", core_version))
        }
        let data: AudioQueryType = self
            .request("POST", "/audio_query")
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn synthesis(&self, audio_query: &AudioQueryType, speaker: i32) -> Result<Bytes> {
        let data = self
            .request("POST", "/synthesis")
            .json(audio_query)
            .query(&[("speaker", speaker)])
            .send()
            .await?
            .bytes()
            .await?;
        Ok(data)
    }
}
