use core::str;
use core::str::FromStr as _;
use dupdir_core::find;
use indicatif::ProgressIterator as _;
use std::env;
use std::error;
use std::io;

// ===============
// === Command ===
// ===============

#[derive(Debug)]
enum Command {
    Find,
    Hash,
    DirHashes,
    DupDirs,
    All,
}

// === Trait `impl`s ===

impl str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = match s {
            "find" => Self::Find,
            "hash" => Self::Hash,
            "dir_hashes" => Self::DirHashes,
            "dup_dirs" => Self::DupDirs,
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
        let mut state = dupdir_core::State::load(dupdir_core::STATE_JSON);
        match command {
            Command::Find => {
                let path = path_arg(&mut args)?;
                let files = find::WalkDirFinder::new(&path);
                let files = files.into_iter();
                let files = files.map(|p| {
                    let p = p.as_ref();
                    let s = dupdir_core::path_to_str(p);
                    s.to_string()
                });
                let files = files.collect();
                let lines = dupdir_core::Lines(files);

                // Write the resulting strings to stdout.
                let mut writer = stdout_writer();
                let dupdir_core::Lines(lines) = lines;
                write_output(&mut writer, lines)?;
            }
            Command::Hash => {
                let path = path_arg(&mut args)?;
                let paths = dupdir_core::Lines::from_path(path)?;
                dupdir_core::run_hash(&mut state, &paths);
                state.save();
            }
            Command::DirHashes => {
                let path = path_arg(&mut args)?;
                let lines = dupdir_core::run_dir_hashes(&mut state, path);

                // Write the resulting strings to stdout.
                let mut writer = stdout_writer();
                write_output(&mut writer, lines)?;
            }
            Command::DupDirs => {
                let dir_hashes = path_arg(&mut args)?;
                let dir_hashes = dupdir_core::Lines::from_path(dir_hashes)?;
                let lines = dupdir_core::run_dup_dirs(&dir_hashes);

                // Write the resulting strings to stdout.
                let mut writer = stdout_writer();
                let dupdir_core::Lines(lines) = lines;
                write_output(&mut writer, lines)?;
            }
            Command::All => {
                let search_path = path_arg(&mut args)?;
                let files = find::WalkDirFinder::new(&search_path);
                let files = files.into_iter();
                let files = files.map(|p| {
                    let p = p.as_ref();
                    let s = dupdir_core::path_to_str(p);
                    s.to_string()
                });
                let files = files.collect();
                let files = dupdir_core::Lines(files);
                dupdir_core::run_hash(&mut state, &files);
                state.save();
                let dir_hashes = dupdir_core::run_dir_hashes(&mut state, &search_path);
                let dir_hashes = dupdir_core::Lines(dir_hashes);
                let lines = dupdir_core::run_dup_dirs(&dir_hashes);

                // Write the resulting strings to stdout.
                let mut writer = stdout_writer();
                let dupdir_core::Lines(lines) = lines;
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
