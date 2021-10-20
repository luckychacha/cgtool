use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub type Tokens = Vec<Token>;

pub enum Configs {
    Price { ids: String, vs_currencies: String },
    Id { symbols: String },
}

// pub struct PriceC { ids: String, vs_currencies: String }
// impl Query for PriceC {
//     fn query(&self) -> Result<(), Box<dyn Error>> {
//         todo!()
//     }
// }

// pub fn query<T: Query>(config: T) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

pub trait Query {
    fn query(&self) -> Result<(), Box<dyn Error>>;
}

impl Query for Configs {
    fn query(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Configs::Price { ids, vs_currencies } => {
                let url = format!(
                    "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true",
                    ids,
                    vs_currencies
                );
                let res = reqwest::blocking::get(url)?.json::<SimplePrices>()?;
                for prices in res {
                    println!("⭐️token id: {}", prices.0);
                    let mut detail: Vec<String> = prices.1
                        .iter()
                        .map(|(key, value)| {
                            if key.ends_with("24h_change") {
                                if value.ge(&Decimal::ZERO) {
                                    return format!("📈{}:{}", key, value);
                                }
                                return format!("📉{}:{}", key, value);
                            }
                            format!("💰vs_currency:{},price:{}", key, value)
                        })
                        .collect::<Vec<String>>();
                    detail.sort();
                    for item in detail {
                        println!("{}", item);
                    }
                    println!("");
                }
                Ok(())
            }
            Configs::Id { symbols } => {
                let url = "https://api.coingecko.com/api/v3/coins/list";
                let res = reqwest::blocking::get(url)?.json::<Tokens>()?;
                let symbols: Vec<&str> = symbols.split(',').collect();
                let result: Vec<&Token> = res
                    .iter()
                    .filter(|&token| symbols.contains(&token.symbol.as_str()))
                    .collect();

                let mut ids = Vec::<&str>::new();
                for token in result {
                    println!(
                        "token id: [{}], symbol: [{}], name: [{}]",
                        token.id, token.symbol, token.name
                    );
                    ids.push(token.id.as_str());
                }
                println!("ids are: {}", ids.join(","));
                Ok(())
            }
        }
    }
}

pub type SimplePrices = HashMap<String, SimplePrice>;
pub type SimplePrice = HashMap<String, Decimal>;

impl Configs {
    pub fn init(mut args: std::env::Args) -> Result<Configs, &'static str> {
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
            return Err(
                "Need 2 parameters to search ids, first is id, second is symbols[comma-seperated].",
            );
        }

        if parameter_nums == 4 && "price" != operate_type {
            return Err("Need 3 parameters to query price, first is price, second is ids[comma-seperated], third is vs_currencies[comma-seperated].");
        }

        let ids = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get ids"),
        };

        if parameter_nums == 3 {
            return Ok(Configs::Id { symbols: ids });
        }

        if parameter_nums == 4 {
            let vs_currencies = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get vs_currencies"),
            };

            return Ok(Configs::Price { ids, vs_currencies });
        }

        Err("Parameter error")
    }
}
