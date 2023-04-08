# Rust

## Example
```rust
use voicevox_client::Client;
use reqwest::Result;
use std::{io::Write, fs::File};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("http://localhost:50021".to_string());
    let audio_query = client
        .create_audio_query("こんにちは", 1, None)
        .await?;
    let audio = audio_query.synthesis(1).await?;
    let mut file = File::create("examples/hello.wav").unwrap();
    file.write_all(&audio).unwrap();
    Ok(())
}
```