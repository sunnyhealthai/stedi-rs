use serde::{Deserialize, Serialize};

/// Request structure for performing healthcare eligibility checks through Stedi.
///
/// This struct represents a 270/271 eligibility check request that verifies whether
/// a patient has coverage for specific medical benefits under their health insurance plan.
///
/// ## Basic Requirements
///
/// Each eligibility check must include at least:
/// - A control number for transaction identification
/// - Trading partner service ID (payer ID)
/// - Provider information with name and identifier (typically NPI)
/// - Subscriber information with at minimum one of: member ID, date of birth, or last name
/// - Encounter information specifying service dates and types
///
/// ## Best Practices
///
/// - Always provide member ID, first name, last name, and date of birth when available
/// - Include service type codes to get specific benefit information  
/// - Use current date for service date when checking eligibility for today
/// - Provide complete demographic information to avoid matching issues
///
/// See the [Stedi Healthcare Documentation](https://www.stedi.com/docs/healthcare/send-eligibility-checks)
/// for detailed guidance on constructing eligibility check requests.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibilityCheckRequestContent {
    /// An integer used to identify the transaction. This is a requirement for the X12 EDI 270 transaction that Stedi will generate and send to the payer. It doesn't need to be globally unique - you can use the same number for every request.
    #[serde(rename = "controlNumber")]
    pub control_number: String,
    /// A dependent for which you want to retrieve benefits information. - You can only submit one dependent per eligibility check. - Only include the patient's information here when they are listed as a dependent on the subscriber's insurance plan AND the payer cannot uniquely identify them through information outside the subscriber's policy. For example, if the dependent has their own member ID number, you should identify them in the `subscriber` object instead. This includes member IDs that differ only by a suffix, such as `01`, because the patient can still be uniquely identified. - Each payer has different requirements, so you should supply the fields necessary for each payer to identify the dependent in their system. However, we **strongly recommend** including the dependent's date of birth in the request when available because many payers return errors without it. - Enter the patient's name exactly as written on their insurance card, if available, including any special or punctuation characters such as apostrophes, hyphens (dashes), or spaces. Visit [patient names](https://www.stedi.com/docs/healthcare/send-eligibility-checks#patient-names) for all best practices to avoid unnecessary failures.
    #[serde(rename = "dependents", skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<super::RequestDependent>>,
    /// An identifier that allows Stedi to group eligibility checks for the same patient into a unified record within [Eligibility Manager](https://www.stedi.com/docs/healthcare/eligibility-manager) called an eligibility search.  This property is for use by Stedi tools only, such as Stedi's MCP server.
    #[serde(
        rename = "eligibilitySearchId",
        skip_serializing_if = "Option::is_none"
    )]
    pub eligibility_search_id: Option<String>,
    /// Details about the eligibility or benefit information you are requesting for the patient.
    ///
    /// The encounter object specifies what types of benefits information to retrieve and when
    /// the services are expected to occur. This information helps payers return relevant
    /// benefits data for specific service types and dates.
    ///
    /// ## Service Type Codes and Procedure Codes
    ///
    /// You must specify either:
    /// - **Service Type Codes** (`serviceTypeCodes`): Two-character codes that group similar
    ///   healthcare services into standard categories (e.g., `30` for Plan coverage, `47` for
    ///   Hospital, `UC` for Urgent Care). If you don't specify either service type codes or
    ///   procedure codes, Stedi defaults to using `30` (Plan coverage and general benefits).
    /// - **Procedure Codes** (`procedureCode` + `productOrServiceIDQualifier`): Specific CPT,
    ///   HCPCS, or other procedure codes for procedure-specific eligibility checks. Many payers
    ///   don't support procedure code-specific eligibility checks.
    ///
    /// Visit [STCs and procedure codes](https://www.stedi.com/docs/healthcare/eligibility-stc-procedure-codes)
    /// for detailed guidance on using service type codes and procedure codes.
    ///
    /// ## Service Dates
    ///
    /// You can specify service dates in two ways:
    /// - **Single date**: Use `dateOfService` for a specific date (e.g., a doctor's visit)
    /// - **Date range**: Use `beginningDateOfService` and `endDateOfService` together for a
    ///   range of dates
    ///
    /// If no dates are provided, the payer defaults to using the current date in their timezone.
    /// When checking eligibility for today, omit the `dateOfService` property to ensure consistent
    /// behavior across payers.
    ///
    /// ### Date Recommendations
    ///
    /// - Submit dates up to 12 months in the past or up to the end of the current month
    /// - Some payers (like CMS) support future dates, especially the next calendar month
    /// - Always use `YYYYMMDD` format for dates
    /// - Dates must be valid and not in the far future (most payers only support dates through
    ///   the current calendar month)
    ///
    /// ## Additional Encounter Information
    ///
    /// The encounter object can also include:
    /// - **Prior authorization numbers**: For services requiring prior authorization
    /// - **Diagnosis code pointers**: References to ICD-10 codes included in subscriber or
    ///   dependent information
    /// - **Procedure modifiers**: Additional information related to procedure performance
    /// - **Multiple procedure codes**: Use `medicalProcedures` array when you need to send
    ///   multiple procedure codes in a single request
    ///
    /// ## Best Practices
    ///
    /// - Include service type codes to get specific benefit information (e.g., dental, vision,
    ///   pharmacy benefits)
    /// - Use current date for service date when checking eligibility for today (or omit the
    ///   date field)
    /// - Include only one service type code per request unless you're sure the payer supports
    ///   multiple STCs
    /// - Test payer support for specific service type codes and procedure codes, as not all
    ///   payers support all codes
    #[serde(rename = "encounter", skip_serializing_if = "Option::is_none")]
    pub encounter: Option<super::Encounter>,
    /// A unique identifier for the patient that Stedi uses to identify and correlate historical eligibility checks for the same individual. We recommend including this value in all requests.
    #[serde(rename = "externalPatientId", skip_serializing_if = "Option::is_none")]
    pub external_patient_id: Option<String>,
    /// Use the corresponding properties in the `provider` object instead.
    #[serde(
        rename = "informationReceiverName",
        skip_serializing_if = "Option::is_none"
    )]
    pub information_receiver_name: Option<super::InformationReceiverName>,
    /// The password that the provider uses to log in to the payer's portal. This is not commonly used.
    #[serde(rename = "portalPassword", skip_serializing_if = "Option::is_none")]
    pub portal_password: Option<String>,
    /// The username that the provider uses to log in to the payer's portal. This is not commonly used.
    #[serde(rename = "portalUsername", skip_serializing_if = "Option::is_none")]
    pub portal_username: Option<String>,
    /// Information about the entity requesting the eligibility check. This may be an individual practitioner, a medical group, a hospital, or another type of healthcare provider.  - You must provide the `organizationName` (if the entity is an organization), or `firstName` and `lastName` (if the provider is an individual).  - You must also provide an identifier - this is typically the provider's [National Provider Identifier](https://www.stedi.com/docs/healthcare/national-provider-identifier) (`npi`). If the provider doesn't have an NPI, you can supply an alternative, such as their `taxId` or `ssn`.  - Don't include additional properties, such as `taxId` or `address`, unless they are specifically required or suggested by the payer.
    #[serde(rename = "provider")]
    pub provider: super::Provider,
    /// This property is only relevant for asynchronous batch eligibility checks.
    #[serde(
        rename = "submitterTransactionIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub submitter_transaction_identifier: Option<String>,
    /// Information about the primary policyholder for the insurance plan, or a dependent with a unique member ID.
    ///
    /// The subscriber object contains identifying information about the person whose insurance
    /// coverage you want to verify. This can be either:
    /// - The primary policyholder (the person who holds the insurance policy)
    /// - A dependent who has their own unique member ID (in this case, include their information
    ///   here and leave `dependents` empty)
    ///
    /// ## Minimum Requirements
    ///
    /// At a minimum, you must supply at least one of these fields:
    /// - `memberId`: The subscriber's insurance member ID
    /// - `dateOfBirth`: The subscriber's date of birth (format: `YYYYMMDD`)
    /// - `lastName`: The subscriber's last name
    ///
    /// However, each payer has different requirements, so you should supply the fields necessary
    /// for each payer to identify the subscriber in their system.
    ///
    /// ## Recommended Fields
    ///
    /// When you provide all four of `memberId`, `dateOfBirth`, `firstName`, and `lastName`,
    /// payers are required to return a response if the member is in their database. Some payers
    /// may be able to search with less information, but this varies by payer.
    ///
    /// We **strongly recommend** always including:
    /// - `memberId`: The subscriber's insurance member ID (when available)
    /// - `firstName`: The subscriber's first name
    /// - `lastName`: The subscriber's last name
    /// - `dateOfBirth`: The subscriber's date of birth (format: `YYYYMMDD`)
    ///
    /// ## Patient Names
    ///
    /// Enter the patient's name exactly as written on their insurance card, if available, including
    /// any special or punctuation characters such as apostrophes, hyphens (dashes), or spaces.
    /// Use the full legal name (e.g., "Robert" not "Bob") and avoid nicknames or abbreviations.
    ///
    /// Visit [patient names](https://www.stedi.com/docs/healthcare/send-eligibility-checks#patient-names)
    /// for all best practices to avoid unnecessary failures.
    ///
    /// ## Additional Information
    ///
    /// The subscriber object can also include:
    /// - `address`: Subscriber's address (required by some payers)
    /// - `gender`: Subscriber's gender code
    /// - `groupNumber`: Insurance group number
    /// - `healthCareCodeInformation`: Diagnosis codes (ICD-10) for condition-specific eligibility
    /// - `additionalIdentification`: Additional identifiers like Medicaid recipient ID, SSN, etc.
    /// - `providerIdentifier`: Referring provider identifier (when applicable)
    #[serde(rename = "subscriber")]
    pub subscriber: super::RequestSubscriber,
    /// The payer's name, such as Cigna or Aetna.
    #[serde(rename = "tradingPartnerName", skip_serializing_if = "Option::is_none")]
    pub trading_partner_name: Option<String>,
    /// This is the payer ID. Visit the [Payer Network](https://www.stedi.com/healthcare/network) for a complete list. You can send requests using the primary payer ID, the Stedi payer ID, or any alias listed in the payer record.
    #[serde(rename = "tradingPartnerServiceId")]
    pub trading_partner_service_id: String,
}

impl EligibilityCheckRequestContent {
    /// Creates a new `EligibilityCheckRequestContent` with the required fields.
    ///
    /// This constructor initializes an eligibility check request with the minimum required fields:
    /// - `control_number`: Transaction identifier (doesn't need to be globally unique)
    /// - `provider`: Provider information (must include name and identifier, typically NPI)
    /// - `subscriber`: Subscriber information (must include at least one of: member ID, date of birth, or last name)
    /// - `trading_partner_service_id`: Payer ID from the [Payer Network](https://www.stedi.com/healthcare/network)
    ///
    /// All optional fields are initialized to `None` and can be set individually after construction.
    /// For example, you may want to add `encounter` information to specify service dates and types,
    /// or `dependents` if checking eligibility for a dependent.
    ///
    /// # Arguments
    ///
    /// * `control_number` - An integer used to identify the transaction (required for X12 EDI 270)
    /// * `provider` - Provider information with name and identifier (typically NPI)
    /// * `subscriber` - Subscriber information with at minimum one of: member ID, date of birth, or last name
    /// * `trading_partner_service_id` - The payer ID (visit [Payer Network](https://www.stedi.com/healthcare/network) for a complete list)
    ///
    /// # Returns
    ///
    /// A new `EligibilityCheckRequestContent` instance with required fields set and all optional
    /// fields initialized to `None`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use stedi_rs::models::{EligibilityCheckRequestContent, Provider, RequestSubscriber};
    ///
    /// let provider = Provider::new(/* ... */);
    /// let subscriber = RequestSubscriber::new(/* ... */);
    /// let request = EligibilityCheckRequestContent::new(
    ///     "12345".to_string(),
    ///     provider,
    ///     subscriber,
    ///     "CIGNA".to_string(),
    /// );
    /// ```
    pub fn new(
        control_number: String,
        provider: super::Provider,
        subscriber: super::RequestSubscriber,
        trading_partner_service_id: String,
    ) -> EligibilityCheckRequestContent {
        EligibilityCheckRequestContent {
            control_number,
            dependents: None,
            eligibility_search_id: None,
            encounter: None,
            external_patient_id: None,
            information_receiver_name: None,
            portal_password: None,
            portal_username: None,
            provider,
            submitter_transaction_identifier: None,
            subscriber,
            trading_partner_name: None,
            trading_partner_service_id,
        }
    }
}
