use serde::{Deserialize, Serialize};

/// Relationship of a dependent to the subscriber in eligibility check responses.
///
/// Specifies the familial or legal relationship between a dependent and the primary subscriber
/// (policyholder) of an insurance plan. This human-readable name corresponds to the
/// `relationToSubscriberCode` field, which contains the standardized code version.
///
/// This field is used in `responseDependent.relationToSubscriber` to identify how the dependent
/// is related to the subscriber. Understanding this relationship is important for:
/// - Verifying eligibility and coverage rules (some plans have different rules for spouses vs. children)
/// - Determining age limits for dependent coverage (children typically age out at 26)
/// - Processing coordination of benefits scenarios
/// - Understanding family structure for benefit calculations
///
/// ## Common Relationships
///
/// - **Spouse**: The subscriber's legally married spouse
/// - **Child**: Biological, adopted, or stepchild of the subscriber (typically covered until age 26)
/// - **Employee**: When the subscriber is an employer covering an employee
/// - **Life Partner**: Domestic partner or civil union partner (varies by state/plan)
/// - **Organ Donor**: Person donating an organ to the subscriber
/// - **Cadaver Donor**: Deceased person who donated an organ
/// - **Other Relationship**: Relationship that doesn't fit standard categories
/// - **Unknown**: Relationship is not specified or cannot be determined
///
/// The corresponding `DependentRelationshipCode` enum contains the standardized codes:
/// - `01` - Spouse
/// - `19` - Child
/// - `20` - Employee
/// - `21` - Unknown
/// - `39` - Organ Donor
/// - `40` - Cadaver Donor
/// - `53` - Life Partner
/// - `G8` - Other Relationship
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DependentRelationship {
    #[serde(rename = "Spouse")]
    #[default]
    Spouse,
    #[serde(rename = "Child")]
    Child,
    #[serde(rename = "Employee")]
    Employee,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Organ Donor")]
    OrganDonor,
    #[serde(rename = "Cadaver Donor")]
    CadaverDonor,
    #[serde(rename = "Life Partner")]
    LifePartner,
    #[serde(rename = "Other Relationship")]
    OtherRelationship,
}

impl std::fmt::Display for DependentRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Spouse => write!(f, "Spouse"),
            Self::Child => write!(f, "Child"),
            Self::Employee => write!(f, "Employee"),
            Self::Unknown => write!(f, "Unknown"),
            Self::OrganDonor => write!(f, "Organ Donor"),
            Self::CadaverDonor => write!(f, "Cadaver Donor"),
            Self::LifePartner => write!(f, "Life Partner"),
            Self::OtherRelationship => write!(f, "Other Relationship"),
        }
    }
}
