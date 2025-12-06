use serde::{Deserialize, Serialize};

/// Time qualifier codes providing information about the time period to which benefits apply.
///
/// These codes are returned in the `benefitsInformation.timeQualifierCode` and
/// `benefitsInformation.benefitsServiceDelivery.timePeriodQualifierCode` properties.
///
/// Note: Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum TimeQualifierCode {
    /// Hour
    #[serde(rename = "6")]
    #[default]
    Hour,

    /// Day
    #[serde(rename = "7")]
    Day,

    /// 24 Hours
    #[serde(rename = "13")]
    TwentyFourHours,

    /// Years
    #[serde(rename = "21")]
    Years,

    /// Service Year
    #[serde(rename = "22")]
    ServiceYear,

    /// Calendar Year
    #[serde(rename = "23")]
    CalendarYear,

    /// Year to Date
    #[serde(rename = "24")]
    YearToDate,

    /// Contract
    #[serde(rename = "25")]
    Contract,

    /// Episode
    #[serde(rename = "26")]
    Episode,

    /// Visit
    #[serde(rename = "27")]
    Visit,

    /// Outlier
    #[serde(rename = "28")]
    Outlier,

    /// Remaining
    #[serde(rename = "29")]
    Remaining,

    /// Exceeded
    #[serde(rename = "30")]
    Exceeded,

    /// Not Exceeded
    #[serde(rename = "31")]
    NotExceeded,

    /// Lifetime
    #[serde(rename = "32")]
    Lifetime,

    /// Lifetime Remaining
    #[serde(rename = "33")]
    LifetimeRemaining,

    /// Month
    #[serde(rename = "34")]
    Month,

    /// Week
    #[serde(rename = "35")]
    Week,

    /// Admission
    #[serde(rename = "36")]
    Admission,
}

impl std::fmt::Display for TimeQualifierCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hour => write!(f, "6"),
            Self::Day => write!(f, "7"),
            Self::TwentyFourHours => write!(f, "13"),
            Self::Years => write!(f, "21"),
            Self::ServiceYear => write!(f, "22"),
            Self::CalendarYear => write!(f, "23"),
            Self::YearToDate => write!(f, "24"),
            Self::Contract => write!(f, "25"),
            Self::Episode => write!(f, "26"),
            Self::Visit => write!(f, "27"),
            Self::Outlier => write!(f, "28"),
            Self::Remaining => write!(f, "29"),
            Self::Exceeded => write!(f, "30"),
            Self::NotExceeded => write!(f, "31"),
            Self::Lifetime => write!(f, "32"),
            Self::LifetimeRemaining => write!(f, "33"),
            Self::Month => write!(f, "34"),
            Self::Week => write!(f, "35"),
            Self::Admission => write!(f, "36"),
        }
    }
}
