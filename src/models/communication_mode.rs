use serde::{Deserialize, Serialize};

/// Communication mode used for healthcare transactions and contact information.
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum CommunicationMode {
    /// Electronic Data Interchange (EDI) access number for healthcare transactions
    #[serde(rename = "Electronic Data Interchange Access Number")]
    #[default]
    ElectronicDataInterchangeAccessNumber,
    /// Email address for electronic communication
    #[serde(rename = "Electronic Mail")]
    ElectronicMail,
    /// Fax number for document transmission
    #[serde(rename = "Facsimile")]
    Facsimile,
    /// Primary telephone number for voice communication
    #[serde(rename = "Telephone")]
    Telephone,
    /// Web URL for online access
    #[serde(rename = "Uniform Resource Locator (URL)")]
    UniformResourceLocatorLeftParenthesisUrlRightParenthesis,
    /// Telephone extension number
    #[serde(rename = "Telephone Extension")]
    TelephoneExtension,
    /// Work or business telephone number
    #[serde(rename = "Work Phone Number")]
    WorkPhoneNumber,
}

impl std::fmt::Display for CommunicationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ElectronicDataInterchangeAccessNumber => {
                write!(f, "Electronic Data Interchange Access Number")
            }
            Self::ElectronicMail => write!(f, "Electronic Mail"),
            Self::Facsimile => write!(f, "Facsimile"),
            Self::Telephone => write!(f, "Telephone"),
            Self::UniformResourceLocatorLeftParenthesisUrlRightParenthesis => {
                write!(f, "Uniform Resource Locator (URL)")
            }
            Self::TelephoneExtension => write!(f, "Telephone Extension"),
            Self::WorkPhoneNumber => write!(f, "Work Phone Number"),
        }
    }
}
