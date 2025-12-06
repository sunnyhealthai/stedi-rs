use serde::{Deserialize, Serialize};

/// UnitForMeasurementCode : Code specifying the unit of measurement for the quantity.
///
/// These codes specify the unit of measurement used for benefit quantities and limits in
/// eligibility check responses. They help interpret benefit amounts by indicating whether
/// quantities are measured in days, months, visits, weeks, or years.
///
/// This field is typically used in conjunction with benefit quantity information to specify
/// how benefit limits are measured. For example, a benefit might have a limit of "30" with
/// unit "DA" (Days), meaning 30 days of coverage.
///
/// These codes help determine:
/// - How benefit limits are measured (e.g., 30 days, 12 visits per year)
/// - The time period or frequency for benefit calculations
/// - How to interpret quantity values in benefit information
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum UnitForMeasurementCode {
    /// Days - Unit of measurement is days
    #[serde(rename = "DA")]
    #[default]
    Da,
    /// Months - Unit of measurement is months
    #[serde(rename = "MO")]
    Mo,
    /// Visits - Unit of measurement is visits
    #[serde(rename = "VS")]
    Vs,
    /// Week - Unit of measurement is weeks
    #[serde(rename = "WK")]
    Wk,
    /// Years - Unit of measurement is years
    #[serde(rename = "YR")]
    Yr,
}

impl std::fmt::Display for UnitForMeasurementCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Da => write!(f, "DA"),
            Self::Mo => write!(f, "MO"),
            Self::Vs => write!(f, "VS"),
            Self::Wk => write!(f, "WK"),
            Self::Yr => write!(f, "YR"),
        }
    }
}
