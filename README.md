# taurus

Taurus is Wraper for coingeko API to get crypto currency data. It is written in rust and it is a cli.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![Actions Status](https://github.com/bujosa/taurus/actions/workflows/rust.yml/badge.svg)

## Overview

```sh
A simple cli for coingecko api v3 written in rust

Usage: taurus [OPTIONS] [COMMAND]

Commands:
  category  Category command to get the list of the categories and the market data for each category
  coin      Coin command to get the list of the coins and the market data
  simple    Simple command to get the current price of a coin, and other data

Options:
  -p, --ping     Ping the api
  -h, --help     Print help
  -V, --version  Print version

Usage: taurus coin [OPTIONS] [COMMAND]

Commands:
  markets  Return the list of the market data
  id       Return one coin by id

Options:
  -l, --list <LIST>  Return the list of the coins [default: 0]
  -h, --help         Print help
  

Usage: taurus category [OPTIONS]

Options:
  -l, --list <LIST>                Return the list of the categories [default: 0]
  -m, --market-data <MARKET_DATA>  Return the list of category with the market data ordered by market cap desc by default [default: market_cap_desc] [possible values: market_cap_desc, market_cap_asc, name_desc, name_asc, market_cap_change_24h_desc, market_cap_change_24h_asc]
  -h, --help                       Print help
```

## Installation

```bash
cargo install --path .
```

## Examples

```bash
taurus --help
```

## Work in progress

- [x] Ping

- [x] Coins:

  - [x] list
  - [x] markets
  - [x] id

- [x] Categories:

  - [x] list
  - [x] categories

- [ ] Exchanges:

  - [ ] exchanges
  - [ ] list
  - [ ] id
  - [ ] volume_chart

- [ ] Simple:
  - [ ] price
  - [ ] token_price
  - [ ] supported_vs_currencies

## License

taurus is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in this repository for more information.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.
