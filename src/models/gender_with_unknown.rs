use serde::{Deserialize, Serialize};

/// Gender values used in healthcare eligibility checks.
///
/// This enum represents the standard gender codes used in healthcare industry:
/// - M for Male
/// - F for Female  
/// - U for Unknown/Unspecified
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum GenderWithUnknown {
    /// Male
    #[serde(rename = "M")]
    #[default]
    Male,
    /// Female
    #[serde(rename = "F")]
    Female,
    /// Unknown or Unspecified
    #[serde(rename = "U")]
    Unknown,
}

impl std::fmt::Display for GenderWithUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Male => write!(f, "M"),
            Self::Female => write!(f, "F"),
            Self::Unknown => write!(f, "U"),
        }
    }
}
