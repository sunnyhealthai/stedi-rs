use serde::{Deserialize, Serialize};

/// Human-readable name identifying the category of insurance policy or health plan type.
///
/// This enum provides the human-readable name for insurance type codes, which categorize
/// different types of health insurance policies and programs. It is used in
/// `benefitsInformation.insuranceType` to identify the type of health plan the patient has,
/// which can affect benefit structures, coverage rules, and regulatory requirements.
///
/// The corresponding code version (`InsuranceTypeCode`) contains standardized codes (e.g.,
/// `C1` for Commercial, `MA` for Medicare Part A, `MB` for Medicare Part B) for programmatic
/// use, while this enum provides human-readable names for display and understanding.
///
/// ## Insurance Type Categories
///
/// The insurance types can be grouped into several categories:
///
/// ### Medicare Types
/// - **Medicare Part A**: Hospital insurance coverage
/// - **Medicare Part B**: Medical insurance coverage
/// - **Medicare Primary**: Medicare is the primary payer
/// - **Medicare Secondary**: Medicare is secondary to another payer (various scenarios)
/// - **Medicare Conditionally Primary**: Medicare may be primary under certain conditions
/// - **Medigap Part A/B**: Medicare supplemental insurance
/// - **Medicare Risk Plans**: HMO plans for Medicare beneficiaries
///
/// ### Commercial Insurance
/// - **Commercial**: Standard commercial health insurance
/// - **Group Policy**: Employer-sponsored group health insurance
/// - **Individual Policy**: Individual health insurance policies
/// - **COBRA**: Continuation coverage under COBRA
///
/// ### Managed Care Organizations
/// - **HMO**: Health Maintenance Organization
/// - **PPO**: Preferred Provider Organization
/// - **EPO**: Exclusive Provider Organization
/// - **POS**: Point of Service plan
///
/// ### Government Programs
/// - **Medicaid**: State and federal healthcare program for low-income individuals
/// - **TRICARE**: Military healthcare program
/// - **Veteran's Administration**: VA healthcare
///
/// ### Special Programs
/// - **Workers Compensation**: Coverage for work-related injuries
/// - **Disability**: Disability insurance benefits
/// - **Long Term Care**: Long-term care insurance
/// - **Supplemental Policy**: Additional coverage beyond primary insurance
///
/// ### Other Types
/// - **Auto Insurance Policy**: Automobile insurance for medical coverage
/// - **Property Insurance**: Property-related insurance
/// - **Personal Payment**: Cash payment or self-pay (no insurance)
/// - **Other**: Other insurance types not specifically categorized
///
/// ## Usage Context
///
/// Insurance type helps identify:
/// - **Plan structure**: Whether it's an HMO, PPO, or other managed care type
/// - **Coverage source**: Whether it's Medicare, Medicaid, commercial, or other program
/// - **Coordination of benefits**: How multiple insurance policies coordinate (e.g., Medicare
///   Secondary scenarios)
/// - **Regulatory requirements**: Different rules apply to different insurance types
/// - **Benefit structures**: Coverage rules and benefit calculations vary by insurance type
///
/// For example, Medicare Advantage plans may be identified by the presence of both
/// `Medicare Part A` and `Medicare Part B` insurance types in the benefits information.
///
/// ## Medicare Secondary Scenarios
///
/// Many variants relate to Medicare Secondary Payer (MSP) scenarios, where Medicare is
/// secondary to another payer. These scenarios include:
/// - Working aged beneficiaries with employer group health plans
/// - End-stage renal disease beneficiaries
/// - Workers' compensation cases
/// - Liability insurance (auto, no-fault, other)
/// - Black Lung benefits
/// - Veteran's Administration benefits
///
/// Understanding these scenarios is important for coordination of benefits and determining
/// which payer is primary.
///
/// Payers may sometimes return other non-compliant values.
#[allow(missing_docs)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InsuranceTypeName {
    #[serde(
        rename = "Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan"
    )]
    #[default]
    MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan,
    #[serde(
        rename = "Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan"
    )]
    MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployerQuoteSGroupHealthPlan,
    #[serde(rename = "Medicare Secondary, No-fault Insurance including Auto is Primary")]
    MedicareSecondaryCommaNoFaultInsuranceIncludingAutoIsPrimary,
    #[serde(rename = "Medicare Secondary Worker's Compensation")]
    MedicareSecondaryWorkerQuoteSCompensation,
    #[serde(rename = "Medicare Secondary Public Health Service (PHS)or Other Federal Agency")]
    MedicareSecondaryPublicHealthServiceLeftParenthesisPhsRightParenthesisOrOtherFederalAgency,
    #[serde(rename = "Medicare Secondary Black Lung")]
    MedicareSecondaryBlackLung,
    #[serde(rename = "Medicare Secondary Veteran's Administration")]
    MedicareSecondaryVeteranQuoteSAdministration,
    #[serde(
        rename = "Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)"
    )]
    MedicareSecondaryDisabledBeneficiaryUnderAge65WithLargeGroupHealthPlanLeftParenthesisLghpRightParenthesis,
    #[serde(rename = "Medicare Secondary, Other Liability Insurance is Primary")]
    MedicareSecondaryCommaOtherLiabilityInsuranceIsPrimary,
    #[serde(rename = "Auto Insurance Policy")]
    AutoInsurancePolicy,
    #[serde(rename = "Commercial")]
    Commercial,
    #[serde(rename = "Consolidated Omnibus Budget Reconciliation Act (COBRA)")]
    ConsolidatedOmnibusBudgetReconciliationActLeftParenthesisCobraRightParenthesis,
    #[serde(rename = "Medicare Conditionally Primary")]
    MedicareConditionallyPrimary,
    #[serde(rename = "Disability")]
    Disability,
    #[serde(rename = "Disability Benefits")]
    DisabilityBenefits,
    #[serde(rename = "Exclusive Provider Organization")]
    ExclusiveProviderOrganization,
    #[serde(rename = "Family or Friends")]
    FamilyOrFriends,
    #[serde(rename = "Group Policy")]
    GroupPolicy,
    #[serde(rename = "Health Maintenance Organization (HMO)")]
    HealthMaintenanceOrganizationLeftParenthesisHmoRightParenthesis,
    #[serde(rename = "Health Maintenance Organization (HMO) - Medicare Risk")]
    HealthMaintenanceOrganizationLeftParenthesisHmoRightParenthesisMedicareRisk,
    #[serde(rename = "Special Low Income Medicare Beneficiary")]
    SpecialLowIncomeMedicareBeneficiary,
    #[serde(rename = "Indemnity")]
    Indemnity,
    #[serde(rename = "Individual Policy")]
    IndividualPolicy,
    #[serde(rename = "Long Term Care")]
    LongTermCare,
    #[serde(rename = "Long Term Policy")]
    LongTermPolicy,
    #[serde(rename = "Life Insurance")]
    LifeInsurance,
    #[serde(rename = "Litigation")]
    Litigation,
    #[serde(rename = "Medicare Part A")]
    MedicarePartA,
    #[serde(rename = "Medicare Part B")]
    MedicarePartB,
    #[serde(rename = "Medicaid")]
    Medicaid,
    #[serde(rename = "Medigap Part A")]
    MedigapPartA,
    #[serde(rename = "Medigap Part B")]
    MedigapPartB,
    #[serde(rename = "Medicare Primary")]
    MedicarePrimary,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Property Insurance - Personal")]
    PropertyInsurancePersonal,
    #[serde(rename = "Personal")]
    Personal,
    #[serde(rename = "Personal Payment (Cash - No Insurance)")]
    PersonalPaymentLeftParenthesisCashNoInsuranceRightParenthesis,
    #[serde(rename = "Preferred Provider Organization (PPO)")]
    PreferredProviderOrganizationLeftParenthesisPpoRightParenthesis,
    #[serde(rename = "Point of Service (POS)")]
    PointOfServiceLeftParenthesisPosRightParenthesis,
    #[serde(rename = "Qualified Medicare Beneficiary")]
    QualifiedMedicareBeneficiary,
    #[serde(rename = "Property Insurance - Real")]
    PropertyInsuranceReal,
    #[serde(rename = "Supplemental Policy")]
    SupplementalPolicy,
    #[serde(rename = "Tax Equity Fiscal Responsibility Act (TEFRA)")]
    TaxEquityFiscalResponsibilityActLeftParenthesisTefraRightParenthesis,
    #[serde(rename = "Workers Compensation")]
    WorkersCompensation,
    #[serde(rename = "Wrap Up Policy")]
    WrapUpPolicy,
}

impl std::fmt::Display for InsuranceTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MedicareSecondaryWorkingAgedBeneficiaryOrSpouseWithEmployerGroupHealthPlan => write!(f, "Medicare Secondary Working Aged Beneficiary or Spouse with Employer Group Health Plan"),
            Self::MedicareSecondaryEndStageRenalDiseaseBeneficiaryInTheMandatedCoordinationPeriodWithAnEmployerQuoteSGroupHealthPlan => write!(f, "Medicare Secondary End-Stage Renal Disease Beneficiary in the Mandated Coordination Period with an Employer's Group Health Plan"),
            Self::MedicareSecondaryCommaNoFaultInsuranceIncludingAutoIsPrimary => write!(f, "Medicare Secondary, No-fault Insurance including Auto is Primary"),
            Self::MedicareSecondaryWorkerQuoteSCompensation => write!(f, "Medicare Secondary Worker's Compensation"),
            Self::MedicareSecondaryPublicHealthServiceLeftParenthesisPhsRightParenthesisOrOtherFederalAgency => write!(f, "Medicare Secondary Public Health Service (PHS)or Other Federal Agency"),
            Self::MedicareSecondaryBlackLung => write!(f, "Medicare Secondary Black Lung"),
            Self::MedicareSecondaryVeteranQuoteSAdministration => write!(f, "Medicare Secondary Veteran's Administration"),
            Self::MedicareSecondaryDisabledBeneficiaryUnderAge65WithLargeGroupHealthPlanLeftParenthesisLghpRightParenthesis => write!(f, "Medicare Secondary Disabled Beneficiary Under Age 65 with Large Group Health Plan (LGHP)"),
            Self::MedicareSecondaryCommaOtherLiabilityInsuranceIsPrimary => write!(f, "Medicare Secondary, Other Liability Insurance is Primary"),
            Self::AutoInsurancePolicy => write!(f, "Auto Insurance Policy"),
            Self::Commercial => write!(f, "Commercial"),
            Self::ConsolidatedOmnibusBudgetReconciliationActLeftParenthesisCobraRightParenthesis => write!(f, "Consolidated Omnibus Budget Reconciliation Act (COBRA)"),
            Self::MedicareConditionallyPrimary => write!(f, "Medicare Conditionally Primary"),
            Self::Disability => write!(f, "Disability"),
            Self::DisabilityBenefits => write!(f, "Disability Benefits"),
            Self::ExclusiveProviderOrganization => write!(f, "Exclusive Provider Organization"),
            Self::FamilyOrFriends => write!(f, "Family or Friends"),
            Self::GroupPolicy => write!(f, "Group Policy"),
            Self::HealthMaintenanceOrganizationLeftParenthesisHmoRightParenthesis => write!(f, "Health Maintenance Organization (HMO)"),
            Self::HealthMaintenanceOrganizationLeftParenthesisHmoRightParenthesisMedicareRisk => write!(f, "Health Maintenance Organization (HMO) - Medicare Risk"),
            Self::SpecialLowIncomeMedicareBeneficiary => write!(f, "Special Low Income Medicare Beneficiary"),
            Self::Indemnity => write!(f, "Indemnity"),
            Self::IndividualPolicy => write!(f, "Individual Policy"),
            Self::LongTermCare => write!(f, "Long Term Care"),
            Self::LongTermPolicy => write!(f, "Long Term Policy"),
            Self::LifeInsurance => write!(f, "Life Insurance"),
            Self::Litigation => write!(f, "Litigation"),
            Self::MedicarePartA => write!(f, "Medicare Part A"),
            Self::MedicarePartB => write!(f, "Medicare Part B"),
            Self::Medicaid => write!(f, "Medicaid"),
            Self::MedigapPartA => write!(f, "Medigap Part A"),
            Self::MedigapPartB => write!(f, "Medigap Part B"),
            Self::MedicarePrimary => write!(f, "Medicare Primary"),
            Self::Other => write!(f, "Other"),
            Self::PropertyInsurancePersonal => write!(f, "Property Insurance - Personal"),
            Self::Personal => write!(f, "Personal"),
            Self::PersonalPaymentLeftParenthesisCashNoInsuranceRightParenthesis => write!(f, "Personal Payment (Cash - No Insurance)"),
            Self::PreferredProviderOrganizationLeftParenthesisPpoRightParenthesis => write!(f, "Preferred Provider Organization (PPO)"),
            Self::PointOfServiceLeftParenthesisPosRightParenthesis => write!(f, "Point of Service (POS)"),
            Self::QualifiedMedicareBeneficiary => write!(f, "Qualified Medicare Beneficiary"),
            Self::PropertyInsuranceReal => write!(f, "Property Insurance - Real"),
            Self::SupplementalPolicy => write!(f, "Supplemental Policy"),
            Self::TaxEquityFiscalResponsibilityActLeftParenthesisTefraRightParenthesis => write!(f, "Tax Equity Fiscal Responsibility Act (TEFRA)"),
            Self::WorkersCompensation => write!(f, "Workers Compensation"),
            Self::WrapUpPolicy => write!(f, "Wrap Up Policy"),
        }
    }
}
