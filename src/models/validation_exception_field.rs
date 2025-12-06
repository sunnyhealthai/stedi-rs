use serde::{Deserialize, Serialize};

/// ValidationExceptionField : Describes one specific validation failure for an input member.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationExceptionField {
    /// A detailed description of the validation failure.
    #[serde(rename = "message")]
    pub message: String,
    /// A JSONPointer expression to the structure member whose value failed to satisfy the modeled constraints.
    #[serde(rename = "path")]
    pub path: String,
}

impl ValidationExceptionField {
    /// Describes one specific validation failure for an input member.
    pub fn new(message: String, path: String) -> ValidationExceptionField {
        ValidationExceptionField { message, path }
    }
}
