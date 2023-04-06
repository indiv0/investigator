use indicatif::ProgressIterator as _;
use std::collections;
use std::fs;
use std::io;
use std::io::BufRead as _;
use std::path;
use std::str;

// ================
// === DirFiles ===
// ================

#[derive(Clone, Debug, Default)]
pub struct DirFiles<'a> {
    files: &'a str,
}

impl<'a> DirFiles<'a> {
    pub fn files(mut self, files: &'a str) -> Self {
        self.files = files;
        self
    }

    pub fn dir_files(&self) -> Vec<String> {
        // Read the list of files
        let files = self.read_files();

        // Get a mapping of each ancestor -> file
        let ancestors_and_file = files_to_ancestors_and_file(files);
        // Insert the mapping into a BTreeMap to sort it by ancestor path.
        // This also joins the files for each ancestor into a Vec.
        let mut map = collections::BTreeMap::new();
        ancestors_and_file
            .into_iter()
            .progress()
            .for_each(|(a, f)| map.entry(a).or_insert_with(Vec::new).push(f));

        // Flatten the Map<Ancestor, Vec<File>> back into a Vec<(Ancestor, File)>
        let dirs_and_files = map.into_iter().progress();
        let dirs_and_files =
            dirs_and_files.flat_map(|(a, f)| f.into_iter().map(move |f| (a.clone(), f)));
        let dirs_and_files = dirs_and_files.inspect(|(a, f)| {
            assert!(
                !a.contains(crate::UNIQUE_SEPARATOR),
                "Ancestor contains separator: {a:?}"
            );
            assert!(
                !f.contains(crate::UNIQUE_SEPARATOR),
                "File contains separator: {f:?}"
            );
        });
        let dirs_and_files = dirs_and_files
            .map(|(a, f)| format!("{a}{s}{f}", s = crate::UNIQUE_SEPARATOR, a = a, f = f));
        let dirs_and_files = dirs_and_files.collect::<Vec<_>>();
        dirs_and_files
    }

    fn read_files(&self) -> Vec<String> {
        let file = fs::File::open(self.files).expect("Failed to open file");
        let file = io::BufReader::new(file);
        let lines = file.lines().map(|l| l.expect("Failed to read line"));
        let files = lines.inspect(|l| crate::assert_path_rules(&l));
        let files = files.collect();
        files
    }
}

fn files_to_ancestors_and_file(files: Vec<String>) -> Vec<(String, String)> {
    let files = files.iter().progress();
    let dirs_and_files = files.flat_map(|f| {
        // For each file, get it's parent dir
        let dir = file_dir(f);
        // For each file, get it's ancestor dirs (including the parent)
        let ancestors = ancestors(dir);
        let ancestors = ancestors.into_iter().map(|a| crate::path_to_str(a));
        let ancestors = ancestors.map(|s| s.to_string());

        // For each ancestor dir, map it to the file
        let ancestors_and_file = ancestors.map(move |a| (a, f));
        ancestors_and_file
    });
    let dirs_and_files = dirs_and_files.map(|(a, f)| (a, f.to_string()));
    let dirs_and_files = dirs_and_files.collect();
    dirs_and_files
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

pub fn main(path: &str) -> Vec<String> {
    let dir_files = DirFiles::default().files(&path);
    dir_files.dir_files()
}
