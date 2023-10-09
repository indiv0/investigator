use crate::prelude::*;

use find_files_core::shell::app;
use find_files_core::channel;
use find_files_core::executor;
use std::mem;
use std::sync;



// =================
// === Constants ===
// =================

/// Root directory to use for the recursive directory search.
const ROOT: &str = "/Users/indiv0/Desktop";



// ===============
// === Prelude ===
// ===============

mod prelude {
    pub(crate) use core::future::Future;
    pub(crate) use core::pin::Pin;
}



// ============
// === Main ===
// ============

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread().build().expect("Runtime");
    let future = run();
    runtime.block_on(future)
}

async fn run() {
    let tasks = vec![];
    let tasks = sync::Mutex::new(tasks);
    let tasks = sync::Arc::new(tasks);
    let spawner_tasks = tasks.clone();
    let spawner = move |f: Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'static>>| {
        let handle = tokio::task::spawn(f);
        spawner_tasks.lock().expect("Lock").push(handle);
    };

    let (render_tx, render_rx) = channel::channel();
    let shell = find_files_shell::Shell::new(spawner, render_tx);
    let path = ROOT.into();
    shell.run(vec![app::Event::Restore]).expect("Run shell");
    shell.run(vec![app::Event::Walkdir(path)]).expect("Run shell");
    loop {
        // Wait for core to settle.
        // Process each render effect here.
        while let Some(_effect) = render_rx.recv() {
            let view = shell.core.view();
            render(view);
        }

        // Wait for any remaining tasks to complete.
        let mut tasks = tasks.lock().expect("Lock");
        let current = mem::take(&mut *tasks);
        let current = current.into_iter().filter(|t| !t.is_finished()).collect();
        *tasks = current;
        if tasks.is_empty() {
            break;
        }
    }
}

fn render(view: app::ViewModel) {
    let app::ViewModel {
        paths,
        hashes,
        dir_files,
        dir_hashes,
        dup_dirs,
    } = view;
    let percent = (hashes as f64 / paths as f64) * 100.0;
    println!("Paths: {paths}, Hashes: {hashes} ({percent:.2}%), Dir files: {dir_files}, Dir hashes {dir_hashes}, Dup dirs {dup_dirs}");
}
