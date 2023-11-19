use crate::error;
use reqwest::Client;
use std::ops::Deref;

pub mod market_cap;
pub mod token;
pub mod token_price;
pub mod tokens_market_cap;

pub fn parse_bool(s: &str) -> Result<bool, error::CgtoolError> {
    Ok("true" == s)
}

pub struct MyClient {
    pub inner: Client,
}

impl Deref for MyClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl MyClient {
    pub fn get_builder(&self, url: String) -> reqwest::RequestBuilder {
        self.inner
            .get(url)
            .header("accept", "application/json")
            .header("user-agent", "C")
    }
}
