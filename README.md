# Dotfiles Server

Rust server to handle dotfile configuration profiles.

## Setup

```sh
cargo install diesel_cli --no-default-features --features sqlite
echo "DATABASE_URL=test.db" > .env
diesel migration run
```

## Build

```sh
cargo build
```

## Running

```sh
RUST_LOG=debug target/debug/server -vvv --config ~/.dotfiles/server.conf start
```
