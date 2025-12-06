use serde::{Deserialize, Serialize};

/// RequestSubscriberProviderCode : Use this for providers that are not requesting the eligibility check - the requestor is specified in the `provider` object. For example, if you are a hospital making an eligibility request, this is where you would specify information about a referring provider's role.    This property is **required** when the `providerIdentifier` and `referenceIdentificationQualifier` properties are populated.    You can use one of the following: `AD` - Admitting, `AT` - Attending, `BI` - Billing, `CO` - Consulting, `CV` - Covering, `H` - Hospital, `HH` - Home Health Care, `LA` - Laboratory, `OT` - Other Physician, `P1` - Pharmacist, `P2` - Pharmacy, `PC` - Primary Care Physician, `PE` - Performing, `R` - Rural Health Clinic, `RF` - Referring, `SB` - Submitting, `SK` - Skilled Nursing Facility, `SU` - Supervising
/// Use this for providers that are not requesting the eligibility check - the requestor is specified in the `provider` object. For example, if you are a hospital making an eligibility request, this is where you would specify information about a referring provider's role.    This property is **required** when the `providerIdentifier` and `referenceIdentificationQualifier` properties are populated.    You can use one of the following: `AD` - Admitting, `AT` - Attending, `BI` - Billing, `CO` - Consulting, `CV` - Covering, `H` - Hospital, `HH` - Home Health Care, `LA` - Laboratory, `OT` - Other Physician, `P1` - Pharmacist, `P2` - Pharmacy, `PC` - Primary Care Physician, `PE` - Performing, `R` - Rural Health Clinic, `RF` - Referring, `SB` - Submitting, `SK` - Skilled Nursing Facility, `SU` - Supervising
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum RequestSubscriberProviderCode {
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
    /// Skilled Nursing Facility
    #[serde(rename = "SK")]
    SkilledNursingFacility,
    /// Supervising
    #[serde(rename = "SU")]
    Supervising,
}

impl std::fmt::Display for RequestSubscriberProviderCode {
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
            Self::SkilledNursingFacility => write!(f, "SK"),
            Self::Supervising => write!(f, "SU"),
        }
    }
}
