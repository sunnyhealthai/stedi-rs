use serde::{Deserialize, Serialize};

/// Provider entity information returned in healthcare eligibility check responses.
///
/// This struct represents information about the healthcare provider entity that submitted
/// the original eligibility check request. The provider information is returned by payers
/// in eligibility responses to identify who requested the eligibility information and may
/// be associated with specific benefits or benefit-related entities.
///
/// Providers can be individual practitioners, medical groups, hospitals, or other types of
/// healthcare providers. This object will always include at least one identifier, such as
/// the provider's NPI, tax ID, or EIN, to uniquely identify the provider.
///
/// ## Required Information
///
/// When present, this object will always include:
/// - **At least one identifier**: NPI, Federal Taxpayer ID (EIN), SSN, or other provider
///   identification number
///
/// Many payers will also return:
/// - **Provider name**: Either individual name (`providerFirstName`, `providerName`) or
///   organization name (`providerOrgName`)
/// - **Entity type**: `entityType` indicating Person or NonPersonEntity
/// - **Entity identifier**: `entityIdentifier` indicating the type of provider entity
/// - **Address**: Provider address information
///
/// ## Provider Identification
///
/// Providers can be identified through multiple methods:
///
/// - **NPI** (`npi`): National Provider Identifier - the standard healthcare provider identifier
/// - **Federal Taxpayer ID** (`federalTaxpayersIdNumber`): EIN or tax identification number
/// - **Social Security Number** (`ssn`): SSN for individual providers
/// - **Service Provider Number** (`serviceProviderNumber`): Payer-assigned provider number
/// - **Pharmacy Processor Number** (`pharmacyProcessorNumber`): For pharmacy providers
/// - **Payor Identification** (`payorIdentification`): Payer-specific provider identifier
/// - **CMS Plan ID** (`servicesPlanID`): Centers for Medicare and Medicaid Services plan ID
/// - **Taxonomy Code** (`referenceIdentification`): Healthcare Provider Taxonomy Code
///
/// ## Provider Types
///
/// Providers can be:
///
/// - **Individual Practitioners**: Doctors, nurses, therapists, or other individual healthcare
///   providers (use `providerFirstName`, `providerName`, `middleName`, `suffix`)
/// - **Organizations**: Hospitals, clinics, medical groups, or other healthcare organizations
///   (use `providerOrgName`)
///
/// The `entityType` field indicates whether the provider is a `Person` (individual) or
/// `NonPersonEntity` (organization).
///
/// ## Provider Entity Identifier
///
/// The `entityIdentifier` field specifies the type of provider entity:
/// - **Provider**: General healthcare provider
/// - **Hospital**: Hospital facility
/// - **Facility**: Healthcare facility
/// - **Third-Party Administrator**: TPA organization
/// - **Payer**: Insurance company or payer
/// - **Employer**: Employer organization
/// - **Plan Sponsor**: Plan sponsor organization
/// - **Gateway Provider**: Gateway or clearinghouse provider
///
/// ## Provider Code
///
/// The `providerCode` field indicates the provider's role in relation to the benefits
/// information, such as primary care provider, specialist, pharmacy, or facility. Visit
/// [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes)
/// for a complete list of provider codes.
///
/// ## Usage Context
///
/// Provider information in responses is used to:
///
/// - **Identify the requester**: Know which provider requested the eligibility information
/// - **Associate with benefits**: Link providers to specific benefits or benefit-related entities
/// - **Network matching**: Determine if providers are in-network or out-of-network
/// - **Provider relationships**: Understand provider relationships in care delivery
/// - **Error handling**: Identify and handle provider-specific errors
///
/// ## Error Handling
///
/// The `aaaErrors` field contains provider-specific errors that occurred during the eligibility
/// check. These errors are separate from general transaction errors and provide provider-level
/// error details.
///
/// ## X12 HIPAA
///
/// Maps to provider identification segments in X12 271 transactions, including:
/// - Provider identification segments (NM1, REF)
/// - Provider entity type qualifiers
/// - Provider address segments (N3, N4)
/// - Error segments (AAA) for provider-specific errors
///
/// ## Examples
///
/// Individual practitioner provider:
/// - `providerFirstName`: "Jane"
/// - `providerName`: "Smith"
/// - `npi`: "1234567890"
/// - `entityType`: `Person`
/// - `entityIdentifier`: `Provider`
///
/// Hospital provider:
/// - `providerOrgName`: "City General Hospital"
/// - `npi`: "1234567890"
/// - `entityType`: `NonPersonEntity`
/// - `entityIdentifier`: `Hospital`
///
/// ## Stedi Notes
///
/// This object will always include at least one identifier (NPI, tax ID, or EIN) to uniquely
/// identify the provider. The specific fields included vary by payer, but provider identification
/// is always present.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseProvider {
    /// Provider-specific errors that occurred during the eligibility check.
    ///
    /// Contains error information specific to the provider entity, separate from general
    /// transaction errors. These errors may indicate provider-specific issues such as
    /// invalid provider IDs, enrollment problems, or other provider-level errors.
    #[serde(rename = "aaaErrors", skip_serializing_if = "Option::is_none")]
    pub aaa_errors: Option<Vec<super::EligibilityCheckProviderError>>,
    /// The provider's address information.
    ///
    /// Contains physical address details for the provider, including street address, city,
    /// state, and postal code. Address information helps with provider identification and
    /// may be used for administrative purposes or network matching.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::Address>,
    /// Deprecated; The provider's identification number for the entity receiving the benefits information. This shape is deprecated: This property is no longer used.
    #[serde(rename = "employersId", skip_serializing_if = "Option::is_none")]
    pub employers_id: Option<String>,
    /// The type of provider entity, such as Provider, Hospital, Facility, or Payer.
    ///
    /// Classifies the provider entity type, which helps understand the nature of the provider
    /// entity. Common types include `Provider` for individual practitioners, `Hospital` for
    /// hospitals, `Facility` for clinics and other facilities, or `Payer` for payer entities.
    /// This identifier helps categorize providers for benefit determination and network matching.
    #[serde(rename = "entityIdentifier", skip_serializing_if = "Option::is_none")]
    pub entity_identifier: Option<super::ResponseProviderEntityIdentifier>,
    /// The entity type qualifier indicating whether the provider is a Person or NonPersonEntity.
    ///
    /// Specifies whether the provider is an individual person (`Person`) or an organization
    /// (`NonPersonEntity`). When the entity type is `Person`, use individual name fields
    /// (`providerFirstName`, `providerName`). When it's `NonPersonEntity`, use the organization
    /// name field (`providerOrgName`). In practice, most providers are `NonPersonEntity`
    /// organizations, though individual practitioners may be `Person` entities.
    ///
    /// Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<super::EntityTypeQualifier>,
    /// The Federal Taxpayer Identification Number (also known as an EIN).
    #[serde(
        rename = "federalTaxpayersIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub federal_taxpayers_id_number: Option<String>,
    /// The provider's middle name. This applies to providers that are an individual.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The provider's [National Provider Identifier (NPI)](https://www.stedi.com/docs/healthcare/national-provider-identifier).
    #[serde(rename = "npi", skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    /// The Payor Identification.
    #[serde(
        rename = "payorIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub payor_identification: Option<String>,
    /// The pharmacy processor number.
    #[serde(
        rename = "pharmacyProcessorNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub pharmacy_processor_number: Option<String>,
    /// A code that communicates the provider's role in the type of benefits information in the response. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "providerCode", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<super::ResponseProviderCode>,
    /// The provider's first name. This applies to providers that are an individual.
    #[serde(rename = "providerFirstName", skip_serializing_if = "Option::is_none")]
    pub provider_first_name: Option<String>,
    /// The provider's last name. This applies to providers that are an individual.
    #[serde(rename = "providerName", skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// The provider's organization name.
    #[serde(rename = "providerOrgName", skip_serializing_if = "Option::is_none")]
    pub provider_org_name: Option<String>,
    /// The Health Care Provider Taxonomy Code.
    #[serde(
        rename = "referenceIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_identification: Option<String>,
    /// The service provider number. This is an identification number assigned by the payer.
    #[serde(
        rename = "serviceProviderNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_provider_number: Option<String>,
    /// The Centers for Medicare and Medicaid Services (CMS) Plan ID.
    #[serde(rename = "servicesPlanID", skip_serializing_if = "Option::is_none")]
    pub services_plan_id: Option<String>,
    /// The Social Security Number (SSN).
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The provider's name suffix, such as Jr., Sr., or III.
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl ResponseProvider {
    /// Information about the entity that submitted the original eligibility check request. This may be an individual practitioner, a medical group, a hospital, or another type of healthcare provider. This object will always include at least one identifier, such as the provider's [NPI](https://www.stedi.com/docs/healthcare/national-provider-identifier), tax ID, or EIN.
    pub fn new() -> ResponseProvider {
        ResponseProvider {
            aaa_errors: None,
            address: None,
            employers_id: None,
            entity_identifier: None,
            entity_type: None,
            federal_taxpayers_id_number: None,
            middle_name: None,
            npi: None,
            payor_identification: None,
            pharmacy_processor_number: None,
            provider_code: None,
            provider_first_name: None,
            provider_name: None,
            provider_org_name: None,
            reference_identification: None,
            service_provider_number: None,
            services_plan_id: None,
            ssn: None,
            suffix: None,
        }
    }
}
