use std::{process};
use cgtool::{Config, run};

fn main() {
    let config: Config = Config::new(std::env::args()).unwrap_or_else(|message| {
        eprintln!("Problem parsing arguments: {}", message);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
