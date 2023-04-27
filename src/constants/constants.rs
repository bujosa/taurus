pub const BASE_URL: &str = "https://api.coingecko.com/api/v3/";

pub fn get_url(path: &str, params: Option<&[(&str, &str)]>) -> String {
    let query_string = match params {
        Some(params) => {
            let params: Vec<String> = params
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect();
            format!("?{}", params.join("&"))
        }
        None => "".to_string(),
    };

    format!("{}{}{}", BASE_URL, path, query_string)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_url() {
        assert_eq!(
            get_url("ping", None),
            "https://api.coingecko.com/api/v3/ping"
        );
    }

    #[test]
    fn test_get_url_with_params() {
        assert_eq!(
            get_url(
                "coins/markets",
                Some(&[("vs_currency", "usd"), ("ids", "bitcoin")])
            ),
            "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=bitcoin"
        );
    }
}
