//! ```shell,no_run
//! cargo install cargo-watch
//! cargo install dioxus-cli
//! ~/.cargo/bin/dioxus serve
//! ~/.cargo/bin/dioxus build --release
//! rm -f find-files.db
//! ~/.cargo/bin/cargo-watch -s "cargo test --package find-files && cargo run --release --package find-files ~/Desktop/files"
//! > find_by_ext tif,tiff,bmp,jpg,jpeg,gif,png
//!
//! # Links
//!
//! - [Dioxus - Custom Assets](https://github.com/DioxusLabs/dioxus/blob/c113d96bbe0a952f51652f019f5c313ac5c0257b/examples/custom_assets.rs)
//! - [Reddit - Published a Dioxus+TailwindCSS Example](https://old.reddit.com/r/rust/comments/1224elh/published_a_dioxustailwindcss_example_with_up_to/)
//! - [Dioxus - Managing State](https://github.com/DioxusLabs/dioxus/blob/35cb6616af3dd85d2370583d2a2e8d575df23d73/docs/guide/src/en/__unused/index.md)
//! - [Dioxus - Tests For Hooks](https://github.com/DioxusLabs/dioxus/issues/955#issuecomment-1531639013)
//! - [Dioxus - File Explorer Example](https://github.com/DioxusLabs/example-projects/blob/9a59be6f6506a15f868e64b95512dbbd479c9c0c/file-explorer/src/main.rs)
//! ```

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
