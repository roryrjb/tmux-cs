use std::{io::Error, io::ErrorKind, process::Command};

const BIN: &str = "tmux";

pub fn new_detached_session(dir: &str, session_name: &str) -> Result<bool, Error> {
    let cmd = Command::new(BIN)
        .args(&["new-session", "-c", dir, "-d", "-s", session_name])
        .status();

    match cmd {
        Ok(result) => Ok(result.success()),
        Err(error) => Err(error),
    }
}

pub fn new_session(dir: &str, session_name:&str) -> Result<bool, Error> {
    match new_detached_session(dir, session_name) {
        Ok(session_created) => {
            if !session_created {
                return Err(Error::new(ErrorKind::Other, "Could not create session."));
            }

            let switched = switch_client(session_name);

            if !switched {
                let attached = attach(session_name);
                Ok(attached)
            } else {
                Ok(true)
            }

        }
        Err(_) => new(dir, session_name),
    }
}

pub fn kill_session() -> Result<bool, Error> {
    let cmd = Command::new(BIN)
        .arg("kill-session")
        .status();

    match cmd {
        Ok(result) => Ok(result.success()),
        Err(error) => Err(error),
    }
}

pub fn new(dir: &str, session_name: &str) -> Result<bool, Error> {
    let cmd = Command::new(BIN)
        .args(&["new", "-c", dir, "-A", "-s", session_name])
        .status();

    match cmd {
        Ok(result) => return Ok(result.success()),
        Err(error) => return Err(error),
    }
}

fn switch_client(session_name: &str) -> bool {
    let cmd = Command::new(BIN)
        .args(&["switch-client", "-t", session_name])
        .status()
        .unwrap();

    cmd.success()
}

fn attach(session_name: &str) -> bool {
    let cmd = Command::new(BIN)
        .args(&["attach", "-t", session_name])
        .status()
        .unwrap();

    cmd.success()
}
