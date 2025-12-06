use serde::{Deserialize, Serialize};

/// Healthcare entity identification codes used in eligibility benefit responses.
///
/// These codes identify the type of identifier being provided for benefit-related entities
/// such as payers, providers, or members. They are used in HIPAA X12 transactions to
/// specify the format and meaning of identification numbers. Payers may sometimes return
/// non-standard values that are not part of the official code set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BenefitRelatedEntityIdentification {
    /// Employer's Identification Number (24)
    #[serde(rename = "24")]
    EmployerId,
    /// Social Security Number (34)
    #[serde(rename = "34")]
    SocialSecurityNumber,
    /// Electronic Transmitter Identification Number (46)
    #[serde(rename = "46")]
    ElectronicTransmitterId,
    /// Federal Agency identifier (FA)
    #[serde(rename = "FA")]
    FederalAgency,
    /// Federal Taxpayer's Identification Number (FI)
    #[serde(rename = "FI")]
    FederalTaxpayerId,
    /// Standard Unique Identifier (II)
    #[serde(rename = "II")]
    StandardUniqueIdentifier,
    /// Member Identification Number (MI)
    #[serde(rename = "MI")]
    MemberId,
    /// National Identifier (NI)
    #[serde(rename = "NI")]
    NationalId,
    /// Payor Identification Number (PI)
    #[serde(rename = "PI")]
    PayorId,
    /// Pharmacy Processor Number (PP)
    #[serde(rename = "PP")]
    PharmacyProcessorId,
    /// Service Provider Number (SV)
    #[serde(rename = "SV")]
    ServiceProviderId,
    /// Health Care Financing Administration National Provider Identifier (XV)
    #[serde(rename = "XV")]
    HealthCareFinancingAdministrationNationalProviderId,
    /// Health Care Financing Administration National Supplier Identifier (XX)
    #[serde(rename = "XX")]
    HealthCareFinancingAdministrationNationalSupplierId,
}

impl std::fmt::Display for BenefitRelatedEntityIdentification {
    /// Formats the enum variant as its corresponding healthcare identification code.
    ///
    /// This implementation maintains the original code values used in HIPAA transactions,
    /// regardless of the descriptive Rust variant names.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmployerId => write!(f, "24"),
            Self::SocialSecurityNumber => write!(f, "34"),
            Self::ElectronicTransmitterId => write!(f, "46"),
            Self::FederalAgency => write!(f, "FA"),
            Self::FederalTaxpayerId => write!(f, "FI"),
            Self::StandardUniqueIdentifier => write!(f, "II"),
            Self::MemberId => write!(f, "MI"),
            Self::NationalId => write!(f, "NI"),
            Self::PayorId => write!(f, "PI"),
            Self::PharmacyProcessorId => write!(f, "PP"),
            Self::ServiceProviderId => write!(f, "SV"),
            Self::HealthCareFinancingAdministrationNationalProviderId => write!(f, "XV"),
            Self::HealthCareFinancingAdministrationNationalSupplierId => write!(f, "XX"),
        }
    }
}

impl Default for BenefitRelatedEntityIdentification {
    /// Returns the default identification code, which is Employer ID Number (24).
    fn default() -> BenefitRelatedEntityIdentification {
        Self::EmployerId
    }
}
