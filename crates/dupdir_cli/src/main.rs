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
    Find,
    Hash,
    DirFiles,
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
            "dir_files" => Self::DirFiles,
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
        let lines = match command {
            Command::Find => {
                let path = path_arg(&mut args)?;
                dupdir_core::run_find(&path)
            }
            Command::Hash => {
                let path = path_arg(&mut args)?;
                let paths = dupdir_core::Lines::from_path(path)?;
                dupdir_core::run_hash(&paths)
            }
            Command::DirFiles => {
                let path = path_arg(&mut args)?;
                let files = dupdir_core::Lines::from_path(path)?;
                dupdir_core::run_dir_files(&files)
            }
            Command::DirHashes => {
                let dir_files = path_arg(&mut args)?;
                let hashes = path_arg(&mut args)?;
                let dir_files = dupdir_core::Lines::from_path(dir_files)?;
                let hashes = dupdir_core::Lines::from_path(hashes)?;
                dupdir_core::run_dir_hashes(&dir_files, &hashes)
            }
            Command::DupDirs => {
                let dir_hashes = path_arg(&mut args)?;
                let dir_hashes = dupdir_core::Lines::from_path(dir_hashes)?;
                dupdir_core::run_dup_dirs(&dir_hashes)
            }
            Command::All => {
                let search_path = path_arg(&mut args)?;
                let files = dupdir_core::run_find(&search_path);
                let hashes = dupdir_core::run_hash(&files);
                let dir_files = dupdir_core::run_dir_files(&files);
                let dir_hashes = dupdir_core::run_dir_hashes(&dir_files, &hashes);
                dupdir_core::run_dup_dirs(&dir_hashes)
            },
        };
        // Write the resulting strings to stdout.
        let mut writer = stdout_writer();
        let dupdir_core::Lines(lines) = lines;
        write_output(&mut writer, lines)?;
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

fn write_output(
    writer: &mut dyn io::Write,
    strings: Vec<String>,
) -> Result<(), io::Error> {
    let strings = strings.iter();
    let strings = strings.progress();
    let strings = strings.map(|string| writeln!(writer, "{string}"));
    strings.collect::<Result<(), _>>()
}
