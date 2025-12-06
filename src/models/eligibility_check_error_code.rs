use serde::{Deserialize, Serialize};

/// Comprehensive error codes for AAA errors in eligibility check responses.
///
/// This enum is a **superset** of all possible error codes from sub-loops (provider, subscriber,
/// and dependent levels). All errors are bubbled up to the top level of the response, allowing
/// you to review all errors in a central location via `eligibilityCheckResponse.errors`.
///
/// These codes represent standardized X12 error codes (AAA errors) that payers return when
/// there are issues with an eligibility check request. AAA errors indicate problems that
/// prevented the payer from processing the eligibility check successfully.
///
/// ## Error Aggregation
///
/// Errors can occur at multiple levels in the eligibility response:
/// - **Payer level**: Errors related to the payer or overall transaction
/// - **Provider level**: Errors related to provider identification or enrollment
/// - **Subscriber level**: Errors related to the primary policyholder
/// - **Dependent level**: Errors related to dependents covered under the subscriber's plan
///
/// All errors from these levels are included in the top-level `errors` array, making this enum
/// a comprehensive collection of all possible error codes. This allows you to:
/// - Review all errors in one place without checking multiple nested objects
/// - Understand the full scope of issues preventing successful eligibility verification
/// - Implement centralized error handling logic
///
/// ## Error Categories
///
/// The error codes can be grouped into several categories:
///
/// - **Patient Identification Errors** (`64`, `65`, `66`, `67`, `68`, `71`): Missing or invalid
///   patient identifiers, names, gender codes, or patient not found
/// - **Subscriber Identification Errors** (`72`, `73`, `74`, `75`, `76`, `77`, `78`): Missing
///   or invalid subscriber identifiers, names, gender codes, subscriber not found, or group plan
///   issues
/// - **Date Validation Errors** (`56`, `57`, `58`, `60`, `61`, `62`, `63`): Invalid dates,
///   missing dates, date logic errors, or dates outside allowable periods
/// - **Provider Errors** (`43`, `44`, `45`, `46`, `47`, `48`, `49`, `50`, `51`, `52`, `53`, `97`):
///   Provider identification, name, specialty, contact information, network status, enrollment,
///   or authorization issues
/// - **Service/Procedure Errors** (`04`, `54`, `55`, `98`, `AF`, `AG`): Invalid or missing
///   procedure codes, diagnosis codes, experimental services, or quantity exceeded
/// - **Authorization Errors** (`41`, `AA`, `AE`, `IA`, `MA`, `CI`): Authorization number issues,
///   missing authorizations, access restrictions, or certification mismatches
/// - **General Errors** (`15`, `33`, `35`, `42`, `69`, `70`, `79`, `80`, `AO`, `E8`, `T4`):
///   Application data, input errors, network issues, data inconsistencies, review requirements,
///   transaction termination, or payer identification issues
///
/// ## Usage
///
/// When a payer rejects an eligibility check, the response contains one or more AAA errors
/// in the top-level `errors` array. Each error includes:
/// - The error code (this enum)
/// - An error description
/// - Recommended follow-up actions
/// - Possible resolutions
/// - Error location within the original X12 EDI response
///
/// Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
/// for detailed guidance on understanding and resolving AAA errors. Payers may sometimes
/// return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EligibilityCheckErrorCode {
    #[serde(rename = "04")]
    #[default]
    AuthorizedQuantityExceeded,
    #[serde(rename = "15")]
    RequiredApplicationDataMissing,
    #[serde(rename = "33")]
    InputErrors,
    #[serde(rename = "35")]
    OutOfNetwork,
    #[serde(rename = "41")]
    AuthorizationAccessRestrictions,
    #[serde(rename = "42")]
    UnableToRespondAtCurrentTime,
    #[serde(rename = "43")]
    InvalidMissingProviderIdentification,
    #[serde(rename = "44")]
    InvalidMissingProviderName,
    #[serde(rename = "45")]
    InvalidMissingProviderSpecialty,
    #[serde(rename = "46")]
    InvalidMissingProviderPhoneNumber,
    #[serde(rename = "47")]
    InvalidMissingProviderState,
    #[serde(rename = "48")]
    InvalidMissingReferringProviderIdentificationNumber,
    #[serde(rename = "49")]
    ProviderIsNotPrimaryCarePhysician,
    #[serde(rename = "50")]
    ProviderIneligibleForInquiries,
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
    #[serde(rename = "64")]
    InvalidMissingPatientId,
    #[serde(rename = "65")]
    InvalidMissingPatientName,
    #[serde(rename = "66")]
    InvalidMissingPatientGenderCode,
    #[serde(rename = "67")]
    PatientNotFound,
    #[serde(rename = "68")]
    DuplicatePatientIdNumber,
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
    #[serde(rename = "77")]
    SubscriberFoundPatientNotFound,
    #[serde(rename = "78")]
    SubscriberInsuredNotInGroupPlanIdentified,
    #[serde(rename = "79")]
    InvalidParticipantIdentification,
    #[serde(rename = "80")]
    NoResponseReceivedTransactionTerminated,
    #[serde(rename = "97")]
    InvalidOrMissingProviderAddress,
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
    #[serde(rename = "T4")]
    PayerNameOrIdentifierMissing,
}

impl std::fmt::Display for EligibilityCheckErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AuthorizedQuantityExceeded => write!(f, "04"),
            Self::RequiredApplicationDataMissing => write!(f, "15"),
            Self::InputErrors => write!(f, "33"),
            Self::OutOfNetwork => write!(f, "35"),
            Self::AuthorizationAccessRestrictions => write!(f, "41"),
            Self::UnableToRespondAtCurrentTime => write!(f, "42"),
            Self::InvalidMissingProviderIdentification => write!(f, "43"),
            Self::InvalidMissingProviderName => write!(f, "44"),
            Self::InvalidMissingProviderSpecialty => write!(f, "45"),
            Self::InvalidMissingProviderPhoneNumber => write!(f, "46"),
            Self::InvalidMissingProviderState => write!(f, "47"),
            Self::InvalidMissingReferringProviderIdentificationNumber => write!(f, "48"),
            Self::ProviderIsNotPrimaryCarePhysician => write!(f, "49"),
            Self::ProviderIneligibleForInquiries => write!(f, "50"),
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
            Self::InvalidMissingPatientId => write!(f, "64"),
            Self::InvalidMissingPatientName => write!(f, "65"),
            Self::InvalidMissingPatientGenderCode => write!(f, "66"),
            Self::PatientNotFound => write!(f, "67"),
            Self::DuplicatePatientIdNumber => write!(f, "68"),
            Self::InconsistentWithPatientsAge => write!(f, "69"),
            Self::InconsistentWithPatientsGender => write!(f, "70"),
            Self::PatientBirthDateDoesNotMatchThatForThePatientOnTheDatabase => write!(f, "71"),
            Self::InvalidMissingSubscriberInsuredId => write!(f, "72"),
            Self::InvalidMissingSubscriberInsuredName => write!(f, "73"),
            Self::InvalidMissingSubscriberInsuredGenderCode => write!(f, "74"),
            Self::SubscriberInsuredNotFound => write!(f, "75"),
            Self::DuplicateSubscriberInsuredIdNumber => write!(f, "76"),
            Self::SubscriberFoundPatientNotFound => write!(f, "77"),
            Self::SubscriberInsuredNotInGroupPlanIdentified => write!(f, "78"),
            Self::InvalidParticipantIdentification => write!(f, "79"),
            Self::NoResponseReceivedTransactionTerminated => write!(f, "80"),
            Self::InvalidOrMissingProviderAddress => write!(f, "97"),
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
            Self::PayerNameOrIdentifierMissing => write!(f, "T4"),
        }
    }
}
