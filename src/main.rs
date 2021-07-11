#[macro_use]
extern crate partial_application;

extern crate getopts;

mod tmux;

use getopts::Options;
use std::env;
use std::path::Path;
use std::process::exit;

fn bail(msg: &str) {
    println!("Error: {}", msg);
    exit(1);
}

fn usage(program_name: &str, opts: &Options) {
    let brief = format!(
        "Usage: {0} [OPTIONS]\n{1}{0} [DIR]",
        program_name, "       "
    );

    println!("{}", opts.usage(&brief));
}

fn main() {
    let program_name = env::args().next().unwrap();
    let mut opts = Options::new();
    opts.optflag("h", "help", "show this message");
    opts.optflag("k", "kill", "kill this session");

    let print_usage = partial!(usage => &program_name, &opts);
    let matches = opts.parse(env::args()).unwrap();

    if matches.opt_present("h") {
        return print_usage();
    }

    if matches.opt_present("k") {
        match tmux::kill_session() {
            Ok(success) => {
                if success {
                    exit(0);
                } else {
                    bail("Could not start tmux session.");
                }
            }
            Err(err) => bail(&err.to_string()),
        }
    }

    if let Some(arg) = env::args().nth(1) {
        let path = Path::new(&arg);

        if path.exists() {
            let normalised = path.file_name().unwrap();
            let session_name: &str = normalised.to_str().unwrap();

            match tmux::new_session(&arg, session_name) {
                Ok(success) => {
                    if !success {
                        bail("Could not start tmux session.");
                    }
                }
                Err(err) => bail(&err.to_string()),
            }
        }
    } else {
        let home_env = env::var("HOME").unwrap();

        match tmux::new_session(&home_env, "0") {
            Ok(success) => {
                if !success {
                    bail("Could not start tmux session.");
                }
            }
            Err(err) => bail(&err.to_string()),
        }
    }
}
