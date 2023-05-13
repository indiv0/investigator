use std::process;
use std::result;

// ===============
// === Exports ===
// ===============

#[macro_use]
mod cargo;

// =================
// === Constants ===
// =================

const CHECK: &str = "check";
const TEST: &str = "test";
#[allow(dead_code)]
const BENCH: &str = "bench";
const BUILD: &str = "build";
const RELEASE: &str = "--release";

// =============
// === Error ===
// =============

type Error = Box<dyn std::error::Error>;

// ==============
// === Result ===
// ==============

type Result<T = ()> = result::Result<T, Error>;

// ============
// === Main ===
// ============

fn main() {
    fn main() -> Result {
        cargo!(CHECK);
        cargo!(TEST);
        //cargo!(BENCH);
        cargo!(BUILD, RELEASE);
        Ok(())
    }

    if let Err(e) = main() {
        eprintln!("{e}");
        process::exit(1);
    }
}

