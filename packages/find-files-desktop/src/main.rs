use dioxus::prelude::*;
use fermi::prelude::*;

use find_files::sql;
use std::sync;



// ===============
// === Exports ===
// ===============

mod components;
mod model;



// =================
// === Constants ===
// =================

/// Custom index to use instead of the default Dioxus one.
const CUSTOM_INDEX: &str = include_str!("../assets/index.html");
/// Address of the database file to use.
const CONNECTION_ADDRESS: &str = "find-files.db";

static DATABASE_SERVICE: Atom<Option<sync::Arc<sql::Database>>> = |_| None;



// ============
// === Main ===
// ============

/// Entry point of the program.
///
/// # Errors
///
/// If an error occurs, it will be printed to `stderr` and the program will exit with a non-zero
/// exit code.
fn main() {
    init_logging();
    init_panic_hook();

    // Create a configuration for the dioxus app.
    let config = dioxus_desktop::Config::new();
    let config = config.with_custom_index(CUSTOM_INDEX.to_string());
    // Launch the dioxus app in a webview.
    dioxus_desktop::launch_cfg(app, config);
}

/// Initialize logging for the program.
fn init_logging() {
    #[cfg(target = "wasm32-unknown-unknown")]
    let wasm_logger_config = wasm_logger::Config::default();
    #[cfg(target = "wasm32-unknown-unknown")]
    wasm_logger::init(wasm_logger_config);
}

fn init_panic_hook() {
    #[cfg(target = "wasm32-unknown-unknown")]
    console_error_panic_hook::set_once();
}



// =============
// === State ===
// =============

#[derive(Debug)]
enum State {
    Init,
    Loading,
    Ready,
    Done,
}



// ===========
// === App ===
// ===========

fn app(cx: Scope) -> Element {
    use_init_atom_root(&cx);

    let state = use_state(cx, || State::Init);
    let database_service = use_atom_state(cx, DATABASE_SERVICE);
    let args = use_ref(&cx, || find_files::Args::from_env().expect("Failed to parse arguments"));
    //let database = use_ref(&cx, || {
    //    let database_name = &args.read().database_name;
    //    find_files::sql::Database::new(CONNECTION_ADDRESS).await.expect("Failed to create database")
    //});
    let files = use_ref(&cx, || model::FileModel::new());

    if let State::Init = state.get() {
        state.set(State::Loading);
        {
            let database_service = database_service.clone();
            cx.spawn({
                let state = state.clone();
                async move {
                    println!("creating new database");
                    let database = sql::Database::new(CONNECTION_ADDRESS).await.expect("Failed to create database");
                    database_service.set(Some(sync::Arc::new(database)));

                    state.set(State::Ready)
                }
            });
        }
    }

    if let State::Ready = state.get() {
        state.set(State::Done);
        cx.spawn({
            let database_service = database_service.clone();
            let files = files.clone();
            let args = args.clone();
            async move {
                // Populate the database.
                println!("Populating the database");
                let args = args.read();
                let search_directory = &args.search_directory;
                let mut database = database_service.get().clone().unwrap();
                let handle = tokio::spawn({
                    let search_directory = search_directory.clone();
                    async move {
                        find_files::find_inodes(&database, &search_directory).await.expect("Failed to find files");
                    }
                });
                handle.await.expect("Failed to join the handle");

                // Load from the database.
                println!("Loading from the database");
                let database = database_service.get().clone().unwrap();
                let inodes = tokio::spawn(async move {
                    database.select_inodes().await.expect("Failed to select inodes")
                }).await.expect("Failed to join the handle");
                let inodes = inodes
                    .into_iter()
                    .enumerate()
                    .map(|(id, inode)| {
                        let id = id as u32;
                        let path = inode.path;
                        let file_name = inode.file_name;
                        let file = model::FileItem {
                            id,
                            path,
                            file_name,
                        };
                        println!("file_item: {file:?}");
                        (id, file)
                    });
                // FIXME [NP]: don't trim the list of files.
                let inodes = inodes.take(100);
                let mut files = &mut files.write().files;
                files.clear();
                files.extend(inodes);
                println!("Loaded from the database");
            }
        });
    }

    //cx.use_hook(move || {
    //    let search_directory = &args.read().search_directory;
    //    let mut database = database.write();
    //    find_files::find_inodes(&mut database, search_directory).expect("Failed to find inodes");
    //    let inodes = find_files::find_all(&mut database).expect("Failed to find all inodes");
    //    let inodes = inodes
    //        .into_iter()
    //        .enumerate()
    //        .map(|(id, inode)| {
    //            let id = id as u32;
    //            let path = inode.path;
    //            let file_name = inode.file_name;
    //            let file = model::FileItem {
    //                id,
    //                path,
    //                file_name,
    //            };
    //            (id, file)
    //        });
    //    // FIXME [NP]: don't trim the list of TODOs
    //    let inodes = inodes.take(100);
    //    let mut files = &mut files.write().files;
    //    files.clear();
    //    files.extend(inodes);
    //});

    cx.render(rsx! {
        div {
            class: "h-screen flex flex-col",
            components::navbar {}
            div {
                class: "bg-gray-900 flex-1",
                components::file_table {
                    files: files
                }
            }
            components::footer {}
        }
    })
}
