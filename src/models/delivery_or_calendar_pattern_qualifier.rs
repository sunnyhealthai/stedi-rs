use serde::{Deserialize, Serialize};

/// Delivery or calendar pattern qualifier for benefits service delivery.
///
/// Specifies the routine shipments, deliveries, or calendar pattern for healthcare services
/// or benefits. This enum is used in `benefitsInformation.benefitsServiceDelivery` to indicate
/// when services can be delivered or accessed, such as specific days of the week, weeks of the
/// month, or special delivery patterns.
///
/// These patterns are commonly used for:
/// - **Pharmacy benefits**: Specifying when medications can be refilled or delivered
/// - **Home health services**: Indicating delivery schedules for medical equipment or supplies
/// - **Therapy services**: Defining when appointments can be scheduled
/// - **Durable medical equipment**: Specifying delivery windows
///
/// The pattern qualifier works together with other fields in `BenefitsServiceDelivery` such as
/// `quantity`, `timePeriodQualifier`, and `numOfPeriods` to fully describe service delivery
/// frequency and timing.
///
/// ## Common Patterns
///
/// - **Weekly patterns**: `MondayThroughFriday`, `Monday`, `Tuesday`, etc.
/// - **Monthly patterns**: `FirstWeekOfMonth`, `SecondWeekOfMonth`, etc.
/// - **Working day patterns**: `FirstWorkingDayOfPeriod`, `LastWorkingDayOfPeriod`
/// - **Split deliveries**: `HalfMondayHalfTuesday`, `ThirdMondayThirdWednesdayThirdFriday`
/// - **Special patterns**: `Immediately`, `AsDirected`, `WheneverNecessary`
///
/// Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#delivery-frequency-codes)
/// for the complete list of delivery frequency codes. Payers may sometimes return other
/// non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DeliveryOrCalendarPatternQualifier {
    #[serde(rename = "1st Week of the Month")]
    #[default]
    FirstWeekOfMonth,
    #[serde(rename = "2nd Week of the Month")]
    SecondWeekOfMonth,
    #[serde(rename = "3rd Week of the Month")]
    ThirdWeekOfMonth,
    #[serde(rename = "4th Week of the Month")]
    FourthWeekOfMonth,
    #[serde(rename = "5th Week of the Month")]
    FifthWeekOfMonth,
    #[serde(rename = "1st & 3rd Week of the Month")]
    FirstAndThirdWeekOfMonth,
    #[serde(rename = "2nd & 4th Week of the Month")]
    SecondAndFourthWeekOfMonth,
    #[serde(rename = "1st Working Day of Period")]
    FirstWorkingDayOfPeriod,
    #[serde(rename = "Last Working Day of Period")]
    LastWorkingDayOfPeriod,
    #[serde(rename = "Monday through Friday")]
    MondayThroughFriday,
    #[serde(rename = "Monday through Saturday")]
    MondayThroughSaturday,
    #[serde(rename = "Monday through Sunday")]
    MondayThroughSunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday through Thursday")]
    MondayThroughThursday,
    #[serde(rename = "Immediately")]
    Immediately,
    #[serde(rename = "As Directed")]
    AsDirected,
    #[serde(rename = "Daily Mon. Through Fri.")]
    DailyMondayThroughFriday,
    #[serde(rename = "1/2 Mon. & 1/2 Tues.")]
    HalfMondayHalfTuesday,
    #[serde(rename = "1/2 Tues. & 1/2 Thurs.")]
    HalfTuesdayHalfThursday,
    #[serde(rename = "1/2 Wed. & 1/2 Fri.")]
    HalfWednesdayHalfFriday,
    #[serde(rename = "Once Anytime Mon. through Fri.")]
    OnceAnytimeMondayThroughFriday,
    #[serde(rename = "Tuesday through Friday")]
    TuesdayThroughFriday,
    #[serde(rename = "Monday, Tuesday and Thursday")]
    MondayTuesdayAndThursday,
    #[serde(rename = "Monday, Tuesday and Friday")]
    MondayTuesdayAndFriday,
    #[serde(rename = "Wednesday and Thursday")]
    WednesdayAndThursday,
    #[serde(rename = "Monday, Wednesday and Thursday")]
    MondayWednesdayAndThursday,
    #[serde(rename = "Tuesday, Thursday and Friday")]
    TuesdayThursdayAndFriday,
    #[serde(rename = "1/2 Tues. & 1/2 Fri.")]
    HalfTuesdayHalfFriday,
    #[serde(rename = "1/2 Mon. & 1/2 Wed.")]
    HalfMondayHalfWednesday,
    #[serde(rename = "1/3 Mon., 1/3 Wed., 1/3 Fri.")]
    ThirdMondayThirdWednesdayThirdFriday,
    #[serde(rename = "Whenever Necessary")]
    WheneverNecessary,
    #[serde(rename = "1/2 By Wed. Bal. By Fri.")]
    HalfByWednesdayBalanceByFriday,
    #[serde(rename = "None (Also Used to Cancel or Override a Previous Pattern)")]
    NoneOrCancelOverride,
}

impl std::fmt::Display for DeliveryOrCalendarPatternQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FirstWeekOfMonth => write!(f, "1st Week of the Month"),
            Self::SecondWeekOfMonth => write!(f, "2nd Week of the Month"),
            Self::ThirdWeekOfMonth => write!(f, "3rd Week of the Month"),
            Self::FourthWeekOfMonth => write!(f, "4th Week of the Month"),
            Self::FifthWeekOfMonth => write!(f, "5th Week of the Month"),
            Self::FirstAndThirdWeekOfMonth => write!(f, "1st & 3rd Week of the Month"),
            Self::SecondAndFourthWeekOfMonth => write!(f, "2nd & 4th Week of the Month"),
            Self::FirstWorkingDayOfPeriod => write!(f, "1st Working Day of Period"),
            Self::LastWorkingDayOfPeriod => write!(f, "Last Working Day of Period"),
            Self::MondayThroughFriday => write!(f, "Monday through Friday"),
            Self::MondayThroughSaturday => write!(f, "Monday through Saturday"),
            Self::MondayThroughSunday => write!(f, "Monday through Sunday"),
            Self::Monday => write!(f, "Monday"),
            Self::Tuesday => write!(f, "Tuesday"),
            Self::Wednesday => write!(f, "Wednesday"),
            Self::Thursday => write!(f, "Thursday"),
            Self::Friday => write!(f, "Friday"),
            Self::Saturday => write!(f, "Saturday"),
            Self::Sunday => write!(f, "Sunday"),
            Self::MondayThroughThursday => write!(f, "Monday through Thursday"),
            Self::Immediately => write!(f, "Immediately"),
            Self::AsDirected => write!(f, "As Directed"),
            Self::DailyMondayThroughFriday => write!(f, "Daily Mon. Through Fri."),
            Self::HalfMondayHalfTuesday => {
                write!(f, "1/2 Mon. & 1/2 Tues.")
            }
            Self::HalfTuesdayHalfThursday => {
                write!(f, "1/2 Tues. & 1/2 Thurs.")
            }
            Self::HalfWednesdayHalfFriday => {
                write!(f, "1/2 Wed. & 1/2 Fri.")
            }
            Self::OnceAnytimeMondayThroughFriday => {
                write!(f, "Once Anytime Mon. through Fri.")
            }
            Self::TuesdayThroughFriday => write!(f, "Tuesday through Friday"),
            Self::MondayTuesdayAndThursday => write!(f, "Monday, Tuesday and Thursday"),
            Self::MondayTuesdayAndFriday => write!(f, "Monday, Tuesday and Friday"),
            Self::WednesdayAndThursday => write!(f, "Wednesday and Thursday"),
            Self::MondayWednesdayAndThursday => write!(f, "Monday, Wednesday and Thursday"),
            Self::TuesdayThursdayAndFriday => write!(f, "Tuesday, Thursday and Friday"),
            Self::HalfTuesdayHalfFriday => {
                write!(f, "1/2 Tues. & 1/2 Fri.")
            }
            Self::HalfMondayHalfWednesday => {
                write!(f, "1/2 Mon. & 1/2 Wed.")
            }
            Self::ThirdMondayThirdWednesdayThirdFriday => {
                write!(f, "1/3 Mon., 1/3 Wed., 1/3 Fri.")
            }
            Self::WheneverNecessary => write!(f, "Whenever Necessary"),
            Self::HalfByWednesdayBalanceByFriday => {
                write!(f, "1/2 By Wed. Bal. By Fri.")
            }
            Self::NoneOrCancelOverride => {
                write!(
                    f,
                    "None (Also Used to Cancel or Override a Previous Pattern)"
                )
            }
        }
    }
}
