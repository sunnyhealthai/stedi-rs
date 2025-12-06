use serde::{Deserialize, Serialize};

/// Dependent entity information returned in healthcare eligibility check responses.
///
/// This struct represents a dependent (family member) covered under a subscriber's health
/// insurance plan as returned by payers in eligibility responses. Dependents are individuals
/// other than the subscriber who are eligible for coverage, such as spouses, children, or
/// other family members.
///
/// **Important Note**: A dependent submitted in the request may be returned in the subscriber
/// object instead of this dependent object. This can happen when the dependent has their own
/// unique member ID or when payers structure their responses differently.
///
/// ## Required Information
///
/// When present, this object will always include:
/// - **Dependent's name**: `firstName` and `lastName` for identification
///
/// Many payers will also return:
/// - **Date of birth**: `dateOfBirth` for additional identification
/// - **Member ID**: `memberId` for the dependent's insurance policy
/// - **Relationship**: `relationToSubscriber` or `relationToSubscriberCode` indicating
///   relationship to the subscriber
/// - **Other identifying information**: Address, SSN, or other identifiers
///
/// ## Dependent Identification
///
/// Dependents are identified through multiple fields:
///
/// - **Name**: `firstName`, `lastName`, `middleName`, `suffix`
/// - **Member ID**: `memberId` - the dependent's insurance policy member ID
/// - **Date of Birth**: `dateOfBirth` - for identification and matching
/// - **Social Security Number**: `ssn` - when provided by payer
/// - **Unique Health Identifier**: `uniqueHealthIdentifier` - payer-specific identifier
/// - **Entity Identifier**: `entityIdentifier` - standardized entity identification
/// - **Address**: `address` - physical address information
///
/// ## Relationship to Subscriber
///
/// The relationship to the subscriber is indicated by:
/// - **Relation to Subscriber** (`relationToSubscriber`): Detailed relationship information
/// - **Relation to Subscriber Code** (`relationToSubscriberCode`): Standardized relationship
///   code (e.g., Spouse, Child, Other)
///
/// ## Plan and Coverage Information
///
/// Plan and coverage details include:
/// - **Group Number**: `groupNumber` - insurance plan group number
/// - **Group Description**: `groupDescription` - group name
/// - **Plan Number**: `planNumber` - plan number
/// - **Plan Description**: `planDescription` - plan name
/// - **Plan Network**: `planNetworkIdNumber`, `planNetworkDescription` - network information
/// - **Insured Indicator**: `insuredIndicator` - whether the dependent is the insured party
///
/// ## Military Service Information
///
/// For military-related dependents, payers may return:
/// - **Military Service Rank**: `militaryServiceRankCode` - rank code
/// - **Government Service Affiliation**: `governmentServiceAffiliationCode` - service branch
/// - **Employment Status**: `employmentStatusCode` - employment status
/// - **Service Dates**: `dateTimePeriod`, `startDateTimePeriod`, `endDateTimePeriod` - service
///   period dates
/// - **Date Format**: `dateTimePeriodFormatQualifier` - format of date fields
/// - **Military Unit**: `description` - military unit identification
/// - **Information Status**: `informationStatusCode` - status of military information
///
/// ## Diagnosis Codes
///
/// The `healthCareDiagnosisCodes` field contains diagnosis codes associated with the dependent.
/// These codes help understand conditions or diagnoses relevant to benefit determination.
///
/// ## Maintenance Information
///
/// Maintenance codes indicate changes or updates to dependent information:
/// - **Maintenance Type Code**: `maintenanceTypeCode` - type of maintenance operation
/// - **Maintenance Reason Code**: `maintenanceReasonCode` - reason for maintenance
///
/// ## Error Handling
///
/// The `aaaErrors` field contains dependent-specific errors that occurred during the eligibility
/// check. These errors are separate from general transaction errors and provide dependent-level
/// error details.
///
/// ## Provider Information
///
/// The `responseProvider` field may contain provider information associated with the dependent,
/// such as primary care provider or other provider relationships.
///
/// ## Usage Context
///
/// Dependent information in responses is used to:
///
/// - **Verify dependent eligibility**: Confirm that the dependent is covered under the
///   subscriber's plan
/// - **Understand benefits**: Determine what benefits are available to the dependent
/// - **Verify coverage dates**: Confirm active coverage periods
/// - **Network matching**: Determine in-network vs out-of-network benefits
/// - **Relationship verification**: Understand the dependent's relationship to the subscriber
/// - **Error handling**: Identify and handle dependent-specific errors
///
/// ## X12 HIPAA
///
/// Maps to dependent entity loops in X12 271 transactions, including:
/// - Dependent identification segments (NM1, REF)
/// - Dependent relationship segments
/// - Dependent demographic information
/// - Diagnosis code segments (HI) when included
/// - Error segments (AAA) for dependent-specific errors
/// - Military service information segments when applicable
///
/// ## Examples
///
/// A typical dependent response might include:
/// - `firstName`: "Jane"
/// - `lastName`: "Doe"
/// - `dateOfBirth`: "2010-05-15"
/// - `memberId`: "MEMBER123456"
/// - `relationToSubscriberCode`: `Child`
/// - `groupNumber`: "GROUP001"
/// - `planNumber`: "PLAN123"
///
/// ## Stedi Notes
///
/// Note that a dependent submitted in the request may be returned in the subscriber object
/// instead of this dependent object. Always check both subscriber and dependent objects when
/// processing eligibility responses. When present, this object will always include the dependent's
/// name for identification, but many payers will also return the date of birth and other
/// identifying information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseDependent {
    /// Dependent-specific errors that occurred during the eligibility check.
    ///
    /// Contains error information specific to the dependent entity, separate from general
    /// transaction errors. These errors may indicate dependent-specific issues such as
    /// invalid member IDs, coverage terminations, relationship problems, or other dependent-level
    /// errors.
    #[serde(rename = "aaaErrors", skip_serializing_if = "Option::is_none")]
    pub aaa_errors: Option<Vec<super::EligibilityCheckDependentError>>,
    /// The dependent's address information.
    ///
    /// Contains physical address details for the dependent, including street address, city,
    /// state, and postal code. Address information helps with identification and may be used
    /// for administrative purposes or network matching.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::Address>,
    /// The number assigned to each family member born with the same birth date, such as twins or triplets. Indicates the birth order when there are multiple births associated with the provided birth date.
    #[serde(
        rename = "birthSequenceNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub birth_sequence_number: Option<String>,
    /// The member's date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The military service date.
    #[serde(rename = "dateTimePeriod", skip_serializing_if = "Option::is_none")]
    pub date_time_period: Option<String>,
    /// The format of the military service date and time period. Can be `D8` - Date or `RD8` - Range of Dates.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "dateTimePeriodFormatQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_time_period_format_qualifier: Option<super::DateTimePeriodFormatQualifier>,
    /// Context that identifies the exact military unit. Used to report military service data.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The member's employment status code, used to report military service data. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#employment-status-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "employmentStatusCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub employment_status_code: Option<super::EmploymentStatusCode>,
    /// The military service end date.
    #[serde(rename = "endDateTimePeriod", skip_serializing_if = "Option::is_none")]
    pub end_date_time_period: Option<String>,
    /// Standardized entity identifier for the dependent.
    ///
    /// Contains standardized identification codes for the dependent entity. This field helps
    /// uniquely identify the dependent using payer-specific or standardized identifier codes.
    /// Used for entity identification and matching purposes.
    #[serde(rename = "entityIdentifier", skip_serializing_if = "Option::is_none")]
    pub entity_identifier: Option<super::ResponseDependentEntityIdentifier>,
    /// The entity type for the dependent.
    ///
    /// Specifies whether the dependent is a `Person` (individual) or `NonPersonEntity`
    /// (organization). In practice, dependents are always `Person` entities, as they represent
    /// individual family members. This field is included for consistency with entity type
    /// qualifiers used throughout eligibility transactions.
    ///
    /// Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<super::EntityTypeQualifier>,
    /// The member's first name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Code indicating the patient's gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<super::GenderWithUnknown>,
    /// The member's government service affiliation code, used to report military service data. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#government-service-affiliation-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "governmentServiceAffiliationCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub government_service_affiliation_code: Option<super::GovernmentServiceAffiliationCode>,
    /// Group name
    #[serde(rename = "groupDescription", skip_serializing_if = "Option::is_none")]
    pub group_description: Option<String>,
    /// The group number associated with the insurance policy.
    #[serde(rename = "groupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    /// Diagnosis codes associated with the dependent.
    ///
    /// Contains healthcare diagnosis codes (typically ICD-10 codes) associated with the
    /// dependent. These codes help understand conditions or diagnoses relevant to benefit
    /// determination, coverage limitations, or service authorization requirements. Diagnosis
    /// codes may be included when they are relevant to the eligibility check or benefit
    /// determination.
    #[serde(
        rename = "healthCareDiagnosisCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub health_care_diagnosis_codes: Option<Vec<super::HealthCareDiagnosisCode>>,
    /// The status of the member's information, used to report military service data. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#information-status-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "informationStatusCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub information_status_code: Option<super::InformationStatusCode>,
    /// Indicator of whether the dependent is the insured party.
    ///
    /// Specifies whether the dependent is the insured party in the healthcare plan. This
    /// indicator helps understand the dependent's role in the insurance relationship and
    /// may affect benefit determination or coordination of benefits scenarios.
    #[serde(rename = "insuredIndicator", skip_serializing_if = "Option::is_none")]
    pub insured_indicator: Option<super::DependentInsuredIndicator>,
    /// The member's last name.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Code indicating the reason for maintenance operations on dependent information.
    ///
    /// Specifies the reason why maintenance was performed on the dependent's eligibility
    /// information. Maintenance reason codes indicate why dependent data was updated, changed,
    /// or modified in the payer's system. Used for tracking changes to dependent eligibility
    /// records.
    #[serde(
        rename = "maintenanceReasonCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_reason_code: Option<super::MaintenanceReasonCode>,
    /// Code indicating the type of maintenance operation performed on dependent information.
    ///
    /// Specifies the type of maintenance operation that was performed on the dependent's
    /// eligibility information, such as additions, changes, deletions, or corrections.
    /// Maintenance type codes help track what type of update was made to dependent records.
    #[serde(
        rename = "maintenanceTypeCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_type_code: Option<super::MaintenanceTypeCode>,
    /// The member ID for the insurance policy.
    #[serde(rename = "memberId", skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The member's middle name or initial.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The member's military service rank code. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#military-service-rank-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "militaryServiceRankCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub military_service_rank_code: Option<super::MilitaryServiceRankCode>,
    /// Plan name
    #[serde(rename = "planDescription", skip_serializing_if = "Option::is_none")]
    pub plan_description: Option<String>,
    /// Plan network name
    #[serde(
        rename = "planNetworkDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_network_description: Option<String>,
    /// The network identification number associated with the insurance policy.
    #[serde(
        rename = "planNetworkIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_network_id_number: Option<String>,
    /// The plan number associated with the insurance policy.
    #[serde(rename = "planNumber", skip_serializing_if = "Option::is_none")]
    pub plan_number: Option<String>,
    /// Detailed relationship information between the dependent and subscriber.
    ///
    /// Contains comprehensive relationship information including relationship codes and
    /// descriptions. This field provides detailed information about how the dependent
    /// relates to the subscriber, such as spouse, child, or other family relationships.
    /// Used for relationship verification and benefit determination.
    #[serde(
        rename = "relationToSubscriber",
        skip_serializing_if = "Option::is_none"
    )]
    pub relation_to_subscriber: Option<super::DependentRelationship>,
    /// Standardized code indicating the dependent's relationship to the subscriber.
    ///
    /// Specifies the relationship between the dependent and subscriber using standardized
    /// relationship codes. Common relationships include `Spouse`, `Child`, `Self`, or other
    /// family relationships. This code helps payers properly identify the dependent and
    /// determine eligibility based on the relationship.
    #[serde(
        rename = "relationToSubscriberCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub relation_to_subscriber_code: Option<super::DependentRelationshipCode>,
    /// Provider information associated with the dependent.
    ///
    /// Contains provider details associated with the dependent, such as primary care provider,
    /// specialist, or other provider relationships. This field may include provider
    /// identification, contact information, and provider type information relevant to the
    /// dependent's care or benefit administration.
    #[serde(rename = "responseProvider", skip_serializing_if = "Option::is_none")]
    pub response_provider: Option<super::ResponseProvider>,
    /// The member's Social Security Number (SSN).
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The military service start date.
    #[serde(
        rename = "startDateTimePeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_time_period: Option<String>,
    /// The name suffix, such as Jr., Sr., or III.
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// The member's unique health identifier.
    #[serde(
        rename = "uniqueHealthIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_health_identifier: Option<String>,
}

impl ResponseDependent {
    /// Creates a new `ResponseDependent` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty dependent response object. In practice, dependent
    /// information is populated by payers in eligibility responses, so this constructor is
    /// primarily useful for testing or when manually constructing dependent response objects.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{ResponseDependent, DependentRelationshipCode};
    ///
    /// // Create a new dependent response instance
    /// let mut dependent = ResponseDependent::new();
    ///
    /// // Set dependent identification fields (typically populated by payer)
    /// dependent.first_name = Some("Jane".to_string());
    /// dependent.last_name = Some("Doe".to_string());
    /// dependent.date_of_birth = Some("2010-05-15".to_string());
    /// dependent.member_id = Some("MEMBER123456".to_string());
    /// dependent.relation_to_subscriber_code = Some(DependentRelationshipCode::Child);
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Response-only structure**: This struct is typically populated by payers in
    ///   eligibility responses, not constructed by API consumers
    /// - **Name always present**: When present, this object will always include the dependent's
    ///   name (`firstName` and `lastName`) for identification
    /// - **Additional information**: Many payers will also return date of birth and other
    ///   identifying information
    /// - **Dependent may be in subscriber object**: A dependent submitted in the request may
    ///   be returned in the subscriber object instead - always check both locations
    /// - **Relationship information**: The relationship to subscriber is typically provided
    ///   in `relationToSubscriber` or `relationToSubscriberCode`
    /// - **Error handling**: Check `aaaErrors` for dependent-specific errors
    ///
    /// # Returns
    ///
    /// A new `ResponseDependent` instance with all optional fields set to `None`.
    pub fn new() -> ResponseDependent {
        ResponseDependent {
            aaa_errors: None,
            address: None,
            birth_sequence_number: None,
            date_of_birth: None,
            date_time_period: None,
            date_time_period_format_qualifier: None,
            description: None,
            employment_status_code: None,
            end_date_time_period: None,
            entity_identifier: None,
            entity_type: None,
            first_name: None,
            gender: None,
            government_service_affiliation_code: None,
            group_description: None,
            group_number: None,
            health_care_diagnosis_codes: None,
            information_status_code: None,
            insured_indicator: None,
            last_name: None,
            maintenance_reason_code: None,
            maintenance_type_code: None,
            member_id: None,
            middle_name: None,
            military_service_rank_code: None,
            plan_description: None,
            plan_network_description: None,
            plan_network_id_number: None,
            plan_number: None,
            relation_to_subscriber: None,
            relation_to_subscriber_code: None,
            response_provider: None,
            ssn: None,
            start_date_time_period: None,
            suffix: None,
            unique_health_identifier: None,
        }
    }
}
