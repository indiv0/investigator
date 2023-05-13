use dioxus::prelude::*;

use std::process;



// ===============
// === Exports ===
// ===============

mod components;
mod model;



// =================
// === Constants ===
// =================

/// Exit code to use when the program fails.
const FAILURE_EXIT_CODE: i32 = 1;
/// Custom index to use instead of the default Dioxus one.
const CUSTOM_INDEX: &str = include_str!("../assets/index.html");



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
    console_error_panic_hook::set_once();

    // Create a configuration for the dioxus app.
    let config = dioxus_desktop::Config::new();
    let config = config.with_custom_index(CUSTOM_INDEX.to_string());
    // Launch the dioxus app in a webview.
    dioxus_desktop::launch_cfg(app, config);

    if let Err(error) = find_files::run() {
        eprintln!("Error: {error:?}.");
        process::exit(FAILURE_EXIT_CODE);
    }
}

/// Initialize logging for the program.
fn init_logging() {
    let wasm_logger_config = wasm_logger::Config::default();
    wasm_logger::init(wasm_logger_config);
}



// ===========
// === App ===
// ===========

fn app(cx: Scope) -> Element {
    let files = use_ref(&cx, || model::FileModel::new());

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
