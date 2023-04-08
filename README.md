# Rust

## Example
```rust
use voicevox_client::Client;


#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:50021".to_string());
    let audio_query = client.create_audio_query("hello", 1)
        .await
        .unwrap();
    let bytes = audio_query.synthesis(1)
        .await
        .unwrap();
}
```