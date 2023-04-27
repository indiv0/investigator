// =================
// === Constants ===
// =================

pub(crate) const CARGO: &str = "cargo";
pub(crate) const COLOR_ALWAYS: &str = "--color=always";

// =============
// === Cargo ===
// =============

macro_rules! cargo {
    ($command:ident $(, $arg:expr),*) => {
        let mut cargo = std::process::Command::new($crate::cargo::CARGO);
        cargo.arg($command);
        $(
            cargo.arg($arg);
        )*
        cargo.arg($crate::cargo::COLOR_ALWAYS);
        let status = cargo.status()?;
        assert!(status.success(), "cargo {} {} failed", stringify!($command), stringify!($($arg)*));
    };
}
