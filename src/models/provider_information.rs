use serde::{Deserialize, Serialize};

/// Additional provider information associated with benefits in eligibility responses.
///
/// This struct provides supplementary information about healthcare providers that are
/// associated with specific benefits or benefit-related entities. It extends basic provider
/// identification with role codes and taxonomy information to provide context about how
/// providers relate to the benefits being described.
///
/// ## Provider Information Components
///
/// Provider information includes:
///
/// - **Provider Code** (`providerCode`): Indicates the provider's role in relation to the
///   benefits information, such as primary care provider, specialist, or pharmacy
/// - **Reference Identification** (`referenceIdentification`): Typically contains the
///   provider's taxonomy code or other provider-specific identifiers
///
/// ## Provider Codes
///
/// The `providerCode` field communicates the provider's role in the benefits information.
/// Common provider codes include:
///
/// - **Primary Care Provider**: The member's primary care physician
/// - **Specialist**: Specialized healthcare providers
/// - **Pharmacy**: Pharmacy providers for prescription benefits
/// - **Facility**: Healthcare facilities such as hospitals or clinics
/// - **Other Provider Types**: Various other provider roles as defined in the code lists
///
/// Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes)
/// for a complete list of provider codes.
///
/// ## Taxonomy Codes
///
/// The `referenceIdentification` field typically contains the provider's taxonomy code,
/// which is a standardized classification system for healthcare providers. Taxonomy codes
/// identify the provider's specialty, type, and classification, helping to categorize
/// providers for benefit determination and network matching.
///
/// Taxonomy codes follow the Healthcare Provider Taxonomy Code Set maintained by the
/// National Uniform Claim Committee (NUCC) and are typically 10 characters long (e.g.,
/// "207Q00000X" for Family Medicine).
///
/// ## Usage Context
///
/// Provider information is used in eligibility responses to:
///
/// - **Identify provider roles**: Understand what role a provider plays in relation to
///   specific benefits
/// - **Network matching**: Determine if providers are in-network or out-of-network
/// - **Benefit restrictions**: Identify providers that may be required or restricted for
///   certain benefits
/// - **Care coordination**: Understand provider relationships in care delivery
/// - **Claims routing**: Route claims to the appropriate provider entities
///
/// ## Relationship to Other Structures
///
/// Provider information is typically found in:
///
/// - **Benefits Related Entities**: Providers associated with specific benefits, such as
///   PBMs for pharmacy benefits or carveout administrators
/// - **Response Providers**: Additional provider details beyond basic provider identification
/// - **Plan Information**: Providers associated with plan administration or coordination
///
/// ## X12 HIPAA
///
/// Maps to provider information segments in X12 271 transactions, including:
/// - Provider code elements in benefit information loops
/// - Reference identification segments (REF) for provider taxonomy codes
/// - Provider role indicators in benefit-related entity segments
///
/// ## Examples
///
/// Provider information for a primary care provider might include:
/// - `providerCode`: `PrimaryCareProvider`
/// - `referenceIdentification`: "207Q00000X" (Family Medicine taxonomy code)
///
/// Provider information for a pharmacy benefit manager might include:
/// - `providerCode`: `Pharmacy`
/// - `referenceIdentification`: Provider-specific identifier or taxonomy code
///
/// ## Stedi Notes
///
/// Provider information is provided by payers in eligibility responses to give context
/// about providers associated with benefits. The specific provider codes and reference
/// identifications vary by payer and benefit type. Payers may sometimes return other
/// non-compliant values.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderInformation {
    /// A code that communicates the provider's role in the type of benefits information in the response.
    ///
    /// This field indicates what role the provider plays in relation to the benefits being
    /// described. Common roles include primary care provider, specialist, pharmacy, facility,
    /// or other provider types. The provider code helps understand how the provider relates
    /// to the specific benefits or benefit-related entities.
    ///
    /// Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes)
    /// for a complete list of provider codes and their meanings.
    ///
    /// Payers may sometimes return other non-compliant values.
    #[serde(rename = "providerCode", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<super::ResponseProviderCode>,
    /// The provider's taxonomy code or other reference identification.
    ///
    /// This field typically contains the provider's taxonomy code, which is a standardized
    /// 10-character code that classifies healthcare providers by specialty, type, and
    /// classification. Taxonomy codes follow the Healthcare Provider Taxonomy Code Set
    /// maintained by the National Uniform Claim Committee (NUCC).
    ///
    /// Common taxonomy code examples:
    /// - `207Q00000X`: Family Medicine
    /// - `208D00000X`: General Practice
    /// - `332B00000X`: Pharmacy
    /// - `282N00000X`: General Acute Care Hospital
    ///
    /// The taxonomy code helps categorize providers for benefit determination, network
    /// matching, and claims processing. In some cases, this field may contain other
    /// provider-specific identifiers instead of or in addition to taxonomy codes.
    #[serde(
        rename = "referenceIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_identification: Option<String>,
}

impl ProviderInformation {
    /// Creates a new `ProviderInformation` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty provider information object. In practice,
    /// provider information is populated by payers in eligibility responses, so this
    /// constructor is primarily useful for testing or when manually constructing provider
    /// information objects.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{ProviderInformation, ResponseProviderCode};
    ///
    /// // Create a new provider information instance
    /// let mut provider_info = ProviderInformation::new();
    ///
    /// // Set provider information fields
    /// provider_info.provider_code = Some(ResponseProviderCode::PrimaryCarePhysician);
    /// provider_info.reference_identification = Some("207Q00000X".to_string()); // Family Medicine taxonomy
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Response-only structure**: This struct is typically populated by payers in
    ///   eligibility responses, not constructed by API consumers
    /// - **Provider codes**: Indicate the provider's role in relation to benefits
    /// - **Taxonomy codes**: The `referenceIdentification` field typically contains
    ///   standardized taxonomy codes for provider classification
    /// - **Usage context**: Provider information provides context about providers
    ///   associated with specific benefits or benefit-related entities
    ///
    /// # Returns
    ///
    /// A new `ProviderInformation` instance with all optional fields set to `None`.
    pub fn new() -> ProviderInformation {
        ProviderInformation {
            provider_code: None,
            reference_identification: None,
        }
    }
}
