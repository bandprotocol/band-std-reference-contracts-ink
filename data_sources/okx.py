#!/usr/bin/env python3

import asyncio
import statistics
import sys
from decimal import Decimal
from typing import List

import aiohttp
import requests

URL = "https://www.okx.com/api/v5/market/tickers"

SYMBOL_MAP = {
    "ASTR": "ASTR-USDT",
    "AVAX": "AVAX-USDT",
    "DOT": "DOT-USDT",
    "ETH": "ETH-USDT",
    "FTM": "FTM-USDT",
    "SOL": "SOL-USDT",
    "USDC": "USDC-USDT",
}


async def get_usdt_usd_rate() -> Decimal:
    """
    Gets the USDT/USD rate
    Returns:
        The USDT/USD rate
    """

    async def coingecko():
        async with aiohttp.ClientSession() as session:
            res = await session.get(
                "https://api.coingecko.com/api/v3/simple/price",
                params={"ids": "tether", "vs_currencies": "usd"},
            )
            return str((await res.json())["tether"]["usd"])

    async def cryptocompare():
        async with aiohttp.ClientSession() as session:
            res = await session.get(
                "https://min-api.cryptocompare.com/data/price",
                params={"fsym": "USDT", "tsyms": "USD"},
            )
            return str((await res.json())["USD"])

    async def kraken():
        async with aiohttp.ClientSession() as session:
            res = await session.get(
                "https://api.kraken.com/0/public/Ticker",
                params={"pair": "USDTZUSD"},
            )
            return (await res.json())["result"]["USDTZUSD"]["c"][0]

    async def bitfinex():
        async with aiohttp.ClientSession() as session:
            res = await session.get(
                "https://api-pub.bitfinex.com/v2/ticker/tUSTUSD",
            )
            return str((await res.json())[9])

    async def coinbase():
        async with aiohttp.ClientSession() as session:
            res = await session.get(
                "https://api.exchange.coinbase.com/products/USDT-USD/stats",
            )
            return (await res.json())["last"]

    tasks = [
        asyncio.create_task(coingecko()),
        asyncio.create_task(cryptocompare()),
        asyncio.create_task(kraken()),
        asyncio.create_task(bitfinex()),
        asyncio.create_task(coinbase()),
    ]

    await asyncio.wait([asyncio.gather(*tasks, return_exceptions=True)], timeout=3)

    return statistics.median(
        [Decimal(t.result()) for t in tasks if t.done() and not t.exception()]
    )


async def get_prices(symbols: List[str]) -> List[str]:
    """
    Uses the CCXT library to retrieve the prices. Return list of prices
    Args:
        symbols: a list of symbols to get the prices
    Returns:
        the prices from a data source (with the prices sorted in the same sequence of the given symbols)
    """

    r = requests.get(URL, params={"instType": "SPOT"})
    r.raise_for_status()

    usdt_price = await get_usdt_usd_rate()

    price_map = {
        sym_data["instId"]: Decimal(sym_data["last"]) for sym_data in r.json()["data"]
    }
    return [
        "{:f}".format((price_map[SYMBOL_MAP[sym]] * usdt_price).normalize())
        for sym in symbols
    ]


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
