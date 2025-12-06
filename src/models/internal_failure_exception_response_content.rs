use serde::{Deserialize, Serialize};

/// InternalFailureExceptionResponseContent : The server response when an unexpected error occurred while processing request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalFailureExceptionResponseContent {
    /// Error classification code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human-readable error message
    #[serde(rename = "message")]
    pub message: String,
}

impl InternalFailureExceptionResponseContent {
    /// The server response when an unexpected error occurred while processing request.
    pub fn new(message: String) -> InternalFailureExceptionResponseContent {
        InternalFailureExceptionResponseContent {
            code: None,
            message,
        }
    }
}
