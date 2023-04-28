use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    pub id: String,
    symbol: String,
    name: String,
    asset_platform_id: Option<String>,
    block_time_in_minutes: Option<u32>,
    hashing_algorithm: Option<String>,
    categories: Option<Vec<String>>,
    public_notice: Option<String>,
    localization: Option<HashMap<String, String>>,
    description: Option<HashMap<String, String>>,
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
    market_data: Option<MarketData>,
    community_data: Option<CommunityData>,
    developer_data: Option<DeveloperData>,
    last_updated: Option<String>,
    tickers: Option<Vec<Ticket>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ticket {
    base: Option<String>,
    target: Option<String>,
    market: Option<Market>,
    last: Option<f64>,
    volume: Option<f64>,
    converted_last: Option<ConvertedPrice>,
    converted_volume: Option<ConvertedVolume>,
    trust_score: Option<String>,
    bid_ask_spread_percentage: Option<f64>,
    timestamp: Option<String>,
    last_traded_at: Option<String>,
    last_fetch_at: Option<String>,
    is_anomaly: Option<bool>,
    is_stale: Option<bool>,
    trade_url: Option<String>,
    token_info_url: Option<String>,
    coin_id: Option<String>,
    target_coin_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Market {
    name: Option<String>,
    identifier: Option<String>,
    has_trading_incentive: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConvertedPrice {
    btc: Option<f64>,
    eth: Option<f64>,
    usd: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConvertedVolume {
    btc: Option<f64>,
    eth: Option<f64>,
    usd: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeveloperData {
    forks: Option<i32>,
    stars: Option<i32>,
    subscribers: Option<i32>,
    total_issues: Option<i32>,
    closed_issues: Option<i32>,
    pull_requests_merged: Option<i32>,
    pull_request_contributors: Option<i32>,
    code_additions_deletions_4_weeks: Option<CodeAdditionsDeletions4Weeks>,
    commit_count_4_weeks: Option<i32>,
    last_4_weeks_commit_activity_series: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]

struct CodeAdditionsDeletions4Weeks {
    additions: Option<i32>,
    deletions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CommunityData {
    facebook_likes: Option<u64>,
    twitter_followers: Option<u64>,
    reddit_average_posts_48h: Option<f64>,
    reddit_average_comments_48h: Option<f64>,
    reddit_subscribers: Option<u64>,
    reddit_accounts_active_48h: Option<u64>,
    telegram_channel_user_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    current_price: HashMap<String, f64>,
    total_value_locked: Option<f64>,
    mcap_to_tvl_ratio: Option<f64>,
    fdv_to_tvl_ratio: Option<f64>,
    roi: Option<f64>,
    ath: HashMap<String, Option<f64>>,
    ath_change_percentage: HashMap<String, Option<f64>>,
    ath_date: HashMap<String, Option<String>>,
    atl: HashMap<String, Option<f64>>,
    atl_change_percentage: HashMap<String, Option<f64>>,
    atl_date: HashMap<String, Option<String>>,
    market_cap: HashMap<String, Option<f64>>,
    market_cap_rank: Option<u32>,
    fully_diluted_valuation: HashMap<String, Option<f64>>,
    total_volume: HashMap<String, Option<f64>>,
    high_24h: HashMap<String, Option<f64>>,
    low_24h: HashMap<String, Option<f64>>,
    price_change_24h: Option<f64>,
    price_change_percentage_24h: Option<f64>,
    price_change_percentage_7d: Option<f64>,
    price_change_percentage_14d: Option<f64>,
    price_change_percentage_30d: Option<f64>,
    price_change_percentage_60d: Option<f64>,
    price_change_percentage_200d: Option<f64>,
    price_change_percentage_1y: Option<f64>,
    market_cap_change_24h: Option<f64>,
    market_cap_change_percentage_24h: Option<f64>,
    price_change_24h_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_1h_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_24h_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_7d_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_14d_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_30d_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_60d_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_200d_in_currency: HashMap<String, Option<f64>>,
    price_change_percentage_1y_in_currency: HashMap<String, Option<f64>>,
    market_cap_change_24h_in_currency: HashMap<String, Option<f64>>,
    market_cap_change_percentage_24h_in_currency: HashMap<String, Option<f64>>,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    circulating_supply: Option<f64>,
    last_updated: String,
}
