use serde::Serialize;

/// Empty json object (used as arguments for some APIs)
#[derive(Debug, Serialize)]
pub(crate) struct Empty {}

#[derive(Debug, Serialize)]
pub(crate) struct DepositStatus<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) asset: Option<&'a str>,
}
