use serde::{Deserialize, Serialize};

use crate::constants::constants::get_url;

pub fn ping() -> Result<PingResult, anyhow::Error> {
    let body: String = ureq::get(&get_url("ping", None)).call()?.into_string()?;

    let res = serde_json::from_str::<PingResult>(&body).unwrap();

    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResult {
    pub gecko_says: String,
}
