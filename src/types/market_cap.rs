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
    id: String,
    market_cap: u64,
    market_cap_rank: u32,
}

// bitcoin, ethereum, solana, etc

impl MarketCapQuery {
    pub async fn query(&self) -> Result<(), error::CgtoolError> {
        println!("{:?}", self);
        let page = self.market_cap_rank / 20 + 1;

        // index in current page
        let rank_id = self.market_cap_rank - (page - 1) * 20 - 1;
        let url = format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=20&page={}", page);
        let response = Self::get_data(&url).await?;
        if let Ok(token_market_cap_info) = Self::parse_result(response, rank_id as usize).await {
            println!("{:?}", token_market_cap_info);
        }

        Ok(())
    }

    async fn get_data(url: &str) -> Result<Response, error::CgtoolError> {
        let client = reqwest::Client::new();

        let resp = client.get(url).header("accept", "application/json")
            .header("user-agent", "C")
            .send().await?
            .json::<Vec<TokenMarketCap>>()
            .await?;
        println!("resp: {:?}", resp);

        let resp = client.get(url).header("accept", "application/json")
            .header("user-agent", "C")
            .send().await?;
        return Ok(resp);
        // match res {
        //     Ok(response) => {
        //         return Ok(response);
        //     }
        //     Err(err) => {
        //         return Err(error::CgtoolError::GetDataError {
        //             url: url.to_string(),
        //             error: err,
        //         });
        //     }
        // }
    }

    async fn parse_result(response: Response, rank_id: usize) -> Result<TokenMarketCap, error::CgtoolError> {
        match response.json::<Vec<TokenMarketCap>>().await {
            Ok(tokens) => {
                if let Some(tokenMarketCap) = tokens.get(rank_id) {
                    return Ok(TokenMarketCap {
                        id: tokenMarketCap.id.clone(),
                        market_cap: tokenMarketCap.market_cap,
                        market_cap_rank: tokenMarketCap.market_cap_rank,
                    });
                } else {
                    return Err(error::CgtoolError::JsonParseError);
                }
            }
            Err(_) => {
                return Err(error::CgtoolError::JsonParseError);
            }
        }
    }
}