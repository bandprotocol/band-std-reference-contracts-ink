#!/usr/bin/env python3

import asyncio
import sys
from typing import List

import aiohttp

SOCKET_URL = "wss://ws-feed.pro.coinbase.com"

SYMBOL_MAP = {
    "ATOM": "ATOM-USD",
    "AVAX": "AVAX-USD",
    "DAI": "DAI-USD",
    "DOT": "DOT-USD",
    "ETH": "ETH-USD",
    "SOL": "SOL-USD",
    "USDT": "USDT-USD",
}


async def get_prices(symbols: List[str]) -> List[str]:
    """
    Opens a websocket connection and returns the retrieved prices
    Args:
        symbols: a list of symbols to get the prices
    Returns:
        the prices from a data source (with the prices sorted in the same sequence of the given symbols)
    """

    sym_pairs = [SYMBOL_MAP[sym] for sym in symbols]

    async with aiohttp.ClientSession() as session:
        async with session.ws_connect(SOCKET_URL) as ws:
            await ws.send_json(
                {
                    "type": "subscribe",
                    "product_ids": [SYMBOL_MAP[sym] for sym in symbols],
                    "channels": ["ticker"],
                }
            )
            await ws.receive_json()

            price_map = dict.fromkeys(sym_pairs, None)
            while not all(price_map.values()):
                res = await ws.receive_json()
                price_map[res["product_id"]] = res["price"]

    return [price_map[sym_pair] for sym_pair in sym_pairs]


def main(symbols: List[str]) -> str:
    if not all(x in SYMBOL_MAP for x in symbols):
        raise Exception("Contains Unsupported Symbols")

    prices = asyncio.run(get_prices(symbols))

    if len(prices) != len(symbols):
        raise Exception("Input Length Not Equivalent to Output Length")

    return ",".join(prices)


if __name__ == "__main__":
    try:
        print(main(sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
