#![deny(missing_docs, missing_debug_implementations)]

//! *Hull* is a build tool for Rust projects, written in Rust.
//!
//! This tool aims to make development life easier with the following environments in mind:
//!
//! * **Hosts:** Development machine, CI machine
//! * **Cross OS:** Linux, Windows
//! * **Crate Type:** Single, Virtual Workspace

extern crate cargo_metadata;
extern crate duct;
extern crate shell_words;
#[macro_use]
extern crate structopt;
extern crate which;

mod command;

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "cargo-do", about = "Compiles crates, binaries, and examples in a workspace.",
            raw(global_setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    /// Active cargo manifest.
    #[structopt(long = "manifest-path", default_value = "./Cargo.toml",
                help = "Path to the workspace or crate Cargo.toml", parse(from_os_str))]
    manifest_path: Option<PathBuf>,

    /// Subcommand to run.
    #[structopt(subcommand)]
    cmd: command::Entry,
}

fn main() {
    let opt = Opt::from_args();
    let manifest_path = opt.manifest_path.unwrap();

    opt.cmd
        .command(&manifest_path)
        .run()
        .expect("Failed to execute command");
}
