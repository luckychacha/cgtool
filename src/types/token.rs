use crate::{error, MyClient};
use clap::Parser;

use serde::{Deserialize, Serialize};

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
    pub async fn query(&self, client: &MyClient) -> Result<(), error::CgtoolError> {
        let url = "https://api.coingecko.com/api/v3/coins/list";
        let tokens = client
            .get_builder(url.into())
            .send()
            .await?
            .json::<Tokens>()
            .await?;
        let symbols: Vec<&str> = self.tokens.split(',').collect();
        tokens
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
}
