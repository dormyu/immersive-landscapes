# Contributing

The project is in its first prototype stage. Read the README and architecture
limits before proposing a large subsystem. Small changes with independently
specified expected terrain or hydrology behavior are easiest to review.

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

Use synthetic fixtures for drainage and geometry. Tests should detect wrong
behavior, including missing output: a loop over zero deltas is not proof that
deltas work. Preserve runoff conservation and avoid repeated basin computation
in column queries. Keep third-party provenance explicit.

Never commit game assets, prepared resources, accounts, access tokens, player
worlds or generated build directories. Python preview tools are development
helpers; everything executed as part of the intended game stays Rust.
