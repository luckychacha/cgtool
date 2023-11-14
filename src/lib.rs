mod error;
mod types;

pub use types::{
    market_cap::MarketCapQuery,
    token_price::PriceQuery,
    token::TokenQuery,
    tokens_market_cap::TokensMarketCap,
};

pub enum Configs {
    Price { ids: String, vs_currencies: String },
    Id { symbols: String },
}
