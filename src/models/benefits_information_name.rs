use serde::{Deserialize, Serialize};

/// Represents human-readable names for benefit information codes.
///
/// This enum provides variants for various types of benefit information that may be returned
/// by healthcare payers. The variants cover different categories including coverage status,
/// cost-sharing requirements, benefit details, and special situations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BenefitsInformationName {
    /// Indicates the patient has active coverage.
    #[serde(rename = "Active Coverage")]
    ActiveCoverage,
    /// Indicates active coverage with full risk capitation.
    #[serde(rename = "Active - Full Risk Capitation")]
    ActiveFullRiskCapitation,
    /// Indicates active coverage where services are capitated.
    #[serde(rename = "Active - Services Capitated")]
    ActiveServicesCapitated,
    /// Indicates active coverage where services are capitated to a primary care physician.
    #[serde(rename = "Active - Services Capitated to Primary Care Physician")]
    ActiveServicesCapitatedToPrimaryCarePhysician,
    /// Indicates active coverage pending investigation.
    #[serde(rename = "Active - Pending Investigation")]
    ActivePendingInvestigation,
    /// Indicates the patient's coverage is inactive.
    #[serde(rename = "Inactive")]
    Inactive,
    /// Indicates inactive coverage pending an eligibility update.
    #[serde(rename = "Inactive - Pending Eligibility Update")]
    InactivePendingEligibilityUpdate,
    /// Indicates inactive coverage pending investigation.
    #[serde(rename = "Inactive - Pending Investigation")]
    InactivePendingInvestigation,
    /// Represents co-insurance cost-sharing requirements.
    #[serde(rename = "Co-Insurance")]
    CoInsurance,
    /// Represents co-payment cost-sharing requirements.
    #[serde(rename = "Co-Payment")]
    CoPayment,
    /// Represents deductible cost-sharing requirements.
    #[serde(rename = "Deductible")]
    Deductible,
    /// Indicates the basis for coverage.
    #[serde(rename = "Coverage Basis")]
    CoverageBasis,
    /// Provides a description of the benefit.
    #[serde(rename = "Benefit Description")]
    BenefitDescription,
    /// Lists exclusions that are not covered under the benefit.
    #[serde(rename = "Exclusions")]
    Exclusions,
    /// Lists limitations on the benefit coverage.
    #[serde(rename = "Limitations")]
    Limitations,
    /// Represents out of pocket stop loss limits.
    #[serde(rename = "Out of Pocket (Stop Loss)")]
    OutOfPocketLeftParenthesisStopLossRightParenthesis,
    /// Indicates unlimited coverage for the benefit.
    #[serde(rename = "Unlimited")]
    Unlimited,
    /// Indicates services that are not covered.
    #[serde(rename = "Non-Covered")]
    NonCovered,
    /// Represents cost containment measures applied to the benefit.
    #[serde(rename = "Cost Containment")]
    CostContainment,
    /// Indicates reserved or future use benefits.
    #[serde(rename = "Reserve")]
    Reserve,
    /// Identifies the primary care provider associated with the benefit.
    #[serde(rename = "Primary Care Provider")]
    PrimaryCareProvider,
    /// Indicates benefits related to pre-existing conditions.
    #[serde(rename = "Pre-existing Condition")]
    PreExistingCondition,
    /// Identifies the managed care coordinator for the benefit.
    #[serde(rename = "Managed Care Coordinator")]
    ManagedCareCoordinator,
    /// Indicates services are restricted to a specific provider.
    #[serde(rename = "Services Restricted to Following Provider")]
    ServicesRestrictedToFollowingProvider,
    /// Indicates services are not deemed medically necessary.
    #[serde(rename = "Not Deemed a Medical Necessity")]
    NotDeemedAMedicalNecessity,
    /// Provides disclaimers about the benefit coverage.
    #[serde(rename = "Benefit Disclaimer")]
    BenefitDisclaimer,
    /// Indicates a second surgical opinion is required for coverage.
    #[serde(rename = "Second Surgical Opinion Required")]
    SecondSurgicalOpinionRequired,
    /// Identifies other or additional payors involved in coverage.
    #[serde(rename = "Other or Additional Payor")]
    OtherOrAdditionalPayor,
    /// Provides prior year history information for the benefit.
    #[serde(rename = "Prior Year(s) History")]
    PriorYearLeftParenthesisSRightParenthesisHistory,
    /// Indicates cards have been reported lost or stolen.
    #[serde(rename = "Card(s) Reported Lost/Stolen")]
    CardLeftParenthesisSRightParenthesisReportedLostSlashStolen,
    /// Indicates the need to contact another entity for eligibility or benefit information.
    #[serde(rename = "Contact Following Entity for Eligibility or Benefit Information")]
    ContactFollowingEntityForEligibilityOrBenefitInformation,
    /// Indicates the benefit information cannot be processed.
    #[serde(rename = "Cannot Process")]
    CannotProcess,
    /// Indicates benefit information comes from another source.
    #[serde(rename = "Other Source of Data")]
    OtherSourceOfData,
    /// Identifies healthcare facilities related to the benefit.
    #[serde(rename = "Health Care Facility")]
    HealthCareFacility,
    /// Indicates spend down requirements for eligibility.
    #[serde(rename = "Spend Down")]
    SpendDown,
}

impl std::fmt::Display for BenefitsInformationName {
    /// Formats the enum variant as its human-readable string representation.
    ///
    /// This implementation returns the exact string value that corresponds to each
    /// benefit information name as defined by the healthcare payer standards.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ActiveCoverage => write!(f, "Active Coverage"),
            Self::ActiveFullRiskCapitation => write!(f, "Active - Full Risk Capitation"),
            Self::ActiveServicesCapitated => write!(f, "Active - Services Capitated"),
            Self::ActiveServicesCapitatedToPrimaryCarePhysician => {
                write!(f, "Active - Services Capitated to Primary Care Physician")
            }
            Self::ActivePendingInvestigation => write!(f, "Active - Pending Investigation"),
            Self::Inactive => write!(f, "Inactive"),
            Self::InactivePendingEligibilityUpdate => {
                write!(f, "Inactive - Pending Eligibility Update")
            }
            Self::InactivePendingInvestigation => write!(f, "Inactive - Pending Investigation"),
            Self::CoInsurance => write!(f, "Co-Insurance"),
            Self::CoPayment => write!(f, "Co-Payment"),
            Self::Deductible => write!(f, "Deductible"),
            Self::CoverageBasis => write!(f, "Coverage Basis"),
            Self::BenefitDescription => write!(f, "Benefit Description"),
            Self::Exclusions => write!(f, "Exclusions"),
            Self::Limitations => write!(f, "Limitations"),
            Self::OutOfPocketLeftParenthesisStopLossRightParenthesis => {
                write!(f, "Out of Pocket (Stop Loss)")
            }
            Self::Unlimited => write!(f, "Unlimited"),
            Self::NonCovered => write!(f, "Non-Covered"),
            Self::CostContainment => write!(f, "Cost Containment"),
            Self::Reserve => write!(f, "Reserve"),
            Self::PrimaryCareProvider => write!(f, "Primary Care Provider"),
            Self::PreExistingCondition => write!(f, "Pre-existing Condition"),
            Self::ManagedCareCoordinator => write!(f, "Managed Care Coordinator"),
            Self::ServicesRestrictedToFollowingProvider => {
                write!(f, "Services Restricted to Following Provider")
            }
            Self::NotDeemedAMedicalNecessity => write!(f, "Not Deemed a Medical Necessity"),
            Self::BenefitDisclaimer => write!(f, "Benefit Disclaimer"),
            Self::SecondSurgicalOpinionRequired => write!(f, "Second Surgical Opinion Required"),
            Self::OtherOrAdditionalPayor => write!(f, "Other or Additional Payor"),
            Self::PriorYearLeftParenthesisSRightParenthesisHistory => {
                write!(f, "Prior Year(s) History")
            }
            Self::CardLeftParenthesisSRightParenthesisReportedLostSlashStolen => {
                write!(f, "Card(s) Reported Lost/Stolen")
            }
            Self::ContactFollowingEntityForEligibilityOrBenefitInformation => write!(
                f,
                "Contact Following Entity for Eligibility or Benefit Information"
            ),
            Self::CannotProcess => write!(f, "Cannot Process"),
            Self::OtherSourceOfData => write!(f, "Other Source of Data"),
            Self::HealthCareFacility => write!(f, "Health Care Facility"),
            Self::SpendDown => write!(f, "Spend Down"),
        }
    }
}

impl Default for BenefitsInformationName {
    /// Returns the default variant, which is ActiveCoverage.
    ///
    /// This is typically used when no specific benefit information name is provided
    /// or when initializing a new instance of this enum.
    fn default() -> BenefitsInformationName {
        Self::ActiveCoverage
    }
}
