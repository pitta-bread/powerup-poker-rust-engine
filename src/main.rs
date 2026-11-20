use clap::Parser;
use powerup_poker_rust_engine::run;
use std::process;

#[derive(Parser)]
#[command(version, about, name = "PowerUp Poker")]
struct Cli {
    #[arg(long)]
    #[arg(default_value_t = "Player 1".to_string())]
    player_1_name: String,
    #[arg(long)]
    #[arg(default_value_t = "Player 2".to_string())]
    player_2_name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Welcome to PowerUp Poker !");
    if let Err(err) = run(&cli.player_1_name, &cli.player_2_name) {
        eprintln!("Application Error: {err}");
        process::exit(1);
    }
}
