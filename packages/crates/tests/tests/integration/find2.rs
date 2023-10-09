use crate::prelude::*;

use find_files_core::request;
use find_files_core::walkdir::WalkdirResponse;
use std::process;
use std::str;



// =================
// === Constants ===
// =================

/// Path to the directory to use as the root of the recursive directory search.
const ROOT: &str = "/Users/indiv0/Desktop";
/// Accepted characters in paths aside from ASCII.
const ALLOWED_PATH_CHARS: &str = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя";



// ====================
// === find_walkdir ===
// ====================

fn find_walkdir() -> Vec<String> {
    let entries = find_files_shell::walkdir::walkdir(ROOT);
    let entries = entries.map(|e| {
        let path = e.path();
        let path = path.to_str();
        let path = path.expect("UTF-8");
        path.to_string()
    });
    entries.collect()
}



// =================
// === find_unix ===
// =================

fn find_unix() -> Vec<String> {
    let args = vec![ROOT, "-type", "f"];
    let cmd = &mut process::Command::new("find");
    let cmd = cmd.args(args);
    let output = cmd.output();
    let output = output.expect("Output");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = output.stdout;
    let stdout = str::from_utf8(&stdout);
    let stdout = stdout.expect("UTF-8");
    let paths = stdout.lines();
    let paths = paths.map(str::to_string);
    paths.collect()
}



// ======================================
// === test_walkdir_matches_unix_find ===
// ======================================

#[test]
fn test_walkdir_matches_unix_find() {
    pretty_assertions::assert_eq!(find_walkdir(), find_unix());
}



// =======================================
// === test_walkdir_returns_only_files ===
// =======================================

#[test]
fn test_walkdir_returns_only_files() {
    let entries = find_files_shell::walkdir::walkdir(ROOT);
    let entries = entries.filter(|e| !e.file_type().is_file());
    let entries = entries.map(|e| {
        let path = e.path();
        path.to_path_buf()
    });
    let entries = entries.collect::<Vec<_>>();
    pretty_assertions::assert_eq!(Vec::<PathBuf>::new(), entries);
}



// =======================================
// === test_paths_are_ascii_or_russian ===
// =======================================

#[test]
fn test_paths_are_ascii_or_russian() {
    let paths = find_walkdir();
    let paths = paths.iter();
    let paths = paths.filter(|p| {
        let mut chars = p.chars();
        chars.any(|c| !c.is_ascii() && !ALLOWED_PATH_CHARS.contains(c))
    });
    let paths = paths.collect::<Vec<_>>();
    pretty_assertions::assert_eq!(Vec::<&String>::new(), paths);
}



// ================
// === test_app ===
// ================

#[test]
fn test_app() {
    let app = AppTester::default();
    let mut model = Model::default();

    let path = ROOT.into();
    let event = Event::Walkdir(path);
    let mut update = app.update(event, &mut model);
    let mut effects = update.effects_mut();

    assert_let!(Effect::Walkdir(request) = effects.next().unwrap());
    let actual = &request.operation;
    let expected = WalkdirRequest { path: PathBuf::from(ROOT) };
    pretty_assertions::assert_eq!(*actual, expected);

    let path_strs = [
        "/Users/indiv0/Desktop/Screenshot 2023-07-29 at 16.08.19.png",
        "/Users/indiv0/Desktop/Screen Recording 2023-05-12 at 16.06.18.mov",
        "/Users/indiv0/Desktop/Screenshot 2023-08-28 at 05.40.57.png",
        "/Users/indiv0/Desktop/document.pdf",
        "/Users/indiv0/Desktop/Koncertas 2015-04-02 (3).mp4",
        "/Users/indiv0/Desktop/Screenshot 2023-04-17 at 09.40.33.png",
        "/Users/indiv0/Desktop/fototessera.png",
        "/Users/indiv0/Desktop/Screenshot 2023-09-27 at 18.29.56.png",
        "/Users/indiv0/Desktop/Screenshot 2023-04-17 at 20.33.02.png",
        "/Users/indiv0/Desktop/book club/.DS_Store",
    ];
    let paths = path_strs.into_iter();
    let paths = paths.map(PathBuf::from);
    let paths = paths.collect::<Vec<_>>();
    let response = WalkdirResponse { paths: paths.clone() };

    let update = app.resolve(request, response.clone());
    let update = update.expect("Update");
    pretty_assertions::assert_eq!(update.events, vec![Event::SetWalkdir(paths.clone())]);

    let mut hash = 0;

    for event in update.events {
        let mut update = app.update(event, &mut model);

        let mut effects = update.effects_mut();
        for _ in 0..paths.len() {
            // Respond to each `Hash` effect with a `SetHash` event.
            assert_let!(Effect::Hash(request) = effects.next().unwrap());
            let path = request.operation.path.clone();
            let hash_str = hash.to_string();
            let response = HashResponse { hash: hash_str.clone() };
            let update = app.resolve(request, response.clone());
            let update = update.expect("Update");
            assert_let!(None = update.effects().next());

            pretty_assertions::assert_eq!(update.events, vec![Event::SetHash(path, hash_str)]);
            for (idx, event) in update.events.into_iter().enumerate() {
                let mut update = app.update(event, &mut model);
                pretty_assertions::assert_eq!(update.events, vec![]);
                // FIXME [NP]: uncomment?
                //let mut effects = update.effects_mut();
                //if idx == paths.len() - 1 {
                //    assert_let!(Effect::KeyValue(request::Request { operation: KeyValueOperation::Write(_, _), .. }) = effects.next().unwrap());
                //} else {
                //    assert_let!(Effect::KeyValue(request::Request { operation: KeyValueOperation::Write(_, _), .. }) = effects.next().unwrap());
                //    assert_let!(Effect::Render(_) = effects.next().unwrap());
                //}
                //assert_let!(None = effects.next());
            }

            hash += 1;
        }

        // Consume the `Write` effect.
        assert_let!(Effect::KeyValue(request::Request { operation: KeyValueOperation::Write(_, _), .. }) = effects.next().unwrap());

        // Consume the `Render` effect.
        assert_let!(Effect::Render(_) = effects.next().unwrap());
        assert_let!(None = effects.next());
    }
    assert_let!(None = effects.next());

    let mut hash = 0;
    let hashes = path_strs.iter().cloned();
    let hashes = hashes.map(|path| {
        let path = path.to_string();
        let hash_str = hash.to_string();
        hash += 1;
        (path, hash_str)
    });
    let hashes = hashes.collect::<Vec<_>>();

    pretty_assertions::assert_eq!(model.paths, path_strs);
    pretty_assertions::assert_eq!(model.hashes, hashes);
}
