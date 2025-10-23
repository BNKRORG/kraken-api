//! Kraken API responses

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, de};

use crate::constant::TICKERS;
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub(crate) struct KrakenResult<T> {
    /// Kraken API returns error strings in an array marked "error"
    error: Vec<String>,
    /// Kraken API returns results here, separated from error
    /// Sometimes result is omitted if errors occured.
    result: Option<T>,
}

impl<T> KrakenResult<T> {
    pub(crate) fn extract(self) -> Result<T, Error> {
        if !self.error.is_empty() {
            return Err(Error::Kraken(self.error));
        }

        self.result.ok_or(Error::MissingResult)
    }
}

/// Bitcoin balances
///
/// This struct deserializes ONLY the bitcoin balances (see [`TICKERS`]).
pub(crate) struct BitcoinBalances(HashMap<String, f64>);

impl BitcoinBalances {
    /// Get the sum of the bitcoin balances
    #[inline]
    pub(crate) fn sum(self) -> f64 {
        self.0.values().sum()
    }
}

impl<'de> Deserialize<'de> for BitcoinBalances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Kraken returns the balances as string
        let map: HashMap<String, String> = Deserialize::deserialize(deserializer)?;

        // HashMap to store the parsed balances
        let mut balances: HashMap<String, f64> = HashMap::new();

        for (coin, amount) in map.into_iter() {
            // If the ticker is NOT a bitcoin ticker, skip it.
            if !TICKERS.contains(coin.as_str()) {
                continue;
            }

            // Convert to f64
            let amount: f64 = amount.parse().map_err(de::Error::custom)?;

            // Insert into the balances map
            balances.insert(coin, amount);
        }

        Ok(Self(balances))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balances_deserialize_and_sum() {
        let json = r#"{
            "XBT": "1.5",
            "XBT.F": "2.3",
            "ETH": "10.0",
            "USD": "1000.50"
        }"#;

        let balances: BitcoinBalances = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(balances.0.len(), 2);
        assert!(balances.0.contains_key("XBT"));
        assert!(balances.0.contains_key("XBT.F"));
        assert!(!balances.0.contains_key("ETH"));
        assert!(!balances.0.contains_key("USD"));

        // Check sum
        let sum = balances.sum();
        assert!((sum - 3.8).abs() < 0.0001);
    }

    #[test]
    fn test_balances_parse_error() {
        let json = r#"{
            "XBT": "invalid_number"
        }"#;

        let result: Result<BitcoinBalances, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_balances_empty() {
        let json = r#"{
            "ETH": "10.0",
            "USD": "1000.50"
        }"#;

        let balances: BitcoinBalances = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(balances.0.len(), 0);
        assert_eq!(balances.sum(), 0.0);
    }
}
