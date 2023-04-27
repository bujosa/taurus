use crate::cli::{category::CategoryOrders, coin::Currencies};

pub fn currencies_to_string(currencies: Currencies) -> String {
    return match currencies {
        Currencies::Usd => String::from("usd"),
        Currencies::Eur => String::from("eur"),
        Currencies::Gbp => String::from("gbp"),
        Currencies::Jpy => String::from("jpy"),
    };
}

pub fn categories_orders_to_string(category_orders: CategoryOrders) -> String {
    return match category_orders {
        CategoryOrders::MarketCapDesc => String::from("market_cap_desc"),
        CategoryOrders::MarketCapAsc => String::from("market_cap_asc"),
        CategoryOrders::NameDesc => String::from("name_desc"),
        CategoryOrders::NameAsc => String::from("name_asc"),
        CategoryOrders::MarketCapChange24hDesc => String::from("market_cap_change_24h_desc"),
        CategoryOrders::MarketCapChange24hAsc => String::from("market_cap_change_24h_asc"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currencies_to_string() {
        assert_eq!(currencies_to_string(Currencies::Usd), String::from("usd"));
        assert_eq!(currencies_to_string(Currencies::Eur), String::from("eur"));
        assert_eq!(currencies_to_string(Currencies::Gbp), String::from("gbp"));
        assert_eq!(currencies_to_string(Currencies::Jpy), String::from("jpy"));
    }

    #[test]
    fn test_categories_orders_to_string() {
        assert_eq!(
            categories_orders_to_string(CategoryOrders::MarketCapDesc),
            String::from("market_cap_desc")
        );
        assert_eq!(
            categories_orders_to_string(CategoryOrders::MarketCapAsc),
            String::from("market_cap_asc")
        );
        assert_eq!(
            categories_orders_to_string(CategoryOrders::NameDesc),
            String::from("name_desc")
        );
        assert_eq!(
            categories_orders_to_string(CategoryOrders::NameAsc),
            String::from("name_asc")
        );
        assert_eq!(
            categories_orders_to_string(CategoryOrders::MarketCapChange24hDesc),
            String::from("market_cap_change_24h_desc")
        );
        assert_eq!(
            categories_orders_to_string(CategoryOrders::MarketCapChange24hAsc),
            String::from("market_cap_change_24h_asc")
        );
    }
}
