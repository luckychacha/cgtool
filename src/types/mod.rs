use crate::error;

pub mod market_cap;
pub mod tokens_market_cap;
pub mod token;
pub mod token_price;

pub fn parse_bool(s: &str) -> Result<bool, error::CgtoolError> {
    Ok("true" == s)
}
