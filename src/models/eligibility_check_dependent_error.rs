use serde::{Deserialize, Serialize};

/// EligibilityCheckDependentError : When a payer rejects your eligibility check, the response contains one or more `AAA` errors that specify the reasons for the rejection and any recommended follow-up actions. Common reasons for rejection at the `subscriber` or `dependent` level include missing or incorrect identifying information and that the payer was unable to locate the patient in their system. [Learn more](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibilityCheckDependentError {
    /// The error code.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<super::EligibilityCheckDependentErrorCode>,
    /// The error description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The error type, `AAA`.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Allowed actions you can take, based on the rejection reason code. For example `Please Correct and Resubmit`.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "followupAction", skip_serializing_if = "Option::is_none")]
    pub followup_action: Option<super::EligibilityCheckProviderAndMemberFollowupAction>,
    /// The location of the error within the original X12 EDI response.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Information to help you correct the error.  We periodically update this guidance, so these strings may change at any time and may differ between eligibility responses. **Don't build programmatic logic that depends on matching these strings exactly.**
    #[serde(
        rename = "possibleResolutions",
        skip_serializing_if = "Option::is_none"
    )]
    pub possible_resolutions: Option<String>,
}

impl EligibilityCheckDependentError {
    /// When a payer rejects your eligibility check, the response contains one or more `AAA` errors that specify the reasons for the rejection and any recommended follow-up actions. Common reasons for rejection at the `subscriber` or `dependent` level include missing or incorrect identifying information and that the payer was unable to locate the patient in their system. [Learn more](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors)
    pub fn new() -> EligibilityCheckDependentError {
        EligibilityCheckDependentError {
            code: None,
            description: None,
            field: None,
            followup_action: None,
            location: None,
            possible_resolutions: None,
        }
    }
}
