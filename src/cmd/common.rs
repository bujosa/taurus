use crate::cli::coin::Currencies;

pub fn currencies_to_string(currencies: Currencies) -> String {
    return match currencies {
        Currencies::Usd => String::from("usd"),
        Currencies::Eur => String::from("eur"),
        Currencies::Gbp => String::from("gbp"),
        Currencies::Jpy => String::from("jpy"),
    };
}
