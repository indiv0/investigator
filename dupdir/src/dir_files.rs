use indicatif::ProgressIterator as _;
use std::collections;
use std::path;
use std::str;

// ================
// === DirFiles ===
// ================

#[derive(Debug)]
pub struct DirFiles<'a> {
    files: &'a crate::Lines,
}

impl<'a> DirFiles<'a> {
    pub fn new(files: &'a crate::Lines) -> Self {
        Self { files }
    }

    pub fn dir_files(&self) -> Vec<String> {
        self.files.verify_paths();

        // Read the list of files
        let crate::Lines(files) = self.files;

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
        dirs_and_files.collect::<Vec<_>>()
    }
}

fn files_to_ancestors_and_file(files: &[String]) -> Vec<(String, String)> {
    let files = files.iter().progress();
    let dirs_and_files = files.flat_map(|f| {
        // For each file, get it's parent dir
        let dir = file_dir(f);
        // For each file, get it's ancestor dirs (including the parent)
        let ancestors = ancestors(dir);
        let ancestors = ancestors.into_iter();
        let ancestors = ancestors.map(crate::path_to_str);
        let ancestors = ancestors.map(|s| s.to_string());

        // For each ancestor dir, map it to the file
        ancestors.map(move |a| (a, f))
    });
    let dirs_and_files = dirs_and_files.map(|(a, f)| (a, f.to_string()));
    dirs_and_files.collect()
}

fn file_dir(path: &str) -> &path::Path {
    let path = path::Path::new(path);
    let dir = path.parent().unwrap();
    assert_dir_rules(dir);
    dir
}

fn ancestors(dir: &path::Path) -> Vec<&path::Path> {
    let ancestors = dir.ancestors();
    let ancestors = ancestors.inspect(|a| assert_dir_rules(a));
    ancestors.collect::<Vec<_>>()
}

#[inline]
fn assert_dir_rules(p: &path::Path) {
    assert!(p != path::Path::new("."), "Path is not valid: {p:?}");
    assert!(p != path::Path::new(""), "Path is not valid: {p:?}");
}

// ============
// === Main ===
// ============

pub fn main(files: &crate::Lines) -> crate::Lines {
    let dir_files = DirFiles::new(files);
    let dir_files = dir_files.dir_files();
    crate::Lines(dir_files)
}
