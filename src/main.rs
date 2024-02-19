extern crate clap;

use anyhow::{Error, Result};
use clap::{arg, command, value_parser};
use config::Config;
use gpt4::{Llm, LlmOutputFormat};
use serde::Deserialize;
use std::env;
use tokio::main;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    openai_api_key: String,
}

#[main]
async fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                -f --format <FORMAT> "Sets the output format"
            )
            .required(false)
            .value_parser(value_parser!(LlmOutputFormat)),
        )
        .arg(
            arg!(
                [PROMPT]... "The prompt the for llm"
            )
            .required(true),
        )
        .get_matches();

    let prompt = matches
        .get_one::<String>("PROMPT")
        .ok_or(Error::msg("prompt expected"))?;

    let format = matches
        .get_one::<LlmOutputFormat>("format")
        .unwrap_or(&LlmOutputFormat::Markdown);

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

    let model_name = "gpt-4-1106-preview";
    let llm = Llm {
        token: &openai_api_key,
        format,
        model_name,
    };
    let output = llm.call(prompt).await?;

    termimad::print_text(&output);

    Ok(())
}
