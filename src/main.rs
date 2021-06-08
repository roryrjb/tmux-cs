extern crate getopts;

use getopts::Options;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{exit, Command};

fn new_session(dir: &str, session_name: &str) {
    let cmd = Command::new("tmux")
        .args(&["new-session", "-c", dir, "-d", "-s", session_name])
        .status();

    if cmd.unwrap().success() {
        if !switch_client(session_name) {
            attach_session(session_name);
        }
    } else {
        new(dir, session_name);
    }
}

fn new(dir: &str, session_name: &str) {
    let cmd = Command::new("tmux")
        .args(&["new", "-c", dir, "-A", "-s", session_name])
        .status();

    if !cmd.unwrap().success() {
        switch_client(session_name);

        if !env::current_dir().unwrap().to_str().unwrap().eq(dir) {
            exit(1);
        }
    }
}

fn switch_client(session_name: &str) -> bool {
    let cmd = Command::new("tmux")
        .args(&["switch-client", "-t", session_name])
        .status()
        .unwrap();

    cmd.success()
}

fn attach_session(session_name: &str) -> bool {
    let cmd = Command::new("tmux")
        .args(&["attach", "-t", session_name])
        .status()
        .unwrap();

    cmd.success()
}

fn home() {
    let home_env = env::var("HOME").unwrap();

    Command::new("tmux")
        .args(&["new", "-c", &home_env])
        .status()
        .unwrap();
}

fn kill_session() {
    Command::new("tmux").arg("kill-session").status().unwrap();
}

fn list_sessions() {
    Command::new("tmux").arg("list-sessions").status().unwrap();
}

fn main() {
    let program_name = env::args().next();
    let mut opts = Options::new();
    opts.optflag("h", "help", "show this message");
    opts.optflag("k", "kill", "kill this session (tmux kill-session)");
    opts.optflag("l", "list", "list sessions (tmux list-sessions)");

    let matches = opts.parse(env::args()).unwrap();

    if matches.opt_present("h") {
        let brief = format!(
            "Usage: {0} [OPTIONS]\n{1}{0} [DIR]\n{1}{0} [DIR] [SESSION_NAME]\n{1}{0} [SESSION_NAME]",
            program_name.unwrap(),
            "       "
        );
        println!("{}", opts.usage(&brief));
        return;
    }

    if matches.opt_present("k") {
        kill_session();
        return;
    }

    if matches.opt_present("l") {
        list_sessions();
        return;
    }

    let dir = env::args().nth(1);

    match dir {
        Some(arg) => {
            let target_dir: &str = &arg;

            if target_dir.eq(".") {
                new_session(target_dir, "main");
            } else {
                let path = Path::new(&arg);

                if path.exists() {
                    let normalised: &OsStr =
                        Path::new(&arg).file_name().unwrap();
                    let session_name: &str = normalised.to_str().unwrap();
                    new_session(target_dir, session_name);
                }
            }
        }
        None => home(),
    }
}
