# Rust Mistral Client (WIP)

## Usage
Make sure to set the `MISTRAL_API_KEY` environment variable.

```rust
use mistral_api::{ChatCompletion, Query};
use reqwest::Client;

async fn main () {
    let client = Client::new();
    
    let mut completion = ChatCompletion::builder()
        .build("mistral-tiny");
        
    completion.append_message(
        Message{
            role: MessageRole::User,
            content: "Say Hello!".to_string(),
        }
    );
    
    let rsp = completion.query(&client).await.unwrap();
    println!("{:?}", rsp);
}
```