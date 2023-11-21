use clap::{Parser};
use log::{info, warn, LevelFilter, error, debug};
use nohuman::CommandRunner;
use env_logger::Builder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Check that all required dependencies are available
    #[arg(short, long)]
    check: bool,

    /// Set the logging level to verbose
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Initialize logger
    let log_lvl = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let mut log_builder = Builder::new();
    log_builder
        .filter(None, log_lvl)
        .filter_module("reqwest", LevelFilter::Off)
        .format_module_path(false)
        .format_target(false)
        .init();

    let conda = CommandRunner::new("conda");

    let external_commands = vec![conda];

    let mut missing_commands = Vec::new();
    for cmd in external_commands {
        if !cmd.is_executable() {
            debug!("{} is not executable", cmd.command);
            missing_commands.push(cmd.command);
        } else {
            debug!("{} is executable", cmd.command);
        }
    }

    if args.check {
        if !missing_commands.is_empty() {
            error!("The following dependencies are missing:");
            for cmd in missing_commands {
                error!("{}", cmd);
            }
            std::process::exit(1);
        }
        info!("Dependencies are available");
        std::process::exit(0);
    }
}