use serde::{Deserialize, Serialize};

/// ResourceNotFoundExceptionResponseContent : //// Errors The server response when the specified resource cannot be found after an API request passes authentication and authorization.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceNotFoundExceptionResponseContent {
    /// Classification of the exception type.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human readable exception message.
    #[serde(rename = "message")]
    pub message: String,
}

impl ResourceNotFoundExceptionResponseContent {
    /// //// Errors The server response when the specified resource cannot be found after an API request passes authentication and authorization.
    pub fn new(message: String) -> ResourceNotFoundExceptionResponseContent {
        ResourceNotFoundExceptionResponseContent {
            code: None,
            message,
        }
    }
}
