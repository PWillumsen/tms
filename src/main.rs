use clap::Parser;
use cli::{Commands, TmsArgs};

mod cli;
mod config;
mod tmux;

fn main() -> anyhow::Result<()> {
    let args = TmsArgs::parse();

    let _res = match args.command {
        Some(Commands::Kill { session }) => tmux::kill_session(session)?,
        Some(Commands::List) => tmux::list_sessions()?,
        Some(Commands::Config(command)) => config::update_config(command)?,
        Some(Commands::Completions { shell }) => cli::generate_completions(&shell)?,
        None => tmux::invoke_tms()?,
    };

    Ok(())
}
