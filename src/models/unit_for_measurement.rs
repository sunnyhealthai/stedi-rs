use serde::{Deserialize, Serialize};

/// Measurement unit qualifier used in healthcare eligibility and benefits transactions.
///
/// This enum specifies the unit of measurement for benefit quantities, time periods, and service
/// limitations in HIPAA X12 transactions. It corresponds to the X12 Unit of Measurement Code
/// (UM) segment qualifier codes and is used to define how benefit limits, durations, or frequencies
/// are measured and interpreted.
///
/// These units are commonly used in:
/// - Benefit time period limits (e.g., coverage duration)
/// - Visit count restrictions (e.g., maximum visits per time period)
/// - Service frequency limitations (e.g., units per week)
/// - Age eligibility requirements
///
/// Note: Payers may sometimes return non-standard or custom values that don't conform to the
/// official X12 unit codes. Applications should be prepared to handle such cases gracefully.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum UnitForMeasurement {
    /// Days - Represents a time period measured in days
    ///
    /// Typically used for:
    /// - Short-term benefit limitations
    /// - Daily visit or service limits
    /// - Age restrictions measured in days (e.g., newborn coverage)
    ///
    /// X12 HIPAA Code: DA (Days)
    /// Stedi Note: Commonly used for precise time-based restrictions and short-term coverage periods
    #[serde(rename = "Days")]
    #[default]
    Days,

    /// Months - Represents a time period measured in months
    ///
    /// Typically used for:
    /// - Monthly benefit or coverage limits
    /// - Age eligibility measured in months (e.g., infant coverage)
    /// - Benefit period durations
    ///
    /// X12 HIPAA Code: MO (Months)
    /// Stedi Note: Frequently used for standard benefit period calculations and age-based eligibility
    #[serde(rename = "Months")]
    Months,

    /// Visits - Represents a count of patient visits or encounters
    ///
    /// Typically used for:
    /// - Maximum allowed visits per time period
    /// - Visit-based benefit limitations
    /// - Authorization or referral visit counts
    ///
    /// X12 HIPAA Code: VS (Visits)
    /// Stedi Note: Used specifically for counting patient-provider encounters or service visits
    #[serde(rename = "Visits")]
    Visits,

    /// Visit - Singular form of `Visits` as returned by some payers.
    ///
    /// Non-standard variant accepted for compatibility with payers that emit the singular
    /// "Visit" rather than the X12-standard "Visits".
    #[serde(rename = "Visit")]
    Visit,

    /// Week - Represents a time period measured in weeks
    ///
    /// Typically used for:
    /// - Weekly service or visit limits
    /// - Short-term treatment period restrictions
    /// - Frequency limitations (e.g., services per week)
    ///
    /// X12 HIPAA Code: WK (Week)
    /// Stedi Note: Singular form "Week" is used rather than "Weeks" in the X12 standard
    #[serde(rename = "Week")]
    Week,

    /// Years - Represents a time period measured in years
    ///
    /// Typically used for:
    /// - Annual benefit maximums
    /// - Yearly visit or service limits
    /// - Age eligibility measured in years
    /// - Long-term coverage durations
    ///
    /// X12 HIPAA Code: YR (Years)
    /// Stedi Note: Used for annual benefit periods and long-term coverage specifications
    #[serde(rename = "Years")]
    Years,

    /// Catch-all for non-standard or unrecognized unit values returned by payers.
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for UnitForMeasurement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Days => write!(f, "Days"),
            Self::Months => write!(f, "Months"),
            Self::Visits => write!(f, "Visits"),
            Self::Visit => write!(f, "Visit"),
            Self::Week => write!(f, "Week"),
            Self::Years => write!(f, "Years"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
