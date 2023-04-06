//! Usage:
//! ```
//! mkdir -p target/data
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
//! cat target/data/dupdirs_by_path.txt | awk '{ print length, $0 }' | sort -n -s -r | cut -d" " -f2- > tmp.txt
//! scp tmp.txt 172.30.194.6:
//! ssh 172.30.194.6
//! sudo mv tmp.txt /storage/tmp.txt
//! sudo su
//! cd /storage
//! cat tmp.txt | grep -v "'" | grep -v ' \./lap-ca-nik-01\| \./lab-ca-kvm-02' | cut -d' ' -f2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27 | xargs -I{} du -d 0 "{}" | sort -n
//! ```
//!
//! Other usage:
//! ```
//! mkdir -p target/data
//! clear && cargo check && RUST_BACKTRACE=1 time cargo run --release old_dup_dirs
//! cat target/data/dupdirs_by_path.txt | cut -d' ' -f2- | xargs -d '\n' du -d0 | sort -n
//! ```
//!
//! New usage:
//! ```
//! clear && \
//!   cargo check && \
//!   cargo test --color=always 2>&1 && \
//!   cargo build --release
//! find /Users/indiv0/Desktop/files -type f -name '*'$'\r''*'
//! find /Users/indiv0/Desktop/files -type f -name '*'$'\r''*' -delete
//! find /Users/indiv0/Desktop/files -not -perm -u=r -not -perm -u=w -not -perm -u=x -ls
//! find /Users/indiv0/Desktop/files -not -perm -u=r -not -perm -u=w -not -perm -u=x -delete
//! mkdir -p target/data
//! sudo su
//! time ./target/release/dupdir find /Users/indiv0/Desktop/files > target/data/files.txt && chown indiv0 target/data/files.txt
//! time ./target/release/dupdir hash target/data/files.txt > target/data/hashes.txt && chown indiv0 target/data/hashes.txt
//! time ./target/release/dupdir dir_files target/data/files.txt > target/data/dir_files.txt && chown indiv0 target/data/dir_files.txt
//! time ./target/release/dupdir dir_hashes target/data/dir_files.txt target/data/hashes.txt > target/data/dir_hashes.txt && chown indiv0 target/data/dir_hashes.txt
//! time ./target/release/dupdir dup_dirs target/data/dir_hashes.txt > target/data/dup_dirs.txt && chown indiv0 target/data/dup_dirs.txt
//! exit
//! cat target/data/dup_dirs.txt | cut -d';' -f2 | xargs -d '\n' du -d0 | sort -n
//! ```
use indicatif::ProgressIterator as _;
use investigator::Hasher as _;
use std::env;
use std::error;
use std::io;
use std::path;
use std::fs;
use std::io::BufRead as _;

mod dir_files;
mod dir_hashes;
mod dup_dirs;
mod find;
mod hash;

// =================
// === Constants ===
// =================

const UNIQUE_SEPARATOR: &str = "    ";

// =============
// === Lines ===
// =============

#[derive(Debug, Default)]
pub struct Lines(pub Vec<String>);

// === Main `impl` ===

impl Lines {
    fn from_path(path: impl AsRef<path::Path>) -> Result<Self, io::Error> {
        let path = path.as_ref();
        Self::try_from(path)
    }

    // FIXME [NP]: encode this check in the type so we don't forget?
    fn verify_paths(&self) {
        let crate::Lines(lines) = self;
        let lines = lines.iter();
        lines.for_each(|line| {
            crate::assert_path_rules(line);
        });
    }
}

// === Trait `impls` ===

impl TryFrom<&path::Path> for Lines {
    type Error = io::Error;

    fn try_from(path: &path::Path) -> Result<Self, Self::Error> {
        let file = fs::File::open(path)?;
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|line| {
            line.map(|line| {
                assert_path_rules(&line);
                line
            })
        });
        let paths = lines.collect::<Result<Vec<_>, _>>()?;
        let paths = Self(paths);
        Ok(paths)
    }
}

// ============
// === Main ===
// ============

fn main() {
    fn main_inner() -> Result<(), Box<dyn error::Error>> {
        let mut args = env::args();
        let _command = args.next();
        let command = args.next().expect("Command required");
        let lines = match command.as_str() {
            "find" => {
                let path = path_arg(&mut args)?;
                find::main(&path)
            }
            "hash" => {
                let path = path_arg(&mut args)?;
                let paths = Lines::from_path(path)?;
                hash::main(&paths)
            }
            "dir_files" => {
                let path = path_arg(&mut args)?;
                let files = Lines::from_path(path)?;
                dir_files::main(&files)
            }
            "dir_hashes" => {
                let dir_files = path_arg(&mut args)?;
                let hashes = path_arg(&mut args)?;
                let dir_files = Lines::from_path(dir_files)?;
                let hashes = Lines::from_path(hashes)?;
                dir_hashes::main(&dir_files, &hashes)
            }
            "dup_dirs" => {
                let dir_hashes = path_arg(&mut args)?;
                let dir_hashes = Lines::from_path(dir_hashes)?;
                dup_dirs::main(&dir_hashes)
            }
            "all" => {
                let search_path = path_arg(&mut args)?;
                let files = find::main(&search_path);
                let hashes = hash::main(&files);
                let dir_files = dir_files::main(&files);
                let dir_hashes = dir_hashes::main(&dir_files, &hashes);
                dup_dirs::main(&dir_hashes)
            }
            other => panic!("Unknown command: {other:?}"),
        };
        // Write the resulting strings to stdout.
        let mut writer = stdout_writer();
        let Lines(lines) = lines;
        write_output(&mut writer, lines)?;
        Ok(())
    }

    if let Err(e) = main_inner() {
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

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = investigator::T1ha2::default();
    investigator::copy_wide(&mut &bytes[..], &mut hasher).unwrap();
    let hash = hasher.finish().to_vec();
    hex::encode(hash)
}

#[inline]
fn assert_path_rules(p: &str) {
    assert!(!p.contains('\r'), "Unsupported character in path");
    assert!(!p.is_empty(), "Empty path");
    assert_eq!(p, p.trim(), "Extra whitespace in path");
}

#[inline]
fn path_to_str(p: &path::Path) -> &str {
    p.to_str().expect("Path should be valid UTF-8")
}

// =============
// === Tests ===
// =============

#[cfg(test)]
mod tests {
    use crate::dir_files;
    use crate::dir_hashes;
    use crate::find;
    use crate::hash;
    use std::env;
    use std::fs;
    use std::io;
    use std::io::Write as _;
    use std::process;

    // =================
    // === Constants ===
    // =================

    const REAL_FIND_PATH: &str = "/Users/indiv0/Desktop/files";
    const MOCK_FIND_PATH: &str = "src";
    const OUT_FILES: &str = "out/files.txt";
    const OUT_HASHES: &str = "out/hashes.txt";
    const OUT_DIR_FILES: &str = "out/dir_files.txt";
    const OUT_DIR_HASHES: &str = "out/dir_hashes.txt";
    const OUT_DUP_DIRS: &str = "out/dup_dirs.txt";

    // ============
    // === Find ===
    // ============

    #[test]
    fn test_unix_and_walkdir_are_identical() {
        let finder = find::Finder::default().path(MOCK_FIND_PATH);
        let unix = finder.clone().strategy(find::Strategy::Unix).find();
        let walk_dir = finder.strategy(find::Strategy::WalkDir).find();
        assert_eq!(unix, walk_dir);
    }

    // ============
    // === Hash ===
    // ============

    #[test]
    fn test_hash() -> Result<(), io::Error> {
        let paths = crate::Lines::from_path(OUT_FILES)?;
        let _hashes = hash::main(&paths);
        Ok(())
    }

    // ================
    // === DirFiles ===
    // ================

    #[test]
    fn test_dir_files() -> Result<(), io::Error> {
        let files = crate::Lines::from_path(OUT_FILES)?;
        let _dir_files = dir_files::main(&files);
        Ok(())
    }

    // =================
    // === DirHashes ===
    // =================

    #[test]
    fn test_dir_hashes() -> Result<(), io::Error> {
        let dir_files = crate::Lines::from_path(OUT_DIR_FILES)?;
        let hashes = crate::Lines::from_path(OUT_HASHES)?;
        let _dir_hashes = dir_hashes::main(&dir_files, &hashes);
        Ok(())
    }

    // ===============
    // === DupDirs ===
    // ===============

    #[test]
    fn test_dup_dirs() {
        create_dir("out").unwrap();
        find();
        hash();
        dir_files();
        dir_hashes();
        dup_dirs();
    }

    fn find() {
        let src = src_dir();
        let args = format!("find {src}");
        cargo_run_command(&args, Some(OUT_FILES));
    }

    fn hash() {
        let args = format!("hash {OUT_FILES}");
        cargo_run_command(&args, Some(OUT_HASHES));
    }

    fn dir_files() {
        let args = format!("dir_files {OUT_FILES}");
        cargo_run_command(&args, Some(OUT_DIR_FILES));
    }

    fn dir_hashes() {
        let args = format!("dir_hashes {OUT_DIR_FILES} {OUT_HASHES}");
        cargo_run_command(&args, Some(OUT_DIR_HASHES));
    }

    fn dup_dirs() {
        let args = format!("dup_dirs {OUT_DIR_HASHES}");
        cargo_run_command(&args, Some(OUT_DUP_DIRS));
    }

    // ===========
    // === All ===
    // ===========

    #[test]
    fn test_all() {
        create_dir("out").unwrap();
        all();
    }

    fn all() {
        let src = src_dir();
        let args = format!("all {src} {OUT_FILES}");
        // FIXME [NP]: output
        cargo_run_command(&args, None);
    }

    fn create_dir(path: &str) -> io::Result<()> {
        match fs::create_dir(path) {
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(()),
            result => result,
        }
    }

    fn src_dir() -> String {
        let cwd = env::current_dir().unwrap();
        let src = cwd.join("src");
        let src = src.to_str();
        let src = src.unwrap();
        assert!(!src.contains(' '), "src path contains spaces: {src:?}");
        src.to_string()
    }

    fn cargo_run_command(args: &str, out_file: Option<&str>) {
        let args = args.split(' ');
        let output = process::Command::new("cargo")
            .arg("run")
            .args(args)
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        eprintln!("stderr: {stderr}");
        println!("stdout: {stdout}");
        let status = output.status;
        assert!(status.success(), "Failed to run cargo: {status:?}");
        if let Some(out_file) = out_file {
            write_to_file(out_file, &stdout);
        }
    }

    fn write_to_file(path: &str, contents: &str) {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}
