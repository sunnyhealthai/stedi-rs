use serde::{Deserialize, Serialize};

/// Relationship code for subscribers in eligibility check responses.
///
/// Indicates the relationship of the subscriber to themselves. For subscribers, this value is
/// always `18` (Self), indicating that the subscriber is the primary policyholder and the
/// relationship is to themselves.
///
/// This field is used in `responseSubscriber.relationToSubscriberCode` to distinguish between:
/// - **Subscriber**: The primary policyholder (uses `SubscriberRelationshipCode` with value `18` - Self)
/// - **Dependent**: A person covered under the subscriber's plan (uses `DependentRelationshipCode` with values like `01` - Spouse, `19` - Child, etc.)
///
/// ## Usage Context
///
/// In healthcare eligibility transactions, relationship codes help identify the relationship
/// between the person and the insurance policy:
/// - The subscriber is the primary policyholder who holds the insurance contract
/// - Dependents are family members (spouse, children, etc.) who are covered under the subscriber's plan
/// - The subscriber's relationship code is always "18" (Self) because they are the primary insured
///
/// This distinction is important for:
/// - Determining who is responsible for the insurance policy
/// - Understanding coverage relationships in coordination of benefits scenarios
/// - Processing claims and benefits correctly
/// - Identifying family relationships for dependent coverage
///
/// Note: This enum currently only contains the `18` variant, as subscribers are always
/// marked as "Self". The corresponding `DependentRelationshipCode` enum contains various
/// relationship codes (e.g., `01` - Spouse, `19` - Child) for dependents.
///
/// The relationship code `18` corresponds to the X12 EDI element 1069 (Individual Relationship Code)
/// standard, which defines "18" as "Self".
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum SubscriberRelationshipCode {
    /// Self - Indicates that the subscriber is the primary policyholder.
    ///
    /// This is the only value returned for subscribers, as they are the primary insured
    /// party who holds the insurance contract. The code `18` is the standard X12 EDI
    /// relationship code for "Self".
    #[serde(rename = "18")]
    #[default]
    Variant18,
}

impl std::fmt::Display for SubscriberRelationshipCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant18 => write!(f, "18"),
        }
    }
}
