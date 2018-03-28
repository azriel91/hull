# Hull

*Hull* is a build tool to aid Rust development.

This aims to make development life easier with the following environments in mind:

* **Hosts:** Development machine, CI machine
* **Cross OS:** Linux, Windows
* **Crate Type:** Single, Virtual Workspace

## Status

Currently this is in "code it fast" mode, I will gradually productionize the code quality over time.

## Motivation

This tool was created out of frustration that I couldn't simply "just run this" and compile all my crate compilables across OSes.

[`cargo-make`][cargo_make] solved *many* of my build issues, but Windows had too many quirks that meant I'd have to code my build steps in both *bash* and *batch* to allow seamless integration, but *batch* is a language you never want to touch *\*shudder\**. So I decided to write a build tool in Rust.

[cargo_make]: https://github.com/sagiegurari/cargo-make

## Questions

* Why not OSX?

    I don't have a Mac currently, else I'd include OS X
