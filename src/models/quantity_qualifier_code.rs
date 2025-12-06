use serde::{Deserialize, Serialize};

/// Quantity qualifier codes providing information about the type of quantity for benefits.
///
/// These codes are returned in the `benefitsInformation.quantityQualifierCode` property
/// and provide more information about the type of quantity. Payers may sometimes return
/// other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum QuantityQualifierCode {
    /// Minimum quantity
    #[serde(rename = "8H")]
    #[default]
    Minimum,
    /// Quantity used
    #[serde(rename = "99")]
    QuantityUsed,
    /// Covered - actual quantity
    #[serde(rename = "CA")]
    CoveredActual,
    /// Covered - estimated quantity
    #[serde(rename = "CE")]
    CoveredEstimated,
    /// Number of co-insurance days
    #[serde(rename = "D3")]
    CoInsuranceDays,
    /// Deductible blood units
    #[serde(rename = "DB")]
    DeductibleBloodUnits,
    /// Days
    #[serde(rename = "DY")]
    Days,
    /// Hours
    #[serde(rename = "HS")]
    Hours,
    /// Life-time reserve - actual
    #[serde(rename = "LA")]
    LifetimeReserveActual,
    /// Life-time reserve - estimated
    #[serde(rename = "LE")]
    LifetimeReserveEstimated,
    /// Maximum quantity
    #[serde(rename = "M2")]
    Maximum,
    /// Month
    #[serde(rename = "MN")]
    Month,
    /// Number of services or procedures
    #[serde(rename = "P6")]
    ServicesOrProcedures,
    /// Quantity approved
    #[serde(rename = "QA")]
    QuantityApproved,
    /// Age, high value - use when a benefit is based on a maximum age for the patient
    #[serde(rename = "S7")]
    AgeHighValue,
    /// Age, low value - use when a benefit is based on a minimum age for the patient
    #[serde(rename = "S8")]
    AgeLowValue,
    /// Visits
    #[serde(rename = "VS")]
    Visits,
    /// Years
    #[serde(rename = "YY")]
    Years,
}

impl std::fmt::Display for QuantityQualifierCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Minimum => write!(f, "8H"),
            Self::QuantityUsed => write!(f, "99"),
            Self::CoveredActual => write!(f, "CA"),
            Self::CoveredEstimated => write!(f, "CE"),
            Self::CoInsuranceDays => write!(f, "D3"),
            Self::DeductibleBloodUnits => write!(f, "DB"),
            Self::Days => write!(f, "DY"),
            Self::Hours => write!(f, "HS"),
            Self::LifetimeReserveActual => write!(f, "LA"),
            Self::LifetimeReserveEstimated => write!(f, "LE"),
            Self::Maximum => write!(f, "M2"),
            Self::Month => write!(f, "MN"),
            Self::ServicesOrProcedures => write!(f, "P6"),
            Self::QuantityApproved => write!(f, "QA"),
            Self::AgeHighValue => write!(f, "S7"),
            Self::AgeLowValue => write!(f, "S8"),
            Self::Visits => write!(f, "VS"),
            Self::Years => write!(f, "YY"),
        }
    }
}
