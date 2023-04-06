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
use std::env;
use std::path;
use investigator::Hasher as _;
use std::io;
use indicatif::ProgressIterator as _;
use std::error;

mod find;
mod dir_files;
mod dir_hashes;
mod dup_dirs;
mod hash;

const UNIQUE_SEPARATOR: &str = "    ";



fn main() {
    fn main_inner() -> Result<(), Box<dyn error::Error>> {
        let mut args = env::args();
        let _command = args.next();
        let command = args.next().expect("Command required");
        match command.as_str() {
            "find" => {
                let path = path_arg(&mut args)?;
                let paths = find::main(&path);
                let mut writer = stdout_writer();
                write_output(&mut writer, paths)?;
            },
            "hash" => {
                let path = path_arg(&mut args)?;
                let paths = hash::main(&path);
                let mut writer = stdout_writer();
                write_output(&mut writer, paths)?;
            },
            "dir_files" => {
                let path = path_arg(&mut args)?;
                let dir_files = dir_files::main(&path);
                let mut writer = stdout_writer();
                write_output(&mut writer, dir_files)?;
            },
            "dir_hashes" => {
                let files = path_arg(&mut args)?;
                let hashes = path_arg(&mut args)?;
                let dir_hashes = dir_hashes::main(&files, &hashes);
                let mut writer = stdout_writer();
                write_output(&mut writer, dir_hashes)?;
            },
            "dup_dirs" => {
                let dir_hashes = path_arg(&mut args)?;
                let dup_dirs = dup_dirs::main(&dir_hashes);
                let mut writer = stdout_writer();
                write_output(&mut writer, dup_dirs)?;
            },
            other => panic!("Unknown command: {other:?}"),
        }
        Ok(())
    }

    if let Err(e) = main_inner() {
        eprintln!("Error: {e}");
    }
}

fn path_arg(args: &mut env::Args) -> Result<String, &'static str> {
    let arg = args.next();
    arg.ok_or_else(|| "Path not provided.")
}

fn stdout_writer() -> io::StdoutLock<'static> {
    let stdout = io::stdout();
    stdout.lock()
}

fn write_output(writer: &mut dyn io::Write, strings: Vec<String>) ->  Result<(), io::Error> {
    let strings = strings.iter();
    let strings = strings.progress();
    let strings = strings.map(|string| write!(writer, "{string}\n"));
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
    assert!(!p.contains("\r"), "Unsupported character in path");
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
    use std::fs;
    use std::io::Write as _;
    use std::process;
    use std::io;
    use std::env;



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
    #[ignore]
    fn test_hash() {
        const PATH: &str = "./data/files.txt";
        let hasher = hash::Hasher::default()
            .path(PATH);
        let _hashes = hasher.hash();
    }



    // ================
    // === DirFiles ===
    // ================

    #[test]
    #[ignore]
    fn test_dir_files() {
        const PATH: &str = "./data/files.txt";
        let dir_files = dir_files::DirFiles::default()
            .files(PATH);
        let _dir_files = dir_files.dir_files();
    }



    // =================
    // === DirHashes ===
    // =================

    #[test]
    #[ignore]
    fn test_dir_hashes() {
        const FILES: &str = "./data/dir_files.txt";
        const HASHES: &str = "./data/hashes.txt";
        let dir_hashes = dir_hashes::DirHashes::default()
            .files(FILES)
            .hashes(HASHES);
        let _dir_hashes = dir_hashes.dir_hashes();
    }



    // ===========
    // === All ===
    // ===========

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
        cargo_run_command(&args, OUT_FILES);
    }

    fn hash() {
        let args = format!("hash {OUT_FILES}");
        cargo_run_command(&args, OUT_HASHES);
    }

    fn dir_files() {
        let args = format!("dir_files {OUT_FILES}");
        cargo_run_command(&args, OUT_DIR_FILES);
    }

    fn dir_hashes() {
        let args = format!("dir_hashes {OUT_DIR_FILES} {OUT_HASHES}");
        cargo_run_command(&args, OUT_DIR_HASHES);
    }

    fn dup_dirs() {
        let args = format!("dup_dirs {OUT_DIR_HASHES}");
        cargo_run_command(&args, OUT_DUP_DIRS);
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
        assert!(!src.contains(" "), "src path contains spaces: {:?}", src);
        src.to_string()
    }

    fn cargo_run_command(args: &str, out_file: &str) {
        let args = args.split(" ");
        let output = process::Command::new("cargo")
            .arg("run")
            .args(args)
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        eprintln!("stderr: {}", stderr);
        println!("stdout: {}", stdout);
        let status = output.status;
        assert!(status.success(), "Failed to run cargo: {:?}", status);
        write_to_file(out_file, &stdout);
    }

    fn write_to_file(path: &str, contents: &str) {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}
