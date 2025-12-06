use serde::{Deserialize, Serialize};

/// Response structure containing the results of a healthcare eligibility check.
///
/// This represents the payer's response to an eligibility verification request (270/271 transaction).
/// The response contains detailed information about a patient's insurance coverage, benefits,
/// and any errors encountered during processing.
///
/// # Response Structure
///
/// A successful eligibility check response typically includes:
/// - Patient coverage status and active dates
/// - Detailed benefits information including deductibles, copays, and out-of-pocket maximums
/// - Service type specific coverage details
/// - Provider and subscriber information as confirmed by the payer
///
/// # Error Handling
///
/// If the payer cannot process the request, the response will contain:
/// - Error codes and descriptions in the `errors` field
/// - Follow-up action recommendations
/// - Status information indicating the nature of the failure
///
/// # Examples
///
/// ```rust
/// use stedi_rs::models::EligibilityCheckResponseContent;
///
/// // Process a successful response
/// fn handle_eligibility_response(response: EligibilityCheckResponseContent) {
///     if let Some(benefits) = response.benefits_information {
///         for benefit in benefits {
///             println!("Benefit type: {:?}", benefit.code);
///             if let Some(amount) = benefit.benefit_amount {
///                 println!("Benefit amount: ${}", amount);
///             }
///         }
///     }
///     
///     if let Some(errors) = response.errors {
///         for error in errors {
///             eprintln!("Eligibility error: {}", error.description.unwrap_or_default());
///         }
///     }
/// }
/// ```
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibilityCheckResponseContent {
    /// Information about the patient's healthcare benefits, such as coverage level (individual vs. family), coverage type (deductibles, co-pays, etc.), out of pocket maximums, and more.     Payers typically return at least the following properties: `code`, `coverageLevelCode`, `serviceTypeCodes`, and either `benefitAmount` or `benefitPercent`. However, the exact properties returned in this object are up to the payer's discretion.  The payer may send benefits information for service type codes (STCs) you didn't request - this is expected. The STC you send in the request tells the payer the types of benefits information you want, but they aren't required to respond with exactly the same STC(s) in the response. Receiving different STCs than you requested can also mean that the payer is ignoring the STC you sent, which is why we recommend [testing payers](https://www.stedi.com/docs/healthcare/eligibility-stc-procedure-codes#test-payer-stc-support) to determine their support for specific STCs.  Visit [Determine patient benefits](https://www.stedi.com/docs/healthcare/eligibility-active-coverage-benefits) for more information about benefit types, details about how to interpret the `benefitsInformation` array, and additional examples.
    #[serde(
        rename = "benefitsInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub benefits_information: Option<Vec<super::BenefitsInformation>>,
    /// An identifier for the payer's response.
    #[serde(rename = "controlNumber", skip_serializing_if = "Option::is_none")]
    pub control_number: Option<String>,
    /// Information about the patient when they are a dependent. When the patient is a dependent, this array will contain a single object with the patient's information. When the patient is a subscriber, or considered to be a subscriber because they have a unique member ID, their information is returned in the `subscriber` object, and this array will be empty.   When present, this object will always include the dependent's name for identification, but many payers will also return the date of birth and other identifying information.
    #[serde(rename = "dependents", skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<super::ResponseDependent>>,
    /// An identifier that allows Stedi to group eligibility checks for the same patient into a unified record within [Eligibility Manager](https://www.stedi.com/docs/healthcare/eligibility-manager) called an eligibility search.  This property is for use by Stedi tools only, such as Stedi's MCP server.
    #[serde(
        rename = "eligibilitySearchId",
        skip_serializing_if = "Option::is_none"
    )]
    pub eligibility_search_id: Option<String>,
    /// When a payer rejects your eligibility check, the response contains one or more [`AAA` errors](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors) that specify the reasons for the rejection and any recommended follow-up actions.  Any errors that occur at the `payer`, `provider`, `subscriber`, or `dependents` levels are also included in this array, allowing you to review all errors in a central location. If there are no `AAA` errors, this array will be empty.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<super::EligibilityCheckError>>,
    /// The implementation transaction set error code provided in `IK502` of the 999 transaction.
    #[serde(
        rename = "implementationTransactionSetSyntaxError",
        skip_serializing_if = "Option::is_none"
    )]
    pub implementation_transaction_set_syntax_error: Option<String>,
    /// Metadata about the eligibility check response used by Stedi for tracking and troubleshooting.
    ///
    /// This object contains internal tracking information that Stedi uses to monitor and debug
    /// eligibility transactions. It includes identifiers such as:
    /// - `outboundTraceId`: The unique ID Stedi assigns to the request
    /// - `traceId`: The transaction identifier the payer sends in the response (should match `outboundTraceId`)
    /// - `billerId`, `senderId`, `submitterId`: Various IDs Stedi assigns for transaction routing
    /// - `applicationMode`: Whether the request was in `production` or `test` mode
    ///
    /// This metadata is primarily used by Stedi's internal systems and tools (such as Stedi's
    /// MCP server) for transaction correlation, debugging, and support purposes. You typically
    /// don't need to use this information in your application logic, but it may be useful for
    /// support requests or advanced troubleshooting scenarios.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<super::EligibilityMetaDataJson>,
    /// Information about the payer providing the benefits information.
    ///
    /// This object contains identifying information about the insurance payer (insurance company)
    /// that processed the eligibility check and is providing the benefits information. The response
    /// will always include the payer's business name and an identifier, such as the payer's tax ID.
    /// Most payers also include contact information.
    ///
    /// ## Common Payer Information
    ///
    /// The payer object typically includes:
    /// - `name`: The payer's business name (e.g., "Cigna", "Aetna", "Blue Cross Blue Shield")
    /// - `federalTaxpayersIdNumber`: The payer's federal taxpayer identification number (EIN)
    /// - `contactInformation`: Contact details including phone numbers, addresses, and provider portal URLs
    /// - `npi`: The payer's National Provider Identifier (if applicable)
    /// - `naic`: The payer's National Association of Insurance Commissioners identification number
    /// - `etin`: The payer's Electronic Transmitter Identification Number
    ///
    /// ## Payer-Level Errors
    ///
    /// If there are errors at the payer level (such as connectivity issues or payer system problems),
    /// they will be included in `payer.aaaErrors`. These errors are also aggregated in the top-level
    /// `errors` array for centralized error review.
    ///
    /// ## Usage
    ///
    /// Use this information to:
    /// - Identify which payer processed the eligibility check
    /// - Access payer contact information for support or follow-up
    /// - Verify payer identification when troubleshooting issues
    /// - Access provider portal URLs for additional information
    #[serde(rename = "payer", skip_serializing_if = "Option::is_none")]
    pub payer: Option<super::Payer>,
    /// Dates associated with the subscriber's and dependents' (if applicable) insurance plan.
    ///
    /// This object contains important dates that determine eligibility for benefits under the
    /// insurance plan. These dates apply to every benefit within the patient's health plan
    /// unless specifically overridden by dates in `benefitsInformation.benefitsDateInformation`.
    ///
    /// ## Key Dates
    ///
    /// The most commonly returned dates include:
    /// - `planBegin` and `planEnd`: The date range when coverage from the plan is active
    /// - `eligibilityBegin` and `eligibilityEnd`: The dates when the patient is eligible for benefits
    /// - `policyEffective` and `policyExpiration`: When the insurance policy becomes effective and expires
    /// - `plan`: Plan effective dates (can be a single date or date range)
    ///
    /// Most payers return either `plan` or `planBegin` and `planEnd`, but the exact dates
    /// returned depend on the payer's discretion and the patient's insurance plan.
    ///
    /// ## Date Formats
    ///
    /// - Most fields contain a single date in `YYYYMMDD` format
    /// - Some fields (`plan`, `eligibility`, `planBegin`, `admission`, `service`) can contain
    ///   either a single date or a date range
    /// - Each field's documentation specifies its format
    ///
    /// ## Coverage Determination
    ///
    /// If the date of service is after the earliest ending date among `plan`, `eligibility`,
    /// `planEnd`, `eligibilityEnd`, `policyEffective`, or `policyExpiration`, the patient
    /// likely doesn't have active coverage.
    ///
    /// ## Dependent Dates
    ///
    /// If the payer sends back dates that are different for the subscriber and dependents,
    /// Stedi includes only the dates for the dependent in this object and omits the subscriber's
    /// dates. Dependents can have different coverage dates than the subscriber due to qualifying
    /// life events, such as starting a new job or passing the age limit for coverage through
    /// their parent's plan.
    #[serde(
        rename = "planDateInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_date_information: Option<super::PlanDateInformation>,
    /// Additional identification information for the subscriber's healthcare plan.
    ///
    /// This object contains various identifiers and numbers associated with the insurance plan
    /// that may be needed for claims processing, benefit verification, or plan identification.
    /// The specific fields returned depend on the payer and the type of insurance plan.
    ///
    /// ## Common Plan Identifiers
    ///
    /// The plan information object may include:
    /// - `groupNumber`: The insurance group number
    /// - `planNumber`: The plan number
    /// - `policyNumber`: The group or policy number
    /// - `memberId`: The member ID (may also appear in subscriber information)
    /// - `planDescription`: Description of the plan
    /// - `groupDescription`: Description of the group
    ///
    /// ## Specialized Identifiers
    ///
    /// Depending on the plan type, additional identifiers may be present:
    /// - **Medicare**: `hicNumber` (Health Insurance Claim Number), `medicareProviderNumber`
    /// - **Medicaid**: `medicaidRecipientIdNumber`, `medicaidProviderNumber`
    /// - **Pharmacy**: `drugFormularyNumber`, `coverageListId`, `alternativeListId`
    /// - **Contract-based**: `contractNumber`, `classOfContractCode`
    /// - **Network**: `planNetworkIdNumber`, `planNetworkIdDescription`
    ///
    /// ## Usage
    ///
    /// Use this information to:
    /// - Identify the specific plan and group for claims submission
    /// - Access plan-specific identifiers required for certain transactions
    /// - Understand plan network information
    /// - Retrieve formulary or coverage list information for pharmacy benefits
    #[serde(rename = "planInformation", skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<super::PlanInformation>,
    /// Please use `benefitsInformation` instead.
    #[serde(rename = "planStatus", skip_serializing_if = "Option::is_none")]
    pub plan_status: Option<Vec<super::PlanStatus>>,
    /// Information about the entity that submitted the original eligibility check request.
    ///
    /// This object contains information about the healthcare provider (individual practitioner,
    /// medical group, hospital, or other type of healthcare provider) that requested the eligibility
    /// check. The payer echoes back the provider information as it was received in the request,
    /// confirming the provider's identity and identifiers.
    ///
    /// ## Provider Identification
    ///
    /// The provider object will always include at least one identifier, such as:
    /// - `npi`: The provider's [National Provider Identifier](https://www.stedi.com/docs/healthcare/national-provider-identifier)
    /// - `federalTaxpayersIdNumber`: The provider's Federal Taxpayer Identification Number (EIN)
    /// - `ssn`: The provider's Social Security Number (for individual providers)
    ///
    /// ## Provider Information
    ///
    /// The object may also include:
    /// - **Name**: `providerName` and `providerFirstName` (for individuals) or `providerOrgName` (for organizations)
    /// - **Address**: Provider's address information
    /// - **Provider Code**: A code indicating the provider's role in the benefits information
    /// - **Taxonomy**: `referenceIdentification` containing the Health Care Provider Taxonomy Code
    /// - **Specialized IDs**: `pharmacyProcessorNumber`, `serviceProviderNumber`, `servicesPlanID` (CMS Plan ID)
    ///
    /// ## Provider-Level Errors
    ///
    /// If there are errors at the provider level (such as invalid NPI, provider not enrolled,
    /// or authorization issues), they will be included in `provider.aaaErrors`. These errors are
    /// also aggregated in the top-level `errors` array for centralized error review.
    ///
    /// ## Usage
    ///
    /// Use this information to:
    /// - Verify that the payer received the correct provider information
    /// - Confirm provider identification for claims processing
    /// - Understand provider-specific errors or restrictions
    /// - Access provider taxonomy or specialty information
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<super::ResponseProvider>,
    /// Deprecated; do not use.
    #[serde(rename = "reassociationKey", skip_serializing_if = "Option::is_none")]
    pub reassociation_key: Option<String>,
    /// Errors Stedi encountered when generating or sending the final X12 EDI transaction to the payer. These can include validation errors and payer unavailable errors that prevent delivery.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Information about the primary policyholder for the insurance plan.
    ///
    /// This object contains identifying and demographic information about the subscriber (the
    /// primary policyholder) whose insurance coverage was verified. The response will always
    /// include either the subscriber's name or member ID for identification, but most payers
    /// will also return the subscriber's date of birth and other identifying information.
    ///
    /// ## Subscriber Identification
    ///
    /// The subscriber object typically includes:
    /// - `memberId`: The member ID for the insurance policy
    /// - `firstName` and `lastName`: The subscriber's name
    /// - `dateOfBirth`: The member's date of birth
    /// - `gender`: The subscriber's gender code
    /// - `groupNumber`: The group number associated with the insurance policy
    /// - `groupDescription`: The group name
    ///
    /// ## Additional Subscriber Information
    ///
    /// The object may also include:
    /// - **Address**: Subscriber's address information
    /// - **Plan Information**: `planNumber`, `planDescription`, `planNetworkIdNumber`, `planNetworkDescription`
    /// - **Entity Information**: `entityIdentifier`, `entityType`, `insuredIndicator`
    /// - **Relationship**: `relationToSubscriber` and `relationToSubscriberCode` (typically "Self" for subscribers)
    /// - **Military/Government**: Military service information, government service affiliation codes
    /// - **Maintenance Codes**: `maintenanceTypeCode`, `maintenanceReasonCode` indicating plan changes
    /// - **Diagnosis Codes**: `healthCareDiagnosisCodes` for condition-specific eligibility
    ///
    /// ## Subscriber-Level Errors
    ///
    /// If there are errors at the subscriber level (such as subscriber not found, invalid member ID,
    /// or name mismatches), they will be included in `subscriber.aaaErrors`. These errors are also
    /// aggregated in the top-level `errors` array for centralized error review.
    ///
    /// ## Usage
    ///
    /// Use this information to:
    /// - Verify subscriber identification and demographics
    /// - Confirm plan and group information for claims processing
    /// - Understand coverage relationships and plan details
    /// - Access subscriber-specific benefit information
    /// - Identify plan changes or maintenance events
    ///
    /// Note: When the patient is a dependent (not the subscriber), their information is returned
    /// in the `dependents` array instead, and this object contains the subscriber's information.
    #[serde(rename = "subscriber", skip_serializing_if = "Option::is_none")]
    pub subscriber: Option<super::ResponseSubscriber>,
    /// A unique identifier the payer may assign to the transaction. Note that Stedi doesn't support setting a subscriber trace number in the eligibility check request because there is no need to include a trace number for real-time queries.
    #[serde(
        rename = "subscriberTraceNumbers",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscriber_trace_numbers: Option<Vec<super::SubscriberTraceNumber>>,
    /// An ID for the payer you identified in the original eligibility check request. This value may differ from the `tradingPartnerServiceId` you submitted in the original request because it reflects the payer's internal concept of their ID, not necessarily the ID Stedi uses to route requests to this payer.
    #[serde(
        rename = "tradingPartnerServiceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_partner_service_id: Option<String>,
    /// The transaction set acknowledgment code provided in in the [X12 EDI 999 response](https://portal.stedi.com/app/guides/view/hipaa/implementation-acknowledgment-x231/01HRF41ES1DVGCA6X1EHSRPFXZ#properties.heading.properties.transaction_set_response_header_AK2_loop.items.properties.transaction_set_response_trailer_IK5).
    #[serde(
        rename = "transactionSetAcknowledgement",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_set_acknowledgement: Option<String>,
    /// Warnings indicate non-fatal issues with your eligibility check or a non-standard response from the payer.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<super::Warning>>,
    /// Typically this property contains the raw X12 EDI [271 Eligibility Benefit Response](https://portal.stedi.com/app/guides/view/hipaa/health-care-eligibility-benefit-response-x279a1/01GS66YHZPB37ABF34DBPSR213) from the payer.  In some circumstances, this property may contain a [999 Implementation Acknowledgment](https://portal.stedi.com/app/guides/view/hipaa/implementation-acknowledgment-x231a1/01HMRQV0N8SPHG58M4ZG1CRHH0) instead of a 271. A 999 indicates validation errors in the X12 EDI transaction, such as improper formatting or missing or invalid values.  If the 999 is returned in this property, many of the other response properties will be empty, as they are mapped to information in the 271.
    #[serde(rename = "x12", skip_serializing_if = "Option::is_none")]
    pub x12: Option<String>,
}

impl EligibilityCheckResponseContent {
    /// Creates a new empty eligibility check response.
    ///
    /// This constructor creates a response with all fields set to `None` or their default values.
    /// Typically used internally by deserialization or when building responses programmatically.
    pub fn new() -> EligibilityCheckResponseContent {
        EligibilityCheckResponseContent {
            benefits_information: None,
            control_number: None,
            dependents: None,
            eligibility_search_id: None,
            errors: None,
            implementation_transaction_set_syntax_error: None,
            meta: None,
            payer: None,
            plan_date_information: None,
            plan_information: None,
            plan_status: None,
            provider: None,
            reassociation_key: None,
            status: None,
            subscriber: None,
            subscriber_trace_numbers: None,
            trading_partner_service_id: None,
            transaction_set_acknowledgement: None,
            warnings: None,
            x12: None,
        }
    }
}
