use serde::{Deserialize, Serialize};

/// Code specifying the type of quantity for service delivery in healthcare eligibility benefits.
///
/// This enum provides the standardized X12 codes that correspond to the human-readable names
/// in BenefitsServiceDeliveryQuantityQualifier. These codes are used in HIPAA X12 270/271
/// transactions to specify how a benefit quantity should be measured or interpreted.
///
/// Payers may sometimes return other non-compliant values not listed in this enum.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum BenefitsServiceDeliveryQuantityQualifierCode {
    /// DY - Days
    ///
    /// Used to specify that the quantity is measured in days. Commonly used for:
    /// - Length of stay limitations
    /// - Duration of benefits coverage
    /// - Waiting periods
    /// - Benefit period calculations
    ///
    /// X12 HIPAA Note: This is the standard code for day-based quantity measurements in
    /// healthcare eligibility transactions.
    #[serde(rename = "DY")]
    #[default]
    Days,
    /// FL - Fluid ounces (or other volume unit)
    ///
    /// Used to specify that the quantity is measured in fluid ounces or similar volume units.
    /// Commonly used for:
    /// - Prescription medication quantities
    /// - Liquid medical supplies
    /// - Nutritional supplement benefits
    ///
    /// X12 HIPAA Note: Represents volume measurements, typically for pharmaceutical or
    /// medical supply benefits.
    #[serde(rename = "FL")]
    FluidOunces,
    /// HS - Hours
    ///
    /// Used to specify that the quantity is measured in hours. Commonly used for:
    /// - Therapy session durations
    /// - Home health care time limits
    /// - Skilled nursing coverage hours
    /// - Equipment rental periods
    ///
    /// X12 HIPAA Note: Standard code for hour-based service delivery quantities in
    /// healthcare eligibility responses.
    #[serde(rename = "HS")]
    Hours,
    /// MN - Minutes
    ///
    /// Used to specify that the quantity is measured in minutes. Commonly used for:
    /// - Procedure time limitations
    /// - Appointment duration benefits
    /// - Telehealth session lengths
    /// - Physical therapy time allocations
    ///
    /// X12 HIPAA Note: Used for minute-based quantity measurements, particularly for
    /// time-sensitive medical services.
    #[serde(rename = "MN")]
    Minutes,
    /// VS - Visits
    ///
    /// Used to specify that the quantity is measured in visits. Commonly used for:
    /// - Annual visit limitations
    /// - Physical therapy visit caps
    /// - Mental health counseling sessions
    /// - Specialist visit restrictions
    ///
    /// X12 HIPAA Note: Standard code for visit-based benefit quantities. This is one of
    /// the most frequently used qualifiers in healthcare eligibility transactions.
    #[serde(rename = "VS")]
    Visits,
}

impl std::fmt::Display for BenefitsServiceDeliveryQuantityQualifierCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Days => write!(f, "DY"),
            Self::FluidOunces => write!(f, "FL"),
            Self::Hours => write!(f, "HS"),
            Self::Minutes => write!(f, "MN"),
            Self::Visits => write!(f, "VS"),
        }
    }
}
