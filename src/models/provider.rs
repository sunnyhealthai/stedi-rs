use serde::{Deserialize, Serialize};

/// Healthcare provider information for eligibility check requests.
///
/// This structure contains identifying information about the healthcare provider or organization
/// requesting the eligibility check. The provider information helps payers identify who is
/// requesting eligibility information and may affect benefit determination, authorization
/// requirements, and network matching.
///
/// ## Provider Types
///
/// Providers can be:
///
/// - **Individual Practitioners**: Doctors, nurses, therapists, or other individual healthcare
///   providers (use `first_name` and `last_name`)
/// - **Organizations**: Hospitals, clinics, medical groups, or other healthcare organizations
///   (use `organization_name`)
/// - **Facilities**: Healthcare facilities such as hospitals, ambulatory surgery centers, or clinics
/// - **Other Entities**: Payers, TPAs, employers, or other entities in specific contexts
///
/// ## Required Fields
///
/// You must provide either:
/// - `organization_name` for organizational providers (hospitals, clinics, etc.)
/// - `first_name` and `last_name` for individual practitioners
///
/// You must also provide a primary identifier, typically the provider's National Provider
/// Identifier (NPI). If the provider doesn't have an NPI, alternative identifiers like
/// `tax_id` or `ssn` may be accepted, though this is rare and typically only for
/// non-traditional providers.
///
/// ## Provider Identification
///
/// The primary identifier is the **NPI** (National Provider Identifier), which is required
/// for all healthcare providers eligible to receive one. Some non-traditional providers
/// (transportation services, DME suppliers, alternative medicine practitioners) may not be
/// eligible for an NPI.
///
/// Alternative identifiers (when NPI is not available):
/// - **Tax ID** (`taxId`): Federal Taxpayer Identification Number (EIN or SSN)
/// - **SSN** (`ssn`): Social Security Number (rare, only when specifically required)
/// - **Pharmacy Processor Number**: For pharmacy providers without NPI
/// - **Service Provider Number**: For non-medical providers without NPI
///
/// ## Optional Identification Fields
///
/// Many payers require additional provider identifiers:
///
/// - **Medicare Provider Number**: For Medicare providers
/// - **Medicaid Provider Number**: For Medicaid providers
/// - **State License Number**: Provider's state professional license
/// - **Contract Number**: Provider's contract number with the payer
/// - **Facility ID**: Facility-specific identification numbers
/// - **Network ID**: Provider network identification numbers
/// - **Taxonomy Code**: Provider specialty classification code
///
/// ## Usage Context
///
/// Provider information is used to:
///
/// - **Identify the requester**: Know which provider is requesting eligibility information
/// - **Network matching**: Determine if the provider is in-network or out-of-network
/// - **Authorization requirements**: Different providers may have different authorization
///   requirements
/// - **Benefit determination**: Some benefits may be provider-specific or network-specific
/// - **Claims routing**: Route claims to the appropriate payer or administrator
/// - **Enrollment verification**: Verify that providers are properly enrolled with payers
///
/// ## Best Practices
///
/// - **Always use NPI when available**: It's the standard healthcare provider identifier
/// - **Only include required fields**: Most optional fields are only needed when specifically
///   required by a payer
/// - **Avoid sensitive information**: Don't include SSN unless absolutely necessary
/// - **Address information**: Only include when the provider has multiple locations and you
///   need to identify a specific location
/// - **Provider type**: Include when it helps clarify the provider's role or when required
///   by the payer
///
/// ## X12 HIPAA
///
/// Maps to provider identification segments in X12 270 transactions, including:
/// - NM1 segments for provider name and identification
/// - REF segments for provider identifiers (NPI, tax ID, state license, etc.)
/// - PRV segments for provider specialty information
/// - N3/N4 segments for provider address (when included)
///
/// ## Examples
///
/// Individual practitioner:
/// ```rust
/// use stedi_rs::models::{Provider, ProviderType};
///
/// let provider = Provider {
///     first_name: Some("Jane".to_string()),
///     last_name: Some("Smith".to_string()),
///     npi: Some("1234567890".to_string()),
///     provider_type: Some(ProviderType::Provider),
///     ..Default::default()
/// };
/// ```
///
/// Healthcare organization:
/// ```rust
/// use stedi_rs::models::{Provider, ProviderType};
///
/// let provider = Provider {
///     organization_name: Some("City General Hospital".to_string()),
///     npi: Some("1234567890".to_string()),
///     provider_type: Some(ProviderType::Hospital),
///     ..Default::default()
/// };
/// ```
///
/// ## Stedi Notes
///
/// Most payers require the NPI as the primary identifier. Requests with alternate IDs are
/// virtually never supported unless the payer has specifically instructed you to use them.
/// This is typically only for non-traditional providers who are not eligible for an NPI.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    /// The address of the provider requesting the information. Only include when specifically instructed by a payer, such as when the provider has multiple locations and you need to identify the specific location making the request. You must include at least the `address1` and `city` properties in this object.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::RequestAddress>,
    /// The provider's contract number. Only include when required by a payer. This shape is deprecated: Use `contractNumber` instead.
    #[serde(rename = "contactNumber", skip_serializing_if = "Option::is_none")]
    pub contact_number: Option<String>,
    /// The provider's contract number. Only include when required by a payer.
    #[serde(rename = "contractNumber", skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// The provider's electronic device pin number. Only include when required by a payer.
    #[serde(rename = "devicePinNumber", skip_serializing_if = "Option::is_none")]
    pub device_pin_number: Option<String>,
    /// Deprecated; The submitter's Employer's Identification Number (EIN). Only use when an employer is checking the eligibility and benefits of their employees. This shape is deprecated: This property is no longer used.
    #[serde(rename = "employersId", skip_serializing_if = "Option::is_none")]
    pub employers_id: Option<String>,
    /// The ID number for the provider's facility. Only include when required by a payer.
    #[serde(rename = "facilityIdNumber", skip_serializing_if = "Option::is_none")]
    pub facility_id_number: Option<String>,
    /// The provider's facility network identification number. Only include when required by a payer.
    #[serde(
        rename = "facilityNetworkIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub facility_network_id_number: Option<String>,
    /// The provider's first name. This property is required if the provider is an individual.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The two-character state ID of the state that assigned the `stateLicenseNumber`. Only include when required by a payer.
    #[serde(
        rename = "informationReceiverAdditionalIdentifierState",
        skip_serializing_if = "Option::is_none"
    )]
    pub information_receiver_additional_identifier_state: Option<String>,
    /// The provider's last name. This property is required if the provider is an individual.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The provider's Medicaid provider number. Only include when required by a payer.
    #[serde(
        rename = "medicaidProviderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub medicaid_provider_number: Option<String>,
    /// The provider's Medicare provider number. Only include when required by a payer.
    #[serde(
        rename = "medicareProviderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub medicare_provider_number: Option<String>,
    /// The provider's [National Provider Identifier (NPI)](https://www.stedi.com/docs/healthcare/national-provider-identifier). This identifier is required for all healthcare providers who are eligible to receive an NPI. Some non-traditional providers such as transportation services, durable medical equipment (DME) suppliers, or alternative medicine practitioners are not eligible to receive an NPI. If the provider doesn't have an NPI, requests with alternate IDs are virtually never supported. In the rare circumstance that a payer has instructed you to use an alternate ID, the payer will typically require you to supply either their `taxId` or `ssn` instead.
    #[serde(rename = "npi", skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    /// The provider's business name. This property is required if the provider is not an individual.
    #[serde(rename = "organizationName", skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    /// Only used for payer-to-payer transactions, which are not currently supported. Do not use.
    #[serde(rename = "payorId", skip_serializing_if = "Option::is_none")]
    pub payor_id: Option<String>,
    /// The provider's pharmacy processor number. Only include when specifically instructed by a payer - for example, when the provider doesn't have an [NPI](https://www.stedi.com/docs/healthcare/national-provider-identifier). This use case is very rarely supported, and is typically when the provider is a non-medical provider, such as a social worker, home health aide, or transportation service.
    #[serde(
        rename = "pharmacyProcessorNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub pharmacy_processor_number: Option<String>,
    /// The provider's prior identifier number. Only include when required by a payer.
    #[serde(
        rename = "priorIdentifierNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub prior_identifier_number: Option<String>,
    /// Communicate the provider's role in the type of benefits specified in the request. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes) for a complete list. Only include when required by a payer.
    #[serde(rename = "providerCode", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<super::RequestProviderCode>,
    /// The provider's plan network identification number. Only include when required by a payer.
    ///
    /// Identifies the provider within a specific payer's network. This is used when payers
    /// assign network-specific identification numbers to providers for tracking and routing
    /// purposes.
    #[serde(
        rename = "providerPlanNetworkIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_plan_network_id_number: Option<String>,
    /// The type of provider entity, such as Provider, Hospital, Facility, or Payer.
    ///
    /// Classifies the provider entity type, which helps payers understand the nature of the
    /// provider requesting eligibility information. Common types include `Provider` for
    /// individual practitioners, `Hospital` for hospitals, `Facility` for clinics and other
    /// facilities, or `Payer` for payer entities. Include when it helps clarify the provider's
    /// role or when required by the payer.
    #[serde(rename = "providerType", skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<super::ProviderType>,
    /// The provider's [Taxonomy Code](https://x12.org/codes/provider-taxonomy-codes). Only used when the provider's taxonomy code is relevant to the eligibility/benefit inquiry. For example, an institutional provider such as a hospital may need to use a taxonomy code to specify a specific unit or department.
    #[serde(
        rename = "referenceIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_identification: Option<String>,
    /// The provider's service provider number. Only include when specifically instructed by a payer - for example, when the provider doesn't have an [NPI](https://www.stedi.com/docs/healthcare/national-provider-identifier). This use case is very rarely supported, and is typically when the provider is a non-medical provider, such as a social worker, home health aide, or transportation service.
    #[serde(
        rename = "serviceProviderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_provider_number: Option<String>,
    /// The provider's services plan identification number.
    ///
    /// Identifies the provider within a specific services plan or program. This field is
    /// used when providers participate in specialized service plans or programs that require
    /// plan-specific identification. Only include when required by a payer.
    #[serde(rename = "servicesPlanID", skip_serializing_if = "Option::is_none")]
    pub services_plan_id: Option<String>,
    /// The provider's Social Security Number (SSN).     - Only include when specifically instructed by a payer - for example, if the provider doesn't have an [NPI](https://www.stedi.com/docs/healthcare/national-provider-identifier). This use case is very rarely supported, and is typically when the provider is a non-medical provider, such as a social worker, home health aide, or transportation service.     - If the payer has instructed you to send an EIN but the provider operates using their SSN, use provider.taxId instead of this field.     - Don't use this for Federally-administered programs, such as Medicare.
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The provider's state license number. If you include this information, you must also include the `informationReceiverAdditionalIdentifierState`. Only include when required by a payer.
    #[serde(rename = "stateLicenceNumber", skip_serializing_if = "Option::is_none")]
    pub state_licence_number: Option<String>,
    /// The provider's submitter identification number. Only include when required by a payer.
    #[serde(rename = "submitterIdNumber", skip_serializing_if = "Option::is_none")]
    pub submitter_id_number: Option<String>,
    /// The provider's Federal Taxpayer Identification Number. This is typically the provider's EIN (Employer Identification Number), but the provider's SSN may be used if the provider does not have an EIN. Only include if required by the payer.
    #[serde(rename = "taxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

impl Provider {
    /// Creates a new `Provider` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty provider object. After creating the provider,
    /// you must populate either the `organization_name` (for organizations) or both `first_name`
    /// and `last_name` (for individuals), along with a primary identifier like `npi`.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{Provider, ProviderType};
    ///
    /// // Create a provider for an individual practitioner
    /// let mut provider = Provider::new();
    /// provider.first_name = Some("Jane".to_string());
    /// provider.last_name = Some("Smith".to_string());
    /// provider.npi = Some("1234567890".to_string());
    /// provider.provider_type = Some(ProviderType::Provider);
    ///
    /// // Or create a provider for an organization
    /// let mut hospital = Provider::new();
    /// hospital.organization_name = Some("Memorial Hospital".to_string());
    /// hospital.npi = Some("1234567890".to_string());
    /// hospital.provider_type = Some(ProviderType::Hospital);
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Required fields**: You must provide either `organization_name` (for organizations)
    ///   or both `first_name` and `last_name` (for individuals)
    /// - **Primary identifier**: Always include `npi` when available - it's the standard
    ///   healthcare provider identifier
    /// - **Optional fields**: Most other fields are only needed when specifically required
    ///   by a payer
    /// - **Provider type**: Include `provider_type` when it helps clarify the provider's
    ///   role or when required by the payer
    /// - **Sensitive information**: Avoid including `ssn` unless absolutely necessary
    ///
    /// # Returns
    ///
    /// A new `Provider` instance with all optional fields set to `None`.
    pub fn new() -> Provider {
        Provider {
            address: None,
            contact_number: None,
            contract_number: None,
            device_pin_number: None,
            employers_id: None,
            facility_id_number: None,
            facility_network_id_number: None,
            first_name: None,
            information_receiver_additional_identifier_state: None,
            last_name: None,
            medicaid_provider_number: None,
            medicare_provider_number: None,
            npi: None,
            organization_name: None,
            payor_id: None,
            pharmacy_processor_number: None,
            prior_identifier_number: None,
            provider_code: None,
            provider_plan_network_id_number: None,
            provider_type: None,
            reference_identification: None,
            service_provider_number: None,
            services_plan_id: None,
            ssn: None,
            state_licence_number: None,
            submitter_id_number: None,
            tax_id: None,
        }
    }
}
