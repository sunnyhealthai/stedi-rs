use serde::{Deserialize, Serialize};

/// Warning message returned by Stedi when processing eligibility check requests.
///
/// Warnings indicate potential issues or automatic corrections made to the request
/// that don't prevent processing but may affect the results. For example, Stedi
/// automatically replaces backticks with apostrophes in patient names.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Warning {
    /// The warning code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The warning description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Warning {
    /// Create a new empty Warning instance.
    pub fn new() -> Warning {
        Warning {
            code: None,
            description: None,
        }
    }
}
