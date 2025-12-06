use serde::{Deserialize, Serialize};

/// Information about the payer providing the benefits information in eligibility responses.
///
/// This struct represents the healthcare payer organization that responds to eligibility
/// inquiries. The payer is the insurance company, third-party administrator, or other
/// entity responsible for providing coverage and processing claims. This information
/// is included in all eligibility responses to identify which payer provided the benefit
/// information.
///
/// ## Required Fields
///
/// The response will always include at least:
/// - **Business name** (`name`): The payer's organizational name
/// - **An identifier**: Typically the payer's tax ID (`federalTaxpayersIdNumber`), but
///   may also include NPI, NAIC, ETIN, or other identification numbers
///
/// Most payers also include contact information for follow-up inquiries or claims submission.
///
/// ## Identification Fields
///
/// Payers can be identified through multiple identification numbers:
///
/// - **Federal Taxpayer ID** (`federalTaxpayersIdNumber`): The payer's federal tax
///   identification number (EIN or SSN), most commonly used identifier
/// - **NPI** (`npi`): National Provider Identifier, used when the payer is also a
///   healthcare provider
/// - **NAIC** (`naic`): National Association of Insurance Commissioners identification
///   number for insurance companies
/// - **ETIN** (`etin`): Electronic Transmitter Identification Number for electronic
///   transaction processing
/// - **CMS Plan ID** (`centersForMedicareAndMedicaidPlanId`): Centers for Medicare and
///   Medicaid Services plan identifier for Medicare/Medicaid plans
/// - **Payor Identification** (`payorIdentification`): Payer-specific identification
///   number or code
///
/// ## Entity Type
///
/// The `entityType` field indicates whether the payer is:
/// - **Non-Person Entity** (most common): Insurance companies, organizations, corporations
/// - **Person** (rare): Individual payers, which is uncommon in modern healthcare
///
/// When the entity type is `NonPersonEntity`, use the `name` field for the organization
/// name. When it's `Person`, use `firstName`, `lastName`, `middleName`, and `suffix` fields.
///
/// ## Contact Information
///
/// Most payers provide contact information (`contactInformation`) including:
/// - Phone numbers for provider services
/// - Fax numbers for document submission
/// - Email addresses for electronic communication
/// - Web URLs for provider portals (note: URLs may be partial and require scheme prefixes)
///
/// ## Error Handling
///
/// The `aaaErrors` field contains payer-specific errors that occurred during the
/// eligibility check. These errors are separate from general transaction errors and
/// provide payer-level error details.
///
/// ## Usage Context
///
/// The payer information is used to:
/// - **Identify the responding payer**: Know which insurance company provided the
///   eligibility information
/// - **Route follow-up communications**: Contact the payer for additional information
///   or claims submission
/// - **Validate responses**: Verify that the response came from the expected payer
/// - **Coordination of benefits**: Identify primary vs secondary payers when multiple
///   insurance policies are involved
/// - **Regulatory compliance**: Track which payer provided information for audit purposes
///
/// ## X12 HIPAA
///
/// Maps to payer identification segments in X12 271 transactions, including:
/// - N1 segments for payer identification and name
/// - REF segments for payer identification numbers (tax ID, NPI, NAIC, etc.)
/// - PER segments for payer contact information
/// - Error segments (AAA) for payer-specific errors
///
/// ## Examples
///
/// A typical payer response might include:
/// - `name`: "Blue Cross Blue Shield of California"
/// - `federalTaxpayersIdNumber`: "12-3456789"
/// - `naic`: "12345"
/// - `contactInformation`: Phone, fax, and web portal information
/// - `entityType`: `NonPersonEntity`
/// - `entityIdentifier`: `Payer`
///
/// ## Stedi Notes
///
/// This structure is populated by payers in eligibility responses. The specific fields
/// included vary by payer, but business name and at least one identifier are always
/// present. Contact information is provided by most but not all payers.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Payer {
    /// Payer-specific errors that occurred during the eligibility check.
    ///
    /// Contains error information specific to the payer entity, separate from general
    /// transaction errors. These errors may indicate payer-specific issues such as
    /// invalid member IDs, coverage terminations, or payer system problems.
    #[serde(rename = "aaaErrors", skip_serializing_if = "Option::is_none")]
    pub aaa_errors: Option<Vec<super::EligibilityCheckPayerError>>,
    /// The payer's Centers for Medicare and Medicaid Services PlanID.
    #[serde(
        rename = "centersForMedicareAndMedicaidPlanId",
        skip_serializing_if = "Option::is_none"
    )]
    pub centers_for_medicare_and_medicaid_plan_id: Option<String>,
    /// The payer's contact information.  Note that when `contacts.communicationMode` is set to `UR`, the `communicationNumber` property may not contain a valid URL. Most payers provide a partial web address for their provider portal, or something similar, such as `www.example.com/portal`. You must add the appropriate scheme and separators, such as `https://` or `http://`, to make it a valid URL.
    #[serde(rename = "contactInformation", skip_serializing_if = "Option::is_none")]
    pub contact_information: Option<super::ContactInformation>,
    /// Deprecated; The payer's identification number for the entity receiving the benefits information. This shape is deprecated: This property is no longer used.
    #[serde(rename = "employersId", skip_serializing_if = "Option::is_none")]
    pub employers_id: Option<String>,
    /// The type of payer entity, such as Payer, Third-Party Administrator, or Employer.
    ///
    /// This field identifies the role or type of the payer entity. Most commonly set to
    /// `Payer` for standard insurance companies, but may also indicate `ThirdPartyAdministrator`,
    /// `Employer`, `PlanSponsor`, or `GatewayProvider` depending on the entity's role in
    /// the healthcare transaction.
    #[serde(rename = "entityIdentifier", skip_serializing_if = "Option::is_none")]
    pub entity_identifier: Option<super::PayerEntityIdentifier>,
    /// The entity type qualifier for the payer. Can be set to `Person` (not commonly used) or `Non-Person Entity` (most common).  Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<super::EntityTypeQualifier>,
    /// The payer's Electronic Transmitter Identification Number (ETIN).
    #[serde(rename = "etin", skip_serializing_if = "Option::is_none")]
    pub etin: Option<String>,
    /// The payer's federal taxpayer's identification number.
    #[serde(
        rename = "federalTaxpayersIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub federal_taxpayers_id_number: Option<String>,
    /// The payer's first name, when the payer is an individual (not commonly used).
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The payer's last name. Used when the payer is an individual (not commonly used).
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The payer's middle name or initial, when the payer is an individual (not commonly used).
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The payer's National Association of Insurance Commissioners (NAIC) identification number.
    #[serde(rename = "naic", skip_serializing_if = "Option::is_none")]
    pub naic: Option<String>,
    /// The payer's business name, when the payer is not a person.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The payer's [National Provider Identifier (NPI)](https://www.stedi.com/docs/healthcare/national-provider-identifier).
    ///
    /// Used when the payer is also a healthcare provider. The NPI is a unique 10-digit
    /// identifier assigned to healthcare providers by the Centers for Medicare and
    /// Medicaid Services (CMS).
    #[serde(rename = "npi", skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    /// The payer's identification number or code used for transaction routing and identification.
    ///
    /// This is a payer-specific identifier that may be used for routing transactions,
    /// identifying the payer in electronic transactions, or referencing the payer in
    /// follow-up communications. The format and meaning vary by payer.
    #[serde(
        rename = "payorIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub payor_identification: Option<String>,
    /// The payer's name suffix, such as Jr. or III. Used when the payer is an individual (not commonly used).
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl Payer {
    /// Creates a new `Payer` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty payer object. In practice, payer information
    /// is populated by payers in eligibility responses, so this constructor is primarily
    /// useful for testing or when manually constructing payer objects.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{Payer, EntityTypeQualifier, PayerEntityIdentifier};
    ///
    /// // Create a new payer instance
    /// let mut payer = Payer::new();
    ///
    /// // Set payer identification fields
    /// payer.name = Some("Blue Cross Blue Shield".to_string());
    /// payer.federal_taxpayers_id_number = Some("12-3456789".to_string());
    /// payer.entity_type = Some(EntityTypeQualifier::NonPersonEntity);
    /// payer.entity_identifier = Some(PayerEntityIdentifier::Payer);
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Response-only structure**: This struct is typically populated by payers in
    ///   eligibility responses, not constructed by API consumers
    /// - **Required fields**: Payers always include at least `name` and one identifier
    ///   field (typically `federalTaxpayersIdNumber`)
    /// - **Entity type**: Most payers are `NonPersonEntity` with organization names,
    ///   not individual persons
    /// - **Contact information**: Most but not all payers include contact information
    ///
    /// # Returns
    ///
    /// A new `Payer` instance with all optional fields set to `None`.
    pub fn new() -> Payer {
        Payer {
            aaa_errors: None,
            centers_for_medicare_and_medicaid_plan_id: None,
            contact_information: None,
            employers_id: None,
            entity_identifier: None,
            entity_type: None,
            etin: None,
            federal_taxpayers_id_number: None,
            first_name: None,
            last_name: None,
            middle_name: None,
            naic: None,
            name: None,
            npi: None,
            payor_identification: None,
            suffix: None,
        }
    }
}
