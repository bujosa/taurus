use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    cli::coin::{GetCoinIdArgs, GetMarketsArgs},
    constants::constants::get_url,
};

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

pub fn coin(get_coin_id_args: GetCoinIdArgs) -> Result<CoinByIdResponse, anyhow::Error> {
    let community_data = get_coin_id_args.community_data.to_string();
    let developer_data = get_coin_id_args.developer_data.to_string();
    let sparkline = get_coin_id_args.sparkline.to_string();
    let market_data = get_coin_id_args.market_data.to_string();
    let tickers = get_coin_id_args.tickers.to_string();
    let localization = get_coin_id_args.localization.to_string();

    let binding: &[(&str, &str)] = &[
        ("localization", localization.as_str()),
        ("tickers", tickers.as_str()),
        ("market_data", market_data.as_str()),
        ("community_data", community_data.as_str()),
        ("developer_data", developer_data.as_str()),
        ("sparkline", sparkline.as_str()),
    ];

    let params: Option<&[(&str, &str)]> = Some(&binding);

    let path = format!("coins/{}", get_coin_id_args.id);

    let body: String = ureq::get(&get_url(&path, params)).call()?.into_string()?;

    // The response is a vec of MarketDataResponse
    let res: CoinByIdResponse = serde_json::from_str::<CoinByIdResponse>(&body).unwrap();

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketDataResponse {
    id: String,
    symbol: String,
    name: String,
    image: String,
    current_price: Option<f64>,
    market_cap: Option<f64>,
    market_cap_rank: Option<u32>,
    fully_diluted_valuation: Option<f64>,
    total_volume: Option<f64>,
    high_24h: Option<f64>,
    low_24h: Option<f64>,
    price_change_24h: Option<f64>,
    price_change_percentage_24h: Option<f64>,
    market_cap_change_24h: Option<f64>,
    market_cap_change_percentage_24h: Option<f64>,
    circulating_supply: Option<f64>,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    ath: Option<f64>,
    ath_change_percentage: Option<f64>,
    ath_date: String,
    atl: Option<f64>,
    atl_change_percentage: Option<f64>,
    atl_date: String,
    last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinResponse {
    id: String,
    symbol: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinByIdResponse {
    id: String,
    symbol: String,
    name: String,
    asset_platform_id: Option<String>,
    block_time_in_minutes: Option<u32>,
    hashing_algorithm: Option<String>,
    categories: Option<Vec<String>>,
    public_notice: Option<String>,
    additional_notices: Option<Vec<String>>,
    localization: Option<HashMap<String, String>>,
    description: Option<HashMap<String, String>>,
    links: Option<HashMap<String, String>>,
    image: Option<HashMap<String, String>>,
    country_origin: Option<String>,
    genesis_date: Option<String>,
    sentiment_votes_up_percentage: Option<f32>,
    sentiment_votes_down_percentage: Option<f32>,
    market_cap_rank: Option<u32>,
    coingecko_rank: Option<u32>,
    coingecko_score: Option<f32>,
    developer_score: Option<f32>,
    community_score: Option<f32>,
    liquidity_score: Option<f32>,
    public_interest_score: Option<f32>,
    market_data: Option<HashMap<String, String>>,
    community_data: Option<HashMap<String, String>>,
    developer_data: Option<HashMap<String, String>>,
    public_interest_stats: Option<HashMap<String, String>>,
    status_updates: Option<Vec<String>>,
    last_updated: Option<String>,
    tickers: Option<Vec<String>>,
}

// Add test for all functions
#[cfg(test)]
mod tests {
    use crate::cli::coin::Currencies;

    use super::*;

    #[test]
    fn test_market() {
        let get_market_args = GetMarketsArgs {
            vs_currency: Currencies::Usd,
            order: "market_cap_desc".to_string(),
            per_page: 10,
            page: 1,
            sparkline: false,
            price_change_percentage: "24h".to_string(),
            locate: "en".to_string(),
        };

        let res = markets(get_market_args).unwrap();

        assert_eq!(res.len(), 10);
    }

    #[test]
    fn test_list() {
        let res = list(10).unwrap();

        assert_eq!(res.len(), 10);
    }
}
