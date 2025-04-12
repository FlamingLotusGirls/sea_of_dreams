# Sea Of Dreams new system

The system in the parent directory is just a copy of Serenity so I am creating a new folder with a new unified system for fire, DMX lights, and LEDs.

This system will use Rust because it is the language I am most familiar with, has pretty good libraries out there, and has a very easy installation process for new collaborators.

## Running

Follow the installation instructions at https://www.rust-lang.org/, then go into the appropriate directory (e.g. `server`) and execute `cargo run` or otherwise specified by the README in that directory.

If it's slow, execute `cargo run --release` for faster program execution at the cost of slower compilation.

## Development

### Toolchain Installation:

- Install Rust at https://www.rust-lang.org/
  - You can verify it's installed by executing `cargo --version` in a command line. Cargo is both the package manager and build tool for Rust.
  - Another tool this will install is Rustup. Over time you can execute `rustup update` (from any directory) to update the Rust toolchain on your system. Rust is generally strongly backwards-compatible, as long as you are not using the nightly toolchain. So you should still be able to run this software with newer Rust toolchain versions.
- Recommended text editor / IDE is Visual Studio Code (a.k.a. VS Code). If you are using it, install the `rust-analyzer` plugin in VS Code. rust-analyzer should be available for other IDEs as well.
  - I also recommend the (free version of) Dependi plugin for easily identifying the newest version of dependencies to add to Cargo.toml.

### Running the software:

- Running from command line:
  - `cd server` (or whichever directory contains `Cargo.toml`) then `cargo run`.
- Running from VS Code:
  - With rust-analyzer installed, you can do the shortcut ctrl+shift+B / cmd+shift+B which brings up the build tasks, then choose "server: cargo run". These tasks are configured in `server/.vscode/tasks.json`. This should have the same result as running it on the command line.

You can then edit any file ending in `.rs` (or `Cargo.toml` for adding dependencies from https://crates.io/) and run the `cargo run` command / "server: cargo run" task again.
