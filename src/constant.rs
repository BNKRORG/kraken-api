use std::collections::HashSet;
use std::sync::LazyLock;

pub(crate) const API_ROOT_URL: &str = "https://api.kraken.com";

pub(crate) const API_VERSION: u16 = 0;

pub(crate) const USER_AGENT_NAME: &str = concat!("kraken-api/", env!("CARGO_PKG_VERSION"));

pub(crate) const XBT_TICKER: &str = "XBT";

/// Kraken BTC tickers
pub(crate) static TICKERS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| HashSet::from(["XBT", "XXBT", "XBT.B", "XBT.M", "XBT.F", "XBT.T"]));
