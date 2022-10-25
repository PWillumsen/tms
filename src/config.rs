use crate::cli::ConfigArgs;
use serde::{Deserialize, Serialize};
use std::env::var;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    paths: Vec<PathBuf>,
    exclude: Vec<PathBuf>,
    default_session: Option<String>,
    full_path: Option<bool>,
}

pub fn update_config(config_args: ConfigArgs) -> anyhow::Result<()> {
    let mut cfg: Config = confy::load("tms-v2", None)?;

    let mut changed = false;

    if let Some(mut paths) = config_args.paths {
        changed = true;
        cfg.paths.append(&mut paths);
    }
    if let Some(remove) = config_args.remove {
        changed = true;
        cfg.paths.retain(|path| !remove.contains(path));
    }

    if let Some(mut exclude) = config_args.exclude {
        changed = true;
        cfg.exclude.append(&mut exclude);
    }
    if let Some(remove_exclude) = config_args.remove_exclude {
        changed = true;
        cfg.exclude.retain(|path| !remove_exclude.contains(path));
    }

    if let Some(full_paths) = config_args.full_paths {
        changed = true;
        cfg.full_path = Some(full_paths);
    }

    if let Some(default_session) = config_args.default_session {
        changed = true;
        cfg.default_session = Some(default_session);
    }

    // TODO: Change to subcommand?
    if config_args.edit {
        let path = confy::get_configuration_file_path("tms-v2", None)?;

        let editor = var("EDITOR");
        if let Ok(editor) = editor {
            Command::new(editor).arg(path).status()?;
        }
    }

    if config_args.show {
        println!("{:#?}", cfg);
    }

    if changed {
        confy::store("tms-v2", None, cfg)?;
    }
    Ok(())
}
