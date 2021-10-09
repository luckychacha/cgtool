use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;


pub struct Config {
    pub ids: String,
    pub vs_currencies: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prices {
    #[serde(flatten)]
    extra: SimplePrices,
}

pub type SimplePrices = HashMap<String, SimplePrice>;
pub type SimplePrice = HashMap<String, Decimal>;

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("need 2 parameters, first is ids, second is vs_currencies, comma-seperated.");
        }

        args.next();
        let ids = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get ids"),
        };

        let vs_currencies = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get vs_currencies"),
        };

        Ok(Config {
            ids,
            vs_currencies
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        config.ids,
        config.vs_currencies
    );
    let res = reqwest::blocking::get(url)?
            .json::<Prices>()?;
    println!("res: {:?}", res);
    // let a = reqwest::Client::new().get("https://api.coingecko.com/api/v3/simple/price")
    // .query(&Config::get_query(config));
    Ok(())
}