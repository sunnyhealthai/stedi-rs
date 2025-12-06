use serde::{Deserialize, Serialize};

/// Human-readable name for the time period qualifier code in eligibility check responses.
///
/// Specifies the time period to which benefit information applies. This human-readable name
/// corresponds to the `timePeriodQualifierCode` field, which contains the standardized code
/// version (e.g., `6` - Hour, `23` - Calendar Year).
///
/// This field is used in `benefitsInformation.timeQualifierCode` and
/// `benefitsInformation.benefitsServiceDelivery.timePeriodQualifierCode` to identify the time
/// period for benefit calculations and limits.
///
/// These qualifiers help determine:
/// - The time frame over which benefit limits apply (e.g., Calendar Year, Lifetime)
/// - How benefit amounts are calculated (e.g., per Visit, per Episode)
/// - Whether limits have been exceeded or remain available
///
/// The corresponding `TimePeriodQualifierCode` enum contains the standardized codes:
/// - `6` - Hour
/// - `7` - Day
/// - `21` - Years
/// - `22` - Service Year
/// - `23` - Calendar Year
/// - `24` - Year to Date
/// - `25` - Contract
/// - `26` - Episode
/// - `27` - Visit
/// - `28` - Outlier
/// - `29` - Remaining
/// - `30` - Exceeded
/// - `31` - Not Exceeded
/// - `32` - Lifetime
/// - `33` - Lifetime Remaining
/// - `34` - Month
/// - `35` - Week
///
/// Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#time-qualifier-codes)
/// for a complete list. Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum TimePeriodQualifier {
    /// Hour - Benefit applies per hour
    #[serde(rename = "Hour")]
    #[default]
    Hour,
    /// Day - Benefit applies per day
    #[serde(rename = "Day")]
    Day,
    /// Years - Benefit applies per year or over multiple years
    #[serde(rename = "Years")]
    Years,
    /// Service Year - Benefit applies per service year (typically runs from enrollment anniversary date)
    #[serde(rename = "Service Year")]
    ServiceYear,
    /// Calendar Year - Benefit applies per calendar year (January 1 to December 31)
    #[serde(rename = "Calendar Year")]
    CalendarYear,
    /// Year to Date - Benefit applies from the beginning of the current year to the present date
    #[serde(rename = "Year to Date")]
    YearToDate,
    /// Contract - Benefit applies per contract period
    #[serde(rename = "Contract")]
    Contract,
    /// Episode - Benefit applies per episode of care
    #[serde(rename = "Episode")]
    Episode,
    /// Visit - Benefit applies per visit
    #[serde(rename = "Visit")]
    Visit,
    /// Outlier - Benefit applies to outlier cases (cases that exceed normal limits)
    #[serde(rename = "Outlier")]
    Outlier,
    /// Remaining - Indicates the remaining benefit amount available
    #[serde(rename = "Remaining")]
    Remaining,
    /// Exceeded - Indicates that the benefit limit has been exceeded
    #[serde(rename = "Exceeded")]
    Exceeded,
    /// Not Exceeded - Indicates that the benefit limit has not been exceeded
    #[serde(rename = "Not Exceeded")]
    NotExceeded,
    /// Lifetime - Benefit applies over the lifetime of the policy or member
    #[serde(rename = "Lifetime")]
    Lifetime,
    /// Lifetime Remaining - Indicates the remaining lifetime benefit amount available
    #[serde(rename = "Lifetime Remaining")]
    LifetimeRemaining,
    /// Month - Benefit applies per month
    #[serde(rename = "Month")]
    Month,
    /// Week - Benefit applies per week
    #[serde(rename = "Week")]
    Week,
}

impl std::fmt::Display for TimePeriodQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hour => write!(f, "Hour"),
            Self::Day => write!(f, "Day"),
            Self::Years => write!(f, "Years"),
            Self::ServiceYear => write!(f, "Service Year"),
            Self::CalendarYear => write!(f, "Calendar Year"),
            Self::YearToDate => write!(f, "Year to Date"),
            Self::Contract => write!(f, "Contract"),
            Self::Episode => write!(f, "Episode"),
            Self::Visit => write!(f, "Visit"),
            Self::Outlier => write!(f, "Outlier"),
            Self::Remaining => write!(f, "Remaining"),
            Self::Exceeded => write!(f, "Exceeded"),
            Self::NotExceeded => write!(f, "Not Exceeded"),
            Self::Lifetime => write!(f, "Lifetime"),
            Self::LifetimeRemaining => write!(f, "Lifetime Remaining"),
            Self::Month => write!(f, "Month"),
            Self::Week => write!(f, "Week"),
        }
    }
}
