use cgtool::{PriceQuery, TokenQuery};
use clap::Parser;

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
}

fn main() {
    let opts = Opts::parse();
    let _ = match opts.subcmd {
        SubCommand::TokenQuery(ref args) => args
            .query()
            .map_err(|e| {
                println!("token query error: {e}");
            })
            .map(|_| println!("token query success.")),
        SubCommand::PriceQuery(ref args) => args
            .query()
            .map_err(|e| {
                println!("price query error: {e}");
            })
            .map(|_| println!("price query success.")),
    };
}
