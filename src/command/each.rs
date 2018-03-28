use std::path::{Path, PathBuf};

use cargo_metadata;
use duct::cmd;
use which::which;

use command;

#[derive(Debug)]
pub struct Each<'c> {
    /// Path to the crate manifest
    pub manifest_path: &'c PathBuf,
    /// Command to execute
    pub command: PathBuf,
    /// Arguments to pass through to `cargo build`
    pub args: Vec<String>,
}

impl<'c> Each<'c> {
    fn run_command_in(&self, path: &Path) {
        println!("Crate directory: {}", path.display());

        let path_result = which(&self.command);
        let binary_path = match path_result {
            Ok(path) => path,
            Err(msg) => panic!("{}: {}", msg, &self.command.display()),
        };

        let exit_status = cmd(&binary_path, &self.args)
            .run()
            .expect("Failed to run command.")
            .status;

        if !exit_status.success() {
            panic!("Failed to execute command.");
        }
    }
}

impl<'c> command::Command for Each<'c> {
    fn run(&mut self) -> Result<(), ()> {
        let metadata = cargo_metadata::metadata(
            Some(self.manifest_path).as_ref().map(|p| p.as_path()),
        ).expect("Failed to read workspace metadata.");

        metadata
            .packages
            .iter()
            .filter_map(|package| Path::new(&package.manifest_path).parent())
            .for_each(|crate_dir| self.run_command_in(&crate_dir));

        Ok(())
    }
}
