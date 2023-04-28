use clap::{builder::PossibleValue, command, Args, Parser, Subcommand, ValueEnum};

use crate::{
    cmd::{self},
    models::coin::{CoinByIdResponse, CoinResponse, MarketDataResponse},
};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command()]
pub struct CoinCommand {
    /// Return the list of the coins
    #[arg(long, short, required = false, default_value = "0")]
    list: i32,

    #[command(subcommand)]
    command: Option<CoinSubCommand>,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum CoinSubCommand {
    /// Return the list of the market data
    Markets(GetMarketsArgs),

    /// Return one coin by id
    Id(GetCoinIdArgs),
}

#[derive(Args, Debug)]
pub struct GetCoinIdArgs {
    /// Indicate the id of the coin to use
    #[arg(long, short, required = true)]
    pub id: String,

    /// Indicate the localization to use
    /// The default value is true
    #[arg(long, short, default_value = "true")]
    pub localization: Option<bool>,

    /// Indicate if the tickers should be returned
    /// The default value is true
    #[arg(long, short, default_value = "true")]
    pub tickers: Option<bool>,

    /// Indicate if the market data should be returned
    /// The default value is true
    #[arg(long, short, default_value = "true")]
    pub market_data: Option<bool>,

    /// Indicate if the community data should be returned
    /// The default value is true
    #[arg(long, short, default_value = "true")]
    pub community_data: Option<bool>,

    /// Indicate if the developer data should be returned
    /// The default value is true
    #[arg(long, short, default_value = "true")]
    pub developer_data: Option<bool>,

    /// Indicate if the sparkline should be returned
    /// The default value is false
    /// The sparkline is only available for the last 7 days
    #[arg(long, short, default_value = "false")]
    pub sparkline: Option<bool>,
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
    Coins(Vec<CoinResponse>),
    Coin(CoinByIdResponse),
}

pub fn parse(coin_command: CoinCommand) -> Result<CoinResult, anyhow::Error> {
    if coin_command.list > 0 {
        return Ok(CoinResult::Coins(cmd::coin::list(coin_command.list)?));
    }

    println!("{:?}", coin_command.command);

    match coin_command.command {
        Some(CoinSubCommand::Markets(cmd)) => cmd::coin::markets(cmd).map(CoinResult::Markets),
        Some(CoinSubCommand::Id(cmd)) => cmd::coin::coin(cmd).map(CoinResult::Coin),
        None => todo!(),
    }
}
