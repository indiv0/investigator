use crate::prelude::*;

use crate::shell::app;

pub(crate) fn dir_files(files: &[String]) -> Vec<String> {
    // FIXME [NP]: port this over
    //self.files.verify_paths();

    // Get a mapping of each ancestor -> file
    let ancestors_and_file = files_to_ancestors_and_file(files);
    // Insert the mapping into a BTreeMap to sort it by ancestor path.
    // This also joins the files for each ancestor into a Vec.
    let mut map = BTreeMap::new();
    ancestors_and_file
        .into_iter()
        // FIXME [NP]: add progress bar
        //.progress()
        .for_each(|(a, f)| map.entry(a).or_insert_with(Vec::new).push(f));

    // Flatten the Map<Ancestor, Vec<File>> back into a Vec<(Ancestor, File)>
    let dirs_and_files = map.into_iter();
    // FIXME [NP]: add progress bar
    //let dirs_and_files = dirs_and_files.progress();
    let dirs_and_files =
        dirs_and_files.flat_map(|(a, f)| f.into_iter().map(move |f| (a.clone(), f)));
    let dirs_and_files = dirs_and_files.inspect(|(a, f)| {
        assert!(
            !a.contains(app::UNIQUE_SEPARATOR),
            "Ancestor contains separator: {a:?}"
        );
        assert!(
            !f.contains(app::UNIQUE_SEPARATOR),
            "File contains separator: {f:?}"
        );
    });
    let dirs_and_files = dirs_and_files
        .map(|(a, f)| format!("{a}{s}{f}", s = app::UNIQUE_SEPARATOR, a = a, f = f));
    dirs_and_files.collect::<Vec<_>>()
}

fn files_to_ancestors_and_file(files: &[String]) -> Vec<(String, String)> {
    let files = files.iter();
    // FIXME [NP]: add progress bar
    //let files = files.progress();
    let dirs_and_files = files.flat_map(|f| {
        // For each file, get it's parent dir
        let dir = file_dir(f);
        // For each file, get it's ancestor dirs (including the parent)
        let ancestors = ancestors(dir);
        let ancestors = ancestors.into_iter();
        let ancestors = ancestors.map(app::path_to_str);
        let ancestors = ancestors.map(|s| s.to_string());

        // For each ancestor dir, map it to the file
        ancestors.map(move |a| (a, f))
    });
    let dirs_and_files = dirs_and_files.map(|(a, f)| (a, f.to_string()));
    dirs_and_files.collect()
}

fn file_dir(path: &str) -> &Path {
    let path = Path::new(path);
    let dir = path.parent().unwrap();
    assert_dir_rules(dir);
    dir
}

fn ancestors(dir: &Path) -> Vec<&Path> {
    let ancestors = dir.ancestors();
    let ancestors = ancestors.inspect(|a| assert_dir_rules(a));
    ancestors.collect::<Vec<_>>()
}

#[inline]
fn assert_dir_rules(p: &Path) {
    assert!(p != Path::new("."), "Path is not valid: {p:?}");
    assert!(p != Path::new(""), "Path is not valid: {p:?}");
}
