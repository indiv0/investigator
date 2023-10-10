use prelude::*;



// =================
// === Constants ===
// =================

const REAL_FIND_PATH: &str = "/Users/indiv0/Desktop/files";



// ===============
// === Prelude ===
// ===============

mod prelude {
    pub(crate) use dupdir_core::prelude::*;
    pub(crate) use std::time::Instant;
}



// ============
// === Find ===
// ============

#[test]
fn test_unix_and_walk_dir_are_identical() {
    let unix = find_unix();
    let walk_dir = find_walk_dir();
    assert_eq!(unix, walk_dir);
}

fn find_unix() -> Vec<String> {
    let unix = crate::UnixFinder::new(REAL_FIND_PATH);
    let unix = unix.into_iter();
    let unix = unix.map(crate::path_to_string);
    unix.collect()
}

fn find_walk_dir() -> Vec<String> {
    let walk_dir = crate::WalkDirFinder::new(REAL_FIND_PATH);
    let walk_dir = walk_dir.into_iter();
    let walk_dir = walk_dir.map(crate::path_to_string);
    walk_dir.collect()
}



// =================
// === DirHashes ===
// =================

#[test]
fn test_dir_hashes_walk_dir_are_identical() {
    let start = Instant::now();
    let mut state = State::load("../../state.json");
    let walk_dir = dupdir_core::dir_hashes(&mut state, REAL_FIND_PATH);
    let end = Instant::now();
    let duration = end - start;
    assert_eq!(walk_dir.len(), 33966);
    println!("WalkDir: {:?}", duration);
}



// ===========
// === All ===
// ===========

#[test]
fn test_all() {
    // FIXME [NP]: const
    let mut state = State::load("../../state.json");
    let dupdirs = dupdir_core::run_all(&mut state, REAL_FIND_PATH);
    assert_eq!(dupdirs.len(), 26160);
}
