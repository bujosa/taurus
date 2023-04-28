use clap::{builder::PossibleValue, command, Parser, ValueEnum};

use serde::Serialize;

use crate::{
    cmd::{self},
    models::category::{CategoryMarketDataResponse, CategoryResponse},
};

#[derive(Parser, Debug)]
#[command()]
pub struct CategoryCommand {
    /// Return the list of the categories
    #[arg(
        long,
        short,
        required = false,
        default_value = "0",
        conflicts_with = "market_data"
    )]
    list: i32,

    /// Return the list of category with the market data ordered by market cap desc by default
    #[arg(
        long,
        short,
        required = false,
        default_value = "market_cap_desc",
        conflicts_with = "list"
    )]
    market_data: CategoryOrders,
}

#[derive(Debug, Clone)]
pub enum CategoryOrders {
    MarketCapDesc,
    MarketCapAsc,
    NameDesc,
    NameAsc,
    MarketCapChange24hDesc,
    MarketCapChange24hAsc,
}

impl ValueEnum for CategoryOrders {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CategoryOrders::MarketCapDesc,
            CategoryOrders::MarketCapAsc,
            CategoryOrders::NameDesc,
            CategoryOrders::NameAsc,
            CategoryOrders::MarketCapChange24hDesc,
            CategoryOrders::MarketCapChange24hAsc,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            CategoryOrders::MarketCapDesc => PossibleValue::new("market_cap_desc"),
            CategoryOrders::MarketCapAsc => PossibleValue::new("market_cap_asc"),
            CategoryOrders::NameDesc => PossibleValue::new("name_desc"),
            CategoryOrders::NameAsc => PossibleValue::new("name_asc"),
            CategoryOrders::MarketCapChange24hDesc => {
                PossibleValue::new("market_cap_change_24h_desc")
            }
            CategoryOrders::MarketCapChange24hAsc => {
                PossibleValue::new("market_cap_change_24h_asc")
            }
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CategoryResult {
    CategoryList(Vec<CategoryResponse>),
    CategoryMarketData(Vec<CategoryMarketDataResponse>),
}

pub fn parse(category_command: CategoryCommand) -> Result<CategoryResult, anyhow::Error> {
    match category_command {
        CategoryCommand { list, .. } if list > 0 => {
            Ok(CategoryResult::CategoryList(cmd::category::list(list)?))
        }
        CategoryCommand { market_data, .. } => Ok(CategoryResult::CategoryMarketData(
            cmd::category::market_data(market_data)?,
        )),
    }
}
