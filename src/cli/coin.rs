use clap::{builder::PossibleValue, command, Args, Parser, Subcommand, ValueEnum};

use crate::cmd::{self, coin::MarketDataResponse};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command()]
pub struct CoinCommand {
    /// Return the list of the coins
    #[arg(long, short)]
    list: Option<bool>,

    #[command(subcommand)]
    command: CoinSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum CoinSubCommand {
    /// Return the list of the market data
    Markets(GetMarketsArgs),
}

#[derive(Args, Debug)]
pub struct GetMarketsArgs {
    /// Indicate the currency to use
    #[arg(long, short, default_value = "usd")]
    pub vs_currency: Currencies,

    /// Indicate the order to use for the list
    #[arg(long, short, default_value = "market_cap_desc")]
    pub order: String,

    /// Indicate the page to use
    #[arg(long, default_value = "1")]
    pub page: u32,

    /// Indicate the number of items to return
    /// The default value is 100
    #[arg(long, default_value = "100")]
    pub per_page: u32,

    /// Indicate the sparkline to use
    /// The default value is false
    #[arg(long, short, default_value = "false")]
    pub sparkline: bool,

    /// Indicate the price change percentage to use
    /// The default value is 24h
    /// Possible values are: 1h, 24h, 7d, 14d, 30d, 200d, 1y
    #[arg(long, short, default_value = "24h")]
    pub price_change_percentage: String,

    /// Indicate the locate to use
    /// The default value is en
    #[arg(long, short, default_value = "en")]
    pub locate: String,
}

#[derive(Debug, Clone)]
pub enum Currencies {
    Usd,
    Eur,
    Gbp,
    Jpy,
}

impl ValueEnum for Currencies {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Currencies::Usd,
            Currencies::Eur,
            Currencies::Gbp,
            Currencies::Jpy,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Currencies::Usd => {
                PossibleValue::new("usd").help("Use the usd currency to get the market data")
            }
            Currencies::Eur => {
                PossibleValue::new("eur").help("Use the eur currency to get the market data")
            }
            Currencies::Gbp => {
                PossibleValue::new("gbp").help("Use the gbp currency to get the market data")
            }
            Currencies::Jpy => {
                PossibleValue::new("jpy").help("Use the jpy currency to get the market data")
            }
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CoinResult {
    Markets(Vec<MarketDataResponse>),
}

pub fn parse(coin_command: CoinCommand) -> Result<CoinResult, anyhow::Error> {
    match coin_command.command {
        CoinSubCommand::Markets(cmd) => cmd::coin::markets(cmd).map(CoinResult::Markets),
    }
}
