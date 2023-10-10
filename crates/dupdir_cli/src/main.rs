use dupdir_core::prelude::*;

use core::str;
use core::str::FromStr as _;
use indicatif::ProgressIterator as _;
use std::env;
use std::error;
use std::io;



// ===============
// === Command ===
// ===============

#[derive(Debug)]
enum Command {
    All,
}

// === Trait `impl`s ===

impl str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = match s {
            "all" => Self::All,
            _ => Err(format!("Invalid command: {s}"))?,
        };
        Ok(command)
    }
}



// ============
// === Main ===
// ============

fn main() {
    fn main() -> Result<(), Box<dyn error::Error>> {
        let mut args = env::args();
        let _command = args.next();
        let command = args.next().expect("Command required");
        let command = Command::from_str(&command)?;
        match command {
            Command::All => {
                let search_path = path_arg(&mut args)?;
                let mut state = State::load(STATE_JSON);
                let lines = dupdir_core::run_all(&mut state, &search_path);

                // Write the resulting strings to stdout.
                let mut writer = stdout_writer();
                write_output(&mut writer, lines)?;
            }
        };
        Ok(())
    }

    if let Err(e) = main() {
        eprintln!("Error: {e}");
    }
}

fn path_arg(args: &mut env::Args) -> Result<String, &'static str> {
    let arg = args.next();
    arg.ok_or("Path not provided.")
}

fn stdout_writer() -> io::StdoutLock<'static> {
    let stdout = io::stdout();
    stdout.lock()
}

fn write_output(writer: &mut dyn io::Write, strings: Vec<String>) -> Result<(), io::Error> {
    let strings = strings.iter();
    let strings = strings.progress();
    let strings = strings.map(|string| writeln!(writer, "{string}"));
    strings.collect::<Result<(), _>>()
}
