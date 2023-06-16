use std::{
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

use crate::args::parse_input;

const PROMPT: &str = "$ ";
const EXIT: &str = "exit";
const CD: &str = "cd";

mod args;

fn main() {
    loop {
        print!("{PROMPT}");
        if let Err(err) = stdout().flush() {
            eprintln!("flush_error {err}");
            continue;
        }

        let mut input = String::default();
        if let Err(err) = stdin().read_line(&mut input) {
            eprintln!("readin_error {err}");
            continue;
        }
        let input = input.trim();
        // if no input just continue
        if input.is_empty() {
            continue;
        };
        //
        // // prepare command and args

        let (command, args) = match parse_input(input) {
            Err(e) => {
                eprintln!("error {e}");
                continue;
            }
            Ok((c, a)) => (c, a),
        };

        match command {
            EXIT => return,
            CD => {
                // new location is first arg or default location
                let loc = args.iter().peekable().peek().map_or("/", |l| *l);
                let root = Path::new(loc);
                // try to go to new location
                if let Err(e) = std::env::set_current_dir(root) {
                    eprintln!("cd_error {}", e);
                }
            }
            command => match Command::new(command).args(args).output() {
                Err(e) => eprintln!("child error {e}"),
                Ok(o) => {
                    if o.status.success() {
                        match String::from_utf8(o.stdout) {
                            Err(e) => eprintln!("UTF_8 error {e}"),
                            Ok(s) => print!("{s}"),
                        };
                    } else {
                        match String::from_utf8(o.stderr) {
                            Err(e) => eprintln!("UTF_8 error {e}"),
                            Ok(s) => {
                                eprint!("{s}");
                                eprintln!("error: command exited with {}", o.status)
                            }
                        };
                    }
                }
            },
        }
    }
}
