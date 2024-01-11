#!/usr/bin/env python3

import sys
from decimal import Decimal
from typing import List

import requests

URL = "https://api.bittrex.com"

SYMBOL_MAP = {
    "ETH": "ETH-USD",
    "USDT": "USDT-USD",
}


def get_prices(symbols: List[str]) -> List[str]:
    """
    Uses the CCXT library to retrieve the prices. Return list of prices
    Args:
        symbols: a list of symbols to get the prices
    Returns:
        the prices from a data source (with the prices sorted in the same sequence of the given symbols)
    """

    r = requests.get(f"{URL}/v3/markets/tickers")
    r.raise_for_status()

    symbol_prices = {
        sym_data["symbol"]: Decimal(sym_data["lastTradeRate"]).normalize()
        for sym_data in r.json()
    }
    return [str(symbol_prices[SYMBOL_MAP[sym]]) for sym in symbols]


def main(symbols: List[str]) -> str:
    if not all(x in SYMBOL_MAP for x in symbols):
        raise Exception("Contains Unsupported Symbols")

    prices = get_prices(symbols)

    if len(prices) != len(symbols):
        raise Exception("Input Length Not Equivalent to Output Length")

    return ",".join(prices)


if __name__ == "__main__":
    try:
        print(main(sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
