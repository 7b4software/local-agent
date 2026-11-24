mod command_line_args;
mod config;
mod tools;
use command_line_args::CommandLineArgs;
use config::Config;
use rig::{AgentBuilder, providers::ollama::Ollama};

use crate::tools::{CargoClippy, ReadFile, WriteFile};

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
    let agent = AgentBuilder::new(client.completion(config.model.as_ref()))
        .preamble(preamble)
        .tool(ReadFile)
        .tool(CargoClippy)
        .tool(WriteFile)
        .default_max_turns(config.max_turns)
        .max_tokens(config.max_tokens)
        .build();

    let answer = agent
        .prompt(&args.prompt)
        .await
        .map_err(|error| format!("{error}"))?;
    println!("{}", answer.output());
    Ok(())
}

fn main() {
    if let Err(error) = run_main() {
        log::error!("{error}");
        std::process::exit(1);
    }
}
