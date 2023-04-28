use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryMarketDataResponse {
    id: String,
    name: String,
    market_cap: f64,
    market_cap_change_24h: f64,
    content: String,
    top_3_coins: Vec<String>,
    volume_24h: f64,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResponse {
    category_id: String,
    name: String,
}
