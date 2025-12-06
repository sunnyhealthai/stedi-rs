use serde::{Deserialize, Serialize};

/// Delivery pattern time qualifier for benefits service delivery.
///
/// Specifies the time of day or shift for routine shipments or deliveries of healthcare services
/// or benefits. This enum is used in `benefitsInformation.benefitsServiceDelivery` to indicate
/// when during the day services can be delivered or accessed, complementing the calendar pattern
/// (which specifies days/weeks) with time-of-day information.
///
/// These time qualifiers are commonly used for:
/// - **Pharmacy benefits**: Specifying when medications can be delivered (e.g., during normal
///   business hours, any shift for 24/7 pharmacies)
/// - **Home health services**: Indicating delivery time windows for medical equipment or supplies
/// - **Durable medical equipment**: Specifying when equipment deliveries can be scheduled
/// - **Therapy services**: Defining appointment time availability
///
/// The time qualifier works together with `deliveryOrCalendarPatternQualifier` (which specifies
/// days/weeks) and other fields in `BenefitsServiceDelivery` to fully describe service delivery
/// scheduling requirements.
///
/// ## Common Time Patterns
///
/// - **Shift-based**: `FirstShiftNormalWorkingHours`, `SecondShift`, `ThirdShift`, `AnyShift`
/// - **Time-of-day**: `AM` (morning), `PM` (afternoon/evening)
/// - **Flexible**: `AsDirected` (provider determines timing)
/// - **Override**: `NoneOrCancelOverride` (cancel or override previous pattern)
///
/// Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#delivery-pattern-time-codes)
/// for the complete list of delivery pattern time codes. Payers may sometimes return other
/// non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DeliveryPatternTimeQualifier {
    #[serde(rename = "1st Shift (Normal Working Hours)")]
    #[default]
    FirstShiftNormalWorkingHours,
    #[serde(rename = "2nd Shift")]
    SecondShift,
    #[serde(rename = "3rd Shift")]
    ThirdShift,
    #[serde(rename = "A.M.")]
    AM,
    #[serde(rename = "P.M.")]
    PM,
    #[serde(rename = "As Directed")]
    AsDirected,
    #[serde(rename = "Any Shift")]
    AnyShift,
    #[serde(rename = "None (Also Used to Cancel or Override a Previous Pattern)")]
    NoneOrCancelOverride,
}

impl std::fmt::Display for DeliveryPatternTimeQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FirstShiftNormalWorkingHours => {
                write!(f, "1st Shift (Normal Working Hours)")
            }
            Self::SecondShift => write!(f, "2nd Shift"),
            Self::ThirdShift => write!(f, "3rd Shift"),
            Self::AM => write!(f, "A.M."),
            Self::PM => write!(f, "P.M."),
            Self::AsDirected => write!(f, "As Directed"),
            Self::AnyShift => write!(f, "Any Shift"),
            Self::NoneOrCancelOverride => {
                write!(
                    f,
                    "None (Also Used to Cancel or Override a Previous Pattern)"
                )
            }
        }
    }
}
