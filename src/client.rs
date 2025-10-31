//! Kraken client

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::{self, KrakenAuth};
use crate::constant::{API_ROOT_URL, API_VERSION, USER_AGENT_NAME, XBT_TICKER};
use crate::error::Error;
use crate::request::{DepositStatus, Empty};
use crate::response::{BitcoinBalances, DepositTransaction, KrakenResult};

enum Api<'a> {
    Balance,
    DepositStatus {
        /// Currency to get transactions for.
        asset: Option<&'a str>,
    },
}

impl Api<'_> {
    fn method(&self) -> &str {
        match self {
            Self::Balance => "Balance",
            Self::DepositStatus { .. } => "DepositStatus",
        }
    }

    fn query_data(&self) -> Result<String, Error> {
        match self {
            Self::Balance => Ok(serde_qs::to_string(&Empty {})?),
            Self::DepositStatus { asset } => Ok(serde_qs::to_string(&DepositStatus {
                asset: asset.as_deref(),
            })?),
        }
    }
}

/// Kraken client
#[derive(Debug, Clone)]
pub struct KrakenClient {
    /// Root URL for the API.
    root_url: Url,
    /// HTTP client.
    client: Client,
    /// Authentication
    auth: KrakenAuth,
}

impl KrakenClient {
    /// Construct a new Coinbase App client.
    pub fn new(auth: KrakenAuth) -> Result<Self, Error> {
        Ok(Self {
            root_url: Url::parse(API_ROOT_URL)?,
            client: Client::builder()
                .user_agent(USER_AGENT_NAME)
                .timeout(Duration::from_secs(25))
                .build()?,
            auth,
        })
    }

    async fn query<T>(&self, url: Url, headers: HeaderMap, post_data: String) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        // Post request
        let response: Response = self
            .client
            .post(url)
            .headers(headers)
            .body(post_data)
            .send()
            .await?;

        // If HTTP error, return error
        let response: Response = response.error_for_status()?;

        // Parse the response as JSON
        let result: KrakenResult<T> = response.json().await?;

        // Extract the result
        result.extract()
    }

    async fn query_private<T>(&self, api: Api<'_>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match &self.auth {
            KrakenAuth::ApiKeys(creds) => {
                let method: &str = api.method();

                let path: String = format!("/{API_VERSION}/private/{method}");
                let mut url: Url = self.root_url.join(&path)?;

                // Get query data as URL query string
                let query_data: String = api.query_data()?;

                // Set query string to URL
                url.set_query(Some(&query_data));

                // Sign the request
                let (post_data, sig) = auth::sign_api(creds, &url)?;

                // Build headers
                let mut headers: HeaderMap = HeaderMap::with_capacity(2);
                headers.insert("API-Key", HeaderValue::from_str(&creds.key)?);
                headers.insert("API-Sign", HeaderValue::from_str(&sig)?);

                // Query
                self.query(url, headers, post_data).await
            }
            KrakenAuth::None => Err(Error::MissingCredentials),
        }
    }

    /// Get **bitcoin** balance.
    pub async fn balance(&self) -> Result<f64, Error> {
        // Get bitcoin balances
        let balances: BitcoinBalances = self.query_private(Api::Balance).await?;

        // Sum balances
        Ok(balances.sum())
    }

    /// Get **bitcoin** transactions.
    pub async fn deposit_transactions(&self) -> Result<Vec<DepositTransaction>, Error> {
        self.query_private(Api::DepositStatus {
            asset: Some(XBT_TICKER),
        })
        .await
    }
}
