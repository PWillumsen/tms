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
    default_session: Option<String>,

    /// Open config in $EDITOR
    #[arg(long)]
    edit: bool,

    #[arg(short, long)]
    exclude: Option<Vec<PathBuf>>,

    /// Display full paths
    #[arg(short, long)]
    full_paths: Option<bool>,

    /// The paths to search through. Paths must be full paths (no support for ~)
    #[arg(short, long)]
    paths: Option<Vec<PathBuf>>,

    #[arg(short, long)]
    remove: Option<Vec<PathBuf>>,

    /// Print config
    #[arg(short, long)]
    show: bool,
}

pub fn generate_completions(shell: &Shell) {
    generate(*shell, &mut TmsArgs::command(), "myapp", &mut io::stdout())
}
