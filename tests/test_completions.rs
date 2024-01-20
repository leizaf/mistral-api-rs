use mistral_api::{ChatCompletion, Message, MessageRole, Query};
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_chat_completion() {
    dotenvy::dotenv().expect("Failed to read .env file");

    // Example from https://docs.mistral.ai/api/
    let value = json!({
        "model": "mistral-tiny",
        "messages": [
          {
            "role": "user",
            "content": "What is the best French cheese?"
          }
        ],
        "temperature": 0.7,
        "top_p": 1,
        "max_tokens": 16,
        "stream": false,
        "safe_prompt": false,
        "random_seed": null
    });

    let completion: ChatCompletion = serde_json::from_value(value).unwrap();
    let client = Client::new();
    let rsp = completion.query(&client).await.unwrap();
}

#[tokio::test]
async fn test_chat_completion2() {
    dotenvy::dotenv().expect("Failed to read .env file");
    let mut completion = ChatCompletion::builder()
        .build("mistral-tiny");
    completion.append_message(
        Message{
            role: MessageRole::User,
            content: "Say Hello!".to_string(),
        }
    );
    let client = Client::new();
    let rsp = completion.query(&client).await.unwrap();
}