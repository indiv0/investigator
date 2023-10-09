use crate::prelude::*;

use crate::hash;
use crate::key_value;
use crate::render;
use crate::shell::core;
use crate::walkdir;
use miniserde::json;
use std::str;



// =================
// === Constants ===
// =================

const UNIQUE_SEPARATOR: &str = "    ";



// ==============
// === Export ===
// ==============

mod dir_files;
mod dir_hashes;
mod dup_dirs;
pub(crate) mod lines;



// ====================
// === Capabilities ===
// ====================

#[inline]
pub(crate) fn assert_path_rules(p: &str) {
    assert!(!p.contains('\r'), "Unsupported character in path");
    assert!(!p.is_empty(), "Empty path");
    assert_eq!(p, p.trim(), "Extra whitespace in path: {p:?}");
}



// ====================
// === Capabilities ===
// ====================

#[derive(Clone, Debug)]
#[must_use]
pub(crate) struct Capabilities {
    key_value: key_value::KeyValue<Event>,
    walkdir: walkdir::Walkdir,
    render: render::Render<Event>,
    hash: hash::Hash<Event>,
}

impl WithContext<core::Effect, Event> for Capabilities {
    fn new_with_context(
        context: ProtoContext<core::Effect, Event>,
    ) -> Self {
        Self {
            key_value: key_value::KeyValue::from(context.specialize(core::Effect::KeyValue)),
            walkdir: walkdir::Walkdir::from(context.specialize(core::Effect::Walkdir)),
            render: render::Render::from(context.specialize(core::Effect::Render)),
            hash: hash::Hash::from(context.specialize(core::Effect::Hash)),
        }
    }
}



// =============
// === Event ===
// =============

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub enum Event {
    Restore,
    SetState(key_value::protocol::KeyValueOutput),
    Walkdir(PathBuf),
    SetWalkdir(Vec<PathBuf>),
    //Hash(PathBuf),
    SetHash(PathBuf, String),
    DirFiles,
    DirHashes,
    DupDirs,
    None,
}



// =============
// === Model ===
// =============

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[must_use]
pub struct Model {
    pub paths: Vec<String>,
    pub hashes: Vec<(String, String)>,
    pub dir_files: Vec<String>,
    pub dir_hashes: Vec<String>,
    pub dup_dirs: Vec<String>,
}



// =================
// === ViewModel ===
// =================

#[derive(Clone, Debug, Default)]
#[must_use]
pub struct ViewModel {
    pub paths: usize,
    pub hashes: usize,
    pub dir_files: usize,
    pub dir_hashes: usize,
    pub dup_dirs: usize,
}



// ===========
// === App ===
// ===========

#[derive(Clone, Copy, Debug, Default)]
#[must_use]
pub(crate) struct App;

impl App {
    pub(crate) fn update(&self, event: Event, model: &mut Model, caps: &Capabilities) {
        //for path in &model.paths {
        //    assert!(!path.contains('\r'), "Path contains carriage return: {path:?}");
        //    assert!(!path.is_empty(), "Path is empty: {path:?}");
        //    pretty_assertions::assert_eq!(path, path.trim(), "Extra whitespace in path");
        //}
        //for (path, _hash) in &model.hashes {
        //    assert!(!path.contains('\r'), "Path contains carriage return: {path:?}");
        //    assert!(!path.is_empty(), "Path is empty: {path:?}");
        //    pretty_assertions::assert_eq!(path, path.trim(), "Extra whitespace in path");
        //}

        match event {
            Event::Restore => {
                println!("App::update: Restore");
                caps.key_value.read("state", Event::SetState);
            },
            Event::SetState(response) => {
                println!("App::update: SetState");
                if let key_value::KeyValueOutput::Read(Ok(bytes)) = response {
                    let str = str::from_utf8(&bytes).expect("UTF-8");
                    match json::from_str::<Model>(&str) {
                        Ok(m) => {
                            *model = m;
                            println!("Set model state: {}", model.paths.len());
                            caps.render.render()
                        },
                        Err(e) => eprintln!("Deserialization error: {e}."),
                    }
                }
            },
            Event::Walkdir(path) => {
                println!("App::update: Walkdir: {path:?}");
                caps.walkdir.run(path).send(Event::SetWalkdir);
            },
            Event::SetWalkdir(paths) => {
                println!("App::update: SetWalkdir: {:?}", paths.len());

                let str_paths = paths_to_strings(paths.clone());

                // Only set if the state hasn't already been set.
                if model.paths.is_empty() {
                    model.paths = str_paths;
                }

                // If the number of hashes isn't the expected value, then re-hash.
                if model.hashes.len() != model.paths.len() {
                    model.hashes = vec![];
                    for path in &paths {
                        caps.hash.hash(path.clone()).send(Event::SetHash);
                    }
                }

                write_model("state", model, caps);

                // If the number of dir files doesn't match the number of hashes, then update.
                if model.hashes.len() != model.dir_files.len() {
                    self.update(Event::DirFiles, model, caps);
                }

                caps.render.render();
            },
            Event::SetHash(path, hash) => {
                // FIXME [NP]: Clean?
                //println!("App::update: SetHash: {path:?}");
                let path = path.to_str();
                let path = path.expect("UTF-8");
                let path = path.to_string();
                model.hashes.push((path, hash));

                caps.render.render();

                // If this was the last hash, then write the state.
                if model.hashes.len() == model.paths.len() {
                    write_model("state", model, caps);

                    // If the number of dir files doesn't match the number of hashes, then update.
                    if model.hashes.len() != model.dir_files.len() {
                        self.update(Event::DirFiles, model, caps);
                    }
                }
            },
            Event::DirFiles => {
                println!("App::update: DirFiles");

                let dir_files = dir_files::dir_files(&model.paths);
                model.dir_files = dir_files;
                write_model("state", model, caps);

                self.update(Event::DirHashes, model, caps);
            },
            Event::DirHashes => {
                println!("App::update: DirHashes");

                let dir_files = Lines(model.dir_files.clone());
                let hashes = model.hashes.iter().map(|(p, h)| format!("{}{}{}", h, UNIQUE_SEPARATOR, p));
                let hashes = hashes.collect::<Vec<_>>();
                let hashes = Lines(hashes);
                dir_files.verify_paths();
                let dir_hashes = dir_hashes::DirHashes::new(&dir_files, &hashes);
                let dir_hashes = dir_hashes.dir_hashes();
                model.dir_hashes = dir_hashes;
                write_model("state", model, caps);

                self.update(Event::DupDirs, model, caps);
            },
            Event::DupDirs => {
                println!("App::update: DupDirs");

                let dir_hashes= Lines(model.dir_hashes.clone());
                let dup_dirs = dup_dirs::DupDirs::new(&dir_hashes);
                let dup_dirs = dup_dirs.dup_dirs();
                model.dup_dirs = dup_dirs;
                write_model("state", model, caps);
                write_model("dup_dirs", &model.dup_dirs, caps);
            },
            Event::None => {},
        }
    }

    pub(crate) fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            paths: model.paths.len(),
            hashes: model.hashes.len(),
            dir_files: model.dir_files.len(),
            dir_hashes: model.dir_hashes.len(),
            dup_dirs: model.dup_dirs.len(),
        }
    }
}

fn paths_to_strings(paths: Vec<PathBuf>) -> Vec<String> {
    let str_paths = paths.iter();
    let str_paths = str_paths.map(|p| {
        let p = p.to_str();
        let p = p.expect("UTF-8");
        p.to_string()
    });
    let str_paths = str_paths.collect();
    str_paths
}

fn path_to_str(path: &Path) -> &str {
    let path = path.to_str();
    let path = path.expect("UTF-8");
    path
}

fn write_model<T>(key: &str, value: &T, caps: &Capabilities)
where
    T: miniserde::Serialize,
{
    let bytes = json::to_string(value);
    let bytes = bytes.as_bytes();
    let bytes = bytes.to_vec();
    caps.key_value.write(key, bytes, |_| Event::None);
}
