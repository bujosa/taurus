use crate::{
    cli::coin::{GetCoinIdArgs, GetMarketsArgs},
    constants::constants::get_url,
    models::coin::{CoinByIdResponse, CoinResponse, MarketDataResponse},
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
    let community_data = get_coin_id_args
        .community_data
        .expect("community_data not provided")
        .to_string();
    let developer_data = get_coin_id_args
        .developer_data
        .expect("developer_data not provided")
        .to_string();
    let sparkline = get_coin_id_args
        .sparkline
        .expect("sparkline not provided")
        .to_string();
    let market_data = get_coin_id_args
        .market_data
        .expect("market_data not provided")
        .to_string();
    let tickers = get_coin_id_args
        .tickers
        .expect("tickers not provided")
        .to_string();
    let localization = get_coin_id_args
        .localization
        .expect("localization not provided")
        .to_string();

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

    let res: CoinByIdResponse = serde_json::from_str::<CoinByIdResponse>(&body).unwrap();

    Ok(res)
}

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

    #[test]
    fn test_coin() {
        let get_coin_id_args = GetCoinIdArgs {
            id: "bitcoin".to_string(),
            localization: Some(true),
            tickers: Some(true),
            market_data: Some(true),
            community_data: Some(true),
            developer_data: Some(true),
            sparkline: Some(true),
        };

        let res = coin(get_coin_id_args).unwrap();

        assert_eq!(res.id, "bitcoin");
    }
}
