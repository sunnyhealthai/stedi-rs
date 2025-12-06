use serde::{Deserialize, Serialize};

/// Entity identifier for dependents in eligibility check responses.
///
/// This identifier categorizes the role of the dependent in the healthcare transaction.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ResponseDependentEntityIdentifier {
    /// A person who is covered under the subscriber's insurance plan as a dependent
    #[serde(rename = "Dependent")]
    #[default]
    Dependent,
}

impl std::fmt::Display for ResponseDependentEntityIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Dependent => write!(f, "Dependent"),
        }
    }
}
