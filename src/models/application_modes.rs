//! Healthcare eligibility API request modes.
//!
//! This module defines the application modes used when making eligibility API requests.
//! The mode determines how the request is processed and what type of data is returned.
//! Production mode uses live data, test mode uses sandbox data for development and
//! testing purposes, and information mode is reserved for future use.

use serde::{Deserialize, Serialize};

/// The type of data in the eligibility request.
///
/// This enum specifies the processing mode for healthcare eligibility API requests.
/// Payers may sometimes return other non-compliant values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApplicationModes {
    /// Production mode - uses live data and standard API keys.
    ///
    /// Requests made with production API keys will process real eligibility data
    /// and return actual benefit information from healthcare payers.
    #[serde(rename = "production")]
    Production,

    /// Test mode - uses sandbox data for development and testing.
    ///
    /// Requests made with test API keys will process simulated eligibility data
    /// and return example benefit information for development purposes.
    #[serde(rename = "test")]
    Test,

    /// Information mode - reserved for future use.
    ///
    /// This mode is not currently used but may be implemented in future versions
    /// of the healthcare eligibility API.
    #[serde(rename = "information")]
    Information,
}

impl std::fmt::Display for ApplicationModes {
    /// Formats the application mode as a string.
    ///
    /// This implementation converts each enum variant to its corresponding
    /// string representation that matches the API specification:
    /// - `Production` becomes "production"
    /// - `Test` becomes "test"
    /// - `Information` becomes "information"
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Production => write!(f, "production"),
            Self::Test => write!(f, "test"),
            Self::Information => write!(f, "information"),
        }
    }
}

impl Default for ApplicationModes {
    /// Returns the default application mode.
    ///
    /// Production mode is used as the default for healthcare eligibility requests,
    /// ensuring that real benefit data is processed when no specific mode is specified.
    fn default() -> ApplicationModes {
        Self::Production
    }
}
