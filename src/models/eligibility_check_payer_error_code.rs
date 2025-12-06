use serde::{Deserialize, Serialize};

/// Error codes for payer-level AAA errors in eligibility check responses.
///
/// These codes represent standardized X12 error codes (AAA errors) that payers return when
/// there are issues with an eligibility check request specifically related to the payer or
/// overall transaction. AAA errors indicate problems that prevented the payer from processing
/// the eligibility check successfully.
///
/// This enum is used in `eligibilityCheckPayerError.code` and `eligibilityCheckResponse.errors`
/// to specify the reason for rejection and any recommended follow-up actions. Common reasons
/// for rejection at the payer level include:
/// - Issues with payer enrollment or identification
/// - Payer system downtime or temporary unavailability
/// - Authorization or access restrictions
/// - Transaction termination or communication failures
/// - Invalid participant identification
/// - Quantity limits exceeded
///
/// ## Error Codes
///
/// The payer-level error codes include:
///
/// - **`04` - AuthorizedQuantityExceeded**: The requested quantity exceeds the authorized limit
/// - **`41` - AuthorizationAccessRestrictions**: Authorization or access restrictions prevent
///   processing the request
/// - **`42` - UnableToRespondAtCurrentTime**: The payer's system is temporarily unavailable
///   or experiencing issues
/// - **`79` - InvalidParticipantIdentification**: Invalid or missing participant identification
///   information
/// - **`80` - NoResponseReceivedTransactionTerminated**: The transaction was terminated without
///   receiving a response, often due to communication failures
/// - **`T4` - PayerNameOrIdentifierMissing**: Missing payer name or identifier in the request
///
/// These errors are also included in the top-level `errors` array for centralized error review,
/// along with errors from provider, subscriber, and dependent levels.
///
/// Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
/// for detailed guidance on understanding and resolving AAA errors. Payers may sometimes
/// return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EligibilityCheckPayerErrorCode {
    #[serde(rename = "04")]
    #[default]
    AuthorizedQuantityExceeded,
    #[serde(rename = "41")]
    AuthorizationAccessRestrictions,
    #[serde(rename = "42")]
    UnableToRespondAtCurrentTime,
    #[serde(rename = "79")]
    InvalidParticipantIdentification,
    #[serde(rename = "80")]
    NoResponseReceivedTransactionTerminated,
    #[serde(rename = "T4")]
    PayerNameOrIdentifierMissing,
}

impl std::fmt::Display for EligibilityCheckPayerErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AuthorizedQuantityExceeded => write!(f, "04"),
            Self::AuthorizationAccessRestrictions => write!(f, "41"),
            Self::UnableToRespondAtCurrentTime => write!(f, "42"),
            Self::InvalidParticipantIdentification => write!(f, "79"),
            Self::NoResponseReceivedTransactionTerminated => write!(f, "80"),
            Self::PayerNameOrIdentifierMissing => write!(f, "T4"),
        }
    }
}
