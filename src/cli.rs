use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::generate;
use clap_complete::Shell;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TmsArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Config(ConfigArgs),
    Kill {
        #[arg(short, long)]
        session: Option<String>,
    },
    List,
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// The default session to switch to (if avaliable) when killing another session
    #[arg(short, long)]
    pub default_session: Option<String>,

    /// Open config in $EDITOR
    #[arg(long)]
    pub edit: bool,

    // Sub directories to exclude from paths
    #[arg(short, long)]
    pub exclude: Option<Vec<PathBuf>>,

    // Remove directories to exclude list
    #[arg(long)]
    pub remove_exclude: Option<Vec<PathBuf>>,

    /// Display full paths
    #[arg(short, long)]
    pub full_paths: Option<bool>,

    /// The paths to search through. Paths must be full paths (no support for ~)
    #[arg(short, long)]
    pub paths: Option<Vec<PathBuf>>,

    #[arg(short, long)]
    pub remove: Option<Vec<PathBuf>>,

    /// Print config
    #[arg(short, long)]
    pub show: bool,
}

pub fn generate_completions(shell: &Shell) -> anyhow::Result<()> {
    generate(*shell, &mut TmsArgs::command(), "myapp", &mut io::stdout());
    Ok(())
}
