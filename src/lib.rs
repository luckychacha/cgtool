use std::collections::HashMap;
use std::error::Error;
use rust_decimal::Decimal;


pub struct Config {
    pub ids: String,
    pub vs_currencies: String,
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
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true",
        config.ids,
        config.vs_currencies
    );
    let res = reqwest::blocking::get(url)?
            .json::<SimplePrices>()?;
    // println!("res: {:?}", res);
    for prices in res {
        println!("token id: {}", prices.0);
        let price_info = prices.1.iter().map(|(key, value)| {
            if key.ends_with("24h_change") {
                return format!(
                    "{}:{}\r\n",
                    key,
                    value
                );
            }
            format!(
                "vs_currency:{},price:{}\r\n",
                key,
                value
            )
        }).collect::<String>();
        println!("{}", price_info);
    }
    // let a = reqwest::Client::new().get("https://api.coingecko.com/api/v3/simple/price")
    // .query(&Config::get_query(config));
    Ok(())
}