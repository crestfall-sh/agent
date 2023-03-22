# agent

#### Notes

```sh
# rustup.rs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# rust-analyzer vscode extension
# https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
# ext install rust-lang.rust-analyzer

# initialize project
cargo init

# run project
cargo run

# build project debug
cargo build
sudo ./target/debug/agent

# build project release
cargo build --release
sudo ./target/release/agent
```

#### Dependencies

- https://crates.io/crates/tokio
- https://crates.io/crates/bollard

#### License

MIT