use serde::{Deserialize, Serialize};

/// BenefitsRelatedEntityRelationshipCode : Code specifying the relationship between the entity and the patient.  Payers may sometimes return other non-compliant values.
/// Code specifying the relationship between the entity and the patient.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum BenefitsRelatedEntityRelationshipCode {
    /// Legally married spouse
    #[serde(rename = "01")]
    #[default]
    Spouse,
    /// Life partner or domestic partner
    #[serde(rename = "02")]
    LifePartner,
    /// Nephew or niece relationship
    #[serde(rename = "27")]
    NephewNiece,
    /// Grandparent or grandchild relationship
    #[serde(rename = "41")]
    GrandparentGrandchild,
    /// Unrelated friend or acquaintance
    #[serde(rename = "48")]
    UnrelatedFriend,
    /// Natural or adopted child
    #[serde(rename = "65")]
    ChildNaturalOrAdopted,
    /// Correspondent or claimant in insurance context
    #[serde(rename = "72")]
    CorrespondentClaimant,
}

impl std::fmt::Display for BenefitsRelatedEntityRelationshipCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Spouse => write!(f, "01"),
            Self::LifePartner => write!(f, "02"),
            Self::NephewNiece => write!(f, "27"),
            Self::GrandparentGrandchild => write!(f, "41"),
            Self::UnrelatedFriend => write!(f, "48"),
            Self::ChildNaturalOrAdopted => write!(f, "65"),
            Self::CorrespondentClaimant => write!(f, "72"),
        }
    }
}
