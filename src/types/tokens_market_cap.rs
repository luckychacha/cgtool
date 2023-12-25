use crate::{error, MyClient};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct TokensMarketCap {
    /// bitcoin, ethereum, solana, etc
    token_ids: String,
    /// usd, eur, jpy, etc
    vs_currencies: Option<String>,
}

impl TokensMarketCap {
    pub fn query(&self, _client: &MyClient) -> Result<(), error::CgtoolError> {
        todo!()
    }
}
