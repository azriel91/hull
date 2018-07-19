use std::env;
use std::path::{Path, PathBuf};

use duct::cmd;
use shell_words::join;

use command;

const CARGO_CMD: &str = "cargo";

#[derive(Debug)]
pub struct Build<'c> {
    /// Path to the crate manifest
    pub manifest_path: &'c PathBuf,
    /// Arguments to pass through to `cargo build`
    pub args: Vec<String>,
}

impl<'c> Build<'c> {
    /// Compiles this crate's sources.
    ///
    /// This is distinct from `cargo test` because crates that do not have tests will not have its
    /// sources compiled.
    fn compile_sources(&self) {
        self.compile_crate(vec!["build"]);
    }

    /// Compiles this crate's tests.
    fn compile_tests(&self) {
        self.compile_crate(vec!["test", "--no-run"]);
    }

    /// Compiles this crate's examples.
    fn compile_examples(&self) {
        self.compile_crate(vec!["build", "--examples"]);
    }

    /// Compiles all crates using the specific subcommand.
    ///
    /// # Parameters
    ///
    /// * `subcommand`: The cargo subcommand to use for compilation
    fn compile_crate(&self, base_args: Vec<&'static str>) {
        let mut args = base_args;
        args.append(
            &mut self
                .args
                .iter()
                .map(|arg| arg.as_str())
                .collect::<Vec<&str>>(),
        );

        println!("Running command: `{} {}`", CARGO_CMD, join(&args));

        let exit_status = cmd(CARGO_CMD, &args)
            .dir(self.manifest_path.parent().unwrap())
            .run()
            .expect("Failed to run command.")
            .status;

        if !exit_status.success() {
            panic!("Failed to execute command.");
        }
    }
}

struct PopDir<P: AsRef<Path>>(P);
impl<P: AsRef<Path>> Drop for PopDir<P> {
    fn drop(&mut self) {
        env::set_current_dir(&self.0).expect("Failed to pop directory.")
    }
}

impl<'c> command::Command for Build<'c> {
    fn run(&mut self) -> Result<(), ()> {
        let current_dir = env::current_dir().expect("Failed to get current directory.");
        let _pop_dir = PopDir(current_dir);

        env::set_current_dir(
            self.manifest_path
                .parent()
                .expect("Failed to get parent directory of manifest path."),
        ).expect("Failed to set current directory for Build command.");

        self.compile_sources();
        self.compile_tests();
        self.compile_examples();

        Ok(())
    }
}
