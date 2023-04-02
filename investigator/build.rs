use std::{{process, process::Stdio}, error::Error, collections::{HashMap, VecDeque}, marker::PhantomData, io, process::Output};
use cargo::*;
use command::*;


const CARGO: &str = "cargo";
const INSTALL: &str = "install";


macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    p!("Running `build.rs`.");
    // Create a collection to contain the list of instructions that have to be performed as part of
    // this build process.
    let mut instructions: Vec<Instruction> = vec![];

    // The benchmarks require that we have hash programs installed to compare against.
    let b3sum = cargo().install("b3sum").into();
    let openssl = command("openssl").arg("version").into();
    instructions.push(b3sum);
    instructions.push(openssl);

    let instructions = instructions.into_iter();
    instructions.for_each(|instruction| instruction.execute());
}

#[derive(Clone, Debug)]
enum Instruction {
    Command(Command<Args>),
}

impl From<Command<Args>> for Instruction {
    fn from(command: Command<Args>) -> Self {
        Instruction::Command(command)
    }
}

impl Instruction {
    fn execute(self) {
        p!("Executing instruction: {self:?}");
        match self {
            Instruction::Command(command) => {
                let program = command.program();
                let args = command.args();
                let args = args.into_iter();
                let args = args.map(AsRef::as_ref);
                let args = args.collect::<Vec<_>>();
                let args = &args[..];
                execute(program, args);
            },
        }
    }
}



mod cargo {
    use super::*;

    pub fn cargo() -> Cargo<Program> {
        let marker = PhantomData;
        let command = Cargo { marker };
        command
    }

    #[derive(Clone, Debug)]
    pub struct Cargo<K: Kind> {
        marker: PhantomData<K>,
    }

    pub struct Program {}

    pub trait Kind {}
    impl Kind for Program {}

    impl Cargo<Program> {
        pub fn install(self, program: &'static str) -> Command<Args> {
            let cargo = command(CARGO);
            let args = vec![INSTALL, program];
            let args = args.into_iter();
            let args = args.map(ToString::to_string);
            let args = args.collect();
            cargo.args(args)
        }
    }
}



mod command {
    use super::*;

    pub fn command(program: &'static str) -> Command<Program> {
        let state = State { program };
        let extra = Program {};
        let command = Command { state, extra };
        command
    }

    #[derive(Clone, Debug)]
    pub struct Command<K: Kind> {
        state: State,
        extra: K,
    }

    pub struct Program {}
    #[derive(Clone, Debug)]
    pub struct Args {
        args: Vec<String>,
    }

    pub trait Kind {}
    impl Kind for Program {}
    impl Kind for Args {}

    #[derive(Clone, Debug)]
    struct State {
        program: &'static str,
    }

    impl Command<Program> {
        pub fn arg<S>(self, arg: S) -> Command<Args>
        where
            S: ToString,
        {
            let state = self.state;
            let arg = arg.to_string();
            let args = vec![arg];
            let extra = Args { args };
            let command = Command { state, extra };
            command
        }

        pub fn args(self, args: Vec<String>) -> Command<Args> {
            let state = self.state;
            let extra = Args { args };
            let command = Command { state, extra };
            command
        }
    }

    impl Command<Args> {
        pub fn program(&self) -> &'static str {
            let state = &self.state;
            let program = state.program;
            program
        }

        pub fn args(&self) -> &[String] {
            let args = &self.extra.args;
            &args[..]
        }
    }
}



/// Spawns a process from the given program and arguments, then executes it to completion.
fn execute(program: &str, args: &[&str]) {
    let mut command = process::Command::new(program);
    let command = command.args(args);
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
