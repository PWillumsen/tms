use tmux_interface::variables::session::session::SESSION_ALL;
use tmux_interface::{Session, Sessions, TmuxCommand};

pub fn invoke_tms() -> anyhow::Result<()> {
    Ok(())
}
pub fn kill_session(
    session: Option<String>,
    _interactive: bool,
    default_session: Option<String>,
) -> anyhow::Result<()> {
    //TODO: Add interactive

    let mut to_kill = session;

    //TODO: attach to default session
    if let Some(session) = default_session {
        todo!();
    }

    if !to_kill.is_some() {
        to_kill = get_attached_session().name;
    }

    let tmux = TmuxCommand::new();

    tmux.kill_session()
        .target_session(to_kill.unwrap())
        .output()
        .unwrap();

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

fn get_sessions() -> Sessions {
    Sessions::get(SESSION_ALL).unwrap()
}

//TODO: refactor this mess, panics if out of bounds
fn get_attached_session() -> Session {
    get_sessions()
        .into_iter()
        .collect::<Vec<Session>>()
        .into_iter()
        .filter(|session| session.attached == Some(1))
        .collect::<Vec<Session>>()
        .swap_remove(0)
}
