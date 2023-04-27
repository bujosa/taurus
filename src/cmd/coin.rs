use serde::{Deserialize, Serialize};

use crate::{cli::coin::GetMarketsArgs, constants::constants::get_url};

use super::common::currencies_to_string;

pub fn markets(get_market_args: GetMarketsArgs) -> Result<Vec<MarketDataResponse>, anyhow::Error> {
    let vs_currency = currencies_to_string(get_market_args.vs_currency);
    let sparkline = get_market_args.sparkline.to_string();
    let page = get_market_args.page.to_string();
    let per_page = get_market_args.per_page.to_string();

    let binding: &[(&str, &str)] = &[
        ("vs_currency", vs_currency.as_str()),
        ("order", get_market_args.order.as_str()),
        ("per_page", per_page.as_str()),
        ("page", page.as_str()),
        ("sparkline", sparkline.as_str()),
        (
            "price_change_percentage",
            get_market_args.price_change_percentage.as_str(),
        ),
        ("locate", get_market_args.locate.as_str()),
    ];

    let params: Option<&[(&str, &str)]> = Some(&binding);

    let body: String = ureq::get(&get_url("coins/markets", params))
        .call()?
        .into_string()?;

    // The response is a vec of MarketDataResponse
    let res: Vec<MarketDataResponse> = serde_json::from_str::<Vec<MarketDataResponse>>(&body)
        .unwrap()
        .into_iter()
        .collect();

    Ok(res)
}

pub fn list(value: i32) -> Result<Vec<CoinResponse>, anyhow::Error> {
    let body: String = ureq::get(&get_url("coins/list", None))
        .call()?
        .into_string()?;

    let res: Vec<CoinResponse> = serde_json::from_str::<Vec<CoinResponse>>(&body)
        .unwrap()
        .into_iter()
        .take(value as usize)
        .collect();

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketDataResponse {
    id: String,
    symbol: String,
    name: String,
    image: String,
    current_price: f64,
    market_cap: f64,
    market_cap_rank: u32,
    fully_diluted_valuation: Option<f64>,
    total_volume: f64,
    high_24h: f64,
    low_24h: f64,
    price_change_24h: f64,
    price_change_percentage_24h: f64,
    market_cap_change_24h: f64,
    market_cap_change_percentage_24h: f64,
    circulating_supply: f64,
    total_supply: f64,
    max_supply: Option<f64>,
    ath: f64,
    ath_change_percentage: f64,
    ath_date: String,
    atl: f64,
    atl_change_percentage: f64,
    atl_date: String,
    last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinResponse {
    id: String,
    symbol: String,
    name: String,
}
