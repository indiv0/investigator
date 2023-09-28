use crate::prelude::*;



// =================
// === Constants ===
// =================

/// Contents of an empty file and/or directory in the file system.
const EMPTY: &[u8] = &[];
/// Path to the directory to use as the root of the recursive directory search.
const ROOT: &str = "/example";



// =======================
// === find_files_test ===
// =======================

fn find_files_test(fs: Fs, expected: Fs) {
    fn entries(fs: Fs) -> Vec<DirEntry> {
        let entries = find(fs, ROOT).collect::<Result<_, _>>();
        entries.expect("Failed to read entries.")
    }

    let fs = entries(fs);
    let expected = entries(expected);
    pretty_assertions::assert_eq!(fs, expected);
}



// ========================
// === find_files_test! ===
// ========================

macro_rules! find_files_test {
    (@initial $fs:expr;) => {};
    (@initial $fs:expr; $(/ $path:tt)*: $id:tt = Directory, $($tail:tt)*) => {
        // Insert each directory entry (e.g., `1 = Directory`), into the file system.
        let path = stringify!($(/ $path)*);
        let path = OsString::from(path);
        let previous = $fs.insert(path, EMPTY);
        assert!(previous.is_none(), concat!("Duplicate directory: ", $id));
        find_files_test!(@initial $fs; $($tail)*);
    };
    (@initial $fs:expr; $(/ $path:tt)*: $id:tt = File contents: $contents:expr, $($tail:tt)*) => {
        // Insert each file entry (e.g., `2 = File contents: hello`), into the file system.
        let path = stringify!($(/ $path)*);
        let path = OsString::from(path);
        let contents = stringify!(/ $contents).as_bytes();
        let previous = $fs.insert(path, contents);
        assert!(previous.is_none(), concat!("Duplicate directory: ", $id));
        find_files_test!(@initial $fs; $($tail)*);
    };
    // Expanded entrypoint.
    (initial: { $($initial:tt)* }, final { $($final:tt)* }) => {
        // First, build the initial state of the file system, and the final state.
        let mut fs = BTreeMap::new();
        // NOTE [NP]: In the case that the expected file system is empty, this will not be mutated.
        #[allow(unused_mut)]
        let mut fs_expected = BTreeMap::<_, &[u8]>::new();
        // Parse the rules for the `initial { .. }` section.
        find_files_test!(@initial fs; $($initial)*);
        // Parse the rules for the `final { .. }` section.
        find_files_test!(@initial fs_expected; $($final)*);
        let fs = Fs::from_map(fs);
        let fs_expected = Fs::from_map(fs_expected);
        // Next, run the test.
        find_files_test(fs, fs_expected);
    };
    // Minimal entrypoint.
    (initial: { $($initial_tail:tt)* }) => {
        find_files_test!(initial: { $($initial_tail)* }, final {});
    };
}

#[test]
#[should_panic = "Duplicate directory: 2"]
fn test_dir_name_collision() {
    find_files_test! {
        initial: {
            /foo: 1 = Directory,
            /foo: 2 = Directory,
        }
    }
}

#[test]
fn test_duplicate_dirs() {
    find_files_test! {
        initial: {
            /foo: 1 = Directory,
            /foo/bar: 2 = File contents: hello,
            /baz: 4 = Directory,
        },
        final {
            /foo: 1 = Directory,
            /foo/bar: 2 = File contents: hello,
            /baz: 4 = Directory,
        }
    }
}

