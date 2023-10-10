use crate::prelude::*;

use std::fs;
use std::io;
use std::io::BufRead as _;
use std::str;

// =================
// === Constants ===
// =================

pub const STATE_JSON: &str = "state.json";

// ==============
// === Export ===
// ==============

mod dir_hashes;
mod dup_dirs;
pub mod find;
mod hash;
#[cfg(test)]
mod tests;

pub use dir_hashes::main as run_dir_hashes;
pub use dup_dirs::main as run_dup_dirs;
pub use hash::main as run_hash;



// ===============
// === Prelude ===
// ===============

mod prelude {
    pub(crate) use serde::Deserialize;
    pub(crate) use serde::Serialize;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::collections::BTreeSet;
    pub(crate) use std::path::Path;
    pub(crate) use std::path::PathBuf;
}



// =============
// === State ===
// =============

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[must_use]
pub struct State {
    hashes: BTreeMap<PathBuf, String>,
}

// === Main `impl` ===

impl State {
    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Serialize");
        let path = Path::new(STATE_JSON);
        fs::write(path, json).expect("Write");
    }

    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        if path.exists() {
            let json = fs::read_to_string(path).expect("Read");
            serde_json::from_str(&json).expect("Deserialize")
        } else {
            panic!();
        }
    }
}



// =============
// === Lines ===
// =============

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Lines(pub Vec<String>);

// === Main `impl` ===

impl Lines {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let path = path.as_ref();
        Self::try_from(path)
    }

    // FIXME [NP]: encode this check in the type so we don't forget?
    fn verify_paths(&self) {
        let crate::Lines(lines) = self;
        let lines = lines.iter();
        lines.for_each(|line| {
            crate::assert_path_rules(line);
        });
    }
}

// === Trait `impls` ===

impl TryFrom<&Path> for Lines {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = fs::File::open(path)?;
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|line| {
            line.map(|line| {
                assert_path_rules(&line);
                line
            })
        });
        let paths = lines.collect::<Result<Vec<_>, _>>()?;
        let paths = Self(paths);
        Ok(paths)
    }
}

#[inline]
fn assert_path_rules(p: &str) {
    assert!(!p.contains('\r'), "Unsupported character in path");
    assert!(!p.is_empty(), "Empty path");
    assert_eq!(p, p.trim(), "Extra whitespace in path");
}

#[inline]
pub fn path_to_str(p: &Path) -> &str {
    p.to_str().expect("Path should be valid UTF-8")
}
