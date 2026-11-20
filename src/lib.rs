pub mod engine;

use std::error::Error;
use std::io::{Write, stdin, stdout};

use crate::engine::{Action, GameState, Phase, PowerUpUse, TurnOutcome};

pub fn run(player_1_name: &str, player_2_name: &str) -> Result<(), Box<dyn Error>> {
    let mut this_game = GameState::setup_new_game(player_1_name, player_2_name);

    // outer game loop
    while this_game.game_winner().is_none() {
        println!("--- NEW HAND ---");
        this_game.start_hand()?;

        // hand loop
        loop {
            println!("--- NEW PLAYER TURN ---");
            print_debug_state(&this_game);

            // move validation loop
            let current_turn_outcome = loop {
                let active_player_name = &this_game.players[this_game.active_player].name;
                let (action, power_up_use) = prompt_turn(active_player_name);
                match this_game.step_turn(this_game.active_player, action, power_up_use) {
                    Ok(outcome) => break outcome,
                    Err(msg) => println!("Invalid move or other error: {msg}"),
                }
            };

            match current_turn_outcome {
                TurnOutcome::NextPlayer => {}
                TurnOutcome::NextPhase => {
                    this_game.advance_phase()?;
                    if this_game.phase == Phase::Showdown {
                        println!("--- RESULT ---");
                        print_debug_state(&this_game);
                        let result_of_hand = this_game.resolve_showdown()?;
                        println!("The result of this hand is {result_of_hand:?}!");
                        break;
                    }
                }
                TurnOutcome::NextHand => {
                    println!("Someone can't handle the heat... we have a FOLD.");
                    break;
                }
            }
        }
    }

    let winner_idx = this_game.game_winner().unwrap();
    println!(
        "Game winner is {}! Congrats and bye for now.",
        this_game.players[winner_idx].name
    );
    Ok(())
}

fn print_debug_state(game: &GameState) {
    println!(
        "\n[Round {} | Phase: {:?} | Pot: {} | Active: {}]",
        game.round, game.phase, game.table.pot, game.players[game.active_player].name
    );
    println!("Community cards:  {:?}", game.table.community_cards);
    println!("Played power-ups: {:?}", game.table.played_power_ups);
    for (i, p) in game.players.iter().enumerate() {
        let marker = if i == game.active_player { "->" } else { "  " };
        println!(
            "{marker} P{} ({}): {} chips | bet: {} | last: {:?} | hand: {:?}",
            i + 1,
            p.name,
            p.chips,
            p.current_bet,
            p.last_action,
            p.hand
        );
    }
}

fn read_line(player_name: &str) -> String {
    print!("Your turn {player_name}.\nYour move: ");
    stdout().flush().expect("failed to flush stdout");
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("failed to read stdin");
    user_input.trim().to_lowercase()
}

fn prompt_turn(player_name: &str) -> (Action, Option<PowerUpUse>) {
    loop {
        let input = read_line(player_name);
        let mut parts = input.split_whitespace();

        let action = match parts.next() {
            Some("f") => Action::Fold,
            Some("c") => Action::CheckOrCall,
            Some("b1") => Action::Bet(1),
            Some("b2") => Action::Bet(2),
            _ => {
                println!("Invalid action. Use f, c, b1, or b2.");
                print_reminder();
                continue;
            }
        };

        let power_up_use = match parts.next() {
            None => None,
            Some("p") => Some(PowerUpUse {
                card_chosen_idx: None,
            }),
            Some("p0") => Some(PowerUpUse {
                card_chosen_idx: Some(0),
            }),
            Some("p1") => Some(PowerUpUse {
                card_chosen_idx: Some(1),
            }),
            _ => {
                println!("Invalid power-up input. Use p, p0, p1, or leave blank.");
                print_reminder();
                continue;
            }
        };

        if parts.next().is_some() {
            println!("Too many inputs. Example: 'b2 p1' or 'c'.");
            print_reminder();
            continue;
        }

        return (action, power_up_use);
    }
}

fn print_reminder() {
    println!(
        "--- HELP REMINDER ---\nMake sure to read the rules at `rules.md` if you haven't already.\n\
         Actions: f (fold), c (call or check), b1 b2 (bet 1 or 2 chips).\n\
         If post-flop, you can use a power-up: p, p0, p1 (optional card choice, blank to skip).\n\
         Example 1 (pre-flop): 'b2' -> 'Bet 2 chips'.\n\
         Example 2 (post-flop): 'c p1' -> 'Check and use power-up on right hold card'.\n"
    )
}
