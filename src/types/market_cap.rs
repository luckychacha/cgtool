use crate::error;
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Find out which token ranks n-th in terms of market cap. Such as `cgtool market-cap 100 jpy`
/// or `cgtool market-cap 100`, default vs_currency is usd
#[derive(Parser, Debug)]
pub struct MarketCapQuery {
    /// 5, 10, 125, etc
    market_cap_rank: u32,
    /// usd, eur, jpy, etc
    vs_currencies: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenMarketCap {
    // bitcoin, ethereum, solana, etc
    id: String,
    market_cap: u64,
    market_cap_rank: u32,
}

type MarketCapQueryParams<'a> = (u32, usize, &'a str);

impl<'a> From<&'a MarketCapQuery> for MarketCapQueryParams<'a> {
    fn from(market_cap_query: &'a MarketCapQuery) -> Self {
        let page = if market_cap_query.market_cap_rank % PAGE_SIZE == 0 {
            market_cap_query.market_cap_rank / PAGE_SIZE
        } else {
            market_cap_query.market_cap_rank / PAGE_SIZE + 1
        };
        let rank_id = market_cap_query.market_cap_rank - (page - 1) * PAGE_SIZE - 1;

        (
            page,
            rank_id as usize,
            market_cap_query.vs_currencies.as_deref().unwrap_or("usd"),
        )
    }
}

const PAGE_SIZE: u32 = 20;

impl Display for TokenMarketCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🆔token id: {}", self.id)?;
        writeln!(f, "💰market cap: {}", self.market_cap)?;
        writeln!(f, "🏅️market cap rank: {}", self.market_cap_rank)?;
        Ok(())
    }
}

impl MarketCapQuery {
    pub async fn query(&self, client: &Client) -> Result<(), error::CgtoolError> {
        self.query_market_cap(client).await
    }

    async fn query_market_cap(&self, client: &Client) -> Result<(), error::CgtoolError> {
        let (page, rank_id, vs_currency): MarketCapQueryParams = self.into();

        let url = format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency={}&order=market_cap_desc&per_page={}&page={}", vs_currency, PAGE_SIZE, page);

        match client
            .get(url)
            .header("accept", "application/json")
            .header("user-agent", "C")
            .send()
            .await?
            .json::<Vec<TokenMarketCap>>()
            .await
        {
            Ok(tokens) => {
                if let Some(token_market_cap) = tokens.get(rank_id) {
                    println!("{token_market_cap}");
                    Ok(())
                } else {
                    Err(error::CgtoolError::JsonParseError)
                }
            }
            Err(_) => Err(error::CgtoolError::JsonParseError),
        }
    }
}
