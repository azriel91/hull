use std::env;
use std::path::{Path, PathBuf};

use cargo_metadata::{self, Metadata};

use command;

#[derive(Debug, StructOpt)]
pub enum Entry {
    /// Variant for the `Build` command
    #[structopt(name = "build")]
    Build {
        /// Command to run inside each workspace crate directory.
        #[structopt(long = "no-detect-workspace",
                    help = "Disable detection of whether the current crate is a workspace crate.",
                    parse(from_occurrences = "parse_detect_workspace"))]
        detect_workspace: bool,

        /// Rest of the arguments to pass through to `cargo build`
        #[structopt(help = "Arguments to pass through to `cargo build`.\nSee `cargo help build` for the full list.",
                    raw(allow_hyphen_values = "true"))]
        args: Vec<String>,
    },

    /// Variant for the `Each` command
    #[structopt(name = "each")]
    Each {
        /// Binary to execute.
        #[structopt(help = "Command to execute", parse(from_os_str))]
        command: PathBuf,

        /// Command to run inside each workspace crate directory.
        #[structopt(help = "For workspaces, the command to run inside each crate directory.",
                    raw(allow_hyphen_values = "true"))]
        args: Vec<String>,
    },
}

/// Returns true if the `--no-detect-workspace` flag was not passed.
///
/// # Parameters
///
/// * `occurrences`: the number of occurrences of the flag
fn parse_detect_workspace(occurrences: u64) -> bool {
    // if the user has not passed the flag, we return `true`
    occurrences == 0
}

impl Entry {
    pub fn command<'c>(self, manifest_path: &'c PathBuf) -> Box<command::Command + 'c> {
        let metadata = cargo_metadata::metadata(Some(manifest_path).as_ref().map(|p| p.as_path()))
            .expect("Failed to read workspace metadata.");

        match self {
            command::Entry::Build {
                detect_workspace,
                args: pass_through_args,
            } => {
                if detect_workspace && is_workspace(&metadata) {
                    let mut args = vec!["build", "--no-detect-workspace", "--"]
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    args.extend(pass_through_args);

                    Box::new(command::Each {
                        manifest_path,
                        command: env::current_exe().expect("Failed to get current exe."),
                        args,
                    })
                } else {
                    Box::new(command::Build {
                        manifest_path,
                        args: pass_through_args,
                    })
                }
            }
            command::Entry::Each { command, args } => Box::new(command::Each {
                manifest_path,
                command,
                args,
            }),
        }
    }
}

fn is_workspace(metadata: &Metadata) -> bool {
    !(metadata.packages.len() == 1
        && Path::new(&metadata.workspace_root)
            .join("Cargo.toml")
            .as_path() == Path::new(&metadata.packages[0].manifest_path))
}
