use reqwest::{Client, RequestBuilder, Result};
use crate::types::audio_query::AudioQueryType;
use bytes::Bytes;

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
        self.client
            .request(method.parse().unwrap(), &format!("{}{}", self.base_path, path))
    }

    pub async fn create_audio_query(&self, text: &str, core_version: Option<&str>) -> Result<AudioQueryType> {
        let mut params = vec![("text", text)];
        if let Some(core_version) = core_version {
            params.push(("core_version", core_version))
        }
        self.request("POST", "/audio_query")
            .param(&params)
            .send()
            .await?
            .json()
            .await?
    }

    pub async fn synthesis(&self, audio_query: &AudioQueryType) -> Result<Bytes> {
        self.request("POST", "/synthesis")
            .json(audio_query)
            .send()
            .await?
            .bytes()
            .await?
    }
}