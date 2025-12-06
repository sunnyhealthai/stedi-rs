use serde::{Deserialize, Serialize};

/// ThrottlingExceptionResponseContent : The server response when usage plan or account-level throttling limits exceeded.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThrottlingExceptionResponseContent {
    /// Error classification code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human-readable error message
    #[serde(rename = "message")]
    pub message: String,
}

impl ThrottlingExceptionResponseContent {
    /// The server response when usage plan or account-level throttling limits exceeded.
    pub fn new(message: String) -> ThrottlingExceptionResponseContent {
        ThrottlingExceptionResponseContent {
            code: None,
            message,
        }
    }
}
