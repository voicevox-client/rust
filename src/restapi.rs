use crate::types::audio_query::AudioQueryType;
use bytes::Bytes;
use reqwest::{Client, RequestBuilder, Result, header::HeaderMap};

#[derive(Clone)]
pub struct RestAPI {
    base_path: String,
    client: Client,
}

impl RestAPI {
    pub fn new(base_path: String, headers: Option<HeaderMap>) -> Self {
        let mut client_builder = Client::builder();
        if let Some(headers) = headers {
            client_builder = client_builder.default_headers(headers);
        }
        let client = client_builder.build().unwrap();
        Self {
            base_path,
            client,
        }
    }

    pub fn request(&self, method: &str, path: &str) -> RequestBuilder {
        self.client.request(
            method.parse().unwrap(),
            format!("{}{}", self.base_path, path),
        )
    }

    pub async fn create_audio_query(
        &self,
        text: &str,
        speaker: &str,
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

    pub async fn synthesis(&self, audio_query: &AudioQueryType, speaker: &str) -> Result<Bytes> {
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
