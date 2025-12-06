use serde::{Deserialize, Serialize};

/// Entity identifier for the subscriber in eligibility check responses.
///
/// This identifier categorizes the role of the subscriber in the healthcare transaction.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ResponseSubscriberEntityIdentifier {
    /// The person who is insured or the primary subscriber on the insurance plan
    #[serde(rename = "Insured or Subscriber")]
    #[default]
    InsuredOrSubscriber,
}

impl std::fmt::Display for ResponseSubscriberEntityIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InsuredOrSubscriber => write!(f, "Insured or Subscriber"),
        }
    }
}
