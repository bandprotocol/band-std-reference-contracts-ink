#!/usr/bin/env python3

import sys
from decimal import Decimal
from typing import List

import requests

URL = "https://api.kraken.com"

SYMBOL_MAP = {
    "AVAX": "AVAXUSD",
    "DAI": "DAIUSD",
    "DOT": "DOTUSD",
    "ETH": "XETHZUSD",
    "SOL": "SOLUSD",
    "USDC": "USDCUSD",
    "USDT": "USDTZUSD",
}


def get_prices(symbols: List[str]) -> List[str]:
    """
    Gets the prices of the requested symbols.

    Args:
        symbols: a list of symbols to get the prices
    Returns:
        the prices from the data source (with the prices sorted in the same sequence of the given symbols)
    """

    r = requests.get(f"{URL}/0/public/Ticker")
    r.raise_for_status()

    price_map = {
        pair: Decimal(data["c"][0]).normalize()
        for pair, data in r.json()["result"].items()
    }

    return [str(price_map[SYMBOL_MAP[sym]]) for sym in symbols]


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
