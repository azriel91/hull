use std::path::PathBuf;

use command;

#[derive(Debug, StructOpt)]
pub enum Entry {
    /// Variant for the `build` command
    #[structopt(name = "build")]
    Build {
        /// Rest of the arguments to pass through to `cargo build`
        #[structopt(help = "Arguments to pass through to `cargo build`.\nSee `cargo help build` for the full list.",
                    raw(allow_hyphen_values = "true"))]
        args: Vec<String>,
    },
}

impl Entry {
    pub fn command<'c>(self, manifest_path: &'c PathBuf) -> Box<command::Command + 'c> {
        match self {
            command::Entry::Build { args } => Box::new(command::Build {
                manifest_path,
                args,
            }),
        }
    }
}
