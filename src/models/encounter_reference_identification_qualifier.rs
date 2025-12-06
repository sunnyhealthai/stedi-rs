use serde::{Deserialize, Serialize};

/// EncounterReferenceIdentificationQualifier : The type of information you provided in the `priorAuthorizationOrReferralNumber` property. You can set this to `9F` - Referral Number or `G1` - Prior Authorization Number.
/// The type of information you provided in the `priorAuthorizationOrReferralNumber` property. You can set this to `9F` - Referral Number or `G1` - Prior Authorization Number.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EncounterReferenceIdentificationQualifier {
    /// Referral Number
    #[serde(rename = "9F")]
    #[default]
    ReferralNumber,
    /// Prior Authorization Number
    #[serde(rename = "G1")]
    PriorAuthorizationNumber,
}

impl std::fmt::Display for EncounterReferenceIdentificationQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ReferralNumber => write!(f, "9F"),
            Self::PriorAuthorizationNumber => write!(f, "G1"),
        }
    }
}
