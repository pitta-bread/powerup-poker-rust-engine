# Power-Up Poker Rust Engine

> [!NOTE]  
> All code written by me. AI was only used in the writing of this README.

A Rust engine and local two-player terminal game for a Texas Hold'em variant
with power-ups. See [rules.md](rules.md) for the game rules.

## Getting started

Install a current stable Rust toolchain with Cargo (the project uses the 2024
edition), then run:

```sh
cargo run
```

The default player names are `Player 1` and `Player 2`. To set them:

```sh
cargo run -- --player-1-name Alice --player-2-name Bob
```

Use `cargo run -- --help` for all CLI options.

## Playing

Players share the terminal and take turns entering an action, optionally
followed by a power-up choice:

| Input | Action |
| --- | --- |
| `f` | Fold |
| `c` | Check, or call an existing bet |
| `b1` | Bet 1 chip |
| `b2` | Bet 2 chips |

If a player has fewer chips than the bet or call requires, they go all-in for
the chips they have. Raising is not allowed.

Power-ups can be used only during post-flop betting, alongside a non-fold
action. Add `p0` or `p1` to choose the first or second hole card shown in the
current hand. For ReDeal, which does not take a card choice, use `p` instead.
For example:

```text
b2
c p1
c p
```

These mean “bet 2”, “check or call and use a power-up on the second hole
card”, and “check or call and use ReDeal”, respectively. Input is
case-insensitive. Invalid moves are rejected and prompted again.

**Privacy:** The CLI prints debug state for both players on every turn,
including both hole cards and unused power-ups. It also displays played
power-ups immediately. It does not enforce the hidden information described
in the rules, so it is not suitable for competitive play.

## Game overview

- Each player starts with 15 chips and antes 1 chip per hand. The first player
  to act alternates between hands.
- Each player receives two hole cards and one power-up. There are two betting
  rounds: pre-flop and after the three-card flop. The turn and river are dealt
  together after post-flop betting, followed by showdown.
- The power-up deck contains Increase Rank (5), Decrease Rank (5), AutoAce
  (2), PocketPair (2), and ReDeal (6). The decks are rebuilt between hands.
- The best five-card hand wins the pot; equal hands split it. Power-ups can
  produce duplicate cards and five of a kind, which ranks above a straight
  flush. A-2-3-4-5 is a five-high straight.
- The game continues until one player has no chips.

## Using the engine

The library exposes the `engine` module in [src/engine.rs](src/engine.rs).
`GameState::setup_new_game` creates a game, and `start_hand` deals and posts
antes. Submit each action with `step_turn` and handle its `TurnOutcome`:

- `NextPlayer`: continue with the new active player.
- `NextPhase`: call `advance_phase`; if it reaches `Phase::Showdown`, call
  `resolve_showdown` to award the pot and clean up.
- `NextHand`: the fold has already awarded the pot and cleaned up the hand.

Check `game_winner` before starting the next hand. The module also exposes
card, deck, power-up, and hand-ranking types and functions. The terminal game
loop lives in [src/lib.rs](src/lib.rs), with argument parsing in
[src/main.rs](src/main.rs).
