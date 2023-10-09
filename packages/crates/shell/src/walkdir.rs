use crate::prelude::*;



// ===============
// === walkdir ===
// ===============

/// Recursively iterates over the directory starting at the file path `root`, returning a list of
/// inodes.
pub fn walkdir(root: impl Into<PathBuf>) -> impl Iterator<Item = walkdir::DirEntry> {
    let root = root.into();
    let walkdir = walkdir::WalkDir::new(root);
    let entries = walkdir.into_iter();
    let entries = entries.map(|e| e.expect("Failed to read entry"));
    // Any entries that are neither files nor directories are unsupported.
    let entries = entries.inspect(|e| assert_file_or_dir(e));
    // Only files will be hashed, so skip any directory entries.
    entries.filter(|e| e.file_type().is_file())
}

/// Asserts the [`DirEntry`] is either a file or a directory.
///
/// [`DirEntry`]: walkdir::DirEntry
fn assert_file_or_dir(entry: &walkdir::DirEntry) {
    let file_type = entry.file_type();
    match file_type {
        ft if ft.is_file() || ft.is_dir() => {},
        ft => unimplemented!("Unsupported file type.\nFile type: {ft:?}.\nEntry: {entry:?}."),
    };
}
