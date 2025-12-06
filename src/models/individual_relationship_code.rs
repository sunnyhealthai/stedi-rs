use serde::{Deserialize, Serialize};

/// IndividualRelationshipCode : The dependent's relationship to the subscriber. You can set this to `01` - Spouse, `19` - Child, `34` - Other Adult.
/// The dependent's relationship to the subscriber. You can set this to `01` - Spouse, `19` - Child, `34` - Other Adult.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum IndividualRelationshipCode {
    /// Spouse
    #[serde(rename = "01")]
    #[default]
    Spouse,
    /// Child
    #[serde(rename = "19")]
    Child,
    /// Other Adult
    #[serde(rename = "34")]
    OtherAdult,
}

impl std::fmt::Display for IndividualRelationshipCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Spouse => write!(f, "01"),
            Self::Child => write!(f, "19"),
            Self::OtherAdult => write!(f, "34"),
        }
    }
}
