use crate::prelude::*;



// =================
// === Constants ===
// =================

/// Path to the directory on the real file system to use for testing.
///
/// This directory will be deleted and re-created repeatedly. Take care not to use a directory that
/// contains important files.
const TEST_DIR: &str = "./tests/test_data/walkdir";
/// Root directory to use for the recursive directory search for the mock file system.
const ROOT: &str = "/";



// ====================
// === test_walkdir ===
// ====================

/// Tests that the custom `walkdir` implementation behaves identically to the real `walkdir`.
#[test]
fn test_walkdir() {
    model! {
        Model => let mut m = crate::fs::create_or_empty_dir(TEST_DIR),
        Implementation => let mut i = Fs::mock(),
        CreateDir(String)(p in crate::fs::prop_path()) => {
            let expected = m.create_dir(&p);
            let actual = i.create_dir(&p);
            crate::fs::assert_eq_io_result(expected, actual)
        },
        WriteFile((String, Vec<u8>))((p, b) in crate::fs::prop_path_and_bytes()) => {
            let expected = m.write(&p, &b);
            let actual = i.write(&p, &b);
            crate::fs::assert_eq_io_result(expected, actual)
        },
        WalkDir(())(_p in pt::arbitrary::any::<()>()) => {
            let expected = walkdir_impl(TEST_DIR);
            // FIXME [NP]: Don't clone the model?
            let actual = walkdir_mock(m.clone());
            pretty_assertions::assert_eq!(expected, actual)
        }
    }
}

/// A recursive directory search on the `root` path, using the custom `walkdir` impl.
fn walkdir_mock(fs: Fs) -> Vec<String> {
    let entries = find(fs, ROOT);
    let entries = entries.filter_map(|e| {
        println!("Entry (Mock): {e:?}.");
        let e = e.expect("Failed to read entry.");
        let path = e.path();
        if !e.file_type().is_file() {
            return None;
        }
        let path = path_to_str(path);
        Some(path.to_string())
    });
    entries.collect()
}

/// A recursive directory search on the `root` path, using the `walkdir` crate.
fn walkdir_impl(root: impl AsRef<Path>) -> Vec<String> {
    let root = root.as_ref();
    let namespace = root.to_str().expect("Path is not string.");
    let walkdir = walkdir::WalkDir::new(root);
    let entries = walkdir.into_iter();
    let entries = entries.filter_map(|e| {
        println!("Entry (Real): {e:?}.");
        let e = e.expect("Failed to read entry.");
        if !e.file_type().is_file() {
            return None;
        }
        let path = e.path();
        let path = path_to_str(path);
        assert_path_rules(path);
        let path = path.trim();
        assert!(!path.is_empty(), "Path is empty.");
        let path = path.strip_prefix(namespace).expect("Missing namespace.");
        Some(path.to_string())
    });
    entries.collect::<Vec<String>>()
}

fn assert_path_rules(p: &str) {
    assert!(!p.contains('\r'), "Unsupported character in path.");
    assert!(!p.is_empty(), "Empty path.");
    assert_eq!(p, p.trim(), "Extra whitespace in path.");
}

fn path_to_str(p: &Path) -> &str {
    p.to_str().expect("Path should be valid UTF-8.")
}
