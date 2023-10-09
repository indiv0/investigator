use std::env;
use std::fs;
use std::io;
use std::io::Write as _;
use std::process;

// =================
// === Constants ===
// =================

const OUT_FILES: &str = "out/files.txt";
const OUT_HASHES: &str = "out/hashes.txt";
const OUT_DIR_FILES: &str = "out/dir_files.txt";
const OUT_DIR_HASHES: &str = "out/dir_hashes.txt";
const OUT_DUP_DIRS: &str = "out/dup_dirs.txt";
const OUT_ALL: &str = "out/all.txt";

// ===============
// === DupDirs ===
// ===============

#[test]
#[ignore]
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
    cargo_run_command(&args, Some(OUT_ALL));
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
        .args(["--bin", "dupdir_cli"])
        .args(args)
        .current_dir("..")
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
