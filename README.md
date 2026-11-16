# Rally Webhook Inbox

Receives Rally partner booking events. One Cargo crate (axum) with subcommands.
Court data lives in `data/courts.json`.

## Requirements

Rust via rustup. The version is pinned in `rust-toolchain.toml`.

## Run

```bash
PORT=3000 cargo run -- serve      # GET http://localhost:3000/health
```

## Test

```bash
cargo test
```
