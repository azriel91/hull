use std::path::{Path, PathBuf};
use std::process::Command;

use cargo_metadata;

use command;

#[derive(Debug)]
pub struct Each<'c> {
    /// Path to the crate manifest
    pub manifest_path: &'c PathBuf,
    /// Arguments to pass through to `cargo build`
    pub command: Vec<String>,
}

impl<'c> Each<'c> {
    fn run_command_in(&self, path: &Path) {
        println!("Crate directory: {}", path.display());

        let mut command = Command::new(&self.command[0]);
        command.args(&self.command[1..]).current_dir(path);

        let mut child = command.spawn().expect("Failed to spawn command.");
        let exit_status = child.wait().expect("Failed to wait on child process.");

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
