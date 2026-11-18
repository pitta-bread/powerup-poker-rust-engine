use clap::Parser;
use powerup_poker_rust_engine::search;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(version, about, name = "PowerUp Poker")]
struct Cli {
    #[arg(long)]
    #[arg(default_value_t = "the".to_string())]
    query: String,
    #[arg(long)]
    #[arg(default_value_t = "poem.txt".to_string())]
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Searching for {}", cli.query);
    println!("In file {}", cli.file_path);
    if let Err(err) = run(cli) {
        println!("Application Error: {err}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(cli.file_path).expect("Should have been able to read the file");

    for line in search(&cli.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
