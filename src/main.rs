use clap::Parser;
use cli::{Commands, TmsArgs};

mod cli;
mod config;
mod tmux;

fn main() {
    let args = TmsArgs::parse();

    match &args.command {
        Some(Commands::Kill { session }) => tmux::kill_session(session),
        Some(Commands::List) => tmux::list_sessions(),
        Some(Commands::Config(command)) => config::handle_config(command),
        Some(Commands::Completions { shell }) => cli::generate_completions(shell),
        None => tmux::invoke_tms(),
    }
}
