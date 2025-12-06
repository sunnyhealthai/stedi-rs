use serde::{Deserialize, Serialize};

/// MaintenanceReasonCode : Code identifying the reason for the changes to subscriber identifying information, such as name, date of birth, or address. This is always `25`  Payers may sometimes return other non-compliant values.
/// Code identifying the reason for the changes to subscriber identifying information, such as name, date of birth, or address. This is always `25`  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum MaintenanceReasonCode {
    /// Change in Subscriber Identifying Information
    #[serde(rename = "25")]
    #[default]
    ChangeInSubscriberIdentifyingInformation,
}

impl std::fmt::Display for MaintenanceReasonCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChangeInSubscriberIdentifyingInformation => write!(f, "25"),
        }
    }
}
