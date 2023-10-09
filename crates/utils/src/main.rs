use core::result;
use std::error;
use std::process;



// =================
// === Constants ===
// =================

const CARGO: &str = "cargo";
const COLOR_ALWAYS: &str = "--color=always";
const CHECK: &str = "check";
const TEST: &str = "test";
const BENCH: &str = "bench";
const BUILD: &str = "build";
const RELEASE: &str = "--release";



// =============
// === Error ===
// =============

type Error = Box<dyn error::Error>;



// ==============
// === Result ===
// ==============

type Result<T = ()> = result::Result<T, Error>;



// ===============
// === Command ===
// ===============

macro_rules! command {
    ($command:ident, $($arg:expr),*) => {
        let mut command = std::process::Command::new($command);
        command.args([$( $arg, )*]);
        let status = command.status()?;
        assert!(
            status.success(),
            "Command \"{} {}\" failed",
            stringify!($command),
            stringify!($( $arg )*),
        );
    };
}



// ============
// === Main ===
// ============

fn main() {
    fn main() -> Result {
        command!(CARGO, COLOR_ALWAYS, CHECK);
        command!(CARGO, COLOR_ALWAYS, TEST);
        command!(CARGO, COLOR_ALWAYS, BENCH);
        command!(CARGO, COLOR_ALWAYS, BUILD, RELEASE);
        Ok(())
    }

    if let Err(e) = main() {
        eprintln!("{e}");
        process::exit(1);
    }
}

