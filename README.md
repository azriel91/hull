# Hull

*Hull* is a build tool to aid Rust development.

This aims to make development life easier with the following environments in mind:

* **Hosts:** Development machine, CI machine
* **Cross OS:** Linux, Windows
* **Crate Type:** Single, Virtual Workspace

## Usage

Single Crates:

```bash
# Compile binaries, tests, and examples:
hull build

# Compile binaries, tests, and examples in release mode:
hull build -- --release

# Compile and specify features:
hull build -- --features "feature1 feature2"
```

Workspaces:

```bash
# Compile binaries, tests, and examples:
hull build

# Compile binaries, tests, and examples in release mode:
hull build -- --release

# Arbitrary command
hull each -- echo $(pwd) # Linux
hull each -- echo %~dp0  # Windows

# Don't do this, because `hull build` automatically detects whether the crate is a workspace:
hull each -- hull build
# If you really need to, you can use this:
hull each -- hull build --no-detect-workspace
```

## Installation

From `crates.io`:

```bash
cargo install hull
```

From source:

```bash
(git clone git@github.com:azriel91/hull.git && cd hull && cargo install)
```

## Status

Currently this is in experimental mode, I will gradually productionize the code quality over time.

Roadmap:

* [ ] Test all code.
* [ ] Use a logging framework.
* [ ] Use `failure` to handle and report failures.
* [ ] Use `indicatif` to reduce log spam when run interactively.

## Motivation

This tool was created out of frustration that I couldn't simply "just run this" and compile all my crate compilables across OSes.

[`cargo-make`][cargo_make] solved *many* of my build issues, but Windows had too many quirks that meant I'd have to code my build steps in both Bash and Batch to allow seamless integration, but Batch is a language you never want to touch *\*shudder\**. So I decided to write a build tool in Rust.

[cargo_make]: https://github.com/sagiegurari/cargo-make

## Questions

* Why not OSX?

    I don't have a Mac currently, else I'd include OS X
