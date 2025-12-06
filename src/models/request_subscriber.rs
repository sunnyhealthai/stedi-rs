use serde::{Deserialize, Serialize};

/// Subscriber (primary policyholder) information for healthcare eligibility check requests.
///
/// This struct represents the primary policyholder for an insurance plan, or a dependent
/// with a unique member ID. The subscriber is the person who holds the insurance policy
/// and through whom dependents receive coverage. If a dependent has their own unique member
/// ID, their information should be included here instead of in the `dependents` array.
///
/// ## Required Fields
///
/// At a minimum, the API requires at least one of these fields:
/// - **`memberId`**: The subscriber's member ID (recommended)
/// - **`dateOfBirth`**: The subscriber's date of birth
/// - **`lastName`**: The subscriber's last name
///
/// However, each payer has different requirements, so you should supply the fields
/// necessary for each payer to identify the subscriber in their system.
///
/// ## Optimal Identification
///
/// When you provide all four of `memberId`, `dateOfBirth`, `firstName`, and `lastName`,
/// payers are required to return a response if the member is in their database. Some
/// payers may be able to search with less information, but this varies by payer.
///
/// **We strongly recommend always including the patient's member ID when possible.**
///
/// ## Subscriber Identification
///
/// Subscribers can be identified through multiple methods:
///
/// - **Member ID** (`memberId`): The primary member identification number (recommended)
/// - **Date of Birth** (`dateOfBirth`): Subscriber's date of birth
/// - **Name Fields**: `firstName`, `lastName`, `middleName`, `suffix`
/// - **Social Security Number** (`ssn`): SSN (many payers ignore this due to privacy concerns,
///   but some Medicaid programs support it)
/// - **ID Card** (`idCard`): Identification card number (common in Medicaid, different from
///   member ID)
/// - **Medicaid Recipient ID** (`medicaidRecipientIdentificationNumber`): For Medicare/Medicaid
///   when different from member ID
/// - **Group Number** (`groupNumber`): Insurance plan group number
/// - **Case Number** (`caseNumber`): Case number associated with the subscriber
/// - **Additional Identification** (`additionalIdentification`): Supplementary ID fields
///
/// ## Patient Names
///
/// Enter the patient's name exactly as written on their insurance card, if available,
/// including any special or punctuation characters such as apostrophes, hyphens (dashes),
/// or spaces. Visit [patient names](https://www.stedi.com/docs/healthcare/send-eligibility-checks#patient-names)
/// for all best practices to avoid unnecessary failures.
///
/// Important naming guidelines:
/// - **Last name**: Don't include name suffix (Jr., III, etc.) - use the `suffix` field instead
/// - **Suffix**: Only include personal name suffixes, not professional titles (M.D., MBA, etc.)
/// - **Exact match**: Match the name exactly as it appears on the insurance card
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
/// - **Provider Code** (`providerCode`): The provider's role in relation to the subscriber
/// - **Provider Identifier** (`providerIdentifier`): Provider ID (NPI, tax ID, or taxonomy)
/// - **Reference Identification Qualifier** (`referenceIdentificationQualifier`): Type of
///   provider identifier (required if `providerIdentifier` is provided)
///
/// ## Medicaid Spend Down
///
/// For some Medicaid programs, individuals must pay a certain amount towards their healthcare
/// cost (spend down) before coverage starts:
/// - **Spend Down Amount** (`spendDownAmount`): Dollar amount the subscriber will apply
///   toward their spend down
/// - **Spend Down Total Billed Amount** (`spendDownTotalBilledAmount`): Total billed amount
///   for spend down calculation
///
/// ## Usage Context
///
/// Subscriber information is used in eligibility requests to:
///
/// - **Identify the policyholder**: Verify the subscriber's identity and policy information
/// - **Check eligibility**: Verify that the subscriber has active coverage
/// - **Determine benefits**: Understand what benefits are available to the subscriber
/// - **Verify coverage dates**: Confirm active coverage periods
/// - **Network matching**: Determine in-network vs out-of-network benefits
/// - **Authorization requirements**: Understand prior authorization needs
///
/// ## Best Practices
///
/// - **Always include member ID when available**: Most reliable identifier
/// - **Include date of birth**: Helps with identification and matching
/// - **Match names exactly**: Use names exactly as they appear on insurance cards
/// - **Include all four key fields**: `memberId`, `dateOfBirth`, `firstName`, `lastName` for
///   best results
/// - **Use ID card for Medicaid**: Include `idCard` when different from member ID (common
///   in Medicaid)
/// - **Be cautious with SSN**: Many payers ignore SSN due to privacy concerns
///
/// ## X12 HIPAA
///
/// Maps to subscriber entity loops in X12 270 transactions, including:
/// - Subscriber identification segments (NM1, REF)
/// - Subscriber demographic information
/// - Diagnosis code segments (HI) when included
/// - Subscriber relationship segments
///
/// ## Examples
///
/// Basic subscriber with member ID:
/// ```rust
/// use stedi_rs::models::RequestSubscriber;
///
/// let mut subscriber = RequestSubscriber::new();
/// subscriber.member_id = Some("MEMBER123456".to_string());
/// subscriber.first_name = Some("John".to_string());
/// subscriber.last_name = Some("Doe".to_string());
/// subscriber.date_of_birth = Some("1980-01-15".to_string());
/// subscriber.group_number = Some("GROUP001".to_string());
/// ```
///
/// Medicaid subscriber with ID card:
/// ```rust
/// use stedi_rs::models::RequestSubscriber;
///
/// let mut medicaid_subscriber = RequestSubscriber::new();
/// medicaid_subscriber.member_id = Some("MEMBER789".to_string());
/// medicaid_subscriber.id_card = Some("IDCARD456".to_string()); // Different from member ID
/// medicaid_subscriber.first_name = Some("Jane".to_string());
/// medicaid_subscriber.last_name = Some("Smith".to_string());
/// medicaid_subscriber.date_of_birth = Some("1975-03-20".to_string());
/// ```
///
/// ## Stedi Notes
///
/// If a dependent has a unique member ID, include their information here and leave the
/// `dependents` array empty. This is common when dependents have their own member IDs
/// separate from the primary subscriber's ID.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestSubscriber {
    /// Additional identification information for the subscriber.
    ///
    /// Provides supplementary identification data beyond the primary member ID. This can
    /// include additional ID numbers, qualifiers, or identification fields that help
    /// uniquely identify the subscriber. Use when payers require additional identification
    /// beyond the standard member ID or SSN.
    #[serde(
        rename = "additionalIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_identification: Option<super::AdditionalIdentificationSubscriber>,
    /// The subscriber's address. You must include at least the `address1` and `city` properties in this object.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::RequestAddress>,
    /// Deprecated; The date the subscriber's identification card was issued. This shape is deprecated: This property is no longer used.
    #[serde(
        rename = "beginningCardIssueDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub beginning_card_issue_date: Option<String>,
    /// Deprecated; The date the subscriber's insurance plan was issued. This shape is deprecated: This property is no longer used.
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
    /// The case number associated with the subscriber.
    #[serde(rename = "caseNumber", skip_serializing_if = "Option::is_none")]
    pub case_number: Option<String>,
    /// This property is no longer used.
    #[serde(rename = "coverageLevelCode", skip_serializing_if = "Option::is_none")]
    pub coverage_level_code: Option<String>,
    /// The subscriber's date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Deprecated; The date the subscriber's identification card expires. This shape is deprecated: This property is no longer used.
    #[serde(rename = "endCardIssueDate", skip_serializing_if = "Option::is_none")]
    pub end_card_issue_date: Option<String>,
    /// Deprecated; The date the subscriber's insurance plan ended. This shape is deprecated: This property is no longer used.
    #[serde(rename = "endPlanIssueDate", skip_serializing_if = "Option::is_none")]
    pub end_plan_issue_date: Option<String>,
    /// The patient's first name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Code indicating the subscriber's gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<super::Gender>,
    /// The group number associated with the subscriber's insurance policy.
    #[serde(rename = "groupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    /// Information about the subscriber's health care diagnosis. You can include up to eight entries in this array.   The first array entry must have `diagnosisTypeCode` set to `ABK`. All subsequent entries must have `diagnosisTypeCode` set to `ABF`.
    #[serde(
        rename = "healthCareCodeInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub health_care_code_information: Option<Vec<super::HealthCareInformation>>,
    /// The subscriber's identification card number. Include this property when this number is different than the subscriber's member ID. This is common in Medicaid.
    #[serde(rename = "idCard", skip_serializing_if = "Option::is_none")]
    pub id_card: Option<String>,
    /// Deprecated; The date the subscriber's identification card was issued. This shape is deprecated: This property is no longer used.
    #[serde(rename = "idCardIssueDate", skip_serializing_if = "Option::is_none")]
    pub id_card_issue_date: Option<String>,
    /// The subscriber's last name. **Don't** include the subscriber's name suffix, such as Jr. or III. Use the designated `suffix` property instead.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The Medicaid Recipient Identification Number. You can provide this number to identify the subscriber when it is the primary number the payer knows a member by (such as for Medicare or Medicaid). Do not supply this value unless it is different from the `memberId`.
    #[serde(
        rename = "medicaidRecipientIdentificationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub medicaid_recipient_identification_number: Option<String>,
    /// The member ID for the subscriber's insurance policy.
    #[serde(rename = "memberId", skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The patient's middle name or middle initial.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// Deprecated; The date the subscriber's insurance plan was issued. This shape is deprecated: This property is no longer used.
    #[serde(rename = "planIssueDate", skip_serializing_if = "Option::is_none")]
    pub plan_issue_date: Option<String>,
    /// Code indicating the provider's role in relation to the subscriber's benefits.
    ///
    /// Specifies the provider's role or relationship to the subscriber for benefit
    /// determination purposes. Common provider codes include primary care provider,
    /// specialist, facility, or other provider types. Include when the provider's role
    /// is relevant to the eligibility check or when required by the payer.
    #[serde(rename = "providerCode", skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<super::RequestSubscriberProviderCode>,
    /// The provider identifier matching the type specified in `referenceIdentificationQualifier`.
    ///
    /// Contains the provider's identification number based on the qualifier type. This field
    /// is **required** if you set the `referenceIdentificationQualifier`. Examples:
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
    /// Must be provided if `providerIdentifier` is included. The `providerIdentifier` field
    /// is required when this qualifier is set.
    #[serde(
        rename = "referenceIdentificationQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_identification_qualifier:
        Option<super::RequestSubscriberReferenceIdentificationQualifier>,
    /// Identify the dollar amount the subscriber will apply toward their spend down amount, if required. For some Medicaid programs, individuals must pay a certain amount towards their healthcare cost (spend down) before coverage starts.
    #[serde(rename = "spendDownAmount", skip_serializing_if = "Option::is_none")]
    pub spend_down_amount: Option<String>,
    /// The subscriber's spend down total billed amount.
    #[serde(
        rename = "spendDownTotalBilledAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub spend_down_total_billed_amount: Option<String>,
    /// The subscriber's Social Security Number (SSN). Many commercial and government payers ignore this property due to concerns about member privacy. However, some Medicaid programs support alternative searches using the patient's Social Security Number, instead of the member ID.
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The name suffix, such as Jr., Sr., or III. Only include the subscriber's personal name suffix - **don't** include professional or academic titles, such as M.D. or MBA.
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

impl RequestSubscriber {
    /// Creates a new `RequestSubscriber` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty subscriber object. After creating the subscriber,
    /// you must populate at least one of the minimum required fields (`memberId`, `dateOfBirth`,
    /// or `lastName`), though providing all four key fields (`memberId`, `dateOfBirth`,
    /// `firstName`, and `lastName`) is recommended for best results.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::RequestSubscriber;
    ///
    /// // Create a subscriber with recommended fields
    /// let mut subscriber = RequestSubscriber::new();
    /// subscriber.member_id = Some("MEMBER123456".to_string());
    /// subscriber.first_name = Some("John".to_string());
    /// subscriber.last_name = Some("Doe".to_string());
    /// subscriber.date_of_birth = Some("1980-01-15".to_string());
    /// subscriber.group_number = Some("GROUP001".to_string());
    /// ```
    ///
    /// Medicaid subscriber with ID card:
    /// ```rust
    /// use stedi_rs::models::RequestSubscriber;
    /// 
    /// let mut medicaid_subscriber = RequestSubscriber::new();
    /// medicaid_subscriber.member_id = Some("MEMBER789".to_string());
    /// medicaid_subscriber.id_card = Some("IDCARD456".to_string()); // Different from member ID
    /// medicaid_subscriber.first_name = Some("Jane".to_string());
    /// medicaid_subscriber.last_name = Some("Smith".to_string());
    /// medicaid_subscriber.date_of_birth = Some("1975-03-20".to_string());
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Minimum required**: At least one of `memberId`, `dateOfBirth`, or `lastName` must
    ///   be provided
    /// - **Recommended fields**: Include all four of `memberId`, `dateOfBirth`, `firstName`,
    ///   and `lastName` for best results - payers are required to return a response if the
    ///   member is in their database when all four are provided
    /// - **Member ID**: Always include when available - it's the most reliable identifier
    /// - **Name matching**: Enter names exactly as written on insurance cards, including
    ///   special characters (apostrophes, hyphens, spaces)
    /// - **Name suffix**: Don't include suffix in `lastName` - use the `suffix` field instead
    /// - **ID card for Medicaid**: Include `idCard` when different from member ID (common
    ///   in Medicaid)
    /// - **SSN usage**: Many payers ignore SSN due to privacy concerns, but some Medicaid
    ///   programs support it
    /// - **Dependent with unique ID**: If a dependent has a unique member ID, include their
    ///   information here and leave the `dependents` array empty
    ///
    /// # Returns
    ///
    /// A new `RequestSubscriber` instance with all optional fields set to `None`.
    pub fn new() -> RequestSubscriber {
        RequestSubscriber {
            additional_identification: None,
            address: None,
            beginning_card_issue_date: None,
            beginning_plan_issue_date: None,
            birth_sequence_number: None,
            case_number: None,
            coverage_level_code: None,
            date_of_birth: None,
            end_card_issue_date: None,
            end_plan_issue_date: None,
            first_name: None,
            gender: None,
            group_number: None,
            health_care_code_information: None,
            id_card: None,
            id_card_issue_date: None,
            last_name: None,
            medicaid_recipient_identification_number: None,
            member_id: None,
            middle_name: None,
            plan_issue_date: None,
            provider_code: None,
            provider_identifier: None,
            reference_identification_qualifier: None,
            spend_down_amount: None,
            spend_down_total_billed_amount: None,
            ssn: None,
            suffix: None,
        }
    }
}
