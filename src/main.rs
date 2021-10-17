use cgtool::{Configs, Query};
// use cgtool::{query, PriceC};
use std::process;

fn main() {
    let config: Configs = Configs::init(std::env::args()).unwrap_or_else(|message| {
        eprintln!("Problem parsing arguments: {}", message);
        process::exit(1);
    });
    // query(config);
    if let Err(e) = config.query() {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
    // if config.vs_currencies == "search-token" {
    //     if let Err(e) = search_by_symbol(config) {
    //         eprintln!("Application Error: {}", e);
    //         process::exit(1);
    //     }
    // } else {
    //     if let Err(e) = run(config) {
    //         eprintln!("Application Error: {}", e);
    //         process::exit(1);
    //     }
    // }
}
