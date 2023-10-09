use crate::prelude::*;

use std::env;
use std::fs;
use std::io;
use std::iter;



// =================
// === Constants ===
// =================

/// Number of random characters to use when generating a temporary directory name.
const NUM_RAND_CHARS: usize = 1;



// ===============
// === TempDir ===
// ===============

/// A directory in the filesystem that is automatically deleted when it goes out of scope.
#[derive(Clone, Debug)]
pub struct TempDir {
    path: PathBuf,
}


// === Main `impl` ===

impl TempDir {
    /// Tries to create a temporary directory inside of [`env::temp_dir`].
    ///
    /// The directory and everything inside it will be automatically deleted once the returned
    /// [`TempDir`] is destroyed.
    ///
    /// [`env::temp_dir`]: std::env::temp_dir
    /// [`TempDir`]: crate::tempdir::TempDir
    pub fn new() -> io::Result<Self> {
        let path = env::temp_dir();
        tempdir_in(path)
    }

    /// Returns the [`Path`] to the temporary directory.
    ///
    /// [`Path`]: std::path::Path
    pub fn path(&self) -> &Path {
        &self.path
    }
}


// === Trait `impl`s ===

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}



// ==================
// === tempdir_in ===
// ==================

fn tempdir_in(path: impl AsRef<Path>) -> io::Result<TempDir> {
    let dir;
    let mut path = path.as_ref();
    if !path.is_absolute() {
        let current_dir = env::current_dir()?;
        dir = current_dir.join(path);
        path = &dir;
    }
    create_helper(path, NUM_RAND_CHARS, create)
}

fn create_helper<T>(base: &Path, random_len: usize, mut f: impl FnMut(PathBuf) -> io::Result<T>) -> io::Result<T> {
    let tmp_name = tmp_name(random_len);
    let path = base.join(tmp_name);
    f(path)
}

fn create(path: PathBuf) -> io::Result<TempDir> {
    fs::create_dir(&path).with_err_path(&path)?;
    Ok(TempDir { path })
}

fn tmp_name(random_len: usize) -> OsString {
    let mut buf = OsString::with_capacity(random_len);
    let mut char_buf = [0u8; 4];
    let char_gen = iter::repeat_with(fastrand::alphanumeric).take(random_len);
    for c in char_gen {
        let c = c.encode_utf8(&mut char_buf);
        buf.push(c);
    }
    buf
}
