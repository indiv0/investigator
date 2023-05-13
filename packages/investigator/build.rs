use std::process;
use std::env;
use std::path;
use std::ops;
use std::error;



// =================
// === Constants ===
// =================

const CARGO: &str = "cargo";
const INSTALL: &str = "install";
const B3SUM: &str = "b3sum";
const OPENSSL: &str = "openssl";

// === Environment Variable Keys ===

const PATH: &str = "PATH";
const HOME: &str = "HOME";

// === Paths ===

const DOT_CARGO_BIN: &str = ".cargo/bin";



// ==============
// === Macros ===
// ==============

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}



// ============
// === Main ===
// ============

fn main() {
    fn main_inner() -> Result<(), Box<dyn error::Error>> {
        p!("Running `build.rs`.");
        // The benchmarks require that we have hash programs installed to compare against.
        let dependencies = [&B3Sum as &dyn Dependency, &OpenSsl];
        install_dependencies(dependencies)?;
        Ok(())
    }

    if let Err(e) = main_inner() {
        eprintln!("Error: {e}");
    }
}

fn install_dependencies(dependencies: impl IntoIterator<Item = &'static dyn Dependency>) -> Result<(), Box<dyn error::Error>> {
    let dependencies = dependencies.into_iter();
    let dependencies = dependencies.map(|program| {
        if !program.installed()? {
            program.install()?
        }
        Ok::<_, Box<dyn error::Error>>(())
    });
    dependencies.collect::<Result<(), _>>()
}



// ==================
// === Dependency ===
// ==================

trait Dependency {
    fn executable_name(&self) -> &'static str;

    fn installed(&self) -> Result<bool, env::VarError> {
        let executable_name = self.executable_name();
        let executable_path = which(executable_name)?;
        let installed = executable_path.is_some();
        Ok(installed)
    }

    fn install(&self) -> Result<(), String> {
        let executable_name = self.executable_name();
        let error = format!("No installation method provided for executable `{executable_name}`.");
        Err(error)
    }
}



// =============
// === b3sum ===
// =============

struct B3Sum;


// === Trait `impl`s ===

impl Dependency for B3Sum {
    fn executable_name(&self) -> &'static str {
        B3SUM
    }

    fn install(&self) -> Result<(), String> {
        let executable_name = self.executable_name();
        let command = cargo().install(executable_name);
        command.run();
        Ok(())
    }
}



// ===============
// === OpenSSL ===
// ===============

struct OpenSsl;


// === Trait `impl`s ===

impl Dependency for OpenSsl {
    fn executable_name(&self) -> &'static str {
        OPENSSL
    }
}



// =============
// === Which ===
// =============

fn which(executable_name: &str) -> Result<Option<path::PathBuf>, env::VarError> {
    // Build an iterator of paths to search for the executable in.
    let path = env::var(PATH)?;
    let paths = env::split_paths(&path);
    let extra_paths = extra_search_paths()?;
    let paths = paths.chain(extra_paths);

    let path = which_inner(paths, executable_name);
    Ok(path)
}

fn which_inner(search_paths: impl IntoIterator<Item = path::PathBuf>, executable_name: &str) -> Option<path::PathBuf> {
    let search_paths = search_paths.into_iter();
    let search_paths = search_paths.map(|path| path.join(executable_name));
    let mut paths = search_paths.filter(|path| path.is_file());
    paths.next()
}

fn extra_search_paths() -> Result<impl Iterator<Item = path::PathBuf>, env::VarError> {
    let home = env::var(HOME)?;
    let home = path::PathBuf::from(home);
    let bin = home.join(DOT_CARGO_BIN);

    let paths = [bin].into_iter();
    Ok(paths)
}



// ===============
// === Command ===
// ===============

trait Command {
    /// Spawns a process from the given program and arguments, then executes it to completion.
    fn run(self);
}

impl<T> Command for T where T: ops::DerefMut<Target = process::Command> {
    fn run(mut self) {
        let command: &mut process::Command = self.deref_mut();
        let result = command.output();
        let output = result.expect("Failed to execute command.");
        let exit_status = output.status;
        let stdout = output.stdout;
        let stderr = output.stderr;
        p!("stdout: {}", String::from_utf8_lossy(&stdout));
        p!("stderr: {}", String::from_utf8_lossy(&stderr));
        assert!(exit_status.success(), "Command failed.");
        p!("Command succeeded.");
    }
}



// =============
// === Cargo ===
// =============

struct Cargo(process::Command);


// === Main `impl` ===

impl Cargo {
    fn install(self, program: &'static str) -> CargoInstall {
        let Self(mut inner) = self;
        inner.arg(INSTALL);
        inner.arg(program);
        CargoInstall(inner)
    }
}

fn cargo() -> Cargo {
    let command = process::Command::new(CARGO);
    Cargo(command)
}



// ====================
// === CargoInstall ===
// ====================

struct CargoInstall(process::Command);


// === Trait `impl`s ===

impl ops::Deref for CargoInstall {
    type Target = process::Command;

    fn deref(&self) -> &Self::Target {
        let Self(inner) = self;
        inner
    }
}

impl ops::DerefMut for CargoInstall {
    fn deref_mut(&mut self) -> &mut process::Command {
        let Self(inner) = self;
        inner
    }
}
