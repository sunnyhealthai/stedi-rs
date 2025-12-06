use serde::{Deserialize, Serialize};

/// InformationStatusCode : Payers may sometimes return other non-compliant values.
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InformationStatusCode {
    /// Active
    #[serde(rename = "A")]
    #[default]
    Active,
    /// Cancelled
    #[serde(rename = "C")]
    Cancelled,
    /// Lapsed
    #[serde(rename = "L")]
    Lapsed,
    /// Open
    #[serde(rename = "O")]
    Open,
    /// Pending
    #[serde(rename = "P")]
    Pending,
    /// Suspended
    #[serde(rename = "S")]
    Suspended,
    /// Terminated
    #[serde(rename = "T")]
    Terminated,
}

impl std::fmt::Display for InformationStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "A"),
            Self::Cancelled => write!(f, "C"),
            Self::Lapsed => write!(f, "L"),
            Self::Open => write!(f, "O"),
            Self::Pending => write!(f, "P"),
            Self::Suspended => write!(f, "S"),
            Self::Terminated => write!(f, "T"),
        }
    }
}
