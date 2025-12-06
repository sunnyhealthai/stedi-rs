use serde::{Deserialize, Serialize};

/// TimePeriodQualifierCode : Code specifying the time period for the benefit information.
///
/// These codes provide more information about the time period to which the benefit applies.
/// Returned in the `benefitsInformation.timeQualifierCode` and
/// `benefitsInformation.benefitsServiceDelivery.timePeriodQualifierCode` properties.
///
/// These codes help determine:
/// - The time frame over which benefit limits apply (e.g., Calendar Year, Lifetime)
/// - How benefit amounts are calculated (e.g., per Visit, per Episode)
/// - Whether limits have been exceeded or remain available
///
/// Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#time-qualifier-codes)
/// for a complete list. Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum TimePeriodQualifierCode {
    /// Hour - Benefit applies per hour
    #[serde(rename = "6")]
    #[default]
    Variant6,
    /// Day - Benefit applies per day
    #[serde(rename = "7")]
    Variant7,
    /// Years - Benefit applies per year or over multiple years
    #[serde(rename = "21")]
    Variant21,
    /// Service Year - Benefit applies per service year (typically runs from enrollment anniversary date)
    #[serde(rename = "22")]
    Variant22,
    /// Calendar Year - Benefit applies per calendar year (January 1 to December 31)
    #[serde(rename = "23")]
    Variant23,
    /// Year to Date - Benefit applies from the beginning of the current year to the present date
    #[serde(rename = "24")]
    Variant24,
    /// Contract - Benefit applies per contract period
    #[serde(rename = "25")]
    Variant25,
    /// Episode - Benefit applies per episode of care
    #[serde(rename = "26")]
    Variant26,
    /// Visit - Benefit applies per visit
    #[serde(rename = "27")]
    Variant27,
    /// Outlier - Benefit applies to outlier cases (cases that exceed normal limits)
    #[serde(rename = "28")]
    Variant28,
    /// Remaining - Indicates the remaining benefit amount available
    #[serde(rename = "29")]
    Variant29,
    /// Exceeded - Indicates that the benefit limit has been exceeded
    #[serde(rename = "30")]
    Variant30,
    /// Not Exceeded - Indicates that the benefit limit has not been exceeded
    #[serde(rename = "31")]
    Variant31,
    /// Lifetime - Benefit applies over the lifetime of the policy or member
    #[serde(rename = "32")]
    Variant32,
    /// Lifetime Remaining - Indicates the remaining lifetime benefit amount available
    #[serde(rename = "33")]
    Variant33,
    /// Month - Benefit applies per month
    #[serde(rename = "34")]
    Variant34,
    /// Week - Benefit applies per week
    #[serde(rename = "35")]
    Variant35,
}

impl std::fmt::Display for TimePeriodQualifierCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant6 => write!(f, "6"),
            Self::Variant7 => write!(f, "7"),
            Self::Variant21 => write!(f, "21"),
            Self::Variant22 => write!(f, "22"),
            Self::Variant23 => write!(f, "23"),
            Self::Variant24 => write!(f, "24"),
            Self::Variant25 => write!(f, "25"),
            Self::Variant26 => write!(f, "26"),
            Self::Variant27 => write!(f, "27"),
            Self::Variant28 => write!(f, "28"),
            Self::Variant29 => write!(f, "29"),
            Self::Variant30 => write!(f, "30"),
            Self::Variant31 => write!(f, "31"),
            Self::Variant32 => write!(f, "32"),
            Self::Variant33 => write!(f, "33"),
            Self::Variant34 => write!(f, "34"),
            Self::Variant35 => write!(f, "35"),
        }
    }
}
