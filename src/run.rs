use clap::{command, Parser, Subcommand};
use serde::Serialize;

use crate::{
    cli::{
        category::{self, CategoryCommand, CategoryResult},
        coin::{self, CoinCommand, CoinResult},
    },
    cmd::ping::{self},
};

#[derive(Parser, Debug)]
#[command(
    author = "bujosa",
    about = "A simple cli for coingecko api v3 written in rust",
    display_name = "taurus",
    disable_help_subcommand = true,
    version
)]
struct Opts {
    /// Ping the api to check if it's online
    #[arg(short, long, help = "Ping the api", required = false)]
    ping: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
#[command()]
enum Command {
    /// Category command to get the list of the categories and the market data
    /// for each category
    #[command()]
    Category(CategoryCommand),

    /// Coin command to get the list of the coins and the market data
    #[command()]
    Coin(CoinCommand),

    /// Simple command to get the current price of a coin, and other data
    #[command(subcommand)]
    Simple(NoSubCommand),
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum NoSubCommand {}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CliResult {
    Coin(CoinResult),
    Category(CategoryResult),
}

pub fn run() {
    let cli = Opts::parse();

    if cli.ping {
        let ping_result = ping::ping().unwrap();
        println!("{}", ping_result.gecko_says);
        return;
    }

    let res = match cli.command {
        Some(Command::Category(cmd)) => {
            let res = category::parse(cmd).unwrap();
            CliResult::Category(res)
        }
        Some(Command::Coin(cmd)) => {
            let res = coin::parse(cmd).unwrap();
            CliResult::Coin(res)
        }
        Some(Command::Simple(_)) => todo!(),
        None => todo!(),
    };

    // Todo export result to json file
    println!("{:?}", res);
}
