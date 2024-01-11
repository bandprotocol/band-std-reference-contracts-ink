#!/usr/bin/env python3

import asyncio
import json
import statistics
import sys
from collections import defaultdict
from decimal import Decimal
from typing import List, Dict, Any

from aiohttp import ClientSession

URL = "https://data-api.binance.vision"


async def get_usdt_usd_rate(session: ClientSession) -> Decimal:
    async def coingecko(s: ClientSession) -> Decimal:
        res = await s.get(
            "https://api.coingecko.com/api/v3/simple/price",
            params={"ids": "tether", "vs_currencies": "usd"},
        )
        return Decimal.from_float((await res.json())["tether"]["usd"])

    async def cryptocompare(s: ClientSession) -> Decimal:
        res = await s.get(
            "https://min-api.cryptocompare.com/data/price",
            params={"fsym": "USDT", "tsyms": "USD"},
        )
        return Decimal.from_float((await res.json())["USD"])

    async def kraken(s: ClientSession) -> Decimal:
        res = await s.get(
            "https://api.kraken.com/0/public/Ticker",
            params={"pair": "USDTZUSD"},
        )
        return Decimal((await res.json())["result"]["USDTZUSD"]["c"][0])

    async def bitfinex(s: ClientSession) -> Decimal:
        res = await s.get(
            "https://api-pub.bitfinex.com/v2/ticker/tUSTUSD",
        )
        return Decimal.from_float((await res.json())[9])

    async def coinbase(s: ClientSession) -> Decimal:
        res = await s.get(
            "https://api.exchange.coinbase.com/products/USDT-USD/stats",
        )
        return Decimal((await res.json())["last"])

    prices = await asyncio.gather(
        coingecko(session),
        cryptocompare(session),
        kraken(session),
        bitfinex(session),
        coinbase(session),
        return_exceptions=True,
    )

    return statistics.median(filter(lambda x: isinstance(x, Decimal), prices))


async def get_requested_pair_info(
    session: ClientSession, symbols: List[str]
) -> Dict[str, str]:
    symbols_set = set(symbols)

    r = await session.get(f"{URL}/api/v3/exchangeInfo")
    r.raise_for_status()

    res = await r.json()

    pair_info = {
        pair["symbol"]: pair["baseAsset"]
        for pair in res["symbols"]
        if pair["quoteAsset"] == "USDT"
        and pair["isSpotTradingAllowed"]
        and pair["baseAsset"] in symbols_set
    }

    return pair_info


async def get_pair_price(session: ClientSession, pairs: List[str]) -> Any:
    r = await session.get(
        f"{URL}/api/v3/ticker/price",
        params={"symbols": json.dumps(pairs, separators=(",", ":"))},
    )
    r.raise_for_status()

    return await r.json()


async def get_price_map(session: ClientSession, symbols: List[str]) -> Dict[str, str]:
    pair_info, usdt_price = await asyncio.gather(
        get_requested_pair_info(session, symbols),
        get_usdt_usd_rate(session),
    )

    pair_price = await get_pair_price(session, list(pair_info.keys()))

    price_map = defaultdict(lambda: "-")
    for pair in pair_price:
        symbol = pair_info[pair["symbol"]]
        price = Decimal(pair["price"]) * usdt_price
        if price < 0:
            raise Exception("Negative number returned")

        price_map[symbol] = "{:.9f}".format(price).rstrip("0").rstrip(".")

    return price_map


async def main(symbols: List[str]) -> str:
    async with ClientSession() as session:
        price_map = await get_price_map(session, symbols)

    return ",".join([price_map[symbol] for symbol in symbols])


if __name__ == "__main__":
    try:
        print(asyncio.run(main(sys.argv[1:])))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
