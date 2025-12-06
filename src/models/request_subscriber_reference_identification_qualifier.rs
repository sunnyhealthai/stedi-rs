use serde::{Deserialize, Serialize};

/// RequestSubscriberReferenceIdentificationQualifier : Use this for providers that are not requesting the eligibility check. This is the type of `providerIdentifier` you are providing. - Set to `HPI` when the National Provider ID is mandated for use. - Set to `PXC` if you're identifying a type of specialty associated with services provided to the subscriber.     Otherwise, you can set to the following: `9K` - Servicer, `D3` - National Council for Prescription Drug Programs Pharmacy Number, `EI` - Employer's Identification Number, `HPI` - Centers for Medicare and Medicaid Services National Provider Identifier, `PXC` - Health Care Provider Taxonomy Code, `SY - Social Security Number, `TJ` - Federal Taxpayer's Identification Number
/// Use this for providers that are not requesting the eligibility check. This is the type of `providerIdentifier` you are providing. - Set to `HPI` when the National Provider ID is mandated for use. - Set to `PXC` if you're identifying a type of specialty associated with services provided to the subscriber.     Otherwise, you can set to the following: `9K` - Servicer, `D3` - National Council for Prescription Drug Programs Pharmacy Number, `EI` - Employer's Identification Number, `HPI` - Centers for Medicare and Medicaid Services National Provider Identifier, `PXC` - Health Care Provider Taxonomy Code, `SY - Social Security Number, `TJ` - Federal Taxpayer's Identification Number
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum RequestSubscriberReferenceIdentificationQualifier {
    /// Servicer
    #[serde(rename = "9K")]
    #[default]
    Servicer,
    /// National Council for Prescription Drug Programs Pharmacy Number
    #[serde(rename = "D3")]
    PharmacyNumber,
    /// Employer's Identification Number
    #[serde(rename = "EI")]
    EmployerIdentificationNumber,
    /// Centers for Medicare and Medicaid Services National Provider Identifier
    #[serde(rename = "HPI")]
    NationalProviderIdentifier,
    /// Health Care Provider Taxonomy Code
    #[serde(rename = "PXC")]
    HealthCareProviderTaxonomyCode,
    /// Social Security Number
    #[serde(rename = "SY")]
    SocialSecurityNumber,
    /// Federal Taxpayer's Identification Number
    #[serde(rename = "TJ")]
    FederalTaxpayerIdentificationNumber,
}

impl std::fmt::Display for RequestSubscriberReferenceIdentificationQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Servicer => write!(f, "9K"),
            Self::PharmacyNumber => write!(f, "D3"),
            Self::EmployerIdentificationNumber => write!(f, "EI"),
            Self::NationalProviderIdentifier => write!(f, "HPI"),
            Self::HealthCareProviderTaxonomyCode => write!(f, "PXC"),
            Self::SocialSecurityNumber => write!(f, "SY"),
            Self::FederalTaxpayerIdentificationNumber => write!(f, "TJ"),
        }
    }
}
