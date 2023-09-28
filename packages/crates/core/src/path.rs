use crate::prelude::*;

use core::iter;
use std::path;



// =================
// === Constants ===
// =================

/// Prefix for absolute directories on UNIX-like operating systems.
const ROOT: &str = "/";
/// A single occurrence of the path separator on UNIX-like operating systems.
const SLASH: &str = "/";
/// Constant defining a double occurrence of the path separator.
const DOUBLE_SLASH: &str = "//";



// =======================
// === AbsolutePathBuf ===
// =======================

/// A [`PathBuf`] that is guaranteed to be absolute.
#[derive(Clone, Debug)]
pub(crate) struct AbsolutePathBuf {
    inner: PathBuf,
}


// === Trait `impl`s ===

impl<T> From<T> for AbsolutePathBuf
where
    T: AsRef<Path>,
{
    fn from(path: T) -> Self {
        let path = path.as_ref();
        let path = canonicalize(path);
        Self { inner: path.to_path_buf() }
    }
}

impl Deref for AbsolutePathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a> AsRef<AbsolutePathBuf> for &'a AbsolutePathBuf {
    fn as_ref(&self) -> &AbsolutePathBuf {
        self
    }
}

/// Returns the canonical, absolute form of the path with all intermediate components normalized and
/// symbolic links resolved.
fn canonicalize(path: &Path) -> PathBuf {
    let components = path.components();
    let mut components = components.peekable();
    // If the first component is not the root, then add it because the path should be absolute.
    let path = match components.peek() {
        Some(path::Component::RootDir) | None => components.collect::<PathBuf>(),
        _ => iter::once(path::Component::RootDir).chain(components).collect(),
    };
    assert_no_double_slash(&path);
    assert_absolute(&path);
    assert_no_trailing_slash(&path);
    path
}

/// Asserts that the given [`Path`] contains no double slashes.
fn assert_no_double_slash(path: impl AsRef<Path>) {
    let path = path.as_ref();
    let path = path.to_str().expect("Not a string.");
    assert!(!path.contains(DOUBLE_SLASH), "Path contains double slash: {path:?}.");
}

/// Asserts that the given [`Path`] is absolute.
fn assert_absolute(path: impl AsRef<Path>) {
    let path = path.as_ref();
    assert!(path.starts_with(SLASH), "Path not absolute: {path:?}.");
}

/// Asserts that the given [`Path`] does not end on a trailing slash, unless it is the root path.
fn assert_no_trailing_slash(path: impl AsRef<Path>) {
    let path = path.as_ref();
    assert!(!path.ends_with(SLASH) || is_root(&path), "Path ends with slash: {path:?}.");
}

/// Returns `true` if the `path` is the root path (i.e., `/`) on a UNIX-like OS.
pub(crate) fn is_root(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    path == AsRef::<Path>::as_ref(&ROOT)
}
