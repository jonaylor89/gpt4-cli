use std::env;
use config::Config;
use gpt4::llm;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    openai_api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_location = dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("gpt4-cli")
        .join("config.toml");
    let settings = Config::builder()
        .add_source(config::File::from(config_location))
        .build()
        .expect("Failed to build config file \n\n `$HOME/.config/gpt4-cli/config.toml` should be a valid toml file with a `openai_api_key` key");

    let cli_config = settings
        .try_deserialize::<ConfigFile>()
        .expect("Failed to parse config file \n\n `$HOME/.config/gpt4-cli/config.toml` should be a valid toml file with a `openai_api_key` key");

    let openai_api_key = cli_config.openai_api_key;

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
