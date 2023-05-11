//! ```shell,no_run
//! rm -f find-files.db
//! ~/.cargo/bin/cargo-watch -s "cargo test --package find-files && cargo run --release --package find-files ~/Desktop/files"
//! > find_by_ext tif,tiff,bmp,jpg,jpeg,gif,png
//! ```

use dioxus::prelude::*;

use std::process;



// =================
// === Constants ===
// =================

/// Exit code to use when the program fails.
const FAILURE_EXIT_CODE: i32 = 1;



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
    // Launch the dioxus app in a webview.
    dioxus_desktop::launch(App);

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

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
