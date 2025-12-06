use serde::{Deserialize, Serialize};

/// InPlanNetworkIndicatorCode : Code indicating whether the benefit is in-network or out-of-network. Can be `Y` - Yes, `N` - No, `U` - Unknown, or `W` - Not Applicable   Code `U` indicates that it is unknown whether the benefits are in or out-of-network. Code `W` indicates that the benefit applies to both in and out-of-network providers.     Note that this property **doesn't indicate** whether the provider is in or out-of-network for the patient. To determine that, you must check with the payer directly.  Payers may sometimes return other non-compliant values.
/// Code indicating whether the benefit is in-network or out-of-network. Can be `Y` - Yes, `N` - No, `U` - Unknown, or `W` - Not Applicable   Code `U` indicates that it is unknown whether the benefits are in or out-of-network. Code `W` indicates that the benefit applies to both in and out-of-network providers.     Note that this property **doesn't indicate** whether the provider is in or out-of-network for the patient. To determine that, you must check with the payer directly.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InPlanNetworkIndicatorCode {
    /// Yes - In Network
    #[serde(rename = "Y")]
    #[default]
    Yes,
    /// No - Out of Network
    #[serde(rename = "N")]
    No,
    /// Unknown
    #[serde(rename = "U")]
    Unknown,
    /// Not Applicable - Applies to Both In and Out of Network
    #[serde(rename = "W")]
    NotApplicable,
}

impl std::fmt::Display for InPlanNetworkIndicatorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "Y"),
            Self::No => write!(f, "N"),
            Self::Unknown => write!(f, "U"),
            Self::NotApplicable => write!(f, "W"),
        }
    }
}
