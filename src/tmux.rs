use std::path::{Path, PathBuf};

use crate::{repos, select};
use tmux_interface::variables::session::session::SESSION_ALL;
use tmux_interface::{KillSession, NewSession, Session, Sessions, TmuxCommand};

pub(crate) fn invoke_tms(
    paths: Vec<PathBuf>,
    exclude: Vec<PathBuf>,
    full_path: Option<bool>,
) -> anyhow::Result<()> {
    let repos = repos::get_repos(paths, exclude, full_path);

    let select_options = repos.keys().cloned().collect::<Vec<String>>().join("\n");

    if let Some(session_name) = select::select_item(select_options) {
        let mut sessions = get_sessions()?.into_iter().map(|s| s.name.unwrap());

        // TODO: Handle worktrees

        let path = repos.get(&session_name).unwrap().path().parent().unwrap();

        let session_name_short = PathBuf::from(&session_name)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned());

        if let Some(name) = session_name_short {
            if !sessions.any(|sess| sess == session_name) {
                tm_new(&name, path)?;
            }
            tm_switch(&name)?;
        }
    }

    Ok(())
}
pub(crate) fn kill_session(
    _interactive: bool,
    default_session: Option<String>,
) -> anyhow::Result<()> {
    //TODO: Add interactive

    let current_session = get_attached_session();
    if let Some(current_session) = current_session? {
        if let Some(name) = current_session.name {
            if let Some(default_session) = default_session {
                tm_switch(&default_session)?;
            }
            tm_kill(&name)?;
        }
    }
    Ok(())
}

pub(crate) fn list_sessions() -> anyhow::Result<()> {
    return Err(anyhow::anyhow!("test error"));

    // for session in get_sessions()? {
    //     let current = match session.attached {
    //         Some(1) => "*",
    //         _ => "",
    //     };
    //     if let Some(name) = session.name {
    //         println!("{}{} ", name, current);
    //     }
    // }

    // Ok(())
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

fn get_sessions() -> Result<Sessions, tmux_interface::Error> {
    Sessions::get(SESSION_ALL)
}

fn get_attached_session() -> Result<Option<Session>, tmux_interface::Error> {
    Ok(get_sessions()?
        .into_iter()
        .find(|session| session.attached == Some(1)))
}
