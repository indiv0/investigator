use crate::find;
use std::time;

// =================
// === Constants ===
// =================

const REAL_FIND_PATH: &str = "/Users/indiv0/Desktop/files";

// ============
// === Find ===
// ============

#[test]
#[ignore]
fn test_unix_and_walk_dir_are_identical() {
    let unix = find_unix();
    let walk_dir = find_walk_dir();
    assert_eq!(unix, walk_dir);
}

fn find_unix() -> Vec<String> {
    let unix = find::UnixFinder::new(REAL_FIND_PATH);
    let unix = unix.into_iter();
    let unix = unix.map(|p| {
        let p = p.as_ref();
        let s = crate::path_to_str(p);
        s.to_string()
    });
    unix.collect()
}

fn find_walk_dir() -> Vec<String> {
    let walk_dir = find::WalkDirFinder::new(REAL_FIND_PATH);
    let walk_dir = walk_dir.into_iter();
    let walk_dir = walk_dir.map(|p| {
        let p = p.as_ref();
        let s = crate::path_to_str(p);
        s.to_string()
    });
    walk_dir.collect()
}



// =================
// === DirHashes ===
// =================

#[test]
fn test_dir_hashes_walk_dir_are_identical() {
    let start = time::Instant::now();
    let mut state = crate::State::load("../../state.json");
    let walk_dir = crate::run_dir_hashes(&mut state, REAL_FIND_PATH);
    let end = time::Instant::now();
    let duration = end - start;
    assert_eq!(walk_dir.len(), 33966);
    println!("WalkDir: {:?}", duration);
}
