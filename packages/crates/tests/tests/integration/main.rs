#![feature(io_error_more)]



// ==================
// === assert_let ===
// ==================

macro_rules! assert_let {
    ($pattern:pat = $expr:expr) => {
        let expr = $expr;
        let $pattern = expr else {
            panic!(
                "Assertion failed: `{:?}` does not match `{}`.",
                expr,
                stringify!($pattern),
            );
        };
    };
}



// ==============
// === Export ===
// ==============

mod find;
mod find2;
pub(crate) mod fs;
pub mod walkdir;

use assert_let;



// ===============
// === Prelude ===
// ===============

mod prelude {
    // Re-exports for in-crate use.
    pub(crate) use core::fmt::Debug;
    pub(crate) use crate::assert_let;
    pub(crate) use find_files_core::prelude::*;
    pub(crate) use find_files_tests::prelude::*;
    pub(crate) use model::model;
    pub(crate) use model::prop_oneof;
    pub(crate) use model::pt;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::ffi::OsString;
    pub(crate) use std::path::Path;
    pub(crate) use std::path::PathBuf;
}
