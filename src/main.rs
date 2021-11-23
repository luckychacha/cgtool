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
    match opts.subcmd {
        SubCommand::TokenQuery(ref args) => args.query(),
        SubCommand::PriceQuery(ref args) => args.query(),
    };
}
