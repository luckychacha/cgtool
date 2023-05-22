mod error;

use clap::Parser;
use reqwest::blocking::Response;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub type Tokens = Vec<Token>;

pub enum Configs {
    Price { ids: String, vs_currencies: String },
    Id { symbols: String },
}

/// Query token name by symbol. Such as `cgtool token-query btc`
#[derive(Parser, Debug)]
pub struct TokenQuery {
    tokens: String,
}

impl TokenQuery {
    pub fn query(&self) -> Result<(), error::CgtoolError> {
        let url = "https://api.coingecko.com/api/v3/coins/list";
        let response = Self::get_data(url)?;
        let symbols: Vec<&str> = self.tokens.split(',').collect();
        Self::parse_tokens(response)?
            .iter()
            .filter(|&token| symbols.contains(&token.symbol.as_str()))
            .for_each(|token| {
                println!(
                    "token id: [{}], symbol: [{}], name: [{}]",
                    token.id, token.symbol, token.name
                );
            });
        Ok(())
    }

    fn get_data(url: &str) -> Result<reqwest::blocking::Response, error::CgtoolError> {
        let res: Result<reqwest::blocking::Response, reqwest::Error> = reqwest::blocking::get(url);

        match res {
            Ok(response) => {
                return Ok(response);
            }
            Err(err) => {
                return Err(error::CgtoolError::GetDataError {
                    url: url.to_string(),
                    error: err,
                });
            }
        }
    }

    fn parse_tokens(response: Response) -> Result<Tokens, error::CgtoolError> {
        match response.json::<Tokens>() {
            Ok(tokens) => {
                return Ok(tokens);
            }
            Err(_) => {
                return Err(error::CgtoolError::JsonParseError);
            }
        }
    }
}

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
            value: value.1.clone(),
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
    pub fn query(&self) -> Result<(), error::CgtoolError> {
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

        let res = reqwest::blocking::get(url)
            .unwrap()
            .json::<SimplePrices>()
            .unwrap();
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

pub fn parse_bool(s: &str) -> Result<bool, error::CgtoolError> {
    Ok("true" == s)
}

pub type SimplePrices = HashMap<String, SimplePrice>;
pub type SimplePrice = HashMap<String, Decimal>;
