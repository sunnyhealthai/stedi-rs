use serde::{Deserialize, Serialize};

/// GatewayTimeoutExceptionResponseContent : The server response for a gateway timeout error.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayTimeoutExceptionResponseContent {
    /// Classification of the exception type.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human readable exception message.
    #[serde(rename = "message")]
    pub message: String,
}

impl GatewayTimeoutExceptionResponseContent {
    /// The server response for a gateway timeout error.
    pub fn new(message: String) -> GatewayTimeoutExceptionResponseContent {
        GatewayTimeoutExceptionResponseContent {
            code: None,
            message,
        }
    }
}
