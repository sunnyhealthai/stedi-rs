use serde::{Deserialize, Serialize};

/// Code that specifies the routine shipments, deliveries, or calendar pattern. For example `9` - Last Working Day of Period. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#delivery-frequency-codes) for a complete list.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum DeliveryOrCalendarPatternQualifierCode {
    /// 1st Week of the Month
    #[serde(rename = "1")]
    #[default]
    FirstWeekOfMonth,
    /// 2nd Week of the Month
    #[serde(rename = "2")]
    SecondWeekOfMonth,
    /// 3rd Week of the Month
    #[serde(rename = "3")]
    ThirdWeekOfMonth,
    /// 4th Week of the Month
    #[serde(rename = "4")]
    FourthWeekOfMonth,
    /// 5th Week of the Month
    #[serde(rename = "5")]
    FifthWeekOfMonth,
    /// 1st & 3rd Weeks of the Month
    #[serde(rename = "6")]
    FirstAndThirdWeeksOfMonth,
    /// 2nd & 4th Weeks of the Month
    #[serde(rename = "7")]
    SecondAndFourthWeeksOfMonth,
    /// 1st Working Day of the Period
    #[serde(rename = "8")]
    FirstWorkingDayOfPeriod,
    /// Last Working Day of the Period
    #[serde(rename = "9")]
    LastWorkingDayOfPeriod,
    /// Monday through Friday
    #[serde(rename = "A")]
    MondayThroughFriday,
    /// Monday through Saturday
    #[serde(rename = "B")]
    MondayThroughSaturday,
    /// Monday through Sunday
    #[serde(rename = "C")]
    MondayThroughSunday,
    /// Monday
    #[serde(rename = "D")]
    Monday,
    /// Tuesday
    #[serde(rename = "E")]
    Tuesday,
    /// Wednesday
    #[serde(rename = "F")]
    Wednesday,
    /// Thursday
    #[serde(rename = "G")]
    Thursday,
    /// Friday
    #[serde(rename = "H")]
    Friday,
    /// Saturday
    #[serde(rename = "J")]
    Saturday,
    /// Sunday
    #[serde(rename = "K")]
    Sunday,
    /// Monday through Thursday
    #[serde(rename = "L")]
    MondayThroughThursday,
    /// Immediate Delivery
    #[serde(rename = "M")]
    ImmediateDelivery,
    /// Delivery As Directed
    #[serde(rename = "N")]
    DeliveryAsDirected,
    /// Daily Mon. through Fri.
    #[serde(rename = "O")]
    DailyMondayThroughFriday,
    /// 1/2 Mon. & 1/2 Thurs.
    #[serde(rename = "P")]
    HalfMondayHalfThursday,
    /// 1/2 Tues. & 1/2 Thurs.
    #[serde(rename = "Q")]
    HalfTuesdayHalfThursday,
    /// 1/2 Wed. & 1/2 Fri.
    #[serde(rename = "R")]
    HalfWednesdayHalfFriday,
    /// Once Anytime Monday through Friday
    #[serde(rename = "S")]
    OnceAnytimeMondayThroughFriday,
    /// Tuesday through Friday
    #[serde(rename = "T")]
    TuesdayThroughFriday,
    /// 1/2 Mon. & 1/2 Wed.
    #[serde(rename = "U")]
    HalfMondayHalfWednesday,
    /// 1/3 Mon., 1/3 Wed., 1/3 Fri.
    #[serde(rename = "V")]
    OneThirdMondayOneThirdWednesdayOneThirdFriday,
    /// Delivery Whenever Necessary
    #[serde(rename = "W")]
    WheneverNecessary,
    /// 1/2 Tues. & 1/2 Fri.
    #[serde(rename = "SG")]
    HalfTuesdayHalfFriday,
    /// Mon., Tues. & Thurs.
    #[serde(rename = "SL")]
    MondayTuesdayAndThursday,
    /// Mon., Tues. & Fri.
    #[serde(rename = "SP")]
    MondayTuesdayAndFriday,
    /// Wed. & Thurs.
    #[serde(rename = "SX")]
    WednesdayAndThursday,
    /// Mon., Wed. & Thurs.
    #[serde(rename = "SY")]
    MondayWednesdayAndThursday,
    /// Tues., Thurs. & Fri.
    #[serde(rename = "SZ")]
    TuesdayThursdayAndFriday,
}

impl std::fmt::Display for DeliveryOrCalendarPatternQualifierCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FirstWeekOfMonth => write!(f, "1"),
            Self::SecondWeekOfMonth => write!(f, "2"),
            Self::ThirdWeekOfMonth => write!(f, "3"),
            Self::FourthWeekOfMonth => write!(f, "4"),
            Self::FifthWeekOfMonth => write!(f, "5"),
            Self::FirstAndThirdWeeksOfMonth => write!(f, "6"),
            Self::SecondAndFourthWeeksOfMonth => write!(f, "7"),
            Self::FirstWorkingDayOfPeriod => write!(f, "8"),
            Self::LastWorkingDayOfPeriod => write!(f, "9"),
            Self::MondayThroughFriday => write!(f, "A"),
            Self::MondayThroughSaturday => write!(f, "B"),
            Self::MondayThroughSunday => write!(f, "C"),
            Self::Monday => write!(f, "D"),
            Self::Tuesday => write!(f, "E"),
            Self::Wednesday => write!(f, "F"),
            Self::Thursday => write!(f, "G"),
            Self::Friday => write!(f, "H"),
            Self::Saturday => write!(f, "J"),
            Self::Sunday => write!(f, "K"),
            Self::MondayThroughThursday => write!(f, "L"),
            Self::ImmediateDelivery => write!(f, "M"),
            Self::DeliveryAsDirected => write!(f, "N"),
            Self::DailyMondayThroughFriday => write!(f, "O"),
            Self::HalfMondayHalfThursday => write!(f, "P"),
            Self::HalfTuesdayHalfThursday => write!(f, "Q"),
            Self::HalfWednesdayHalfFriday => write!(f, "R"),
            Self::OnceAnytimeMondayThroughFriday => write!(f, "S"),
            Self::TuesdayThroughFriday => write!(f, "T"),
            Self::HalfMondayHalfWednesday => write!(f, "U"),
            Self::OneThirdMondayOneThirdWednesdayOneThirdFriday => write!(f, "V"),
            Self::WheneverNecessary => write!(f, "W"),
            Self::HalfTuesdayHalfFriday => write!(f, "SG"),
            Self::MondayTuesdayAndThursday => write!(f, "SL"),
            Self::MondayTuesdayAndFriday => write!(f, "SP"),
            Self::WednesdayAndThursday => write!(f, "SX"),
            Self::MondayWednesdayAndThursday => write!(f, "SY"),
            Self::TuesdayThursdayAndFriday => write!(f, "SZ"),
        }
    }
}
