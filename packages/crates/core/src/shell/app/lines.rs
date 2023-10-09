use crate::prelude::*;

use std::io;
use std::fs;
use std::io::BufRead as _;



// =============
// === Lines ===
// =============

#[derive(Debug, Default)]
pub(crate) struct Lines(pub(crate) Vec<String>);

// === Main `impl` ===

impl Lines {
    fn from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let path = path.as_ref();
        Self::try_from(path)
    }

    // FIXME [NP]: encode this check in the type so we don't forget?
    pub(crate) fn verify_paths(&self) {
        let Self(lines) = self;
        let lines = lines.iter();
        lines.for_each(|line| {
            assert_path_rules(line);
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
