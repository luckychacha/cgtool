use std::fmt::Display;
use clap::Parser;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use crate::error;

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

impl Display for TokenMarketCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🆔token id: {}", self.id)?;
        writeln!(f, "💰market cap: {}", self.market_cap)?;
        writeln!(f, "🏅️market cap rank: {}", self.market_cap_rank)?;
        Ok(())
    }
}

impl MarketCapQuery {
    pub async fn query(&self) -> Result<(), error::CgtoolError> {
        let page = if self.market_cap_rank % 20 == 0 {
            self.market_cap_rank / 20
        } else {
            self.market_cap_rank / 20 + 1
        };

        // index in current page
        let rank_id = self.market_cap_rank - (page - 1) * 20 - 1;

        let vs_currencies = if let Some(vs_currencies) = &self.vs_currencies {
            vs_currencies
        } else {
            "usd"
        };

        let url = format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency={}&order=market_cap_desc&per_page=20&page={}", vs_currencies, page);
        println!("url: {}", url);
        let response = Self::get_data(&url).await?;
        if let Ok(token_market_cap_info) = Self::parse_result(response, rank_id as usize).await {
            println!("{:?}", token_market_cap_info);
        }

        Ok(())
    }

    async fn get_data(url: &str) -> Result<Response, error::CgtoolError> {
        let client = reqwest::Client::new();

        // let resp = client.get(url).header("accept", "application/json")
        //     .header("user-agent", "C")
        //     .send().await?
        //     .json::<Vec<TokenMarketCap>>()
        //     .await?;
        // println!("resp: {:?}", resp);

        let resp = client.get(url).header("accept", "application/json")
            .header("user-agent", "C")
            .send().await?;
        Ok(resp)
    }

    async fn parse_result(response: Response, rank_id: usize) -> Result<TokenMarketCap, error::CgtoolError> {
        match response.json::<Vec<TokenMarketCap>>().await {
            Ok(tokens) => {
                if let Some(token_market_cap) = tokens.get(rank_id) {
                    Ok(TokenMarketCap {
                        id: token_market_cap.id.clone(),
                        market_cap: token_market_cap.market_cap,
                        market_cap_rank: token_market_cap.market_cap_rank,
                    })
                } else {
                    Err(error::CgtoolError::JsonParseError)
                }
            }
            Err(_) => {
                Err(error::CgtoolError::JsonParseError)
            }
        }
    }
}