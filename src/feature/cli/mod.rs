use error_stack::Result;
use clap::{ Parser, Subcommand };

#[derive(Debug, thiserror::Error)]
#[error("a CLI error has occurred")]
pub struct CliError;

#[derive(Debug, Copy, Clone, Subcommand)]
enum Command {
    Start,
    Stop,
    Report,
}

#[derive(Debug, Copy, Clone, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

pub fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    match cli.command {
        Command::Start => {
            println!("Starting the tracker...");
            Ok(())
        }
        Command::Stop => {
            println!("Stopping the tracker...");
            Ok(())
        }
        Command::Report => {
            println!("Generating report...");
            Ok(())
        }
    }
}
