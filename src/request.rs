use serde::Serialize;

/// Empty json object (used as arguments for some APIs)
#[derive(Debug, Serialize)]
pub(crate) struct Empty {}
