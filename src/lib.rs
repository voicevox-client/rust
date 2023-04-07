mod audio_query;
mod client;
mod restapi;
mod types;

pub use client::Client;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = Client::new("http://localhost:50021");
        let audio_query = client.create_audio_query("こんにちは", 1)
            .await
            .unwrap();
        audio_query.synthesis(1)
            .await
            .unwrap();
    }
}
