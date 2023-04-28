use crate::{
    cli::category::CategoryOrders,
    constants::constants::get_url,
    models::category::{CategoryMarketDataResponse, CategoryResponse},
};

use super::common::categories_orders_to_string;

pub fn market_data(
    category_orders: CategoryOrders,
) -> Result<Vec<CategoryMarketDataResponse>, anyhow::Error> {
    let vs_currency = categories_orders_to_string(category_orders);

    let binding: &[(&str, &str)] = &[("order", vs_currency.as_str())];

    let params: Option<&[(&str, &str)]> = Some(&binding);

    let body: String = ureq::get(&get_url("coins/categories", params))
        .call()?
        .into_string()?;

    let res: Vec<CategoryMarketDataResponse> =
        serde_json::from_str::<Vec<CategoryMarketDataResponse>>(&body)
            .unwrap()
            .into_iter()
            .collect();

    Ok(res)
}

pub fn list(value: i32) -> Result<Vec<CategoryResponse>, anyhow::Error> {
    let body: String = ureq::get(&get_url("coins/categories/list", None))
        .call()?
        .into_string()?;

    let res: Vec<CategoryResponse> = serde_json::from_str::<Vec<CategoryResponse>>(&body)
        .unwrap()
        .into_iter()
        .take(value as usize)
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_list() {
        let res = list(1).unwrap();
        assert_eq!(res.len(), 1);
    }
}
