use clap::{ArgAction, Parser};
use log::Level;

#[derive(Parser)]
#[clap(version)]
pub struct CommandLineArgs {
    /// Verbosity level
    /// 0: Error, Warn
    /// 1: Info
    /// 2: Debug
    /// >= 3: Trace
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
    /// prompt to send to the model
    pub prompt: String,
}

impl CommandLineArgs {
    pub fn init() -> Self {
        let args = CommandLineArgs::parse();
        let level = match args.verbose {
            0 => Level::Warn,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        };
        _ = simple_logger::init_with_level(level);
        args
    }
}
