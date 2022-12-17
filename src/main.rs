use clap::{Parser, Subcommand};
use tokio;

mod pull;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    //pull the quesiton and input for a given AoC
    Pull {
        // year: u32
        year: u32,
        // day: u32
        day: u32,
    },
    Run {
        // year: u32
        year: u32,
        // day: u32
        day: u32,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Pull { year, day }) => {
            pull::pull(*year, *day).await;
        }
        Some(Commands::Run { year, day }) => {
            println!("run {} {}", year, day);
        }
        None => {}
    }
}
