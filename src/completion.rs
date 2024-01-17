use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    role: MessageRole,
    content: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatCompletion {
    /// ID of the model to use. You can use the List Available Models API to see all of your available models, or see our Model overview for model descriptions.
    model: String,
    /// The prompt(s) to generate completions for, encoded as a list of dict with role and content. The first prompt role should be user or system.
    messages: Vec<Message>,
    /// What sampling temperature to use, between 0.0 and 1.0. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or top_p but not both.
    temperature: f32,
    /// Nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or temperature but not both.
    top_p: f32,
    /// The maximum number of tokens to generate in the completion.
    ///
    /// The token count of your prompt plus max_tokens cannot exceed the model's context length.
    max_tokens: Option<i32>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    stream: bool,
    /// Whether to inject a safety prompt before all conversations.
    safe_prompt: bool,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    random_seed: Option<i32>,
}

impl ChatCompletion {
    pub fn builder() -> ChatCompletionBuilder {
        ChatCompletionBuilder::default()
    }

    pub fn new(model: &str) -> ChatCompletion {
        ChatCompletionBuilder::default().build(model)
    }

    pub fn messages_mut(&mut self) -> &mut Vec<Message> {
        &mut self.messages
    }
}

pub struct ChatCompletionBuilder {
    temperature: f32,
    top_p: f32,
    max_tokens: Option<i32>,
    stream: bool,
    safe_prompt: bool,
    random_seed: Option<i32>,
}

impl ChatCompletionBuilder {
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn safe_prompt(mut self, safe_prompt: bool) -> Self {
        self.safe_prompt = safe_prompt;
        self
    }

    pub fn random_seed(mut self, random_seed: i32) -> Self {
        self.random_seed = Some(random_seed);
        self
    }

    pub fn build(&self, model: &str) -> ChatCompletion {
        ChatCompletion {
            model: model.to_string(),
            messages: vec![],
            temperature: self.temperature,
            top_p: self.top_p,
            max_tokens: self.max_tokens,
            stream: self.stream,
            safe_prompt: self.safe_prompt,
            random_seed: self.random_seed,
        }
    }
}

impl Default for ChatCompletionBuilder {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 1.0,
            max_tokens: None,
            stream: false,
            safe_prompt: false,
            random_seed: None,
        }
    }
}
