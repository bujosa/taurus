use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryMarketDataResponse {
    id: String,
    name: Option<String>,
    market_cap: Option<f64>,
    market_cap_change_24h: Option<f64>,
    content: Option<String>,
    top_3_coins: Vec<String>,
    volume_24h: Option<f64>,
    updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResponse {
    category_id: String,
    name: String,
}
