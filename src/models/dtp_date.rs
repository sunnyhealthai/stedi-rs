use serde::{Deserialize, Serialize};

/// Represents a Date/Time Period (DTP) segment in X12 HIPAA transactions, specifically used in healthcare eligibility contexts.
///
/// The DTP segment is a fundamental component of HIPAA X12 transactions that conveys date and time information in a standardized format.
/// In healthcare eligibility transactions (such as 270/271), DTP segments are used to specify various temporal aspects of benefits,
/// coverage periods, service dates, and other time-sensitive information.
///
/// This struct supports three different date field configurations:
/// - Single date (`date`): Used when a specific point in time needs to be referenced
/// - Date range (`startDate` and `endDate`): Used when a period of time needs to be specified
/// - Either configuration can be used depending on the specific DTP segment qualifier and business requirement
///
/// Date ranges in eligibility checks are commonly used to:
/// - Specify coverage effective periods (when benefits begin and end)
/// - Define service date ranges for which eligibility is being requested
/// - Indicate benefit period dates (such as plan years or benefit years)
/// - Represent date of birth when exact time is needed
/// - Specify dates for prior authorization periods
///
/// Each field in this struct corresponds to a specific element in the X12 DTP segment:
/// - `date` maps to DTP03 when DTP02 qualifier indicates a single date format
/// - `startDate` maps to DTP03 when DTP02 qualifier indicates a date range start
/// - `endDate` maps to DTP03 when DTP02 qualifier indicates a date range end
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DtpDate {
    /// A single date.
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// The end date of a range.
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The beginning date of a range.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl DtpDate {
    /// Creates a new instance of `DtpDate` with all fields initialized to `None`.
    ///
    /// This constructor is used when building a DTP segment for HIPAA X12 transactions where
    /// date information needs to be specified. The returned instance can be populated with
    /// either a single date or a date range depending on the specific eligibility requirement
    /// being represented.
    ///
    /// The function returns a default `DtpDate` struct that serves as a clean starting point
    /// for constructing date/time period information in healthcare transactions.
    pub fn new() -> DtpDate {
        DtpDate {
            date: None,
            end_date: None,
            start_date: None,
        }
    }
}
