use crate::prelude::*;



// ==============
// === Export ===
// ==============

#[cfg(test)]
mod tests;



// ===============
// === Prelude ===
// ===============

mod prelude {
    pub(crate) use dupdir_core::prelude::*;
    pub(crate) use core::str;
    pub(crate) use std::path::Path;
    #[cfg(test)]
    pub(crate) use std::process::Command;
}



// ==================
// === UnixFinder ===
// ==================

#[must_use]
struct UnixFinder {
    stdout: Vec<u8>,
}


// === Internal `impl` ===

impl UnixFinder {
    #[cfg(test)]
    fn new(path: &str) -> Self {
        let args = vec![path, "-type", "f"];
        let cmd = &mut Command::new("find");
        let cmd = cmd.args(args);
        let output = cmd.output();
        let output = output.expect("Failed to execute process");
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        let stdout = output.stdout;
        Self { stdout }
    }
}


// === Trait `impl`s ===

impl<'a> IntoIterator for &'a UnixFinder {
    type Item = &'a Path;
    type IntoIter = FinderIter<'a, &'a Path>;

    fn into_iter(self) -> Self::IntoIter {
        let stdout = str::from_utf8(&self.stdout);
        let stdout = stdout.expect("Failed to read stdout as utf8");
        let lines = stdout.lines();
        let paths = lines.map(|p| {
            assert_path_rules(p);
            Path::new(p)
        });
        let paths = Box::new(paths);
        Self::IntoIter { paths }
    }
}
