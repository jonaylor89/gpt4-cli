use clap::ValueEnum;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, ValueEnum, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum LlmOutputFormat {
    Plaintext,

    #[default]
    Markdown,

    Html,
}

#[derive(Debug)]
pub struct Llm<'llm> {
    pub token: &'llm str,

    pub format: &'llm LlmOutputFormat,

    pub model_name: &'llm str,
}

impl Llm<'_> {
    pub async fn call(&self, prompt: &str) -> Result<String, reqwest::Error> {
        let url = "https://api.openai.com/v1/chat/completions";

        let system_content = match self.format {
            LlmOutputFormat::Plaintext => "You are a helpful assistant",
            LlmOutputFormat::Markdown => {
                "You are a helpful assistant, keep your answers in markdown"
            }
            LlmOutputFormat::Html => "You are a helpful assistant, keep your answers as html",
        };

        let data = json!({
            "model": self.model_name,
            "messages": [
                {
                    "role": "system",
                    "content": system_content,
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
        });

        let client = reqwest::Client::new();
        let body = client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.token)
            .body(data.to_string())
            .send()
            .await?
            .json::<OpenAiResponse>()
            .await?;

        if cfg!(debug_assertions) {
            println!("Prompt: {}", prompt);
            dbg!("Response: {:?}", &body);
        }

        let response = body.choices[0].message.content.clone();

        Ok(response)
    }
}
