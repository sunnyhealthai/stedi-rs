use serde::{Deserialize, Serialize};

/// Insured indicator for subscribers in eligibility check responses.
///
/// Indicates whether the person is the primary insured party. For subscribers, this value is
/// always `Y` (Yes), indicating that the subscriber is the primary insured and holds the
/// insurance policy contract.
///
/// This field is used in `responseSubscriber.insuredIndicator` to distinguish between:
/// - **Subscriber**: The primary insured party (uses `SubscriberInsuredIndicator` with value `Y`)
/// - **Dependent**: A person covered under the subscriber's plan (uses `DependentInsuredIndicator` with value `N`)
///
/// ## Usage Context
///
/// In healthcare eligibility transactions, the insured indicator helps identify the relationship
/// between the person and the insurance policy:
/// - The subscriber is the primary policyholder who holds the insurance contract
/// - Dependents are family members (spouse, children, etc.) who are covered under the subscriber's plan
/// - The subscriber receives benefits directly from their policy and is responsible for the insurance contract
///
/// This distinction is important for:
/// - Determining who is responsible for the insurance policy
/// - Understanding coverage relationships in coordination of benefits scenarios
/// - Processing claims and benefits correctly
///
/// Note: This enum currently only contains the `Y` variant, as subscribers are always
/// marked as the primary insured. The corresponding `DependentInsuredIndicator`
/// enum contains the `N` variant for dependents.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum SubscriberInsuredIndicator {
    /// Indicates that the subscriber is the primary insured party.
    ///
    /// This is the only value returned for subscribers, as they are the primary policyholder
    /// who holds the insurance contract and is responsible for the policy.
    #[serde(rename = "Y")]
    #[default]
    Y,
}

impl std::fmt::Display for SubscriberInsuredIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Y => write!(f, "Y"),
        }
    }
}
