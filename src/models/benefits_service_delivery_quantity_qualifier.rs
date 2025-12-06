use serde::{Deserialize, Serialize};

/// Quantity qualifier codes used to specify the unit of measurement for benefit service delivery quantities.
///
/// These codes indicate what type of units are being measured when reporting benefit service delivery
/// information in healthcare eligibility responses. They help clarify whether a quantity represents
/// days, units, hours, months, or visits.
///
/// In X12 HIPAA transactions, these qualifiers are used in the 271 (Eligibility) transaction
/// within benefit service delivery information segments to provide context for numerical values.
/// Payers may sometimes return non-standard or custom qualifier values that are not part of
/// the official HIPAA code set.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum BenefitsServiceDeliveryQuantityQualifier {
    /// Represents a quantity measured in calendar days.
    ///
    /// Typically used to specify benefit periods, waiting periods, or duration limits
    /// for services that are measured on a daily basis. For example, a benefit that
    /// covers up to 30 days of skilled nursing facility care.
    ///
    /// X12 HIPAA Code: DA (Days)
    #[serde(rename = "Days")]
    #[default]
    Days,

    /// Represents a quantity measured in service units.
    ///
    /// Used for benefits where services are quantified in discrete units rather than
    /// time periods. This is common for procedures, medications, or equipment where
    /// each unit represents a specific service or item provided. For example, coverage
    /// for 12 physical therapy units per year.
    ///
    /// X12 HIPAA Code: UN (Units)
    #[serde(rename = "Units")]
    Units,

    /// Represents a quantity measured in hours.
    ///
    /// Typically used for services that are billed or limited by time duration,
    /// such as home health care visits, therapy sessions, or equipment rental periods.
    /// For example, a benefit that covers up to 24 hours of home health aide services.
    ///
    /// X12 HIPAA Code: HR (Hours)
    #[serde(rename = "Hours")]
    Hours,

    /// Represents a quantity measured in months.
    ///
    /// Used for benefit periods or limitations that span calendar months.
    /// Common in coverage descriptions for ongoing treatments or services
    /// that are measured over longer timeframes. For example, 6 months of
    /// prescription drug coverage following a procedure.
    ///
    /// X12 HIPAA Code: MO (Months)
    #[serde(rename = "Month")]
    Month,

    /// Represents a quantity measured in visit counts.
    ///
    /// Used to specify benefit limitations based on the number of visits
    /// or encounters with healthcare providers. This is typical for services
    /// like office visits, therapy sessions, or specialist consultations.
    /// For example, 10 chiropractic visits per calendar year.
    ///
    /// X12 HIPAA Code: VS (Visits)
    #[serde(rename = "Visits")]
    Visits,
}

impl std::fmt::Display for BenefitsServiceDeliveryQuantityQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Days => write!(f, "Days"),
            Self::Units => write!(f, "Units"),
            Self::Hours => write!(f, "Hours"),
            Self::Month => write!(f, "Month"),
            Self::Visits => write!(f, "Visits"),
        }
    }
}
