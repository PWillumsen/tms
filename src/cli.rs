use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::generate;
use clap_complete::Shell;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command( version, about, long_about = None)]
pub struct TmsArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Config options
    #[clap(arg_required_else_help = true)]
    Config(ConfigArgs),
    /// Kill session
    Kill {
        #[clap(short, long, exclusive = true)]
        interactive: bool,
    },
    /// List all running tmux sessions
    #[clap(visible_alias = "ls")]
    List,
    /// Generate shell completions
    Completions {
        #[clap(value_enum)]
        shell: Shell,
    },
    //TODO: Add interactive switch command
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// The default session to switch to (if avaliable) when killing another session
    #[clap(short, long)]
    pub default_session: Option<String>,

    /// Open config in $EDITOR
    #[clap(long, exclusive = true)]
    pub edit: bool,

    /// Sub directories to exclude from paths
    #[clap(short, long, value_parser)]
    pub exclude: Option<Vec<PathBuf>>,

    /// Remove directories to exclude list
    #[clap(long)]
    pub exclude_remove: Option<Vec<PathBuf>>,

    /// Display full paths
    #[clap(short, long)]
    pub full_paths: Option<bool>,

    /// The paths to search through. Paths must be full paths (no support for ~)
    #[clap(short, long)]
    pub paths: Option<Vec<PathBuf>>,

    /// Remove previously added paths
    #[clap(short, long)]
    pub paths_remove: Option<Vec<PathBuf>>,

    //TODO: fix how config is displayed, impl display
    /// Print config
    #[clap(short, long)]
    pub show: bool,
}

pub fn generate_completions(shell: &Shell) -> anyhow::Result<()> {
    generate(*shell, &mut TmsArgs::command(), "tms", &mut io::stdout());
    Ok(())
}
