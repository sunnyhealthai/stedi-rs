use serde::{Deserialize, Serialize};

/// ResponseProviderCode : A code indicating the type of provider. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes) for a complete list.  Payers may sometimes return other non-compliant values.
/// A code indicating the type of provider. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes) for a complete list.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ResponseProviderCode {
    /// Admitting
    #[serde(rename = "AD")]
    #[default]
    Admitting,
    /// Attending
    #[serde(rename = "AT")]
    Attending,
    /// Billing
    #[serde(rename = "BI")]
    Billing,
    /// Consulting
    #[serde(rename = "CO")]
    Consulting,
    /// Covering
    #[serde(rename = "CV")]
    Covering,
    /// Hospital
    #[serde(rename = "H")]
    Hospital,
    /// Home Health Care
    #[serde(rename = "HH")]
    HomeHealthCare,
    /// Laboratory
    #[serde(rename = "LA")]
    Laboratory,
    /// Other Physician
    #[serde(rename = "OT")]
    OtherPhysician,
    /// Pharmacist
    #[serde(rename = "P1")]
    Pharmacist,
    /// Pharmacy
    #[serde(rename = "P2")]
    Pharmacy,
    /// Primary Care Physician
    #[serde(rename = "PC")]
    PrimaryCarePhysician,
    /// Performing
    #[serde(rename = "PE")]
    Performing,
    /// Rural Health Clinic
    #[serde(rename = "R")]
    RuralHealthClinic,
    /// Referring
    #[serde(rename = "RF")]
    Referring,
    /// Submitting
    #[serde(rename = "SB")]
    Submitting,
    /// Skilled Nursing Facility
    #[serde(rename = "SK")]
    SkilledNursingFacility,
    /// Supervising
    #[serde(rename = "SU")]
    Supervising,
}

impl std::fmt::Display for ResponseProviderCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Admitting => write!(f, "AD"),
            Self::Attending => write!(f, "AT"),
            Self::Billing => write!(f, "BI"),
            Self::Consulting => write!(f, "CO"),
            Self::Covering => write!(f, "CV"),
            Self::Hospital => write!(f, "H"),
            Self::HomeHealthCare => write!(f, "HH"),
            Self::Laboratory => write!(f, "LA"),
            Self::OtherPhysician => write!(f, "OT"),
            Self::Pharmacist => write!(f, "P1"),
            Self::Pharmacy => write!(f, "P2"),
            Self::PrimaryCarePhysician => write!(f, "PC"),
            Self::Performing => write!(f, "PE"),
            Self::RuralHealthClinic => write!(f, "R"),
            Self::Referring => write!(f, "RF"),
            Self::Submitting => write!(f, "SB"),
            Self::SkilledNursingFacility => write!(f, "SK"),
            Self::Supervising => write!(f, "SU"),
        }
    }
}
