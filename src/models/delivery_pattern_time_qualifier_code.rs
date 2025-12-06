use serde::{Deserialize, Serialize};

/// Delivery Pattern Time Codes correspond to X12 HIPAA delivery pattern time codes.
/// A code specifying the time for routine shipments or deliveries.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DeliveryPatternTimeQualifierCode {
    /// 1st Shift (Normal Working Hours)
    #[serde(rename = "A")]
    #[default]
    FirstShiftNormalWorkingHours,
    /// 2nd Shift
    #[serde(rename = "B")]
    SecondShift,
    /// 3rd Shift
    #[serde(rename = "C")]
    ThirdShift,
    /// A.M.
    #[serde(rename = "D")]
    AM,
    /// P.M.
    #[serde(rename = "E")]
    PM,
    /// As Directed
    #[serde(rename = "F")]
    AsDirected,
    /// Any Shift
    #[serde(rename = "G")]
    AnyShift,
    /// None (Also Used to Cancel or Override a Previous Pattern)
    #[serde(rename = "Y")]
    None,
}

impl std::fmt::Display for DeliveryPatternTimeQualifierCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FirstShiftNormalWorkingHours => write!(f, "A"),
            Self::SecondShift => write!(f, "B"),
            Self::ThirdShift => write!(f, "C"),
            Self::AM => write!(f, "D"),
            Self::PM => write!(f, "E"),
            Self::AsDirected => write!(f, "F"),
            Self::AnyShift => write!(f, "G"),
            Self::None => write!(f, "Y"),
        }
    }
}
