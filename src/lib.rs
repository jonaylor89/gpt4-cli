use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
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

pub async fn llm(prompt: &str, token: &str) -> Result<String, reqwest::Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let model_name = "gpt-4-1106-preview";

    let data = json!({
        "model": model_name,
        "messages": [
            {
                "role": "system",
                "content": "You are a helpful assistant, keep your answers in markdown"
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let client = reqwest::Client::new();
    let body = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .bearer_auth(token)
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
