use crate::error;
use crate::types::parse_bool;
use clap::Parser;
use reqwest::Client;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub type SimplePrices = HashMap<String, SimplePrice>;
pub type SimplePrice = HashMap<String, Decimal>;

/// Query token price by token names. Such as `cgtool price-query bitcoin,ethereum usd,cny true`
#[derive(Parser, Debug)]
pub struct PriceQuery {
    ids: String,
    vs_currencies: String,
    #[clap(parse(try_from_str = parse_bool))]
    include_24hr_change: bool,
}

pub struct TokenItem {
    key: String,
    value: Decimal,
}

impl From<(&String, &Decimal)> for TokenItem {
    fn from(value: (&String, &Decimal)) -> Self {
        Self {
            key: value.0.clone(),
            value: *value.1,
        }
    }
}

impl std::fmt::Display for TokenItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.key.ends_with("24h_change") {
            true if self.value.ge(&Decimal::ZERO) => writeln!(f, "📈{}: {}", self.key, self.value),
            true => writeln!(f, "📉{}: {}", self.key, self.value),
            false => write!(f, "💰vs_currency: {}, price: {}", self.key, self.value),
        }
    }
}

impl PriceQuery {
    pub async fn query(&self, client: &Client) -> Result<(), error::CgtoolError> {
        let url = match self.include_24hr_change {
            true => format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true",
                self.ids,
                self.vs_currencies
            ),
            false => format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=false",
                self.ids,
                self.vs_currencies
            ),
        };

        let res = client
            .get(url)
            .header("accept", "application/json")
            .header("user-agent", "C")
            .send()
            .await?
            .json::<SimplePrices>()
            .await?;
        for prices in res {
            println!("⭐️token id: {}", prices.0);
            let mut detail = prices
                .1
                .iter()
                .map(|item| format!("{}", Into::<TokenItem>::into(item)))
                .collect::<Vec<String>>();
            detail.sort();
            for item in detail {
                println!("{}", item);
            }
        }
        Ok(())
    }
}
