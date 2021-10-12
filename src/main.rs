use std::{process};
use cgtool::{Config, run, search_by_symbol};

fn main() {
    let config: Config = Config::new(std::env::args()).unwrap_or_else(|message| {
        eprintln!("Problem parsing arguments: {}", message);
        process::exit(1);
    });
    if config.vs_currencies == "search-token" {
        if let Err(e) = search_by_symbol(config) {
            eprintln!("Application Error: {}", e);
            process::exit(1);
        }
    } else {
        if let Err(e) = run(config) {
            eprintln!("Application Error: {}", e);
            process::exit(1);
        }
    }
}
