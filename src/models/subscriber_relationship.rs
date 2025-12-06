use serde::{Deserialize, Serialize};

/// Relationship of a subscriber to themselves in eligibility check responses.
///
/// Specifies that the subscriber is the primary policyholder. This human-readable name
/// corresponds to the `relationToSubscriberCode` field, which contains the standardized code
/// version (`18` - Self).
///
/// This field is used in `responseSubscriber.relationToSubscriber` to identify that the subscriber
/// is the primary insured party. Understanding this relationship is important for:
/// - Verifying that the subscriber is the primary policyholder
/// - Determining who is responsible for the insurance policy
/// - Processing coordination of benefits scenarios
/// - Understanding coverage relationships in eligibility checks
///
/// ## Relationship Context
///
/// - **Self**: The subscriber is the primary policyholder who holds the insurance contract.
///   This is the only value returned for subscribers, as they are the primary insured party.
///
/// The corresponding `SubscriberRelationshipCode` enum contains the standardized code:
/// - `18` - Self
///
/// This is distinct from `DependentRelationship`, which contains values like "Spouse", "Child",
/// etc., for dependents who are covered under the subscriber's plan.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum SubscriberRelationship {
    /// Self - Indicates that the subscriber is the primary policyholder.
    ///
    /// This is the only value returned for subscribers, as they are the primary insured
    /// party who holds the insurance contract. This corresponds to relationship code `18`
    /// in the `SubscriberRelationshipCode` enum.
    #[serde(rename = "Self")]
    #[default]
    VariantSelf,
}

impl std::fmt::Display for SubscriberRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::VariantSelf => write!(f, "Self"),
        }
    }
}
