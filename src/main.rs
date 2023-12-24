use cgtool::{EthPrivateKey, MarketCapQuery, MyClient, PriceQuery, TokenQuery, TokensMarketCap};
use clap::Parser;
use reqwest::Client;
use std::sync::Arc;

/// A simply tool to query token info.
#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Luckychacha <luckychachaa@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    TokenQuery(TokenQuery),
    PriceQuery(PriceQuery),
    MarketCap(MarketCapQuery),
    TokenMarketCap(TokensMarketCap),
    EthAddressGenerator(EthPrivateKey),
}

#[tokio::main]
async fn main() {
    let client = Arc::new(MyClient {
        inner: Client::new(),
    });
    let opts = Opts::parse();
    let _ = match opts.subcmd {
        SubCommand::TokenQuery(ref args) => args
            .query(&client)
            .await
            .map_err(|e| {
                println!("token query error: {e}");
            })
            .map(|_| println!("token query success.")),
        SubCommand::PriceQuery(ref args) => args
            .query(&client)
            .await
            .map_err(|e| {
                println!("price query error: {e}");
            })
            .map(|_| println!("price query success.")),
        SubCommand::MarketCap(ref args) => args
            .query(&client)
            .await
            .map_err(|e| {
                println!("market cap query error: {e}");
            })
            .map(|_| println!("price query success.")),
        SubCommand::TokenMarketCap(ref args) => args
            .query()
            .map_err(|e| {
                println!("token market cap query error: {e}");
            })
            .map(|_| println!("token market cap query success.")),
    };
}
