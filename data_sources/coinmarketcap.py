#!/usr/bin/env python3

import os
import sys
import time
from collections import defaultdict
from copy import deepcopy
from typing import List, Dict, Optional

import requests

URL = "https://cmc-gateway.bandchain.org/request"
STALE_THRESHOLD = 300


def set_header_from_env(headers: Dict[str, str], key: str):
    """
    Reads values from Yoda Executor's environment variables
    Args:
        headers: current headers
        key: key
    """
    value = os.environ.get(key)
    if value is not None:
        headers[key] = value


def set_request_verification_headers(
    *, existing_headers: Optional[Dict[str, str]] = None
) -> Dict[str, str]:
    if existing_headers:
        headers = deepcopy(existing_headers)
    else:
        headers = {}

    band_keys = [
        "BAND_CHAIN_ID",
        "BAND_VALIDATOR",
        "BAND_REQUEST_ID",
        "BAND_EXTERNAL_ID",
        "BAND_DATA_SOURCE_ID",
        "BAND_REPORTER",
        "BAND_SIGNATURE",
    ]

    for band_key in band_keys:
        set_header_from_env(headers, band_key)

    return headers


def get_price_map(symbols: List[str]) -> Dict[str, str]:
    headers = set_request_verification_headers()
    payload = {"symbols": ",".join(symbols)}
    r = requests.get(URL, headers=headers, params=payload)

    price_map = defaultdict(lambda: "-")
    for data in r.json()["prices"]:
        price = data["price"]
        timestamp = data["timestamp"]
        if price is not None and (time.time() - timestamp) <= STALE_THRESHOLD:
            if price < 0:
                raise Exception("Negative number returned")

            price_map[data["symbol"]] = str(price)

    return price_map


def main(symbols: List[str]) -> str:
    price_map = get_price_map(symbols)
    return ",".join([price_map[sym] for sym in symbols])


if __name__ == "__main__":
    try:
        print(main(sys.argv[1:]))
    except Exception as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
