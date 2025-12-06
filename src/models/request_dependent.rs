use serde::{Deserialize, Serialize};

/// Dependent entity information for healthcare eligibility check requests.
///
/// This struct represents a dependent (family member) covered under a subscriber's
/// health insurance plan. Dependents are individuals other than the subscriber who are
/// eligible for coverage, such as spouses, children, or other family members.
///
/// ## Required Fields
///
/// While most fields are optional, the following are strongly recommended or required:
///
/// - **Member Identification**: Either `idCard` (member ID) or `ssn` (Social Security
///   Number) - at least one identifier is typically required
/// - **Date of Birth**: `dateOfBirth` is **strongly recommended** - many payers require
///   this to identify the patient and will return errors without it
/// - **Name**: `firstName` and `lastName` are typically required for identification
/// - **Relationship**: `individualRelationshipCode` to specify the relationship to the
///   subscriber
///
/// ## Dependent Identification
///
/// Dependents can be identified through multiple methods:
///
/// - **Member ID** (`idCard`): The dependent's insurance card number or member ID
/// - **Social Security Number** (`ssn`): SSN (not for federally-administered programs)
/// - **Group Number** (`groupNumber`): Insurance plan group number
/// - **Issue Number** (`issueNumber`): Policy issue number
/// - **Additional Identification** (`additionalIdentification`): Supplementary ID fields
///
/// ## Relationship to Subscriber
///
/// The `individualRelationshipCode` field specifies the dependent's relationship to the
/// subscriber, such as:
/// - Spouse
/// - Child
/// - Other family member
/// - Self (when dependent is also the subscriber)
///
/// ## Diagnosis Information
///
/// The `healthCareCodeInformation` field allows you to include up to eight diagnosis codes:
/// - First entry must use `diagnosisTypeCode` set to `ABK` (Admission Diagnosis)
/// - Subsequent entries must use `diagnosisTypeCode` set to `ABF` (Principal Diagnosis)
///
/// Diagnosis codes help payers determine benefit eligibility and coverage for specific
/// conditions or services.
///
/// ## Provider Information
///
/// Optional provider information can be included:
/// - **Provider Code** (`providerCode`): The provider's role in relation to the dependent
/// - **Provider Identifier** (`providerIdentifier`): Provider ID (NPI, tax ID, or taxonomy)
/// - **Reference Identification Qualifier** (`referenceIdentificationQualifier`): Type
///   of provider identifier
///
/// ## Usage Context
///
/// Dependent information is used in eligibility requests to:
///
/// - **Check dependent eligibility**: Verify that the dependent is covered under the
///   subscriber's plan
/// - **Determine benefits**: Understand what benefits are available to the dependent
/// - **Verify coverage dates**: Confirm active coverage periods
/// - **Network matching**: Determine in-network vs out-of-network benefits
/// - **Authorization requirements**: Understand prior authorization needs
///
/// ## Best Practices
///
/// - **Always include date of birth**: Many payers require DOB and will error without it
/// - **Include relationship code**: Helps payers properly identify the dependent
/// - **Use member ID when available**: More reliable than SSN for identification
/// - **Include diagnosis codes when relevant**: Helps with benefit determination
/// - **Don't use SSN for Medicare**: Avoid SSN for federally-administered programs
///
/// ## X12 HIPAA
///
/// Maps to dependent entity loops in X12 270 transactions, including:
/// - Dependent identification segments (NM1, REF)
/// - Dependent relationship segments
/// - Dependent demographic information
/// - Diagnosis code segments (HI) when included
///
/// ## Examples
///
/// Basic dependent with member ID:
/// ```rust
/// use stedi_rs::models::{RequestDependent, IndividualRelationshipCode};
///
/// let mut dependent = RequestDependent::new();
/// dependent.first_name = Some("John".to_string());
/// dependent.last_name = Some("Doe".to_string());
/// dependent.date_of_birth = Some("2010-05-15".to_string());
/// dependent.id_card = Some("MEMBER123456".to_string());
/// dependent.individual_relationship_code = Some(IndividualRelationshipCode::Child);
/// ```
///
/// ## Stedi Notes
///
/// Date of birth is strongly recommended as many payers require it for patient identification.
/// Without DOB, payers may immediately return an error. Include as much identification
/// information as available to ensure successful eligibility checks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestDependent {
    /// Additional identification information for the dependent.
    ///
    /// Provides supplementary identification data beyond the primary member ID. This can
    /// include additional ID numbers, qualifiers, or identification fields that help
    /// uniquely identify the dependent. Use when payers require additional identification
    /// beyond the standard member ID or SSN.
    #[serde(
        rename = "additionalIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_identification: Option<super::AdditionalIdentificationDependent>,
    /// The dependent's address. You must include at least the `address1` and `city` properties in this object.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::RequestAddress>,
    /// Deprecated; The date the insurance card was issued. This shape is deprecated: This property is no longer used.
    #[serde(
        rename = "beginningCardIssueDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub beginning_card_issue_date: Option<String>,
    /// Deprecated; The date the insurance plan begins. This shape is deprecated: This property is no longer used.
    #[serde(
        rename = "beginningPlanIssueDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub beginning_plan_issue_date: Option<String>,
    /// The number assigned to each family member born with the same birth date, such as twins or triplets. Use to indicate the birth order when there are multiple births associated with the provided birth date.
    #[serde(
        rename = "birthSequenceNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub birth_sequence_number: Option<String>,
    /// The dependent's date of birth (DOB). We **strongly recommend** including the DOB in your request. Many payers need this information to identify the patient in their system and will immediately return an error when it's not provided.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The eligibility category for the dependent.
    #[serde(
        rename = "eligibilityCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub eligibility_category: Option<String>,
    /// Deprecated; The date the insurance card expires. This shape is deprecated: This property is no longer used.
    #[serde(rename = "endCardIssueDate", skip_serializing_if = "Option::is_none")]
    pub end_card_issue_date: Option<String>,
    /// Deprecated; The date the insurance plan ends. This shape is deprecated: This property is no longer used.
    #[serde(rename = "endPlanIssueDate", skip_serializing_if = "Option::is_none")]
    pub end_plan_issue_date: Option<String>,
    /// The dependent's first name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Code indicating the dependent's gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<super::Gender>,
    /// The group number for the dependent's insurance plan.
    #[serde(rename = "groupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    /// Information about the dependent's health care diagnosis. You can include up to eight entries in this array.   The first array entry must have `diagnosisTypeCode` set to `ABK`. All subsequent entries must have `diagnosisTypeCode` set to `ABF`.
    #[serde(
        rename = "healthCareCodeInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub health_care_code_information: Option<Vec<super::HealthCareInformation>>,
    /// The dependent's insurance card number.
    #[serde(rename = "idCard", skip_serializing_if = "Option::is_none")]
    pub id_card: Option<String>,
    /// Deprecated; The date the identification card was issued. This shape is deprecated: This property is no longer used.
    #[serde(rename = "idCardIssueDate", skip_serializing_if = "Option::is_none")]
    pub id_card_issue_date: Option<String>,
    /// Code indicating the dependent's relationship to the subscriber.
    ///
    /// Specifies how the dependent is related to the subscriber, such as spouse, child,
    /// or other family member. This relationship code helps payers properly identify
    /// the dependent and determine eligibility based on the relationship. Common
    /// relationships include `Spouse`, `Child`, `Self`, or other family relationships.
    ///
    /// This field is typically required or strongly recommended to ensure proper
    /// dependent identification and benefit determination.
    #[serde(
        rename = "individualRelationshipCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub individual_relationship_code: Option<super::IndividualRelationshipCode>,
    /// The issue number for the dependent's insurance policy.
    #[serde(rename = "issueNumber", skip_serializing_if = "Option::is_none")]
    pub issue_number: Option<String>,
    /// The dependent's last name. **Don't** include the dependent's name suffix, such as Jr. or III. Use the designated `suffix` property instead.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// This shape is deprecated: This property is no longer used.
    #[serde(rename = "memberId", skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The dependent's middle name or middle initial.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// This shape is deprecated: This property is no longer used.
    #[serde(rename = "planIssueDate", skip_serializing_if = "Option::is_none")]
    pub plan_issue_date: Option<String>,
    /// Code indicating the provider's role in relation to the dependent's benefits.
    ///
    /// Specifies the provider's role or relationship to the dependent for benefit
    /// determination purposes. Common provider codes include primary care provider,
    /// specialist, facility, or other provider types. Include when the provider's role
    /// is relevant to the eligibility check or when required by the payer.
    #[serde(rename = "providerCode", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<super::RequestDependentProviderCode>,
    /// The provider identifier matching the type specified in `referenceIdentificationQualifier`.
    ///
    /// Contains the provider's identification number based on the qualifier type:
    /// - **NPI**: When qualifier is `NationalProviderIdentifier`, provide the provider's NPI
    /// - **Tax ID**: When qualifier is `FederalTaxpayerIdentificationNumber`, provide the
    ///   provider's tax ID (EIN or SSN)
    /// - **Taxonomy Code**: When qualifier is `ProviderTaxonomyCode` (`PXC`), provide the
    ///   provider's taxonomy code
    ///
    /// This field works together with `referenceIdentificationQualifier` to properly identify
    /// the provider. Only include when provider information is relevant to the eligibility check.
    #[serde(rename = "providerIdentifier", skip_serializing_if = "Option::is_none")]
    pub provider_identifier: Option<String>,
    /// Qualifier indicating the type of provider identifier used in `providerIdentifier`.
    ///
    /// Specifies what type of identifier is provided in the `providerIdentifier` field:
    /// - **National Provider Identifier**: Provider's NPI
    /// - **Federal Taxpayer Identification Number**: Provider's tax ID (EIN or SSN)
    /// - **Provider Taxonomy Code** (`PXC`): Provider's specialty taxonomy code
    /// - **Other qualifiers**: Additional provider identification types as defined
    ///
    /// This qualifier tells the payer how to interpret the `providerIdentifier` value.
    /// Must be provided if `providerIdentifier` is included.
    #[serde(
        rename = "referenceIdentificationQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_identification_qualifier:
        Option<super::RequestDependentReferenceIdentificationQualifier>,
    /// The dependent's social security number. Don't use this for Federally-administered programs, such as Medicare.
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The dependent's name suffix, such as Sr. or III. Only include the dependent's personal name suffix - **don't** include professional or academic titles, such as M.D. or MBA.
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl RequestDependent {
    /// Creates a new `RequestDependent` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty dependent object. After creating the dependent,
    /// you must populate the required identification fields (member ID or SSN), date of birth
    /// (strongly recommended), name fields, and relationship code.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{RequestDependent, IndividualRelationshipCode};
    ///
    /// // Create a dependent with required fields
    /// let mut dependent = RequestDependent::new();
    /// dependent.first_name = Some("Jane".to_string());
    /// dependent.last_name = Some("Doe".to_string());
    /// dependent.date_of_birth = Some("2010-05-15".to_string());
    /// dependent.id_card = Some("MEMBER123456".to_string());
    /// dependent.individual_relationship_code = Some(IndividualRelationshipCode::Child);
    /// dependent.group_number = Some("GROUP001".to_string());
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Date of birth**: Strongly recommended - many payers require DOB and will return
    ///   errors without it
    /// - **Member identification**: Provide either `idCard` (member ID) or `ssn` - at least
    ///   one identifier is typically required
    /// - **Name fields**: `firstName` and `lastName` are typically required for identification
    /// - **Relationship code**: Include `individualRelationshipCode` to specify relationship
    ///   to subscriber
    /// - **Diagnosis codes**: Include `healthCareCodeInformation` when relevant to benefit
    ///   determination (first entry uses `ABK`, subsequent entries use `ABF`)
    /// - **SSN usage**: Don't use SSN for federally-administered programs like Medicare
    ///
    /// # Returns
    ///
    /// A new `RequestDependent` instance with all optional fields set to `None`.
    pub fn new() -> RequestDependent {
        RequestDependent {
            additional_identification: None,
            address: None,
            beginning_card_issue_date: None,
            beginning_plan_issue_date: None,
            birth_sequence_number: None,
            date_of_birth: None,
            eligibility_category: None,
            end_card_issue_date: None,
            end_plan_issue_date: None,
            first_name: None,
            gender: None,
            group_number: None,
            health_care_code_information: None,
            id_card: None,
            id_card_issue_date: None,
            individual_relationship_code: None,
            issue_number: None,
            last_name: None,
            member_id: None,
            middle_name: None,
            plan_issue_date: None,
            provider_code: None,
            provider_identifier: None,
            reference_identification_qualifier: None,
            ssn: None,
            suffix: None,
        }
    }
}
