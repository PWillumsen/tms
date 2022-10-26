use clap::Parser;
use cli::{Commands, TmsArgs};

mod cli;
mod config;
mod repos;
mod select;
mod tmux;

fn main() -> anyhow::Result<()> {
    let args = TmsArgs::parse();
    let config = config::load_config()?;

    match args.command {
        Some(Commands::Kill {
            session,
            interactive,
        }) => tmux::kill_session(session, interactive, config.default_session),
        Some(Commands::List) => tmux::list_sessions(),
        Some(Commands::Config(command)) => config::update_config(command),
        Some(Commands::Completions { shell }) => cli::generate_completions(&shell),
        None => tmux::invoke_tms(config.paths, config.exclude),
    }?;

    Ok(())
}
