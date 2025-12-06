use serde::{Deserialize, Serialize};

/// Information receiver name structure for provider identification (deprecated).
///
/// **Deprecated**: Use the corresponding properties in the `provider` object instead.
/// This struct was previously used to identify the entity receiving healthcare eligibility
/// information, but the `provider` object now serves this purpose with a cleaner, more
/// standardized structure.
///
/// ## Deprecation Note
///
/// This struct is maintained for backward compatibility but should not be used in new code.
/// Instead, use the `provider` object in `eligibilityCheckRequestContent`, which provides
/// equivalent functionality with better structure and clearer naming.
///
/// ## Historical Context
///
/// In X12 HIPAA transactions, this structure maps to information receiver name segments
/// (NM1 segments) that identify the entity receiving eligibility information. The structure
/// contains various provider identifiers and contact information that may be required by
/// some payers for transaction routing or identification purposes.
///
/// ## Provider Identification Fields
///
/// This struct contains various provider identifiers including:
/// - National Provider Identifier (NPI)
/// - Federal Taxpayer Identification Number (EIN/SSN)
/// - Medicare and Medicaid provider numbers
/// - Facility and network identifiers
/// - State license numbers
/// - Contract and submitter identifiers
///
/// All of these fields are optional and should only be included when specifically required
/// or suggested by the payer. Most payers only require the NPI or Tax ID.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationReceiverName {
    /// The provider's address information.
    ///
    /// Contains the physical address of the provider or organization. Only include this
    /// field when specifically required by the payer, as most payers don't need address
    /// information for eligibility checks.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::RequestAddress>,
    /// The provider's contact phone number.
    ///
    /// A phone number where the provider can be reached. This is rarely required for
    /// eligibility checks and should only be included when specifically requested by
    /// the payer.
    #[serde(rename = "contactNumber", skip_serializing_if = "Option::is_none")]
    pub contact_number: Option<String>,
    /// The contract number between the provider and payer.
    ///
    /// Some payers use contract numbers to identify specific provider agreements or
    /// network contracts. Only include this when the payer specifically requires it.
    #[serde(rename = "contractNumber", skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// The device PIN number for electronic transactions.
    ///
    /// Some payers require a device PIN for electronic transaction authentication or
    /// identification. This is payer-specific and should only be included when required.
    #[serde(rename = "devicePinNumber", skip_serializing_if = "Option::is_none")]
    pub device_pin_number: Option<String>,
    /// The facility identification number.
    ///
    /// A payer-assigned identifier for a specific healthcare facility or location.
    /// This may be required for multi-location providers or when the payer needs to
    /// identify a specific facility.
    #[serde(rename = "facilityIdNumber", skip_serializing_if = "Option::is_none")]
    pub facility_id_number: Option<String>,
    /// The facility network identification number.
    ///
    /// Identifies a specific facility within a provider network. Used when payers need
    /// to distinguish between multiple facilities operated by the same provider organization.
    #[serde(
        rename = "facilityNetworkIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub facility_network_id_number: Option<String>,
    /// The provider's Federal Taxpayer Identification Number (EIN or SSN).
    ///
    /// Also known as Employer Identification Number (EIN) for organizations or Social
    /// Security Number (SSN) for individual providers. Some payers require this for
    /// provider identification, especially when the NPI is not available or as an
    /// additional identifier. Only include when specifically required by the payer.
    #[serde(
        rename = "federalTaxpayerIdentificationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub federal_taxpayer_identification_number: Option<String>,
    /// The state associated with an additional provider identifier.
    ///
    /// Used when providing state-specific provider identifiers. This field specifies
    /// which state the additional identifier belongs to.
    #[serde(
        rename = "informationReceiverAdditionalIdentifierState",
        skip_serializing_if = "Option::is_none"
    )]
    pub information_receiver_additional_identifier_state: Option<String>,
    /// The provider's Medicaid provider number.
    ///
    /// A state-assigned identifier for providers enrolled in Medicaid programs. This
    /// is typically required when checking eligibility for Medicaid beneficiaries or
    /// when the payer needs to verify Medicaid provider enrollment.
    #[serde(
        rename = "medicaidProviderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub medicaid_provider_number: Option<String>,
    /// The provider's Medicare provider number.
    ///
    /// A Centers for Medicare and Medicaid Services (CMS) assigned identifier for
    /// providers enrolled in Medicare programs. This may be required when checking
    /// eligibility for Medicare beneficiaries or when verifying Medicare provider status.
    #[serde(
        rename = "medicareProviderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub medicare_provider_number: Option<String>,
    /// The provider's [National Provider Identifier](https://www.stedi.com/docs/healthcare/national-provider-identifier) (NPI).
    ///
    /// The standard 10-digit identifier assigned to healthcare providers by CMS. This
    /// is the most commonly used provider identifier and is typically required for
    /// eligibility checks. The NPI uniquely identifies healthcare providers in the
    /// National Plan and Provider Enumeration System (NPPES).
    #[serde(
        rename = "nationalProviderIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub national_provider_identifier: Option<String>,
    /// A prior or previous identifier number for the provider.
    ///
    /// Used when a provider has changed identifiers (e.g., changed from an old NPI to
    /// a new one, or from a legacy identifier to an NPI). This helps payers match
    /// historical records or verify provider identity during transitions.
    #[serde(
        rename = "priorIdentifierNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub prior_identifier_number: Option<String>,
    /// The provider's plan network identification number.
    ///
    /// A payer-assigned identifier that links the provider to a specific network or
    /// plan. This may be used when providers participate in multiple networks or when
    /// the payer needs to identify the specific network contract.
    #[serde(
        rename = "providerPlanNetworkIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_plan_network_id_number: Option<String>,
    /// The provider's Social Security Number (SSN).
    ///
    /// Used for individual providers who don't have an NPI or when the payer requires
    /// SSN as an additional identifier. Many payers prefer NPI over SSN due to privacy
    /// concerns, so only include SSN when specifically required.
    #[serde(
        rename = "socialSecurityNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub social_security_number: Option<String>,
    /// The provider's state license number.
    ///
    /// A state-issued professional license number (e.g., medical license, nursing
    /// license). Some payers require this for certain provider types or when verifying
    /// professional credentials. Only include when specifically required by the payer.
    #[serde(rename = "stateLicenceNumber", skip_serializing_if = "Option::is_none")]
    pub state_licence_number: Option<String>,
    /// The submitter identification number.
    ///
    /// An identifier assigned by the payer to the entity submitting the eligibility
    /// check request. This may be used for transaction tracking, billing, or
    /// administrative purposes. Only include when the payer requires it.
    #[serde(rename = "submitterIdNumber", skip_serializing_if = "Option::is_none")]
    pub submitter_id_number: Option<String>,
}

impl InformationReceiverName {
    /// Creates a new `InformationReceiverName` instance with all fields initialized to `None`.
    ///
    /// **Deprecated**: This struct is deprecated. Use the `provider` object in
    /// `eligibilityCheckRequestContent` instead.
    ///
    /// This constructor provides a clean starting point for building information receiver
    /// name data structures. All optional fields are set to `None` and can be populated
    /// individually based on payer requirements.
    ///
    /// ## Usage Note
    ///
    /// This struct should not be used in new code. Instead, use the `provider` object
    /// which provides equivalent functionality with better structure. The `provider`
    /// object includes fields like `npi`, `taxId`, `ssn`, `address`, etc., which map
    /// to the fields in this deprecated struct.
    ///
    /// # Returns
    ///
    /// A new `InformationReceiverName` instance with all optional fields set to `None`.
    pub fn new() -> InformationReceiverName {
        InformationReceiverName {
            address: None,
            contact_number: None,
            contract_number: None,
            device_pin_number: None,
            facility_id_number: None,
            facility_network_id_number: None,
            federal_taxpayer_identification_number: None,
            information_receiver_additional_identifier_state: None,
            medicaid_provider_number: None,
            medicare_provider_number: None,
            national_provider_identifier: None,
            prior_identifier_number: None,
            provider_plan_network_id_number: None,
            social_security_number: None,
            state_licence_number: None,
            submitter_id_number: None,
        }
    }
}
