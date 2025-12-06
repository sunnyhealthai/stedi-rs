use serde::{Deserialize, Serialize};

/// Employment status codes indicating the employment status of the subscriber or dependent
/// as it relates to military service.
///
/// These codes are returned in the `subscriber.employmentStatusCode` and
/// `dependents.employmentStatusCode` properties.
///
/// Note: Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EmploymentStatusCode {
    /// Active Reserve
    #[serde(rename = "AE")]
    #[default]
    ActiveReserve,

    /// Active Military - Overseas
    #[serde(rename = "AO")]
    ActiveMilitaryOverseas,

    /// Academy Student
    #[serde(rename = "AS")]
    AcademyStudent,

    /// Presidential Appointee
    #[serde(rename = "AT")]
    PresidentialAppointee,

    /// Active Military - USA
    #[serde(rename = "AU")]
    ActiveMilitaryUsa,

    /// Contractor
    #[serde(rename = "CC")]
    Contractor,

    /// Dishonorably Discharged
    #[serde(rename = "DD")]
    DishonorablyDischarged,

    /// Honorably Discharged
    #[serde(rename = "HD")]
    HonorablyDischarged,

    /// Inactive Reserves
    #[serde(rename = "IR")]
    InactiveReserves,

    /// Leave of Absence: Military
    #[serde(rename = "LX")]
    LeaveOfAbsenceMilitary,

    /// Plan to Enlist
    #[serde(rename = "PE")]
    PlanToEnlist,

    /// Recommissioned
    #[serde(rename = "RE")]
    Recommissioned,

    /// Retired Military - Overseas
    #[serde(rename = "RM")]
    RetiredMilitaryOverseas,

    /// Retired Without Recall
    #[serde(rename = "RR")]
    RetiredWithoutRecall,

    /// Retired Military - USA
    #[serde(rename = "RU")]
    RetiredMilitaryUsa,
}

impl std::fmt::Display for EmploymentStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ActiveReserve => write!(f, "AE"),
            Self::ActiveMilitaryOverseas => write!(f, "AO"),
            Self::AcademyStudent => write!(f, "AS"),
            Self::PresidentialAppointee => write!(f, "AT"),
            Self::ActiveMilitaryUsa => write!(f, "AU"),
            Self::Contractor => write!(f, "CC"),
            Self::DishonorablyDischarged => write!(f, "DD"),
            Self::HonorablyDischarged => write!(f, "HD"),
            Self::InactiveReserves => write!(f, "IR"),
            Self::LeaveOfAbsenceMilitary => write!(f, "LX"),
            Self::PlanToEnlist => write!(f, "PE"),
            Self::Recommissioned => write!(f, "RE"),
            Self::RetiredMilitaryOverseas => write!(f, "RM"),
            Self::RetiredWithoutRecall => write!(f, "RR"),
            Self::RetiredMilitaryUsa => write!(f, "RU"),
        }
    }
}
