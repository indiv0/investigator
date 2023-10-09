use crate::find;

// =================
// === Constants ===
// =================

const REAL_FIND_PATH: &str = "/Users/indiv0/Desktop/files";

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
