use serde::{Deserialize, Serialize};

/// Error codes for provider-level AAA errors in eligibility check responses.
///
/// These codes represent standardized X12 error codes (AAA errors) that payers return when
/// there are issues with an eligibility check request specifically related to the provider
/// information. AAA errors indicate problems that prevented the payer from processing the
/// eligibility check successfully.
///
/// This enum is used in `eligibilityCheckProviderError.code` and `eligibilityCheckResponse.errors`
/// to specify the reason for rejection and any recommended follow-up actions. Common reasons
/// for rejection at the provider level include:
/// - Missing or invalid provider identification (NPI, Tax ID, name, address, phone, state)
/// - Provider not enrolled or registered with the payer
/// - Provider enrollment issues (specialty mismatch, ineligible for inquiries)
/// - Authorization or access restrictions
/// - Invalid referring provider identification
/// - Transaction enrollment requirements not met
///
/// ## Error Categories
///
/// The error codes can be grouped into several categories:
///
/// - **Provider Identification Errors** (`43`, `44`, `45`, `46`, `47`, `97`): Missing or invalid
///   provider NPI, name, specialty, phone number, state, or address. These errors often indicate
///   that the provider information doesn't match what's registered with the payer or in the
///   NPPES (National Plan and Provider Enumeration System) database.
///
/// - **Provider Enrollment Errors** (`50`, `51`): Provider not registered with the payer, not
///   enrolled for eligibility checks, or ineligible for inquiries. These errors typically require
///   the provider to contact the payer directly to complete credentialing or enrollment processes.
///
/// - **Authorization/Access Errors** (`41`): Authorization or access restrictions preventing the
///   provider from submitting eligibility checks. This may be due to missing transaction enrollment,
///   invalid portal credentials, or other access restrictions.
///
/// - **Application Data Errors** (`15`): Missing required application data, such as the provider's
///   Tax ID (EIN or SSN). The payer needs additional information to process the eligibility check.
///
/// - **Referring Provider Errors** (`48`): Invalid or missing referring provider identification
///   number. The referring provider specified in the request isn't enrolled correctly with the payer
///   or isn't registered with the payer's health plans.
///
/// - **Participant/Payer Errors** (`79`, `T4`): Invalid participant identification or missing payer
///   name/identifier. These errors may indicate connectivity issues or configuration problems.
///
/// ## Common Resolution Steps
///
/// Many provider-level errors require the provider to contact the payer directly due to PHI/HIPAA
/// guidelines. Common resolution steps include:
///
/// - **Verify provider information**: Ensure the NPI, name, address, phone number, and specialty
///   match what's registered with the payer and in the NPPES system
/// - **Complete transaction enrollment**: Some payers require transaction enrollment before processing
///   eligibility checks. Check the [Stedi Payer Network](https://www.stedi.com/healthcare/network)
///   to determine if enrollment is required
/// - **Complete credentialing/enrollment**: Providers may need to complete credentialing or
///   enrollment processes with the payer before they can submit eligibility checks
/// - **Include required fields**: Ensure all required fields are included, such as `provider.taxId`
///   (EIN or SSN) when requested
/// - **Verify portal credentials**: If using portal credentials, ensure `portalUsername` and/or
///   `portalPassword` are correct
///
/// These errors are also included in the top-level `errors` array for centralized error review,
/// along with errors from payer, subscriber, and dependent levels.
///
/// Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
/// for detailed guidance on understanding and resolving AAA errors. Payers may sometimes
/// return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EligibilityCheckProviderErrorCode {
    #[serde(rename = "15")]
    #[default]
    RequiredApplicationDataMissing,
    #[serde(rename = "41")]
    AuthorizationAccessRestrictions,
    #[serde(rename = "43")]
    InvalidMissingProviderIdentification,
    #[serde(rename = "44")]
    InvalidMissingProviderName,
    #[serde(rename = "45")]
    InvalidMissingProviderSpecialty,
    #[serde(rename = "46")]
    InvalidMissingProviderPhoneNumber,
    #[serde(rename = "47")]
    InvalidMissingProviderState,
    #[serde(rename = "48")]
    InvalidMissingReferringProviderIdentificationNumber,
    #[serde(rename = "50")]
    ProviderIneligibleForInquiries,
    #[serde(rename = "51")]
    ProviderNotOnFile,
    #[serde(rename = "79")]
    InvalidParticipantIdentification,
    #[serde(rename = "97")]
    InvalidMissingProviderAddress,
    #[serde(rename = "T4")]
    PayerNameOrIdentifierMissing,
}

impl std::fmt::Display for EligibilityCheckProviderErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RequiredApplicationDataMissing => write!(f, "15"),
            Self::AuthorizationAccessRestrictions => write!(f, "41"),
            Self::InvalidMissingProviderIdentification => write!(f, "43"),
            Self::InvalidMissingProviderName => write!(f, "44"),
            Self::InvalidMissingProviderSpecialty => write!(f, "45"),
            Self::InvalidMissingProviderPhoneNumber => write!(f, "46"),
            Self::InvalidMissingProviderState => write!(f, "47"),
            Self::InvalidMissingReferringProviderIdentificationNumber => write!(f, "48"),
            Self::ProviderIneligibleForInquiries => write!(f, "50"),
            Self::ProviderNotOnFile => write!(f, "51"),
            Self::InvalidParticipantIdentification => write!(f, "79"),
            Self::InvalidMissingProviderAddress => write!(f, "97"),
            Self::PayerNameOrIdentifierMissing => write!(f, "T4"),
        }
    }
}
