use serde::{Deserialize, Serialize};

/// DependentRelationshipCode : For the dependent, this can be `01` - Spouse, `19` - Child, `20` Employee, `21` - Unknown, `39` - Organ Donor, `40` - Cadaver Donor, `53` - Life Partner, or `G8` - Other Relationship.
/// For the dependent, this can be `01` - Spouse, `19` - Child, `20` Employee, `21` - Unknown, `39` - Organ Donor, `40` - Cadaver Donor, `53` - Life Partner, or `G8` - Other Relationship.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DependentRelationshipCode {
    /// Spouse
    #[serde(rename = "01")]
    #[default]
    Spouse,
    /// Child
    #[serde(rename = "19")]
    Child,
    /// Employee
    #[serde(rename = "20")]
    Employee,
    /// Unknown
    #[serde(rename = "21")]
    Unknown,
    /// Organ Donor
    #[serde(rename = "39")]
    OrganDonor,
    /// Cadaver Donor
    #[serde(rename = "40")]
    CadaverDonor,
    /// Life Partner
    #[serde(rename = "53")]
    LifePartner,
    /// Other Relationship
    #[serde(rename = "G8")]
    OtherRelationship,
    /// Unknown (alternative representation)
    #[serde(rename = "Unknown")]
    UnknownAlt,
}

impl std::fmt::Display for DependentRelationshipCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Spouse => write!(f, "01"),
            Self::Child => write!(f, "19"),
            Self::Employee => write!(f, "20"),
            Self::Unknown => write!(f, "21"),
            Self::OrganDonor => write!(f, "39"),
            Self::CadaverDonor => write!(f, "40"),
            Self::LifePartner => write!(f, "53"),
            Self::OtherRelationship => write!(f, "G8"),
            Self::UnknownAlt => write!(f, "Unknown"),
        }
    }
}
