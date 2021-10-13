use std::collections::HashMap;
use std::error::Error;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub type Tokens = Vec<Token>;

pub struct Config {
    pub ids: String,
    pub vs_currencies: String,
}

pub type SimplePrices = HashMap<String, SimplePrice>;
pub type SimplePrice = HashMap<String, Decimal>;

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        let parameter_nums = args.len();

        if parameter_nums < 3 {
            return Err("Need at least 2 parameters.");
        }
        
        args.next();

        let operate_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get operate type. You can choose id or price"),
        };

        if parameter_nums == 3 && "id" != operate_type {
            return Err("Need 2 parameters to search ids, first is id, second is symbols[comma-seperated].")
        }

        if parameter_nums == 4 && "price" != operate_type {
            return Err("Need 3 parameters to query price, first is price, second is ids[comma-seperated], third is vs_currencies[comma-seperated].")
        }

        let ids = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get ids"),
        };

        if parameter_nums == 3 {
            return Ok(Config {
                ids,
                vs_currencies: String::from("search-token"),
            });
        }
        
        if parameter_nums == 4 {
            let vs_currencies = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get vs_currencies"),
            };

            return Ok(Config {
                ids,
                vs_currencies,
            });
        }

        Err("Parameter error")
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
    Ok(())
}

pub fn search_by_symbol(config: Config) -> Result<(), Box<dyn Error>> {
    let url = "https://api.coingecko.com/api/v3/coins/list";
    let res = reqwest::blocking::get(url)?
            .json::<Tokens>()?;
    let symbols: Vec<&str> = config.ids.split(",").collect();
    let result: Vec<&Token> = res.iter().filter(|&token| {
        symbols.contains(&token.symbol.as_str())
    }).collect();
    
    let mut ids = Vec::<&str>::new();
    for token in result {
        println!("token id: [{}], symbol: [{}], name: [{}]", token.id, token.symbol, token.name);
        ids.push(token.id.as_str());
    }
    println!("ids are: {}", ids.join(","));
    Ok(())

}