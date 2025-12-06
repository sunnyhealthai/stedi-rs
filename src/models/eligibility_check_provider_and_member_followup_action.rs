use serde::{Deserialize, Serialize};

/// Follow-up actions for eligibility check errors at the provider, subscriber, and dependent levels.
///
/// Specifies the recommended actions you should take when a payer returns an AAA error
/// in an eligibility check response at the provider, subscriber, or dependent level. These
/// actions guide how to handle the error and whether (and when) to resubmit the eligibility
/// check request.
///
/// This enum is used in:
/// - `eligibilityCheckProviderError.followupAction` (provider-level errors)
/// - `eligibilityCheckSubscriberError.followupAction` (subscriber-level errors)
/// - `eligibilityCheckDependentError.followupAction` (dependent-level errors)
///
/// Note: For payer-level and top-level errors, use `EligibilityCheckFollowupAction` instead.
///
/// ## Action Types
///
/// The follow-up actions can be categorized as:
///
/// - **Correct and Resubmit**: `PleaseCorrectAndResubmit` - Indicates that you should
///   fix the errors identified and resubmit the request. This is the most common action
///   for correctable errors like missing or invalid data.
///
/// - **Resubmission Allowed**: `ResubmissionAllowed` - Indicates that resubmission is
///   permitted, but doesn't specify whether corrections are needed. Review the error
///   details to determine if changes are required.
///
/// - **Resubmission Not Allowed**: `ResubmissionNotAllowed` - Indicates that you should
///   not resubmit the request. This may occur when the error is permanent or requires
///   manual intervention.
///
/// - **Wait and Resubmit**: `PleaseWait10DaysAndResubmit`, `PleaseWait30DaysAndResubmit` -
///   Indicates that you should wait a specified number of days before resubmitting.
///   This is often used when the payer needs time to process information or when there
///   are timing restrictions.
///
/// - **Do Not Resubmit**: `DoNotResubmitSemicolonInquiryInitiatedToAThirdParty`,
///   `DoNotResubmitSemicolonWeWillHoldYourRequestAndRespondAgainShortly` - Indicates
///   that the payer is handling the request internally (either by forwarding to a third
///   party or holding it for processing) and you should not resubmit.
///
/// ## Usage
///
/// When processing eligibility check errors at the provider, subscriber, or dependent level,
/// check the `followupAction` field to determine the appropriate next steps. Combine this with
/// the error code and description to understand what went wrong and how to resolve it. The
/// `possibleResolutions` field may also provide additional guidance, though you should not
/// build programmatic logic that depends on matching those strings exactly.
///
/// Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
/// for detailed guidance on understanding and resolving AAA errors. Payers may sometimes
/// return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EligibilityCheckProviderAndMemberFollowupAction {
    #[serde(rename = "Please Correct and Resubmit")]
    #[default]
    PleaseCorrectAndResubmit,
    #[serde(rename = "Resubmission Not Allowed")]
    ResubmissionNotAllowed,
    #[serde(rename = "Resubmission Allowed")]
    ResubmissionAllowed,
    #[serde(rename = "Do Not Resubmit; Inquiry Initiated to a Third Party")]
    DoNotResubmitSemicolonInquiryInitiatedToAThirdParty,
    #[serde(rename = "Please Wait 30 Days and Resubmit")]
    PleaseWait30DaysAndResubmit,
    #[serde(rename = "Please Wait 10 Days and Resubmit")]
    PleaseWait10DaysAndResubmit,
    #[serde(rename = "Do Not Resubmit; We Will Hold Your Request and Respond Again Shortly")]
    DoNotResubmitSemicolonWeWillHoldYourRequestAndRespondAgainShortly,
}

impl std::fmt::Display for EligibilityCheckProviderAndMemberFollowupAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PleaseCorrectAndResubmit => write!(f, "Please Correct and Resubmit"),
            Self::ResubmissionNotAllowed => write!(f, "Resubmission Not Allowed"),
            Self::ResubmissionAllowed => write!(f, "Resubmission Allowed"),
            Self::DoNotResubmitSemicolonInquiryInitiatedToAThirdParty => {
                write!(f, "Do Not Resubmit; Inquiry Initiated to a Third Party")
            }
            Self::PleaseWait30DaysAndResubmit => write!(f, "Please Wait 30 Days and Resubmit"),
            Self::PleaseWait10DaysAndResubmit => write!(f, "Please Wait 10 Days and Resubmit"),
            Self::DoNotResubmitSemicolonWeWillHoldYourRequestAndRespondAgainShortly => write!(
                f,
                "Do Not Resubmit; We Will Hold Your Request and Respond Again Shortly"
            ),
        }
    }
}
