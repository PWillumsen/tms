use crate::config::Config;
use crate::repos;
use color_eyre::eyre::{Context, Result};
use skim::prelude::*;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tmux_interface::variables::session::session::SESSION_ALL;
use tmux_interface::{KillSession, NewSession, Session, Sessions, TmuxCommand};

pub(crate) fn run_tms(config: Config) -> Result<()> {
    let Config {
        paths,
        exclude,
        full_path,
        ..
    } = config;
    let repos = repos::get_repos(paths, exclude, full_path);

    let select_options = repos.keys().cloned().collect::<Vec<String>>().join("\n");

    if let Some(session_name) = select_item(select_options) {
        let mut sessions = get_sessions()?.into_iter().map(|s| s.name.unwrap());

        // TODO: Handle worktrees

        let path = repos.get(&session_name).unwrap().path().parent().unwrap();

        let session_name_short = PathBuf::from(&session_name)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned());

        if let Some(name) = session_name_short {
            if !sessions.any(|sess| sess == session_name) {
                tmux_new(&name, path)?;
            }
            tmux_switch(&name)?;
        }
    }

    Ok(())
}
pub(crate) fn kill_session(_interactive: bool, config: Config) -> Result<()> {
    //TODO: Add interactive multi choice

    let current_session = get_attached_session()?;
    if let Some(current_session) = current_session {
        if let Some(name) = current_session.name {
            if let Some(default_session) = config.default_session {
                tmux_switch(&default_session)?;
            }
            tmux_kill(&name)?;
        }
    }
    Ok(())
}

pub(crate) fn list_sessions() -> Result<()> {
    for session in get_sessions()? {
        let current = if session.attached == Some(1) { "*" } else { "" };
        if let Some(name) = session.name {
            println!("{}{} ", name, current);
        }
    }

    Ok(())
}

fn tmux_switch(session: &str) -> Result<()> {
    TmuxCommand::new()
        .switch_client()
        .target_session(session)
        .output()
        .wrap_err(format!("Error switching to session {}", session))?;
    Ok(())
}

fn tmux_kill(session: &str) -> Result<()> {
    KillSession::new()
        .target_session(session)
        .output()
        .wrap_err(format!("Error killing session {}", session))?;
    Ok(())
}

fn tmux_new(session: &str, dir: &Path) -> Result<()> {
    NewSession::new()
        .detached()
        .session_name(session)
        .start_directory(dir.to_string_lossy())
        .output()
        .wrap_err(format!(
            "Error creating new session {} with start directory {:?}",
            session, dir
        ))?;
    Ok(())
}

fn get_sessions() -> Result<Sessions> {
    Sessions::get(SESSION_ALL).wrap_err("Error getting all sessions")
}

fn get_attached_session() -> Result<Option<Session>> {
    Ok(get_sessions()?
        .into_iter()
        .find(|session| session.attached == Some(1)))
}

fn select_item(input: String) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))?;

    if selected_items.is_abort {
        return None;
    }

    //FIX: Can it fail?
    Some(selected_items.selected_items[0].output().to_string())
}
