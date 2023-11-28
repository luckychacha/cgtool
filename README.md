# cgtool

## Project Description
`cgtool` is a command-line tool developed in Rust, using the [clap](https://github.com/clap-rs/clap) library to interface with CoinGecko's API. It offers a range of commands for quick access to market cap rankings, real-time prices, and market data of specific tokens, making the analysis and retrieval of cryptocurrency data more direct and efficient.

## Features
- Retrieve real-time price data for a wide range of cryptocurrencies.
- Access market capitalization, trading volume, and other relevant market data.
- User-friendly command-line interface.
- Quick and efficient access to comprehensive cryptocurrency data.

## Usage

0. `cargo build --release` to get the `cgtool`.

To use `cgtool`, run the following commands depending on the information you seek:

```bash

cgtool <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help                Print this message or the help of the given subcommand(s)
    market-cap          Find out which token ranks n-th in terms of market cap. Such as `cgtool
                        market-cap 100 jpy` or `cgtool market-cap 100`, default vs_currency is
                        usd    
    price-query         Query token price by token names. Such as `cgtool price-query
                        bitcoin,ethereum usd,cny true`
    token-market-cap    Query the market cap of a specified tokens.
    token-query         Query token name by symbol. Such as `cgtool token-query btc`
```

## Project Status
Please note that cgtool is currently under active development and is not yet at a 1.0 release. Features and commands are subject to change.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
