use serde::{Deserialize, Serialize};

/// Error codes for dependent-level AAA errors in eligibility check responses.
///
/// These codes represent standardized X12 error codes (AAA errors) that payers return when
/// there are issues with an eligibility check request specifically related to the dependent
/// information. AAA errors indicate problems that prevented the payer from processing the
/// eligibility check successfully.
///
/// This enum is used in `responseDependent.aaaErrors` and `eligibilityCheckResponse.errors`
/// to specify the reason for rejection and any recommended follow-up actions. Common reasons
/// for rejection at the dependent level include:
/// - Missing or incorrect dependent identifying information (name, date of birth, member ID)
/// - Patient not found in the payer's system
/// - Data inconsistencies (age, gender, date mismatches)
/// - Provider-related issues (network status, enrollment, authorization)
/// - Date validation errors (future dates, invalid date ranges)
/// - Missing required codes (diagnosis, procedure, authorization)
///
/// ## Error Categories
///
/// The error codes can be grouped into several categories:
///
/// - **Patient Identification Errors** (`64`, `65`, `66`, `67`, `68`, `71`, `77`): Missing or
///   invalid patient identifiers, names, gender codes, or patient not found
/// - **Date Validation Errors** (`56`, `57`, `58`, `60`, `61`, `62`, `63`): Invalid dates,
///   missing dates, date logic errors, or dates outside allowable periods
/// - **Provider Errors** (`43`, `45`, `47`, `48`, `49`, `51`, `52`, `53`): Provider identification,
///   network status, enrollment, or authorization issues
/// - **Service/Procedure Errors** (`54`, `55`, `98`, `AF`, `AG`): Invalid or missing procedure
///   codes, diagnosis codes, or experimental services
/// - **Authorization Errors** (`AA`, `AE`, `IA`, `MA`, `CI`): Authorization number issues or
///   missing authorizations
/// - **General Errors** (`15`, `33`, `35`, `42`, `69`, `70`, `AO`, `E8`): Application data,
///   input errors, network issues, data inconsistencies, or review requirements
///
/// When a payer rejects an eligibility check, the response contains one or more AAA errors
/// that specify the reasons for rejection and recommended follow-up actions. These errors
/// are also included in the top-level `errors` array for centralized error review.
///
/// Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
/// for detailed guidance on understanding and resolving AAA errors. Payers may sometimes
/// return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EligibilityCheckDependentErrorCode {
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
    PatientDobDoesNotMatchThatForThePatientOnTheDatabase,
    #[serde(rename = "77")]
    SubscriberFoundPatientNotFound,
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

impl std::fmt::Display for EligibilityCheckDependentErrorCode {
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
            Self::InvalidMissingPatientId => write!(f, "64"),
            Self::InvalidMissingPatientName => write!(f, "65"),
            Self::InvalidMissingPatientGenderCode => write!(f, "66"),
            Self::PatientNotFound => write!(f, "67"),
            Self::DuplicatePatientIdNumber => write!(f, "68"),
            Self::InconsistentWithPatientsAge => write!(f, "69"),
            Self::InconsistentWithPatientsGender => write!(f, "70"),
            Self::PatientDobDoesNotMatchThatForThePatientOnTheDatabase => write!(f, "71"),
            Self::SubscriberFoundPatientNotFound => write!(f, "77"),
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
