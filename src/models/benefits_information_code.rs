use serde::{Deserialize, Serialize};

/// Benefits information codes indicating the type of benefits information.
///
/// These codes are used to categorize different types of benefit information such as
/// coverage status, co-payments, deductibles, and other cost-sharing information.
///
/// Visit [Eligibility and benefit codes](https://www.stedi.com/docs/healthcare/eligibility-active-coverage-benefits#benefit-type-codes)
/// for more information.
///
/// Note: Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum BenefitsInformationCode {
    /// Active Coverage
    #[serde(rename = "1")]
    #[default]
    ActiveCoverage,

    /// Active - Full Risk Capitation
    #[serde(rename = "2")]
    ActiveFullRiskCapitation,

    /// Active - Services Capitated
    #[serde(rename = "3")]
    ActiveServicesCapitated,

    /// Active - Services Capitated to Primary Care Physician
    #[serde(rename = "4")]
    ActiveServicesCapitatedToPcp,

    /// Active - Pending Investigation
    #[serde(rename = "5")]
    ActivePendingInvestigation,

    /// Inactive
    #[serde(rename = "6")]
    Inactive,

    /// Inactive - Pending Eligibility Update
    #[serde(rename = "7")]
    InactivePendingEligibilityUpdate,

    /// Inactive - Pending Investigation
    #[serde(rename = "8")]
    InactivePendingInvestigation,

    /// Co-Insurance
    #[serde(rename = "A")]
    CoInsurance,

    /// Co-Payment
    #[serde(rename = "B")]
    CoPayment,

    /// Deductible
    #[serde(rename = "C")]
    Deductible,

    /// Coverage Basis
    #[serde(rename = "CB")]
    CoverageBasis,

    /// Benefit Description
    #[serde(rename = "D")]
    BenefitDescription,

    /// Exclusions
    #[serde(rename = "E")]
    Exclusions,

    /// Limitations
    #[serde(rename = "F")]
    Limitations,

    /// Out of Pocket (Stop Loss)
    #[serde(rename = "G")]
    OutOfPocketStopLoss,

    /// Unlimited
    #[serde(rename = "H")]
    Unlimited,

    /// Non-Covered
    #[serde(rename = "I")]
    NonCovered,

    /// Cost Containment
    #[serde(rename = "J")]
    CostContainment,

    /// Reserve
    #[serde(rename = "K")]
    Reserve,

    /// Primary Care Provider
    #[serde(rename = "L")]
    PrimaryCareProvider,

    /// Managed Care Coordinator
    #[serde(rename = "M")]
    ManagedCareCoordinator,

    /// Managed Care Organization
    #[serde(rename = "MC")]
    ManagedCareOrganization,

    /// Not Deemed a Medical Necessity
    #[serde(rename = "N")]
    NotDeemedMedicalNecessity,

    /// Other or Additional Payor
    #[serde(rename = "O")]
    OtherOrAdditionalPayor,

    /// Prior Year(s) History
    #[serde(rename = "P")]
    PriorYearsHistory,

    /// Other Source of Data
    #[serde(rename = "Q")]
    OtherSourceOfData,

    /// Restrictions
    #[serde(rename = "R")]
    Restrictions,

    /// Second Surgical Opinion Required
    #[serde(rename = "S")]
    SecondSurgicalOpinionRequired,

    /// Third Party Processing
    #[serde(rename = "T")]
    ThirdPartyProcessing,

    /// Unknown
    #[serde(rename = "U")]
    Unknown,

    /// Plan Waiting Period
    #[serde(rename = "V")]
    PlanWaitingPeriod,

    /// Notification Required
    #[serde(rename = "W")]
    NotificationRequired,

    /// Preadmission Certification
    #[serde(rename = "X")]
    PreadmissionCertification,

    /// Spend Down
    #[serde(rename = "Y")]
    SpendDown,
}

impl std::fmt::Display for BenefitsInformationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ActiveCoverage => write!(f, "1"),
            Self::ActiveFullRiskCapitation => write!(f, "2"),
            Self::ActiveServicesCapitated => write!(f, "3"),
            Self::ActiveServicesCapitatedToPcp => write!(f, "4"),
            Self::ActivePendingInvestigation => write!(f, "5"),
            Self::Inactive => write!(f, "6"),
            Self::InactivePendingEligibilityUpdate => write!(f, "7"),
            Self::InactivePendingInvestigation => write!(f, "8"),
            Self::CoInsurance => write!(f, "A"),
            Self::CoPayment => write!(f, "B"),
            Self::Deductible => write!(f, "C"),
            Self::CoverageBasis => write!(f, "CB"),
            Self::BenefitDescription => write!(f, "D"),
            Self::Exclusions => write!(f, "E"),
            Self::Limitations => write!(f, "F"),
            Self::OutOfPocketStopLoss => write!(f, "G"),
            Self::Unlimited => write!(f, "H"),
            Self::NonCovered => write!(f, "I"),
            Self::CostContainment => write!(f, "J"),
            Self::Reserve => write!(f, "K"),
            Self::PrimaryCareProvider => write!(f, "L"),
            Self::ManagedCareCoordinator => write!(f, "M"),
            Self::ManagedCareOrganization => write!(f, "MC"),
            Self::NotDeemedMedicalNecessity => write!(f, "N"),
            Self::OtherOrAdditionalPayor => write!(f, "O"),
            Self::PriorYearsHistory => write!(f, "P"),
            Self::OtherSourceOfData => write!(f, "Q"),
            Self::Restrictions => write!(f, "R"),
            Self::SecondSurgicalOpinionRequired => write!(f, "S"),
            Self::ThirdPartyProcessing => write!(f, "T"),
            Self::Unknown => write!(f, "U"),
            Self::PlanWaitingPeriod => write!(f, "V"),
            Self::NotificationRequired => write!(f, "W"),
            Self::PreadmissionCertification => write!(f, "X"),
            Self::SpendDown => write!(f, "Y"),
        }
    }
}
