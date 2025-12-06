use serde::{Deserialize, Serialize};

/// MaintenanceTypeCode : The maintenance type code. Used to acknowledge a change in the identifying elements for the subscriber from those submitted in the original eligibility check request. It can also be included when the payer used the birth sequence number from the original request to locate the subscriber in their system. This is always `001`  Payers may sometimes return other non-compliant values.
/// The maintenance type code. Used to acknowledge a change in the identifying elements for the subscriber from those submitted in the original eligibility check request. It can also be included when the payer used the birth sequence number from the original request to locate the subscriber in their system. This is always `001`  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum MaintenanceTypeCode {
    /// Change in Identifying Elements
    #[serde(rename = "001")]
    #[default]
    ChangeInIdentifyingElements,
}

impl std::fmt::Display for MaintenanceTypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChangeInIdentifyingElements => write!(f, "001"),
        }
    }
}
