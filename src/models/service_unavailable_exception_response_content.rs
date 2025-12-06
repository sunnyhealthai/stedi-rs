use serde::{Deserialize, Serialize};

/// ServiceUnavailableExceptionResponseContent : The server is temporarily unavailable.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUnavailableExceptionResponseContent {
    /// Classification of the exception type.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human readable exception message.
    #[serde(rename = "message")]
    pub message: String,
}

impl ServiceUnavailableExceptionResponseContent {
    /// The server is temporarily unavailable.
    pub fn new(message: String) -> ServiceUnavailableExceptionResponseContent {
        ServiceUnavailableExceptionResponseContent {
            code: None,
            message,
        }
    }
}
