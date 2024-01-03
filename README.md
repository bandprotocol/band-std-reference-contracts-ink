# Band Protocol's ink! Standard Reference Contracts

## Overview

This repository contains the ink! code for Band Protocol's StdReference contracts. The live contract addresses can
be found in our [documentation](https://docs.bandchain.org/develop/supported-blockchains/).

### Contract Compilation

#### Option 1: Using Docker

To compile the Standard Reference Contract on Intel architecture, use the following command:

```bash
cd /contracts/std_ref

docker run --rm -it -v $(pwd):/contracts/std_ref paritytech/contracts-ci-linux \
  cargo contract build --release --manifest-path=/contracts/std_ref/Cargo.toml
```

To compile on M1 architecture, use the following command:

```bash
cd /contracts/std_ref

docker run --rm -it -v $(pwd):/contracts/std_ref --platform linux/amd64 paritytech/contracts-ci-linux \
  cargo contract build --release --manifest-path=/contracts/std_ref/Cargo.toml
```


#### Option 2: Using Cargo Contract (Recommended)

To compile the Standard Reference Contract, you'll need to use cargo-contract, a tool for working with smart contracts in the Rust programming language.

1. Install Cargo contract
   - Step 1: `rustup component add rust-src`.
   - Step 2: `cargo install --force --locked cargo-contract`.

2. Run the build command. For contracts intended to run in production, you should always build the contract with --release:

    ```bash
    cargo contract build --release
    ```

## Usage

To query the prices from Band Protocol's StdReference contracts, the contract looking to use the price values should query Band Protocol's `std_reference` contract.

### QueryMsg

The query messages used to retrieve price data for price data are as follows:

```rust
pub enum QueryMsg {
    GetReferenceData {
        // Symbol pair to query where:
        // symbol_pair := (base_symbol, quote_symbol)
        // e.g. BTC/USD ≡ 
        // ("0000000000000000000000000000000000000000000000000000000000425443",
        // "0000000000000000000000000000000000000000000000000000000000555344")
        symbol_pair: (Hash, Hash),
    },
    GetReferenceDataBulk {
        // Vector of Symbol pair to query
        // e.g. <BTC/USD ETH/USD, BAND/BTC> ≡ 
        // <("0000000000000000000000000000000000000000000000000000000000425443",
        // "0000000000000000000000000000000000000000000000000000000000555344"),
        // ("0000000000000000000000000000000000000000000000000000000000455448",
        // "0000000000000000000000000000000000000000000000000000000000555344"),
        // ("0000000000000000000000000000000000000000000000000000000042414e44",
        // "0000000000000000000000000000000000000000000000000000000000425443")>
        symbol_pairs: Vec<(Hash, Hash)>,
    },
}
```

### ReferenceData

`ReferenceData` is the struct that is returned when querying with `GetReferenceData` or `GetReferenceDataBulk` where the
bulk variant returns `Vec<ReferenceData>`

`ReferenceData` is defined as:

```rust
pub struct ReferenceData {
    // Pair rate e.g. rate of BTC/USD
    pub rate: u128,
    // Unix time of when the base asset was last updated. e.g. Last update time of BTC in Unix time
    pub last_updated_base: u64,
    // Unix time of when the quote asset was last updated. e.g. Last update time of USD in Unix time
    pub last_updated_quote: u64,
}
```

## Examples

### Using the Contracts UI

This example use [StdReferenceBasic contract](https://contracts-ui.substrate.io/contract/Yjj2DQA4AznhucyvoYVyNXLqZu6KVDKuC3xvjt7oVF5ucZ1) on Astar Shibuya.

#### Single Query

This example demonstrates how to query the price of cryptocurrencies, such as BTC/USD and ETH/USD, using the provided hashing function and the get_reference_data function.

To query the price of a cryptocurrency pair, you need to first hash the symbols of the base and quote currencies. Here are the hashes for BTC and ETH:

- Hash("BTC") = "0000000000000000000000000000000000000000000000000000000000425443"
- Hash("USD") = "0000000000000000000000000000000000000000000000000000000000555344"

![get_reference_data](img/get_reference_data.png)

The result from the `get_reference_data` function

```text
{
  Ok: {
    rate: '45,222,979,831,850,000,000,000,000,000,000',
    baseResolveTime: '1,704,261,434',
    quoteResolveTime: '1,704,261,434',
  },
}
```

#### Bulk Query

This example demonstrates how to perform bulk queries for multiple cryptocurrency pairs using the get_reference_data_bulk function.

Before making a bulk query, you need to hash the symbols of the base and quote currencies for each pair. Here are the hashes for BTC, ETH, and USD:

- Hash("BTC") = "0000000000000000000000000000000000000000000000000000000000425443"
- Hash("ETH") = "0000000000000000000000000000000000000000000000000000000000455448"
- Hash("USD") = "0000000000000000000000000000000000000000000000000000000000555344"

![get_reference_data_bulk](/img/get_reference_data_bulk.png)

The result from the `get_reference_data_bulk` function

```text
[
  {
    Ok: {
      rate: '45,222,979,831,850,000,000,000,000,000,000',
      baseResolveTime: '1,704,261,434',
      quoteResolveTime: '1,704,261,434',
    },
  },
  {
    Ok: {
      rate: '2,378,489,377,900,000,000,000,000,000,000',
      baseResolveTime: '1,704,261,434',
      quoteResolveTime: '1,704,261,434',
    },
  },
]
```
