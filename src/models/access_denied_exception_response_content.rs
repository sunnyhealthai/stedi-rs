//! Access denied exception response handling.
//!
//! This module contains the error response structure for HTTP 403 Access Denied exceptions
//! that occur when making requests to the Stedi Healthcare Eligibility API.
//!
//! ## Common Causes
//!
//! Access denied errors typically occur due to:
//! - Invalid or missing API key
//! - Expired authentication credentials
//! - Insufficient permissions for the requested operation
//! - Account-level restrictions or suspensions
//!
//! ## Resolution
//!
//! To resolve access denied errors:
//! 1. Verify your API key is correct and properly formatted
//! 2. Ensure your API key hasn't expired
//! 3. Check that your account has the necessary permissions
//! 4. Contact Stedi support if the issue persists
//!
//! For more information, see the [Stedi Authentication documentation](https://www.stedi.com/docs/api-reference#authentication).

use serde::{Deserialize, Serialize};

/// AccessDeniedExceptionResponseContent : The server response for authorization failure.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessDeniedExceptionResponseContent {
    /// Error classification code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Human-readable error message
    #[serde(rename = "message")]
    pub message: String,
}

impl AccessDeniedExceptionResponseContent {
    /// The server response for authorization failure.
    pub fn new(message: String) -> AccessDeniedExceptionResponseContent {
        AccessDeniedExceptionResponseContent {
            code: None,
            message,
        }
    }
}
