use crate::dir_files;
use crate::dir_hashes;
use crate::find;
use crate::hash;
use std::io;

// =================
// === Constants ===
// =================

#[allow(dead_code)]
const REAL_FIND_PATH: &str = "/Users/indiv0/Desktop/files";
const MOCK_FIND_PATH: &str = "src";
const OUT_FILES: &str = "out/files.txt";
const OUT_HASHES: &str = "out/hashes.txt";
const OUT_DIR_FILES: &str = "out/dir_files.txt";

// ============
// === Find ===
// ============

#[test]
fn test_unix_and_walkdir_are_identical() {
    let unix = find::UnixFinder::new(REAL_FIND_PATH);
    let unix = unix.into_iter();
    let unix = unix.map(|p| {
        let p = p.as_ref();
        let s = crate::path_to_str(p);
        s.to_string()
    });
    let unix = unix.collect::<Vec<_>>();
    let walk_dir = find::WalkDirFinder::new(REAL_FIND_PATH);
    let walk_dir = walk_dir.into_iter();
    let walk_dir = walk_dir.map(|p| {
        let p = p.as_ref();
        let s = crate::path_to_str(p);
        s.to_string()
    });
    let walk_dir = walk_dir.collect::<Vec<_>>();
    assert_eq!(unix, walk_dir);
}

// ============
// === Hash ===
// ============

#[test]
#[ignore]
fn test_hash() -> Result<(), io::Error> {
    let paths = crate::Lines::from_path(OUT_FILES)?;
    let _hashes = hash::main(&paths);
    Ok(())
}

// ================
// === DirFiles ===
// ================

#[test]
#[ignore]
fn test_dir_files() -> Result<(), io::Error> {
    let files = crate::Lines::from_path(OUT_FILES)?;
    let _dir_files = dir_files::main(&files);
    Ok(())
}

// =================
// === DirHashes ===
// =================

#[test]
#[ignore]
fn test_dir_hashes() -> Result<(), io::Error> {
    let dir_files = crate::Lines::from_path(OUT_DIR_FILES)?;
    let hashes = crate::Lines::from_path(OUT_HASHES)?;
    let _dir_hashes = dir_hashes::main(&dir_files, &hashes);
    Ok(())
}
