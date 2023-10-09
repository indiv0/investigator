use core::fmt;
use core::marker;
use core::str;
use std::path;
#[cfg(test)]
use std::process;



// ==================
// === UnixFinder ===
// ==================

#[cfg(test)]
#[must_use]
pub(crate) struct UnixFinder {
    stdout: Vec<u8>,
}

// === Internal `impl` ===

#[cfg(test)]
impl UnixFinder {
    pub(crate) fn new(path: &str) -> Self {
        let args = vec![path, "-type", "f"];
        let cmd = &mut process::Command::new("find");
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

#[cfg(test)]
impl<'a> IntoIterator for &'a UnixFinder {
    type Item = &'a path::Path;
    type IntoIter = FinderIter<'a, &'a path::Path>;

    fn into_iter(self) -> Self::IntoIter {
        let stdout = str::from_utf8(&self.stdout);
        let stdout = stdout.expect("Failed to read stdout as utf8");
        let lines = stdout.lines();
        let paths = lines.map(|p| {
            crate::assert_path_rules(p);
            path::Path::new(p)
        });
        let paths = Box::new(paths);
        Self::IntoIter { paths }
    }
}

// =====================
// === WalkdirFinder ===
// =====================

#[must_use]
pub struct WalkDirFinder<'a> {
    entries: walkdir::WalkDir,
    marker: marker::PhantomData<&'a ()>,
}

// === Main `impl` ===

impl WalkDirFinder<'_> {
    pub fn new(path: &str) -> Self {
        let entries = walkdir::WalkDir::new(path);
        Self { entries, marker: marker::PhantomData }
    }
}

// === Trait `impl`s ===

impl<'a> IntoIterator for WalkDirFinder<'a> {
    type Item = path::PathBuf;
    type IntoIter = FinderIter<'a, path::PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        let entries = self.entries.into_iter();
        let paths = entries.filter_map(|e| {
            let e = e.expect("Failed to read entry");
            let file_type = e.file_type();
            if !file_type.is_file() {
                return None;
            }
            let path = e.into_path();
            Some(path)
        });
        let paths = Box::new(paths);
        Self::IntoIter { paths }
    }
}

// ==================
// === FinderIter ===
// ==================

#[must_use]
pub struct FinderIter<'a, P> {
    paths: Box<dyn Iterator<Item = P> + 'a>,
}

// === Trait `impl`s ===

impl<'a, P> Iterator for FinderIter<'a, P>
where
    P: AsRef<path::Path> + 'a,
{
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        match self.paths.next() {
            Some(path) => {
                let path_ref = path.as_ref();
                let path_str = crate::path_to_str(path_ref);
                crate::assert_path_rules(path_str);
                Some(path)
            },
            None => None,
        }
    }
}

impl<P> fmt::Debug for FinderIter<'_, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Finder")
            .field("lines", &"Box<dyn Iterator<Item = P>>")
            .finish()
    }
}

