use serde::{Deserialize, Serialize};

/// Date/Time Period Format Qualifier
///
/// This enum defines the format codes used in X12 HIPAA transactions to specify how dates and
/// date ranges are represented in DTP (Date or Time or Period) segments. These qualifiers are
/// essential for interpreting date values correctly in healthcare eligibility, claims, and
/// other HIPAA transactions.
///
/// The DTP segment is commonly used in healthcare transactions like:
/// - 270/271 (Eligibility Inquiry/Response)
/// - 837 (Healthcare Claims)
/// - 835 (Remittance Advice)
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DateTimePeriodFormatQualifier {
    /// Date expressed in CCYYMMDD format (8 digits)
    ///
    /// Format: CCYYMMDD where:
    /// - CC = Century
    /// - YY = Year
    /// - MM = Month
    /// - DD = Day
    ///
    /// Example: 20240115 for January 15, 2024
    ///
    /// Used in DTP segments when a single date needs to be specified, such as:
    /// - Patient birth date
    /// - Service date
    /// - Eligibility inquiry date
    /// - Plan effective date
    /// - Date of injury/illness
    #[serde(rename = "D8")]
    #[default]
    SingleDate,
    /// Date range expressed in CCYYMMDD-CCYYMMDD format
    ///
    /// Format: CCYYMMDD-CCYYMMDD representing a start date and end date
    ///
    /// Example: 20240101-20241231 for a period from January 1, 2024 to December 31, 2024
    ///
    /// Used in DTP segments when a date range needs to be specified, such as:
    /// - Benefit period
    /// - Coverage effective dates
    /// - Service date ranges
    /// - Plan year periods
    ///
    /// Note: When only a start date is known, the end date may be represented as:
    /// - 99999999 (future date)
    /// - 20991231 (common default for open-ended coverage)
    #[serde(rename = "RD8")]
    DateRange,
}

impl std::fmt::Display for DateTimePeriodFormatQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SingleDate => write!(f, "D8"),
            Self::DateRange => write!(f, "RD8"),
        }
    }
}
