#![feature(io_error_more)]



// =============
// === itry! ===
// =============

/// Like `try!`, but for iterators that return [`Option<Result<_, _>>`].
///
/// [`Option<Result<_, _>>`]: std::option::Option
macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}



// ==============
// === Export ===
// ==============

pub mod fs;
mod path;
mod walkdir;

use itry;



// ===============
// === Prelude ===
// ===============

pub mod prelude {
    // Re-exports for in-crate use.
    pub(crate) use crate::itry;
    pub(crate) use core::ops::Deref;
    pub(crate) use core::ops::DerefMut;
    pub(crate) use std::ffi::OsStr;
    pub(crate) use std::ffi::OsString;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::path::Path;
    pub(crate) use std::path::PathBuf;
    pub(crate) use std::cell::RefCell;
    pub(crate) use std::rc::Rc;
    // Re-exports for public use.
    pub use crate::fs::DirEntry;
    pub use crate::fs::Fs;
    pub use crate::walkdir::find;
}

