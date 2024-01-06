use std::env;
use dotenvy::dotenv;
use gpt4::llm;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: gpt4 <prompt>");
        return Ok(());
    }

    let prompt = &args[1..].join(" ");
    let output = llm(prompt, &openai_api_key).await?;
    println!("{}", output);

    Ok(())
}
