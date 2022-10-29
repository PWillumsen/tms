use clap::Parser;
use cli::{generate_completions, Cli, Commands};
use color_eyre::eyre::Result;
use config::{load_config, update_config};
use tmux::{kill_session, list_sessions, run_tms};

mod cli;
mod config;
mod repos;
mod tmux;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let config = load_config()?;

    match args.command {
        Some(Commands::Kill { interactive }) => kill_session(interactive, config),
        Some(Commands::Completions { shell }) => generate_completions(&shell),
        Some(Commands::Config(command)) => update_config(command),
        Some(Commands::List) => list_sessions(),
        None => run_tms(config),
    }?;

    Ok(())
}
