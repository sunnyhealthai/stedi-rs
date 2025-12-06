use serde::{Deserialize, Serialize};

/// Patient gender codes used in healthcare eligibility checks.
///
/// These are the standard gender codes used in X12 HIPAA transactions for identifying
/// patient gender in eligibility and other healthcare transactions.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Gender {
    /// Male
    #[serde(rename = "M")]
    #[default]
    Male,
    /// Female  
    #[serde(rename = "F")]
    Female,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "M"),
            Self::Female => write!(f, "F"),
        }
    }
}
