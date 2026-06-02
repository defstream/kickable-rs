# Installation

`kickable` ships as both a library crate and a command-line tool.

## Library

Add the crate to your project:

```shell
cargo add kickable
```

## CLI

Install the binary with Cargo:

```shell
cargo install kickable
```

Or download a prebuilt archive for your platform from the
[releases page](https://github.com/defstream/kickable-rs/releases) and place the
binary on your `PATH`. Prebuilt artifacts are published for:

- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-musl`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-gnu`

## Build from source

The toolchain is pinned in `rust-toolchain.toml` (latest stable) and `protoc`
is vendored via `protoc-bin-vendored`, so no system dependencies are required:

```shell
git clone https://github.com/defstream/kickable-rs
cd kickable-rs
make build      # cargo build --release --all-features --locked --all
make install    # cargo install --path . --bin kickable
```

Cross-compiled release binaries are produced with
[cross-rs](https://github.com/cross-rs/cross):

```shell
make cross/build
```
