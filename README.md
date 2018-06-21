# Dotfiles Server

Rust server to handle dotfile configuration profiles.

## Build

```sh
cargo build
```

## Running

```sh
RUST_LOG=debug target/debug/server -vvv --config ~/.dotfiles/server.conf start
```
