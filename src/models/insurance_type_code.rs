use serde::{Deserialize, Serialize};

/// Insurance type codes indicating the type of insurance policy within a specific insurance program.
///
/// These codes are returned in the `benefitsInformation.insuranceTypeCode` property.
///
/// Note: Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InsuranceTypeCode {
    /// Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan
    #[serde(rename = "12")]
    #[default]
    MedicareSecondaryWorkingAged,

    /// Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan
    #[serde(rename = "13")]
    MedicareSecondaryEndStageRenalDisease,

    /// Medicare Secondary, No-fault Insurance including Auto is Primary
    #[serde(rename = "14")]
    MedicareSecondaryNoFaultInsurance,

    /// Medicare Secondary Worker's Compensation
    #[serde(rename = "15")]
    MedicareSecondaryWorkersCompensation,

    /// Medicare Secondary Public Health Service (PHS) or Other Federal Agency
    #[serde(rename = "16")]
    MedicareSecondaryPublicHealthService,

    /// Medicare Secondary Black Lung
    #[serde(rename = "41")]
    MedicareSecondaryBlackLung,

    /// Medicare Secondary Veteran's Administration
    #[serde(rename = "42")]
    MedicareSecondaryVeteransAdministration,

    /// Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)
    #[serde(rename = "43")]
    MedicareSecondaryDisabledBeneficiary,

    /// Medicare Secondary, Other Liability Insurance is Primary
    #[serde(rename = "47")]
    MedicareSecondaryOtherLiability,

    /// Auto Insurance Policy
    #[serde(rename = "AP")]
    AutoInsurancePolicy,

    /// Commercial
    #[serde(rename = "C1")]
    Commercial,

    /// Consolidated Omnibus Budget Reconciliation Act (COBRA)
    #[serde(rename = "CO")]
    Cobra,

    /// Medicare Conditionally Primary
    #[serde(rename = "CP")]
    MedicareConditionallyPrimary,

    /// Disability
    #[serde(rename = "D")]
    Disability,

    /// Disability Benefits
    #[serde(rename = "DB")]
    DisabilityBenefits,

    /// Exclusive Provider Organization
    #[serde(rename = "EP")]
    ExclusiveProviderOrganization,

    /// Family or Friends
    #[serde(rename = "FF")]
    FamilyOrFriends,

    /// Group Policy
    #[serde(rename = "GP")]
    GroupPolicy,

    /// Health Maintenance Organization (HMO)
    #[serde(rename = "HM")]
    HealthMaintenanceOrganization,

    /// Health Maintenance Organization (HMO) - Medicare Risk
    #[serde(rename = "HN")]
    HmoMedicareRisk,

    /// Special Low Income Medicare Beneficiary
    #[serde(rename = "HS")]
    SpecialLowIncomeMedicareBeneficiary,

    /// Indemnity
    #[serde(rename = "IN")]
    Indemnity,

    /// Individual Policy
    #[serde(rename = "IP")]
    IndividualPolicy,

    /// Long Term Care
    #[serde(rename = "LC")]
    LongTermCare,

    /// Long Term Policy
    #[serde(rename = "LD")]
    LongTermPolicy,

    /// Life Insurance
    #[serde(rename = "LI")]
    LifeInsurance,

    /// Litigation
    #[serde(rename = "LT")]
    Litigation,

    /// Medicare Part A
    #[serde(rename = "MA")]
    MedicarePartA,

    /// Medicare Part B
    #[serde(rename = "MB")]
    MedicarePartB,

    /// Medicaid
    #[serde(rename = "MC")]
    Medicaid,

    /// Medigap Part A
    #[serde(rename = "MH")]
    MedigapPartA,

    /// Medigap Part B
    #[serde(rename = "MI")]
    MedigapPartB,

    /// Medicare Primary
    #[serde(rename = "MP")]
    MedicarePrimary,

    /// Other - When returned by Medicare or Medicare Part D administrator, indicates Medicare Part D
    #[serde(rename = "OT")]
    Other,

    /// Property Insurance - Personal
    #[serde(rename = "PE")]
    PropertyInsurancePersonal,

    /// Personal
    #[serde(rename = "PL")]
    Personal,

    /// Personal Payment (Cash - No Insurance)
    #[serde(rename = "PP")]
    PersonalPayment,

    /// Preferred Provider Organization (PPO)
    #[serde(rename = "PR")]
    PreferredProviderOrganization,

    /// Point of Service (POS)
    #[serde(rename = "PS")]
    PointOfService,

    /// Qualified Medicare Beneficiary
    #[serde(rename = "QM")]
    QualifiedMedicareBeneficiary,

    /// Property Insurance - Real
    #[serde(rename = "RP")]
    PropertyInsuranceReal,

    /// Supplemental Policy
    #[serde(rename = "SP")]
    SupplementalPolicy,

    /// Tax Equity Fiscal Responsibility Act (TEFRA)
    #[serde(rename = "TF")]
    Tefra,

    /// Workers Compensation
    #[serde(rename = "WC")]
    WorkersCompensation,

    /// Wrap Up Policy
    #[serde(rename = "WU")]
    WrapUpPolicy,
}

impl std::fmt::Display for InsuranceTypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MedicareSecondaryWorkingAged => write!(f, "12"),
            Self::MedicareSecondaryEndStageRenalDisease => write!(f, "13"),
            Self::MedicareSecondaryNoFaultInsurance => write!(f, "14"),
            Self::MedicareSecondaryWorkersCompensation => write!(f, "15"),
            Self::MedicareSecondaryPublicHealthService => write!(f, "16"),
            Self::MedicareSecondaryBlackLung => write!(f, "41"),
            Self::MedicareSecondaryVeteransAdministration => write!(f, "42"),
            Self::MedicareSecondaryDisabledBeneficiary => write!(f, "43"),
            Self::MedicareSecondaryOtherLiability => write!(f, "47"),
            Self::AutoInsurancePolicy => write!(f, "AP"),
            Self::Commercial => write!(f, "C1"),
            Self::Cobra => write!(f, "CO"),
            Self::MedicareConditionallyPrimary => write!(f, "CP"),
            Self::Disability => write!(f, "D"),
            Self::DisabilityBenefits => write!(f, "DB"),
            Self::ExclusiveProviderOrganization => write!(f, "EP"),
            Self::FamilyOrFriends => write!(f, "FF"),
            Self::GroupPolicy => write!(f, "GP"),
            Self::HealthMaintenanceOrganization => write!(f, "HM"),
            Self::HmoMedicareRisk => write!(f, "HN"),
            Self::SpecialLowIncomeMedicareBeneficiary => write!(f, "HS"),
            Self::Indemnity => write!(f, "IN"),
            Self::IndividualPolicy => write!(f, "IP"),
            Self::LongTermCare => write!(f, "LC"),
            Self::LongTermPolicy => write!(f, "LD"),
            Self::LifeInsurance => write!(f, "LI"),
            Self::Litigation => write!(f, "LT"),
            Self::MedicarePartA => write!(f, "MA"),
            Self::MedicarePartB => write!(f, "MB"),
            Self::Medicaid => write!(f, "MC"),
            Self::MedigapPartA => write!(f, "MH"),
            Self::MedigapPartB => write!(f, "MI"),
            Self::MedicarePrimary => write!(f, "MP"),
            Self::Other => write!(f, "OT"),
            Self::PropertyInsurancePersonal => write!(f, "PE"),
            Self::Personal => write!(f, "PL"),
            Self::PersonalPayment => write!(f, "PP"),
            Self::PreferredProviderOrganization => write!(f, "PR"),
            Self::PointOfService => write!(f, "PS"),
            Self::QualifiedMedicareBeneficiary => write!(f, "QM"),
            Self::PropertyInsuranceReal => write!(f, "RP"),
            Self::SupplementalPolicy => write!(f, "SP"),
            Self::Tefra => write!(f, "TF"),
            Self::WorkersCompensation => write!(f, "WC"),
            Self::WrapUpPolicy => write!(f, "WU"),
        }
    }
}
