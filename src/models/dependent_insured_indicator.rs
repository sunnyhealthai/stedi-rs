use serde::{Deserialize, Serialize};

/// Insured indicator for dependents in eligibility check responses.
///
/// Indicates whether the person is the primary insured party. For dependents, this value is
/// always `N` (No), indicating that the dependent is not the primary insured but is covered
/// under the subscriber's insurance plan.
///
/// This field is used in `responseDependent.insuredIndicator` to distinguish between:
/// - **Subscriber**: The primary insured party (uses `SubscriberInsuredIndicator` with value `Y`)
/// - **Dependent**: A person covered under the subscriber's plan (uses `DependentInsuredIndicator` with value `N`)
///
/// ## Usage Context
///
/// In healthcare eligibility transactions, the insured indicator helps identify the relationship
/// between the person and the insurance policy:
/// - The subscriber is the primary policyholder who holds the insurance contract
/// - Dependents are family members (spouse, children, etc.) who are covered under the subscriber's plan
/// - Dependents receive benefits through the subscriber's policy but are not themselves the primary insured
///
/// This distinction is important for:
/// - Determining who is responsible for the insurance policy
/// - Understanding coverage relationships in coordination of benefits scenarios
/// - Processing claims and benefits correctly
///
/// Note: This enum currently only contains the `N` variant, as dependents are always
/// marked as not being the primary insured. The corresponding `SubscriberInsuredIndicator`
/// enum contains the `Y` variant for subscribers.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DependentInsuredIndicator {
    /// Indicates that the dependent is not the primary insured party.
    ///
    /// This is the only value returned for dependents, as they are covered under
    /// the subscriber's insurance plan rather than being the primary policyholder.
    #[serde(rename = "N")]
    #[default]
    N,
}

impl std::fmt::Display for DependentInsuredIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::N => write!(f, "N"),
        }
    }
}
