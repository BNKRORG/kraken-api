//! Kraken API responses

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, de};

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

pub(crate) struct Balances(HashMap<String, f64>);

impl Balances {
    #[inline]
    pub(crate) fn into_inner(self) -> HashMap<String, f64> {
        self.0
    }
}

impl<'de> Deserialize<'de> for Balances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Kraken returns the balances as string
        let map: HashMap<String, String> = Deserialize::deserialize(deserializer)?;

        // Convert to f64
        let mut balances: HashMap<String, f64> = HashMap::with_capacity(map.len());

        for (coin, amount) in map.into_iter() {
            let amount: f64 = amount.parse().map_err(de::Error::custom)?;
            balances.insert(coin, amount);
        }

        Ok(Self(balances))
    }
}
