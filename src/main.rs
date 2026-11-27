mod command_line_args;
mod config;
mod my_usage;
mod tools;

use std::time::Instant;

use colored::Colorize;

use command_line_args::CommandLineArgs;
use config::Config;
use rig::{AgentBuilder, providers::ollama::Ollama};

use crate::{
    my_usage::MyUsage,
    tools::{CargoClippy, ReadFile, WriteFile},
};

#[tokio::main]
async fn run_main() -> Result<(), String> {
    let args = CommandLineArgs::init();
    let config = Config::from_file(format!(
        "{}/local-agent/default.toml",
        dirs::config_dir().expect("Unsupported OS").display()
    ))?;
    log::info!("Using model: {}", config.model);
    log::info!("Send prompt: {}", args.prompt);
    run_agent(&args, &config).await
}

async fn run_agent(args: &CommandLineArgs, config: &Config) -> Result<(), String> {
    let client = Ollama::new();
    let preamble = config.preamble.clone().unwrap_or_default();
    let agent = AgentBuilder::new(client.native_completion(config.model.as_ref()))
        .preamble(preamble)
        .tool(ReadFile)
        //        .tool(CargoClippy)
        .tool(WriteFile)
        .default_max_turns(config.max_turns)
        .max_tokens(config.max_tokens)
        .build();

    let start = Instant::now();
    let answer = agent
        .prompt(&args.prompt)
        .await
        .map_err(|error| format!("{error}"))?;
    let elapsed = start.elapsed().as_secs_f64();

    if let Some(tokens) = answer.usage.output_tokens {
        if elapsed > 0.0 {
            let tokens_per_second = tokens as f64 / elapsed;
            log::info!(
                "{tokens} output tokens in {elapsed:.2}s ({tokens_per_second:.2} tokens/s)",
                tokens = tokens,
                elapsed = elapsed,
                tokens_per_second = tokens_per_second
            );
        }
    } else {
        log::error!("Output-token usage wasn't reported by the provider.");
    }

    log::info!("tool calls: {}", answer.output_tool_calls());
    log::info!("model turns: {}", answer.requests());
    let usage = MyUsage::from(&answer.usage());

    println!("{}", usage.to_string().truecolor(128, 128, 128));
    println!("{}", answer.output().truecolor(51, 255, 0));

    Ok(())
}

fn main() {
    if let Err(error) = run_main() {
        log::error!("{error}");
        std::process::exit(1);
    }
}
