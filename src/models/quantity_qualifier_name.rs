use serde::{Deserialize, Serialize};

/// Human-readable name for quantity qualifier codes used in benefit service delivery information.
///
/// This enum provides descriptive names for quantity qualifiers that specify the type of
/// quantity measurement used in benefit service delivery information. Quantity qualifiers
/// help interpret benefit limits, coverage amounts, and service delivery restrictions by
/// indicating what the quantity value represents (e.g., visits, days, hours, maximum
/// amounts, etc.).
///
/// The corresponding code version (`QuantityQualifierCode`) contains standardized codes
/// for programmatic use, while this enum provides human-readable names for display and
/// understanding.
///
/// ## Quantity Qualifier Categories
///
/// Quantity qualifiers can be grouped into several categories:
///
/// ### Limits and Ranges
/// - **Minimum**: Minimum quantity allowed or required
/// - **Maximum**: Maximum quantity allowed or covered
/// - **Age, High Value**: Upper age limit for age-based benefits
/// - **Age, Low Value**: Lower age limit for age-based benefits
///
/// ### Time-Based Units
/// - **Days**: Number of days (e.g., days of coverage, days remaining)
/// - **Hours**: Number of hours (e.g., hours of service, hours covered)
/// - **Month**: Number of months (e.g., monthly benefit periods)
/// - **Years**: Number of years (e.g., annual limits, years of coverage)
/// - **Visits**: Number of visits (e.g., office visits, therapy visits)
///
/// ### Coverage and Usage
/// - **Covered - Actual**: Actual quantity covered or used
/// - **Covered - Estimated**: Estimated quantity to be covered
/// - **Quantity Used**: Quantity already consumed or used
/// - **Quantity Approved**: Quantity approved for coverage
///
/// ### Lifetime Reserves
/// - **Life-time Reserve - Actual**: Actual lifetime reserve amount used or remaining
/// - **Life-time Reserve - Estimated**: Estimated lifetime reserve amount
///
/// ### Special Quantities
/// - **Number of Co-insurance Days**: Days subject to co-insurance requirements
/// - **Deductible Blood Units**: Blood units applied to deductible
/// - **Number of Services or Procedures**: Count of services or procedures
///
/// ## Usage Context
///
/// Quantity qualifiers are used in benefit service delivery information to:
///
/// - **Interpret benefit limits**: Understand what type of quantity a limit represents
///   (e.g., maximum visits per year, maximum days of coverage)
/// - **Track usage**: Monitor quantities used, covered, or approved
/// - **Calculate remaining benefits**: Determine how much benefit remains based on
///   quantities used vs. maximums allowed
/// - **Apply restrictions**: Understand quantity-based restrictions on benefits
/// - **Lifetime reserves**: Track lifetime reserve amounts for certain benefit types
///
/// ## Common Usage Patterns
///
/// - **Annual limits**: Often use `Maximum` with `Visits`, `Days`, or other time-based units
/// - **Coverage tracking**: Use `Covered - Actual` or `Quantity Used` to track consumed
///   benefits
/// - **Lifetime benefits**: Use `Life-time Reserve` qualifiers for lifetime-limited benefits
/// - **Age restrictions**: Use `Age, High Value` and `Age, Low Value` for age-based
///   benefit eligibility
///
/// ## Examples
///
/// - A benefit with "Maximum: 20 Visits" would use `Maximum` and `Visits`
/// - A benefit showing "Days Remaining: 10" would use `Days` with the remaining quantity
/// - A lifetime benefit showing "Life-time Reserve - Actual: 50 Days" would use
///   `LifeTimeReserveActual` and `Days`
/// - An age-restricted benefit might use `Age, Low Value` and `Age, High Value` to
///   specify age ranges
///
/// ## X12 HIPAA
///
/// Maps to quantity qualifier elements in X12 271 benefit service delivery segments,
/// including QTY segments that specify quantity types and measurements for benefits.
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum QuantityQualifierName {
    /// Minimum - Minimum quantity allowed or required for a benefit.
    ///
    /// Used to specify the minimum quantity that must be met or is allowed for a benefit.
    /// Often used in conjunction with maximum limits to define benefit ranges.
    #[serde(rename = "Minimum")]
    #[default]
    Minimum,
    /// Quantity Used - Quantity already consumed or used from a benefit.
    ///
    /// Indicates the amount of a benefit that has already been used or consumed. Used to
    /// track benefit utilization and calculate remaining benefits.
    #[serde(rename = "Quantity Used")]
    QuantityUsed,
    /// Covered - Actual - Actual quantity covered or used for a benefit.
    ///
    /// Represents the actual quantity that has been covered or used, as opposed to an
    /// estimated amount. Used for tracking actual benefit consumption.
    #[serde(rename = "Covered - Actual")]
    CoveredActual,
    /// Covered - Estimated - Estimated quantity to be covered for a benefit.
    ///
    /// Represents an estimated quantity that will be covered, as opposed to an actual
    /// amount. Used for projected benefit coverage calculations.
    #[serde(rename = "Covered - Estimated")]
    CoveredEstimated,
    /// Number of Co-insurance Days - Days subject to co-insurance requirements.
    ///
    /// Specifies the number of days that are subject to co-insurance payment requirements,
    /// typically used for inpatient or extended care benefits.
    #[serde(rename = "Number of Co-insurance Days")]
    NumberOfCoInsuranceDays,
    /// Deductible Blood Units - Blood units applied to the deductible.
    ///
    /// Used for blood-related benefits to specify the number of blood units that count
    /// toward the deductible amount.
    #[serde(rename = "Deductible Blood Units")]
    DeductibleBloodUnits,
    /// Days - Number of days for time-based benefit measurements.
    ///
    /// Used to specify quantities measured in days, such as days of coverage, days
    /// remaining, days per benefit period, or days until next benefit availability.
    #[serde(rename = "Days")]
    Days,
    /// Hours - Number of hours for time-based benefit measurements.
    ///
    /// Used to specify quantities measured in hours, such as hours of service coverage,
    /// hours of therapy, or hours per benefit period.
    #[serde(rename = "Hours")]
    Hours,
    /// Life-time Reserve - Actual - Actual lifetime reserve amount used or remaining.
    ///
    /// Represents the actual amount of a lifetime-limited benefit that has been used or
    /// remains available. Used for benefits with lifetime maximums such as mental health
    /// days or skilled nursing facility days.
    #[serde(rename = "Life-time Reserve - Actual")]
    LifeTimeReserveActual,
    /// Life-time Reserve - Estimated - Estimated lifetime reserve amount.
    ///
    /// Represents an estimated amount of a lifetime-limited benefit. Used for projected
    /// lifetime reserve calculations.
    #[serde(rename = "Life-time Reserve - Estimated")]
    LifeTimeReserveEstimated,
    /// Maximum - Maximum quantity allowed or covered for a benefit.
    ///
    /// Used to specify the maximum quantity allowed or covered for a benefit. Often used
    /// with time-based units (e.g., "Maximum: 20 Visits per year") to define benefit limits.
    #[serde(rename = "Maximum")]
    Maximum,
    /// Month - Number of months for time-based benefit measurements.
    ///
    /// Used to specify quantities measured in months, such as monthly benefit periods,
    /// months of coverage, or months until benefit renewal.
    #[serde(rename = "Month")]
    Month,
    /// Number of Services or Procedures - Count of services or procedures.
    ///
    /// Used to specify the number of services or procedures covered, used, or allowed.
    /// Common for procedure-based benefits or service count limits.
    #[serde(rename = "Number of Services or Procedures")]
    NumberOfServicesOrProcedures,
    /// Quantity Approved - Quantity approved for coverage.
    ///
    /// Indicates the quantity that has been approved for coverage, typically used in
    /// authorization or pre-approval contexts.
    #[serde(rename = "Quantity Approved")]
    QuantityApproved,
    /// Age, High Value - Upper age limit for age-based benefit eligibility.
    ///
    /// Specifies the upper age limit for benefits that have age-based eligibility
    /// requirements. Used with `Age, Low Value` to define age ranges.
    #[serde(rename = "Age, High Value")]
    AgeCommaHighValue,
    /// Age, Low Value - Lower age limit for age-based benefit eligibility.
    ///
    /// Specifies the lower age limit for benefits that have age-based eligibility
    /// requirements. Used with `Age, High Value` to define age ranges.
    #[serde(rename = "Age, Low Value")]
    AgeCommaLowValue,
    /// Visits - Number of visits for service-based benefit measurements.
    ///
    /// Used to specify quantities measured in visits, such as office visits, therapy
    /// visits, specialist visits, or other visit-based benefits. Common for annual
    /// visit limits.
    #[serde(rename = "Visits")]
    Visits,
    /// Visit - Singular form of `Visits` as returned by some payers.
    ///
    /// Non-standard variant accepted for compatibility with payers that emit the singular
    /// "Visit" rather than the X12-standard "Visits".
    #[serde(rename = "Visit")]
    Visit,
    /// Years - Number of years for time-based benefit measurements.
    ///
    /// Used to specify quantities measured in years, such as annual limits, years of
    /// coverage, or years until benefit renewal.
    #[serde(rename = "Years")]
    Years,

    /// Catch-all for non-standard or unrecognized qualifier names returned by payers.
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for QuantityQualifierName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Minimum => write!(f, "Minimum"),
            Self::QuantityUsed => write!(f, "Quantity Used"),
            Self::CoveredActual => write!(f, "Covered - Actual"),
            Self::CoveredEstimated => write!(f, "Covered - Estimated"),
            Self::NumberOfCoInsuranceDays => write!(f, "Number of Co-insurance Days"),
            Self::DeductibleBloodUnits => write!(f, "Deductible Blood Units"),
            Self::Days => write!(f, "Days"),
            Self::Hours => write!(f, "Hours"),
            Self::LifeTimeReserveActual => write!(f, "Life-time Reserve - Actual"),
            Self::LifeTimeReserveEstimated => write!(f, "Life-time Reserve - Estimated"),
            Self::Maximum => write!(f, "Maximum"),
            Self::Month => write!(f, "Month"),
            Self::NumberOfServicesOrProcedures => write!(f, "Number of Services or Procedures"),
            Self::QuantityApproved => write!(f, "Quantity Approved"),
            Self::AgeCommaHighValue => write!(f, "Age, High Value"),
            Self::AgeCommaLowValue => write!(f, "Age, Low Value"),
            Self::Visits => write!(f, "Visits"),
            Self::Visit => write!(f, "Visit"),
            Self::Years => write!(f, "Years"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
