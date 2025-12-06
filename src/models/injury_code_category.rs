use serde::{Deserialize, Serialize};

/// InjuryCodeCategory : Payers may sometimes return other non-compliant values.
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InjuryCodeCategory {
    /// Injury Code Category
    #[serde(rename = "44")]
    #[default]
    InjuryCodeCategory,
}

impl std::fmt::Display for InjuryCodeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InjuryCodeCategory => write!(f, "44"),
        }
    }
}
