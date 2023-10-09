// ==============
// === Export ===
// ==============

mod error;
mod path;
mod tempdir;



// ===============
// === Prelude ===
// ===============

pub mod prelude {
    // Re-exports for in-crate use.
    pub(crate) use core::fmt;
    pub(crate) use core::fmt::Display;
    pub(crate) use core::fmt::Formatter;
    pub(crate) use crate::error::IoResultExt as _;
    pub(crate) use std::ffi::OsString;
    pub(crate) use std::path::Path;
    pub(crate) use std::path::PathBuf;
    // Re-exports for public use.
    pub use crate::error::IoResultExt as _;
    pub use crate::path::PathExt as _;
    pub use crate::tempdir::TempDir;
}
