use anyhow::Result;
use clap::{Parser, Subcommand};
use human_panic::setup_panic;
use pity_doctor::prelude::*;
use pity_lib::prelude::{parse_config, LoggingOpts, ParsedConfig, ConfigOptions, FoundConfig};
use pity_report::prelude::{report_root, ReportArgs};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, warn};

/// Pity the Fool
///
/// Pity is a tool to enable teams to manage local machine
/// checks. An example would be a team that supports other
/// engineers may want to verify that the engineer asking
/// for support's machine is setup correctly.
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(flatten)]
    logging: LoggingOpts,

    #[clap(flatten)]
    config: ConfigOptions,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run checks that will "checkup" your machine.
    Doctor(DoctorArgs),
    /// Generate a bug report based from a command that was ran
    Report(ReportArgs),
}

#[tokio::main]
async fn main() {
    setup_panic!();
    dotenv::dotenv().ok();
    let opts = Cli::parse();

    let _guard = opts.logging.configure_logging("root");
    let error_code = run_subcommand(opts).await;

    std::process::exit(error_code);
}

async fn run_subcommand(opts: Cli) -> i32 {
    let loaded_config = match opts.config.load_config() {
        Err(e) => {
            error!(target: "user", "Failed to load configuration: {}", e);
            return 2;
        }
        Ok(c) => c
    };

    match handle_commands(&loaded_config, &opts.command).await {
        Ok(_) => 0,
        Err(e) => {
            error!(target: "user", "Critical Error. {}", e);
            1
        }
    }
}

async fn handle_commands(found_config: &FoundConfig, command: &Command) -> Result<()> {
    match command {
        Command::Doctor(args) => doctor_root(found_config, args).await,
        Command::Report(args) => report_root(args).await,
    }
}
