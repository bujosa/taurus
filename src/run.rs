use std::{fs::File, io::Write};

use clap::{builder::PossibleValue, command, Parser, Subcommand, ValueEnum};
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
    /// Type of output (console, json)
    #[arg(short, long, default_value = "console")]
    output_type: OutputType,

    /// Output file name (default: output.json)
    #[arg(
        short,
        long,
        help = "Output file",
        required = false,
        default_value = "output"
    )]
    file_name: String,

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

#[derive(Debug, Clone, PartialEq)]
pub enum OutputType {
    /// Json output
    Json,

    /// Output to console
    Console,

    /// Output to text file
    Text,
}

impl ValueEnum for OutputType {
    fn value_variants<'a>() -> &'a [Self] {
        &[OutputType::Console, OutputType::Json, OutputType::Text]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            OutputType::Console => {
                PossibleValue::new("console").help("Output the result in the console")
            }
            OutputType::Json => PossibleValue::new("json").help("Output the result in a json file"),
            OutputType::Text => PossibleValue::new("text").help("Output the result in a text file"),
        })
    }
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

    output(cli.file_name, cli.output_type, res);
}

pub fn output(file_name: String, output_type: OutputType, res: CliResult) {
    if output_type == OutputType::Console {
        println!("{:#?}", res);
        return;
    } else if output_type == OutputType::Text {
        let mut file = File::create(format!("{}.txt", file_name)).unwrap();
        file.write_all(format!("{:#?}", res).as_bytes()).unwrap();
        return;
    }

    let mut file = File::create(format!("{}.json", file_name)).unwrap();
    file.write_all(serde_json::to_string_pretty(&res).unwrap().as_bytes())
        .unwrap();
    return;
}
