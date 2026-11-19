# Power-Up Poker Rust Engine

A Rust project for a two-player Texas Hold'em variant with poker power-ups. The
game rules are documented in [rules.md](rules.md); the engine is still under
development and is not yet a playable game.

## Current state

- `src/types.rs` defines cards, decks, power-ups, hands, players, game state,
  and actions. It includes deck creation and shuffling, game setup, and
  preliminary hand start/cleanup logic.
- Power-up application, matching-card draws, and the rest of the game flow are
  unfinished.
- `src/lib.rs` currently contains a basic text-search helper, and the binary in
  `src/main.rs` is a text-search example rather than a poker interface.

## Build and run

This is a Cargo project using Rust 2024 edition. Once the unfinished
implementations are completed, use:

```sh
cargo build
cargo test
```

The current example CLI searches a file for a query. Its defaults are the query
`the` and the file `poem.txt`; for example:

```sh
cargo run -- --query poker --file-path poem.txt
```
