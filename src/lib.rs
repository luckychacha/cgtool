mod error;
mod types;

pub use types::{
    eth_address::EthPrivateKey, market_cap::MarketCapQuery, token::TokenQuery,
    token_price::PriceQuery, tokens_market_cap::TokensMarketCap, MyClient,
};

pub enum Configs {
    Price { ids: String, vs_currencies: String },
    Id { symbols: String },
}
