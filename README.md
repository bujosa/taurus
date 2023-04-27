# taurus

Taurus is Wraper for coingeko API to get crypto currency data.

## Overview

```sh
A simple cli for coingecko api v3 written in rust

Usage: taurus.exe [OPTIONS] [COMMAND]

Commands:
  coin    Coin command to get the list of the coins and the market data
  simple  Simple command to get the current price of a coin, and other data

Options:
  -p, --ping     Ping the api
  -h, --help     Print help
  -V, --version  Print version

Usage: taurus.exe coin [OPTIONS] [COMMAND]

Commands:
  markets  Return the list of the market data

Options:
  -l, --list <LIST>  Return the list of the coins [default: 0]
  -h, --help         Print help


coin markets -h

Options:
  -v, --vs-currency <VS_CURRENCY>
          Indicate the currency to use [default: usd] [possible values: usd, eur, gbp, jpy]
  -o, --order <ORDER>
          Indicate the order to use for the list [default: market_cap_desc]
      --page <PAGE>
          Indicate the page to use [default: 1]
      --per-page <PER_PAGE>
          Indicate the number of items to return The default value is 100 [default: 100]
  -s, --sparkline
          Indicate the sparkline to use The default value is false
  -p, --price-change-percentage <PRICE_CHANGE_PERCENTAGE>
          Indicate the price change percentage to use The default value is 24h Possible values are: 1h, 24h, 7d, 14d, 30d, 200d, 1y [default: 24h]
  -l, --locate <LOCATE>
          Indicate the locate to use The default value is en [default: en]
  -h, --help
          Print help (see more with '--help')
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

- [ ] Coins:

  - [x] list
  - [x] markets
  - [ ] id

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
