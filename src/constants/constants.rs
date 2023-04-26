pub const BASE_URL: String = "https://api.coingecko.com/api/v3/".to_string();

pub fn get_url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

#[derive(Debug, Clone)]
pub enum Currencies {
    Usd,
    Eur,
    Gbp,
    Jpy,
}

impl Currencies {
    fn to_string(&self) -> &'static str {
        match self {
            Currencies::Usd => "usd",
            Currencies::Eur => "eur",
            Currencies::Gbp => "gbp",
            Currencies::Jpy => "jpy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_url() {
        assert_eq!(get_url("ping"), "https://api.coingecko.com/api/v3/ping");
    }
}
