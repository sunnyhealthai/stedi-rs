use serde::{Deserialize, Serialize};

/// Human-readable names for time period qualifier codes.
///
/// These represent the textual descriptions of time qualifier codes used in benefit information.
///
/// ## Important Note for Deductibles
///
/// For patient deductibles:
/// - **Calendar Year**: Patient's total deductible amount for the year
/// - **Remaining**: Amount left to meet the deductible
///
/// Visit [Payer benefit response](https://www.stedi.com/docs/healthcare/eligibility-patient-responsibility-benefits#deductible)
/// to learn more about deductibles.
///
/// Note: Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum TimeQualifierName {
    /// Hour
    #[serde(rename = "Hour")]
    #[default]
    Hour,

    /// Day
    #[serde(rename = "Day")]
    Day,

    /// 24 Hours
    #[serde(rename = "24 Hours")]
    TwentyFourHours,

    /// Years
    #[serde(rename = "Years")]
    Years,

    /// Service Year
    #[serde(rename = "Service Year")]
    ServiceYear,

    /// Calendar Year
    #[serde(rename = "Calendar Year")]
    CalendarYear,

    /// Year to Date
    #[serde(rename = "Year to Date")]
    YearToDate,

    /// Contract
    #[serde(rename = "Contract")]
    Contract,

    /// Episode
    #[serde(rename = "Episode")]
    Episode,

    /// Visit
    #[serde(rename = "Visit")]
    Visit,

    /// Outlier
    #[serde(rename = "Outlier")]
    Outlier,

    /// Remaining
    #[serde(rename = "Remaining")]
    Remaining,

    /// Exceeded
    #[serde(rename = "Exceeded")]
    Exceeded,

    /// Not Exceeded
    #[serde(rename = "Not Exceeded")]
    NotExceeded,

    /// Lifetime
    #[serde(rename = "Lifetime")]
    Lifetime,

    /// Lifetime Remaining
    #[serde(rename = "Lifetime Remaining")]
    LifetimeRemaining,

    /// Month
    #[serde(rename = "Month")]
    Month,

    /// Week
    #[serde(rename = "Week")]
    Week,

    /// Admission
    #[serde(rename = "Admission")]
    Admission,
}

impl std::fmt::Display for TimeQualifierName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hour => write!(f, "Hour"),
            Self::Day => write!(f, "Day"),
            Self::TwentyFourHours => write!(f, "24 Hours"),
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
            Self::Admission => write!(f, "Admission"),
        }
    }
}
