use indicatif::ProgressIterator as _;
use indicatif::ParallelProgressIterator as _;
use rayon::prelude::*;
use std::collections;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead as _;
use std::io::Write as _;
use std::path;
use std::str;



// =================
// === Ancestors ===
// =================

#[derive(Clone, Debug, Default)]
pub struct Ancestors<'a> {
    path: &'a str,
}

impl<'a> Ancestors<'a> {
    pub fn path(mut self, path: &'a str) -> Self {
        self.path = path;
        self
    }

    pub fn ancestors(&self) -> collections::HashSet<String> {
        let paths = self.read_paths();
        let paths = paths.par_iter().progress();
        let dirs = paths.map(|p| file_dir(p));
        let ancestors = dirs.flat_map(|d| ancestors(d));
        let ancestors = ancestors.map(|a| crate::path_to_str(a));
        let ancestors = ancestors.map(|s| s.to_string());
        let ancestors = ancestors.collect::<collections::HashSet<_>>();
        ancestors
    }

    fn read_paths(&self) -> Vec<String> {
        let file = fs::File::open(self.path).expect("Failed to open file");
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|l| l.expect("Failed to read line"));
        let lines = lines.inspect(|l| crate::assert_path_rules(&l));
        let paths = lines.collect();
        paths
    }
}

fn file_dir<'a>(path: &'a str) -> &'a path::Path {
    let path = path::Path::new(path);
    let dir = path.parent().unwrap();
    assert_dir_rules(dir);
    dir
}

fn ancestors<'a>(dir: &'a path::Path) -> Vec<&'a path::Path> {
    let ancestors = dir.ancestors();
    let ancestors = ancestors.inspect(|a| assert_dir_rules(a));
    let ancestors = ancestors.collect::<Vec<_>>();
    ancestors
}

#[inline]
fn assert_dir_rules(p: &path::Path) {
    assert!(p != path::Path::new("."), "Path is not valid: {p:?}");
    assert!(p != path::Path::new(""), "Path is not valid: {p:?}");
}



// ============
// === Main ===
// ============

pub fn main(mut args: env::Args) {
    let path = args.next().expect("Path not provided");

    let ancestors = Ancestors::default()
        .path(&path);
    let ancestors = ancestors.ancestors();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    ancestors
        .iter()
        .progress()
        .for_each(|a| {
            write!(handle, "{a}\n").expect("Failed to write to stdout");
        })
}



// =============
// === Tests ===
// =============

#[cfg(test)]
mod tests {
    use crate::ancestors;

    #[test]
    #[ignore]
    fn test_ancestors() {
        const PATH: &str = "./data/files.txt";
        let ancestors = ancestors::Ancestors::default()
            .path(PATH);
        let _ancestors = ancestors.ancestors();
    }
}
