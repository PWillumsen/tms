use crate::cli::ConfigArgs;
use color_eyre::eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub paths: Vec<PathBuf>,
    pub exclude: Vec<PathBuf>,
    pub default_session: Option<String>,
    pub full_path: Option<bool>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "paths: {:?}\n", self.paths)?;
        write!(f, "exclude: {:?}\n", self.exclude)?;
        if let Some(default_session) = &self.default_session {
            write!(f, "default_session: {}\n", default_session)?;
        }
        if let Some(full_path) = &self.full_path {
            write!(f, "full_path: {}", full_path)?;
        }

        Ok(())
    }
}

pub(crate) fn update_config(config_args: ConfigArgs) -> Result<()> {
    let mut cfg: Config = confy::load("tms-v2", None).wrap_err("Could not load config")?;

    let mut changed = false;

    if let Some(mut paths) = config_args.paths {
        changed = true;
        cfg.paths.append(&mut paths);
    }
    if let Some(remove) = config_args.paths_remove {
        changed = true;
        cfg.paths.retain(|path| !remove.contains(path));
    }

    if let Some(mut exclude) = config_args.exclude {
        changed = true;
        cfg.exclude.append(&mut exclude);
    }
    if let Some(remove_exclude) = config_args.exclude_remove {
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

    if config_args.edit {
        let path = confy::get_configuration_file_path("tms-v2", None)?;

        let editor = var("EDITOR");
        if let Ok(editor) = editor {
            Command::new(&editor).arg(&path).status().wrap_err(format!(
                "Error opening config at {:?} with {}",
                path, editor
            ))?;
        }
    }

    if config_args.show {
        println!("{}", cfg);
    }

    if changed {
        confy::store("tms-v2", None, &cfg).wrap_err(format!("Error saving config: {:?}", cfg))?;
    }
    Ok(())
}

pub(crate) fn load_config() -> Result<Config> {
    let cfg: Config = confy::load("tms-v2", None).wrap_err("Error loading config")?;
    Ok(cfg)
}
