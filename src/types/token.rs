use clap::Parser;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use crate::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub type Tokens = Vec<Token>;

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
                Ok(response)
            }
            Err(err) => {
                Err(error::CgtoolError::GetDataError {
                    url: url.to_string(),
                    error: err,
                })
            }
        }
    }

    fn parse_tokens(response: Response) -> Result<Tokens, error::CgtoolError> {
        match response.json::<Tokens>() {
            Ok(tokens) => {
                Ok(tokens)
            }
            Err(_) => {
                Err(error::CgtoolError::JsonParseError)
            }
        }
    }
}