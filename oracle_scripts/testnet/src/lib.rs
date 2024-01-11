use anyhow::{bail, Result};
use num::{FromPrimitive, Integer};
use std::collections::HashMap;
use std::iter::zip;

use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm_kit::{execute_entry_point, ext, oei, prepare_entry_point};
use phf::phf_map;

const MULTIPLIER: u64 = 1000000000;
const DATA_SOURCE_COUNT: usize = 11;

#[derive(OBIDecode, OBISchema)]
struct Input {
    symbols: Vec<String>,
    minimum_source_count: u8,
}

#[derive(PartialEq, Debug)]
enum ResponseCode {
    Success,
    SymbolNotSupported,
    NotEnoughSources,
    ConversionError,
    Unknown = 127,
}

#[derive(OBIEncode, OBISchema, PartialEq, Debug)]
struct Response {
    symbol: String,
    response_code: u8,
    rate: u64,
}

impl Response {
    fn new(symbol: String, response_code: ResponseCode, rate: u64) -> Self {
        Response {
            symbol,
            response_code: response_code as u8,
            rate,
        }
    }
}

#[derive(OBIEncode, OBISchema, PartialEq, Debug)]
struct Output {
    responses: Vec<Response>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum DataSources {
    BINANCE = 230,
    BITTREX = 227,
    COINBASEPRO = 235,
    COINGECKO = 251,
    COINMARKETCAP = 236,
    CRYPTOCOMPARE = 232,
    HITBTC = 252,
    HUOBIPRO = 224,
    KRAKEN = 228,
    OKX = 229,
}

static SYMBOLS: phf::Map<&'static str, &'static [DataSources]> = phf_map! {
    "ASTR" => &[DataSources::BINANCE, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::OKX],
    "ATOM" => &[DataSources::BINANCE, DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HUOBIPRO],
    "AVAX" => &[DataSources::BINANCE, DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HITBTC, DataSources::HUOBIPRO, DataSources::KRAKEN, DataSources::OKX],
    "BNB" => &[DataSources::BINANCE, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HITBTC],
    "BUSD" => &[DataSources::BINANCE, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE],
    "DAI" => &[DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::KRAKEN],
    "DOT" => &[DataSources::BINANCE, DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HITBTC, DataSources::HUOBIPRO, DataSources::KRAKEN, DataSources::OKX],
    "ETH" => &[DataSources::BINANCE, DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HITBTC, DataSources::HUOBIPRO, DataSources::KRAKEN, DataSources::OKX],
    "FTM" => &[DataSources::BINANCE, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HITBTC, DataSources::HUOBIPRO, DataSources::OKX],
    "GLMR" => &[DataSources::BINANCE, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::HUOBIPRO],
    "MATIC" => &[DataSources::BINANCE, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE],
    "SOL" => &[DataSources::BINANCE, DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HITBTC, DataSources::HUOBIPRO, DataSources::KRAKEN, DataSources::OKX],
    "USDC" => &[DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::HUOBIPRO, DataSources::KRAKEN, DataSources::OKX],
    "USDT" => &[DataSources::BITTREX, DataSources::COINBASEPRO, DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE, DataSources::KRAKEN],
    "WBTC" => &[DataSources::COINGECKO, DataSources::COINMARKETCAP, DataSources::CRYPTOCOMPARE],
};

/// Returns a HashMap mapping the data source id to its supported symbols
fn get_symbols_for_data_sources(symbols: &[String]) -> HashMap<i64, Vec<String>> {
    symbols.iter().fold(
        HashMap::with_capacity(DATA_SOURCE_COUNT),
        |mut acc, symbol| {
            if let Some(data_sources) = SYMBOLS.get(symbol.as_str()) {
                for ds in *data_sources {
                    acc.entry(*ds as i64)
                        .and_modify(|e| {
                            e.push(symbol.clone());
                        })
                        .or_insert(vec![symbol.clone()]);
                }
            }
            acc
        },
    )
}

/// Parses the individual values to assure its value is usable
fn validate_value(v: &str) -> Result<Option<f64>> {
    if v == "-" {
        Ok(None)
    } else {
        let val = v.parse::<f64>()?;
        if val < 0f64 {
            bail!("Invalid value")
        }
        Ok(Some(val))
    }
}

/// Validates and parses the a validator's data source output
fn validate_and_parse_output(ds_output: &str, length: usize) -> Result<Vec<Option<f64>>> {
    let parsed_output = ds_output
        .split(",")
        .map(|v| validate_value(v.trim()))
        .collect::<Result<Vec<Option<f64>>>>()?;

    // If the length of the parsed output is not equal to the expected length, raise an error
    if parsed_output.len() != length {
        bail!("Mismatched length");
    }

    Ok(parsed_output)
}

/// Gets the minimum successful response required given the minimum request count
fn get_minimum_response_count(min_count: i64) -> usize {
    if min_count.is_even() {
        ((min_count + 2) / 2) as usize
    } else {
        ((min_count + 1) / 2) as usize
    }
}

/// Filters and medianizes the parsed data source output
fn filter_and_medianize(
    rates: Vec<Vec<Option<f64>>>,
    length: usize,
    min_response: usize,
) -> Vec<Option<f64>> {
    (0..length)
        .map(|i| {
            let symbol_rates = rates.iter().filter_map(|o| o[i]).collect::<Vec<f64>>();
            if symbol_rates.len() < min_response {
                None
            } else {
                ext::stats::median_by(symbol_rates, ext::cmp::fcmp)
            }
        })
        .collect::<Vec<Option<f64>>>()
}

/// Aggregates the data sources outputs to either a result or error
fn aggregate_value(rates: &[f64], minimum_source_count: usize) -> Result<u64, ResponseCode> {
    if rates.len() < minimum_source_count {
        Err(ResponseCode::NotEnoughSources)
    } else {
        if let Some(price) = ext::stats::median_by(rates.to_owned(), ext::cmp::fcmp) {
            if let Some(mul_price) = u64::from_f64(price * MULTIPLIER as f64) {
                Ok(mul_price)
            } else {
                Err(ResponseCode::ConversionError)
            }
        } else {
            Err(ResponseCode::Unknown)
        }
    }
}

/// Gets the oracle script responses
fn get_responses(
    symbols: &[String],
    symbol_prices: HashMap<String, Vec<f64>>,
    minimum_source_count: usize,
) -> Vec<Response> {
    symbols
        .iter()
        .map(|symbol| {
            if let Some(prices) = symbol_prices.get(symbol) {
                match aggregate_value(&prices, minimum_source_count) {
                    Ok(rate) => Response::new(symbol.clone(), ResponseCode::Success, rate),
                    Err(code) => Response::new(symbol.clone(), code, 0),
                }
            } else {
                Response::new(symbol.clone(), ResponseCode::SymbolNotSupported, 0)
            }
        })
        .collect()
}

fn prepare_impl(input: Input) {
    for (id, symbols) in get_symbols_for_data_sources(&input.symbols) {
        oei::ask_external_data(id, id, symbols.join(" ").as_bytes())
    }
}

fn execute_impl(input: Input) -> Output {
    // HashMap containing all symbols and a vector of their prices from each data source
    let mut symbol_prices: HashMap<String, Vec<f64>> = HashMap::with_capacity(input.symbols.len());

    // Gets the minimum required response count
    let min_resp_count = get_minimum_response_count(oei::get_min_count());

    for (id, symbols) in get_symbols_for_data_sources(&input.symbols) {
        // Parses the validator's responses from a raw string
        let ds_outputs = ext::load_input::<String>(id)
            .filter_map(|r| validate_and_parse_output(&r, symbols.len()).ok())
            .collect::<Vec<Vec<Option<f64>>>>();

        // Gets data source median rates
        let median_rates = filter_and_medianize(ds_outputs, symbols.len(), min_resp_count);

        // Saves symbol rates
        for (symbol, opt_rate) in zip(symbols, median_rates) {
            if let Some(rate) = opt_rate {
                symbol_prices
                    .entry(symbol)
                    .and_modify(|e| e.push(rate))
                    .or_insert(vec![rate]);
            }
        }
    }

    Output {
        responses: get_responses(
            &input.symbols,
            symbol_prices,
            input.minimum_source_count as usize,
        ),
    }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_value() {
        // Test normal case
        let value = validate_value("0.12345").unwrap();
        assert_eq!(value, Some(0.12345));

        // Test null case
        let null_value = validate_value("-").unwrap();
        assert_eq!(null_value, None);

        // Test negative case
        let failed_value = validate_value("-0.555");
        assert!(failed_value.is_err());

        // Test failed case
        let failed_value = validate_value("abc");
        assert!(failed_value.is_err());
    }

    #[test]
    fn test_validate_and_parse_output() {
        // Test normal case
        let ds_outputs = "1.22,1.32,1.44".to_string();
        let parsed_output = validate_and_parse_output(&ds_outputs, 3).unwrap();
        let expected_output = vec![Some(1.22), Some(1.32), Some(1.44)];
        assert_eq!(parsed_output, expected_output);

        // Test normal bad format case
        let ds_outputs = "1.22, 1.32, 1.44".to_string();
        let parsed_output = validate_and_parse_output(&ds_outputs, 3).unwrap();
        let expected_output = vec![Some(1.22), Some(1.32), Some(1.44)];
        assert_eq!(parsed_output, expected_output);

        // Test contains null case
        let ds_outputs = "1.22,1.32,1.44,-,1.23".to_string();
        let parsed_output = validate_and_parse_output(&ds_outputs, 5).unwrap();
        let expected_output = vec![Some(1.22), Some(1.32), Some(1.44), None, Some(1.23)];
        assert_eq!(parsed_output, expected_output);

        // Test invalid case
        let ds_outputs = "NO_DATA,ERROR".to_string();
        let parsed_output = validate_and_parse_output(&ds_outputs, 2);
        assert!(parsed_output.is_err());
    }

    #[test]
    fn test_get_minimum_response_count() {
        let min_request = 1..17;
        let expected_min_responses: Vec<usize> =
            vec![1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9];

        let min_resp_count = min_request
            .map(|x| get_minimum_response_count(x as i64))
            .collect::<Vec<usize>>();
        assert_eq!(min_resp_count, expected_min_responses);
    }

    #[test]
    fn test_filter_and_medianize() {
        // Test normal case
        let rates = vec![
            vec![Some(0.0), Some(1.3), Some(2.3)],
            vec![Some(0.1), Some(1.0), Some(2.0)],
            vec![Some(0.3), Some(1.1), Some(2.3)],
            vec![Some(0.3), Some(1.1), Some(2.3)],
        ];
        let result = filter_and_medianize(rates, 3, 2);
        let expected_result = vec![Some(0.2), Some(1.1), Some(2.3)];
        assert_eq!(result, expected_result);

        // Test too many missing case
        let rates = vec![
            vec![Some(0.0), Some(1.3), None],
            vec![Some(0.1), Some(1.0), None],
            vec![Some(0.3), Some(1.1), None],
            vec![Some(0.3), Some(1.1), Some(2.3)],
        ];
        let result = filter_and_medianize(rates, 3, 2);
        let expected_result = vec![Some(0.2), Some(1.1), None];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_aggregate_value() {
        // Test normal case
        let data = vec![1.23, 1.24, 1.25, 1.26, 1.27];
        let normal_res = aggregate_value(&data, 3);
        assert_eq!(normal_res.unwrap(), 1250000000);

        // Test overflow case
        let invalid_data = vec![f64::MAX, f64::MAX, f64::MAX, f64::MAX, f64::MAX];
        let overflow_res = aggregate_value(&invalid_data, 3);
        assert_eq!(overflow_res.unwrap_err(), ResponseCode::ConversionError);

        // Test underflow case
        let invalid_data = vec![f64::MIN, f64::MIN, f64::MIN, f64::MIN, f64::MIN];
        let overflow_res = aggregate_value(&invalid_data, 3);
        assert_eq!(overflow_res.unwrap_err(), ResponseCode::ConversionError);

        // Test NaN case
        let invalid_data = vec![f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN];
        let overflow_res = aggregate_value(&invalid_data, 3);
        assert_eq!(overflow_res.unwrap_err(), ResponseCode::ConversionError);

        // Test not enough sources case
        let invalid_data = vec![];
        let overflow_res = aggregate_value(&invalid_data, 3);
        assert_eq!(overflow_res.unwrap_err(), ResponseCode::NotEnoughSources);
    }

    #[test]
    fn test_get_responses() {
        let symbols = vec!["BTC".to_string(), "ETH".to_string(), "DNE".to_string()];
        let symbol_prices = HashMap::from([
            (String::from("BTC"), vec![1.23, 1.24, 1.25, 1.26, 1.27]),
            (String::from("ETH"), vec![2.31, 2.32]),
        ]);
        let responses = get_responses(&symbols, symbol_prices, 3);
        assert_eq!(
            responses[0],
            Response::new("BTC".to_string(), ResponseCode::Success, 1250000000)
        );
        assert_eq!(
            responses[1],
            Response::new("ETH".to_string(), ResponseCode::NotEnoughSources, 0)
        );
        assert_eq!(
            responses[2],
            Response::new("DNE".to_string(), ResponseCode::SymbolNotSupported, 0)
        );
    }
}
