use serde::{Deserialize, Serialize};

/// Error codes for subscriber-level AAA errors in eligibility check responses.
///
/// These codes represent standardized X12 error codes (AAA errors) that payers return when
/// there are issues with an eligibility check request specifically related to the subscriber
/// (primary policyholder) information. AAA errors indicate problems that prevented the payer
/// from processing the eligibility check successfully.
///
/// This enum is used in `eligibilityCheckSubscriberError.code` and `eligibilityCheckResponse.errors`
/// to specify the reason for rejection and any recommended follow-up actions. Common reasons
/// for rejection at the subscriber level include:
/// - Missing or incorrect subscriber identifying information (member ID, name, date of birth)
/// - Subscriber not found in the payer's system (most common error)
/// - Data inconsistencies (age, gender, date mismatches)
/// - Provider-related issues (network status, enrollment, authorization)
/// - Date validation errors (future dates, invalid date ranges, date logic errors)
/// - Missing required codes (diagnosis, procedure, authorization)
/// - Service/procedure code issues
///
/// ## Error Categories
///
/// The error codes can be grouped into several categories:
///
/// - **Subscriber Identification Errors** (`72`, `73`, `74`, `75`, `76`, `78`): Missing or invalid
///   subscriber member ID, name, gender code, subscriber not found, duplicate member ID, or subscriber
///   not in the specified group/plan. Error `75` (Subscriber/Insured Not Found) is the most common
///   error and typically indicates issues with member ID, name, or date of birth matching.
///
/// - **Date Validation Errors** (`56`, `57`, `58`, `60`, `61`, `62`, `63`): Invalid dates, missing
///   dates, date logic errors (e.g., date of birth after service date, date of death before service
///   date), or dates outside allowable periods. These errors help ensure date consistency and validity.
///
/// - **Provider Errors** (`35`, `43`, `45`, `47`, `48`, `49`, `51`, `52`, `53`): Provider identification,
///   network status (out of network), specialty, state, enrollment, authorization, or plan enrollment
///   issues. These errors indicate problems with the provider's relationship to the payer or plan.
///
/// - **Service/Procedure Errors** (`54`, `55`, `98`, `AF`, `AG`): Invalid or missing procedure codes,
///   diagnosis codes, product/service ID qualifiers, or experimental services. These errors indicate
///   issues with the service or procedure information in the request.
///
/// - **Authorization Errors** (`AA`, `AE`, `IA`, `MA`, `CI`): Authorization number issues, missing
///   authorizations, access restrictions, or certification mismatches. These errors indicate that
///   prior authorization is required or the authorization information is incorrect.
///
/// - **Application/Input Errors** (`15`, `33`): Missing required application data or input errors.
///   Error `15` indicates missing required fields (often subscriber's member ID, date of birth,
///   first name, and last name), while error `33` indicates the request doesn't meet the payer's
///   requirements.
///
/// - **Data Consistency Errors** (`69`, `70`, `71`): Inconsistencies between patient data and codes
///   (age, gender) or date of birth mismatches. These errors help ensure data accuracy.
///
/// - **System/Connectivity Errors** (`42`): Payer system temporarily unavailable. This is a retryable
///   error indicating temporary payer downtime or connectivity issues.
///
/// - **Review/Additional Information Errors** (`AO`, `E8`): Additional patient condition information
///   required or medical review needed. These errors indicate the payer needs more information or
///   manual review.
///
/// ## Common Resolution Steps
///
/// Many subscriber-level errors can be resolved by:
///
/// - **Verifying subscriber information**: Ensure member ID, first name, last name, and date of birth
///   match exactly what's on the insurance card. Use full legal names (e.g., "Robert" not "Bob").
/// - **Checking date formats**: Ensure all dates are in `YYYYMMDD` format and within allowable ranges
///   (typically up to 12 months in the past or up to the end of the current month).
/// - **Including required fields**: Provide all four recommended fields (member ID, first name,
///   last name, date of birth) when available, as this ensures payers can find the member.
/// - **Verifying provider enrollment**: Ensure the provider is enrolled with the payer and registered
///   for the requested service types.
/// - **Correcting code issues**: Verify procedure codes, diagnosis codes, and service type codes
///   are valid and appropriate for the patient's age and gender.
/// - **Handling authorization**: Include prior authorization numbers when required, or ensure
///   authorization is obtained before submitting the request.
///
/// These errors are also included in the top-level `errors` array for centralized error review,
/// along with errors from payer, provider, and dependent levels.
///
/// Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
/// for detailed guidance on understanding and resolving AAA errors. Payers may sometimes
/// return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EligibilityCheckSubscriberErrorCode {
    #[serde(rename = "15")]
    #[default]
    RequiredApplicationDataMissing,
    #[serde(rename = "33")]
    InputErrors,
    #[serde(rename = "35")]
    OutOfNetwork,
    #[serde(rename = "42")]
    UnableToRespondAtCurrentTime,
    #[serde(rename = "43")]
    InvalidMissingProviderIdentification,
    #[serde(rename = "45")]
    InvalidMissingProviderSpecialty,
    #[serde(rename = "47")]
    InvalidMissingProviderState,
    #[serde(rename = "48")]
    InvalidMissingReferringProviderIdentificationNumber,
    #[serde(rename = "49")]
    ProviderIsNotPrimaryCarePhysician,
    #[serde(rename = "51")]
    ProviderNotOnFile,
    #[serde(rename = "52")]
    ServiceDatesNotWithinProviderPlanEnrollment,
    #[serde(rename = "53")]
    InquiredBenefitInconsistentWithProviderTypeEnrollment,
    #[serde(rename = "54")]
    InappropriateProductServiceIdQualifier,
    #[serde(rename = "55")]
    InappropriateProductServiceId,
    #[serde(rename = "56")]
    InappropriateDate,
    #[serde(rename = "57")]
    InvalidMissingDatesOfService,
    #[serde(rename = "58")]
    InvalidMissingDateOfBirth,
    #[serde(rename = "60")]
    DateOfBirthFollowsDatesOfService,
    #[serde(rename = "61")]
    DateOfDeathPrecedesDatesOfService,
    #[serde(rename = "62")]
    DateOfServiceNotWithinAllowableInquiryPeriod,
    #[serde(rename = "63")]
    DateOfServiceInFuture,
    #[serde(rename = "69")]
    InconsistentWithPatientsAge,
    #[serde(rename = "70")]
    InconsistentWithPatientsGender,
    #[serde(rename = "71")]
    PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase,
    #[serde(rename = "72")]
    InvalidMissingSubscriberInsuredId,
    #[serde(rename = "73")]
    InvalidMissingSubscriberInsuredName,
    #[serde(rename = "74")]
    InvalidMissingSubscriberInsuredGenderCode,
    #[serde(rename = "75")]
    SubscriberInsuredNotFound,
    #[serde(rename = "76")]
    DuplicateSubscriberInsuredIdNumber,
    #[serde(rename = "78")]
    SubscriberInsuredNotInGroupPlanIdentified,
    #[serde(rename = "98")]
    ExperimentalServiceOrProcedure,
    #[serde(rename = "AA")]
    AuthorizationNumberNotFound,
    #[serde(rename = "AE")]
    RequiresPrimaryCarePhysicianAuthorization,
    #[serde(rename = "AF")]
    InvalidMissingDiagnosisCodes,
    #[serde(rename = "AG")]
    InvalidMissingProcedureCodes,
    #[serde(rename = "AO")]
    AdditionalPatientConditionInformationRequired,
    #[serde(rename = "CI")]
    CertificationInformationDoesNotMatchPatient,
    #[serde(rename = "E8")]
    RequiresMedicalReview,
    #[serde(rename = "IA")]
    InvalidAuthorizationNumberFormat,
    #[serde(rename = "MA")]
    MissingAuthorizationNumber,
}

impl std::fmt::Display for EligibilityCheckSubscriberErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RequiredApplicationDataMissing => write!(f, "15"),
            Self::InputErrors => write!(f, "33"),
            Self::OutOfNetwork => write!(f, "35"),
            Self::UnableToRespondAtCurrentTime => write!(f, "42"),
            Self::InvalidMissingProviderIdentification => write!(f, "43"),
            Self::InvalidMissingProviderSpecialty => write!(f, "45"),
            Self::InvalidMissingProviderState => write!(f, "47"),
            Self::InvalidMissingReferringProviderIdentificationNumber => write!(f, "48"),
            Self::ProviderIsNotPrimaryCarePhysician => write!(f, "49"),
            Self::ProviderNotOnFile => write!(f, "51"),
            Self::ServiceDatesNotWithinProviderPlanEnrollment => write!(f, "52"),
            Self::InquiredBenefitInconsistentWithProviderTypeEnrollment => write!(f, "53"),
            Self::InappropriateProductServiceIdQualifier => write!(f, "54"),
            Self::InappropriateProductServiceId => write!(f, "55"),
            Self::InappropriateDate => write!(f, "56"),
            Self::InvalidMissingDatesOfService => write!(f, "57"),
            Self::InvalidMissingDateOfBirth => write!(f, "58"),
            Self::DateOfBirthFollowsDatesOfService => write!(f, "60"),
            Self::DateOfDeathPrecedesDatesOfService => write!(f, "61"),
            Self::DateOfServiceNotWithinAllowableInquiryPeriod => write!(f, "62"),
            Self::DateOfServiceInFuture => write!(f, "63"),
            Self::InconsistentWithPatientsAge => write!(f, "69"),
            Self::InconsistentWithPatientsGender => write!(f, "70"),
            Self::PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase => write!(f, "71"),
            Self::InvalidMissingSubscriberInsuredId => write!(f, "72"),
            Self::InvalidMissingSubscriberInsuredName => write!(f, "73"),
            Self::InvalidMissingSubscriberInsuredGenderCode => write!(f, "74"),
            Self::SubscriberInsuredNotFound => write!(f, "75"),
            Self::DuplicateSubscriberInsuredIdNumber => write!(f, "76"),
            Self::SubscriberInsuredNotInGroupPlanIdentified => write!(f, "78"),
            Self::ExperimentalServiceOrProcedure => write!(f, "98"),
            Self::AuthorizationNumberNotFound => write!(f, "AA"),
            Self::RequiresPrimaryCarePhysicianAuthorization => write!(f, "AE"),
            Self::InvalidMissingDiagnosisCodes => write!(f, "AF"),
            Self::InvalidMissingProcedureCodes => write!(f, "AG"),
            Self::AdditionalPatientConditionInformationRequired => write!(f, "AO"),
            Self::CertificationInformationDoesNotMatchPatient => write!(f, "CI"),
            Self::RequiresMedicalReview => write!(f, "E8"),
            Self::InvalidAuthorizationNumberFormat => write!(f, "IA"),
            Self::MissingAuthorizationNumber => write!(f, "MA"),
        }
    }
}
