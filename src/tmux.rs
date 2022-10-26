use std::path::{Path, PathBuf};

use crate::{repos, select};
use tmux_interface::variables::session::session::SESSION_ALL;
use tmux_interface::{KillSession, NewSession, Session, Sessions, TmuxCommand};

pub fn invoke_tms(paths: Vec<PathBuf>, exclude: Vec<PathBuf>) -> anyhow::Result<()> {
    let repos = repos::get_repos(paths, exclude);

    //TODO: Sorting ? Zoxide
    let select_options = repos
        .keys()
        .map(|e| e.clone())
        .collect::<Vec<String>>()
        .join("\n");

    let selection = select::select_item(select_options);

    if selection.is_none() {
        return Ok(());
    }
    let session_name = selection.unwrap();

    let sessions = get_sessions()
        .into_iter()
        .map(|s| s.name.unwrap())
        .collect::<Vec<String>>();

    // TODO: Handle worktrees

    let path = repos.get(&session_name).unwrap().path().parent().unwrap();

    let session_name_short = PathBuf::from(&session_name)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if !sessions.contains(&session_name) {
        tm_new(&session_name_short, path)?;
    }
    tm_switch(&session_name_short)?;

    Ok(())
}
pub fn kill_session(
    session: Option<String>,
    _interactive: bool,
    default_session: Option<String>,
) -> anyhow::Result<()> {
    //TODO: Add interactive

    let mut to_kill = session;

    if to_kill.is_none() {
        let current_session = get_attached_session();
        if current_session.is_none() {
            return Ok(());
        } else {
            to_kill = current_session.unwrap().name;
        }
    }

    if default_session.is_some() && to_kill.is_some() {
        tm_switch(&default_session.unwrap())?;
    }

    tm_kill(&to_kill.unwrap())?;

    Ok(())
}

pub fn list_sessions() -> anyhow::Result<()> {
    for session in get_sessions() {
        let current = match session.attached {
            Some(1) => "*",
            _ => "",
        };
        println!("{}{} ", session.name.unwrap(), current);
    }

    Ok(())
}
fn tm_switch(session: &str) -> anyhow::Result<()> {
    TmuxCommand::new()
        .switch_client()
        .target_session(session)
        .output()?;
    Ok(())
}

fn tm_kill(session: &str) -> anyhow::Result<()> {
    KillSession::new().target_session(session).output()?;
    Ok(())
}

fn tm_new(session: &str, dir: &Path) -> anyhow::Result<()> {
    NewSession::new()
        .detached()
        .session_name(session)
        .start_directory(dir.to_string_lossy())
        .output()?;
    Ok(())
}

fn get_sessions() -> Sessions {
    Sessions::get(SESSION_ALL).unwrap()
}

fn get_attached_session() -> Option<Session> {
    for session in get_sessions() {
        if session.attached == Some(1) {
            return Some(session);
        }
    }
    None
}
