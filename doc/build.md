# Build from Source

## Setup the Environment

Install Cargo and Rust compiler.

It is recommended to additionally specify `target-cpu=native`,
if you plan to run binaries on the same machine that builds.

## Get the Source Code

- `git clone https://gitlab.com/blacknet-ninja/blacknet.git`

## Run Automatic Tests

This is an optional step.

- `cargo test`

## Make the Build

- `cargo build --release --bin blacknet-daemon`
- `cargo build --release --bin blacknet-cli`

The built programs are in `./target/release/`
