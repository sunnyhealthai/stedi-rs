//! Pure functional Healthcare Benefits Calculation Utilities
//!
//! This module provides a stateless API for calculating patient out-of-pocket costs.
//! It avoids any self-referential structs and exposes pure functions that operate on
//! parsed benefit records.

use serde::{Deserialize, Serialize};

use crate::models::*; // Assuming these models are defined elsewhere in the crate
use std::collections::HashMap;

// --- Data Structures ---

/// Network status indicating whether a healthcare provider is within the patient's insurance network.
///
/// This enum determines which set of benefits and cost-sharing rules apply to a given procedure.
/// In-network providers typically have lower patient costs due to contracted rates with insurers.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::NetworkStatus;
/// let status = NetworkStatus::InNetwork;
/// assert_eq!(status, NetworkStatus::InNetwork);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NetworkStatus {
    /// Provider has a contract with the insurance plan and offers reduced rates
    InNetwork,
    /// Provider does not have a contract with the insurance plan, typically resulting in higher costs
    OutOfNetwork,
    /// Network status could not be determined from available information
    Unknown,
    /// Network status is not applicable for this type of service (e.g., emergency services)
    NotApplicable,
}

impl From<&InPlanNetworkIndicatorCode> for NetworkStatus {
    /// Converts an [`InPlanNetworkIndicatorCode`] from eligibility responses into a `NetworkStatus`.
    ///
    /// This conversion standardizes the various ways network status can be represented
    /// in different payer systems into our internal enum.
    ///
    /// # Arguments
    ///
    /// * `code` - The network indicator code from an eligibility response
    ///
    /// # Examples
    ///
    /// ```
    /// # use stedi_rs::utilities::benefits_calculator::NetworkStatus;
    /// # use stedi_rs::models::InPlanNetworkIndicatorCode;
    /// let code = InPlanNetworkIndicatorCode::Yes;
    /// let status = NetworkStatus::from(&code);
    /// assert_eq!(status, NetworkStatus::InNetwork);
    /// ```
    fn from(code: &InPlanNetworkIndicatorCode) -> Self {
        match code {
            InPlanNetworkIndicatorCode::Yes => NetworkStatus::InNetwork,
            InPlanNetworkIndicatorCode::No => NetworkStatus::OutOfNetwork,
            InPlanNetworkIndicatorCode::Unknown => NetworkStatus::Unknown,
            InPlanNetworkIndicatorCode::NotApplicable => NetworkStatus::NotApplicable,
        }
    }
}

/// Information about a healthcare provider used in benefit calculations.
///
/// This struct contains the essential provider details needed to determine
/// which benefits apply and calculate accurate patient costs. Used by
/// [`calculate_out_of_pocket`] and [`calculate_with_accumulations`].
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::ProviderInfo;
/// let provider = ProviderInfo {
///     is_specialist: true,
/// };
/// assert!(provider.is_specialist);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Whether this provider is considered a specialist (affects copay and benefit rules)
    pub is_specialist: bool,
}

/// Details about a specific medical procedure or service.
///
/// Contains the information needed to identify the procedure, its cost,
/// and determine which insurance benefits apply. The `service_type_codes`
/// are used to match against [`ParsedBenefit`] service types.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::ProcedureDetails;
/// let procedure = ProcedureDetails {
///     code: "99213".to_string(),
///     name: "Office Visit - Established Patient".to_string(),
///     cost: 150.00,
///     service_type_codes: vec!["98".to_string()], // Professional services
/// };
/// assert_eq!(procedure.cost, 150.00);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureDetails {
    /// Medical procedure code (e.g., CPT, HCPCS)
    pub code: String,
    /// Human-readable description of the procedure
    pub name: String,
    /// Provider's charge amount for this procedure
    pub cost: f64,
    /// Healthcare service type codes that classify this procedure for benefit matching
    pub service_type_codes: Vec<String>,
}

/// Previously accumulated amounts toward deductibles and out-of-pocket maximums.
///
/// Tracks how much the patient has already paid toward their annual limits,
/// which affects calculations for additional procedures. Used by
/// [`calculate_with_accumulations`] to account for prior spending.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::AccumulatedAmounts;
/// let accumulated = AccumulatedAmounts {
///     individual_deductible_met: 500.00,
///     family_deductible_met: 750.00,
///     individual_oop_met: 1200.00,
///     family_oop_met: 1800.00,
/// };
/// assert_eq!(accumulated.individual_deductible_met, 500.00);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccumulatedAmounts {
    /// Amount already paid toward individual deductible this year
    pub individual_deductible_met: f64,
    /// Amount already paid toward family deductible this year
    pub family_deductible_met: f64,
    /// Amount already paid toward individual out-of-pocket maximum this year
    pub individual_oop_met: f64,
    /// Amount already paid toward family out-of-pocket maximum this year
    pub family_oop_met: f64,
}

/// Complete result of a patient cost calculation.
///
/// Contains the final amounts and detailed breakdown of how the calculation
/// was performed, including any warnings about missing or ambiguous data.
/// Returned by [`calculate_out_of_pocket`] and [`calculate_with_accumulations`].
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::{CalculationResult, CalculationDetails};
/// let result = CalculationResult {
///     patient_pays: 25.00,
///     insurance_pays: 125.00,
///     calculation_details: CalculationDetails::default(), // ... detailed breakdown
///     warnings: vec!["No specialist copay found, used general copay".to_string()],
/// };
/// assert_eq!(result.patient_pays + result.insurance_pays, 150.00);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    /// Total amount the patient is responsible for paying
    pub patient_pays: f64,
    /// Total amount the insurance will cover
    pub insurance_pays: f64,
    /// Detailed breakdown of the calculation steps and applied benefits
    pub calculation_details: CalculationDetails,
    /// Any warnings about data quality or assumptions made during calculation
    pub warnings: Vec<String>,
}

/// Detailed step-by-step breakdown of how patient costs were calculated.
///
/// Provides transparency into the calculation process, showing exactly
/// which benefits were applied and how much of each cost-sharing mechanism
/// contributed to the final patient responsibility. Contains [`AppliedBenefit`]
/// records for audit trail purposes.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::CalculationDetails;
/// let details = CalculationDetails {
///     provider_charge: 150.00,
///     applied_benefits: vec![], // List of benefits that were used
///     deductible_applied: 0.00,
///     copay_applied: 25.00,
///     coinsurance_applied: 0.00,
///     oop_max_limited: false,
///     remaining_deductible: 1500.00,
///     remaining_oop_max: 4000.00,
///     explanation: "Applied $25.00 copay for office visit".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationDetails {
    /// Original charge amount from the provider
    pub provider_charge: f64,
    /// List of insurance benefits that were applied in this calculation
    pub applied_benefits: Vec<AppliedBenefit>,
    /// Dollar amount applied toward deductible
    pub deductible_applied: f64,
    /// Fixed copay amount applied
    pub copay_applied: f64,
    /// Coinsurance amount applied (percentage of remaining cost)
    pub coinsurance_applied: f64,
    /// Whether the calculation was limited by the out-of-pocket maximum
    pub oop_max_limited: bool,
    /// Remaining deductible amount after this calculation
    pub remaining_deductible: f64,
    /// Remaining out-of-pocket maximum after this calculation
    pub remaining_oop_max: f64,
    /// Human-readable explanation of the calculation steps
    pub explanation: String,
}

impl Default for CalculationDetails {
    /// Creates a default `CalculationDetails` with zero values and empty explanation.
    ///
    /// Useful for creating calculation details structures during incremental
    /// calculation processes or for testing purposes.
    fn default() -> Self {
        Self {
            provider_charge: 0.0,
            applied_benefits: Vec::new(),
            deductible_applied: 0.0,
            copay_applied: 0.0,
            coinsurance_applied: 0.0,
            oop_max_limited: false,
            remaining_deductible: 0.0,
            remaining_oop_max: 0.0,
            explanation: String::new(),
        }
    }
}

/// Record of a specific insurance benefit that was applied during calculation.
///
/// Provides an audit trail showing which benefits were used and how they
/// contributed to the final patient cost. Stored in [`CalculationDetails`]
/// for transparency and debugging purposes.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::AppliedBenefit;
/// let applied = AppliedBenefit {
///     benefit_type: "Copayment".to_string(),
///     amount: 25.00,
///     source: "$25.00 primary care copay per visit".to_string(),
///     network_status: "InNetwork".to_string(),
/// };
/// assert_eq!(applied.amount, 25.00);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedBenefit {
    /// Type of benefit that was applied (e.g., "Copayment", "Deductible")
    pub benefit_type: String,
    /// Dollar amount of this benefit application
    pub amount: f64,
    /// Description of where this benefit value came from
    pub source: String,
    /// Network status context for this benefit application
    pub network_status: String,
}

/// Standardized representation of an insurance benefit parsed from raw eligibility data.
///
/// Transforms complex eligibility response data into a consistent format that can
/// be easily used for cost calculations and benefit matching. Created by
/// [`parse_benefits`].
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::{ParsedBenefit, BenefitType, NetworkStatus, CoverageLevel, TimePeriod};
/// let benefit = ParsedBenefit {
///     benefit_type: BenefitType::Copayment,
///     amount: Some(25.00),
///     percent: None,
///     network_status: NetworkStatus::InNetwork,
///     coverage_level: CoverageLevel::Individual,
///     time_period: TimePeriod::Visit,
///     service_type_codes: vec!["98".to_string()],
///     is_specialist_benefit: false,
/// };
/// assert_eq!(benefit.amount, Some(25.00));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedBenefit {
    /// The category of benefit this represents
    pub benefit_type: BenefitType,
    /// Dollar amount, if this benefit is expressed as a fixed amount
    pub amount: Option<f64>,
    /// Percentage value, if this benefit is expressed as a percentage
    pub percent: Option<f64>,
    /// Whether this benefit applies to in-network, out-of-network, or all providers
    pub network_status: NetworkStatus,
    /// Whether this applies to individual or family coverage limits
    pub coverage_level: CoverageLevel,
    /// How to interpret the amount/percent (annual total, remaining, per visit, etc.)
    pub time_period: TimePeriod,
    /// Service type codes this benefit applies to (empty means applies to all services)
    pub service_type_codes: Vec<String>,
    /// Whether this benefit only applies when seeing specialist providers
    pub is_specialist_benefit: bool,
}

/// Types of insurance benefits that can affect patient cost calculations.
///
/// Each benefit type represents a different mechanism by which insurance plans
/// structure patient cost-sharing and coverage limits.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::BenefitType;
/// let benefit = BenefitType::Deductible;
/// match benefit {
///     BenefitType::Deductible => println!("Patient must meet deductible first"),
///     BenefitType::Copayment => println!("Fixed dollar amount per visit"),
///     _ => println!("Other benefit type"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BenefitType {
    /// Annual amount patient must pay before insurance begins covering costs
    Deductible,
    /// Fixed dollar amount patient pays per visit or service
    Copayment,
    /// Percentage of costs patient pays after deductible is met
    Coinsurance,
    /// Maximum annual amount patient can be required to pay out-of-pocket
    OutOfPocketMax,
    /// Indicates whether coverage is active for a particular service
    ActiveCoverage,
    /// Any other benefit type not specifically categorized
    Other(String),
}

/// Coverage level indicating whether benefits apply to individual or family limits.
///
/// Insurance plans typically have separate deductibles and out-of-pocket maximums
/// for individual members versus the entire family unit.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::CoverageLevel;
/// let level = CoverageLevel::Individual;
/// match level {
///     CoverageLevel::Individual => println!("Applies to single member only"),
///     CoverageLevel::Family => println!("Applies to entire family unit"),
///     CoverageLevel::Unknown => println!("Coverage level not specified"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoverageLevel {
    /// Benefit applies to a single covered member
    Individual,
    /// Benefit applies to the entire covered family
    Family,
    /// Coverage level could not be determined
    Unknown,
}

/// Time period qualifier indicating how benefit amounts should be interpreted.
///
/// Insurance benefits can be expressed as totals, accumulated amounts, or per-service values.
/// Understanding the time period is crucial for accurate cost calculations.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::TimePeriod;
/// let period = TimePeriod::YearToDate;
/// match period {
///     TimePeriod::CalendarYear => println!("Annual maximum amount"),
///     TimePeriod::YearToDate => println!("Amount accumulated so far this year"),
///     TimePeriod::Remaining => println!("Amount remaining to be met"),
///     TimePeriod::Visit => println!("Amount applies per visit"),
///     TimePeriod::Other(_) => println!("Other time qualifier"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimePeriod {
    /// The total annual amount for the calendar year
    CalendarYear,
    /// The amount that has been met/accumulated year-to-date
    YearToDate,
    /// The amount remaining to be met for the year
    Remaining,
    /// Amount that applies per individual visit or service
    Visit,
    /// Any other time period qualifier not specifically categorized
    Other(String),
}

// --- Public API ---

/// Parses raw benefits information from an eligibility response into a structured format.
///
/// Takes the complex, nested benefit structures from payer eligibility responses and
/// converts them into a standardized format that can be easily used for cost calculations.
/// Benefits that cannot be parsed (missing required fields) are filtered out.
///
/// # Arguments
///
/// * `benefits` - Slice of raw [`BenefitsInformation`] structures from eligibility response
///
/// # Returns
///
/// A vector of [`ParsedBenefit`] structures with standardized benefit information.
/// Benefits that couldn't be parsed are omitted from the result.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::{parse_benefits};
/// # use stedi_rs::models::BenefitsInformation;
/// let raw_benefits: Vec<BenefitsInformation> = vec![
///     // ... BenefitsInformation structures from eligibility response
/// ];
/// let parsed = parse_benefits(&raw_benefits);
/// println!("Parsed {} benefits from {} raw entries", parsed.len(), raw_benefits.len());
/// ```
pub fn parse_benefits(benefits: &[BenefitsInformation]) -> Vec<ParsedBenefit> {
    benefits.iter().filter_map(parse_benefit).collect()
}

/// Calculates out-of-pocket costs without considering any prior spending.
///
/// This is a convenience function that assumes the patient has not yet met any
/// deductibles or accumulated any costs toward their out-of-pocket maximum.
/// For calculations that need to account for prior spending, use [`calculate_with_accumulations`].
///
/// # Arguments
///
/// * `parsed_benefits` - Previously parsed insurance benefits for this patient
/// * `procedure` - Details about the medical procedure being priced
/// * `network` - [`NetworkStatus`] of the provider (in-network, out-of-network, etc.)
/// * `provider` - Information about the healthcare provider
///
/// # Returns
///
/// A [`CalculationResult`] containing patient responsibility, insurance coverage,
/// detailed calculation steps, and any warnings about data quality.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::*;
/// let benefits = vec![/* ... parsed benefits ... */];
/// let procedure = ProcedureDetails {
///     code: "99213".to_string(),
///     name: "Office Visit".to_string(),
///     cost: 150.00,
///     service_type_codes: vec!["98".to_string()],
/// };
/// let provider = ProviderInfo {
///     is_specialist: false,
/// };
///
/// let result = calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
/// println!("Patient pays: ${:.2}", result.patient_pays);
/// ```
pub fn calculate_out_of_pocket(
    parsed_benefits: &[ParsedBenefit],
    procedure: &ProcedureDetails,
    network: NetworkStatus,
    provider: &ProviderInfo,
) -> CalculationResult {
    calculate_with_accumulations(
        parsed_benefits,
        procedure,
        network,
        provider,
        &AccumulatedAmounts::default(),
    )
}

/// Calculates out-of-pocket costs accounting for amounts already paid toward deductibles and limits.
///
/// This is the core calculation function that considers both the patient's benefits and their
/// current progress toward annual deductibles and out-of-pocket maximums. The calculation
/// follows standard insurance logic: copays take precedence for covered services, otherwise
/// costs apply toward deductible first, then coinsurance on remaining amounts.
///
/// # Arguments
///
/// * `parsed_benefits` - Previously parsed insurance benefits for this patient
/// * `procedure` - Details about the medical procedure being priced
/// * `network` - [`NetworkStatus`] of the provider (in-network, out-of-network, etc.)
/// * `provider` - Information about the healthcare provider
/// * `accumulated` - [`AccumulatedAmounts`] already paid toward deductibles and out-of-pocket limits
///
/// # Returns
///
/// A [`CalculationResult`] with detailed breakdown of patient and insurance responsibility.
///
/// # Calculation Logic
///
/// 1. **Coverage Check**: Verify active coverage exists for the service type
/// 2. **Benefit Selection**: Find applicable benefits for the procedure
/// 3. **Copay Path**: If in-network with applicable copay, apply fixed copay amount
/// 4. **Deductible/Coinsurance Path**: Otherwise, apply remaining deductible then coinsurance
/// 5. **OOP Maximum**: Limit total patient cost by remaining out-of-pocket maximum
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::*;
/// let benefits = vec![/* ... parsed benefits ... */];
/// let procedure = ProcedureDetails {
///     code: "99213".to_string(),
///     name: "Office Visit".to_string(),
///     cost: 150.0,
///     service_type_codes: vec!["98".to_string()],
/// };
/// let provider = ProviderInfo {
///     is_specialist: false,
/// };
/// let accumulated = AccumulatedAmounts {
///     individual_deductible_met: 500.00,
///     individual_oop_met: 1200.00,
///     ..Default::default()
/// };
///
/// let result = calculate_with_accumulations(
///     &benefits, &procedure, NetworkStatus::InNetwork, &provider, &accumulated
/// );
/// ```
pub fn calculate_with_accumulations(
    parsed_benefits: &[ParsedBenefit],
    procedure: &ProcedureDetails,
    network: NetworkStatus,
    provider: &ProviderInfo,
    accumulated: &AccumulatedAmounts,
) -> CalculationResult {
    let mut warnings = Vec::new();
    let mut applied_benefits = Vec::new();
    let mut explanation_parts = Vec::new();

    // 1. --- Initial Setup and Pre-Checks ---
    if !has_active_coverage(parsed_benefits, &procedure.service_type_codes) {
        warnings.push("No active coverage found for this service type.".to_string());
        return CalculationResult {
            patient_pays: procedure.cost,
            insurance_pays: 0.0,
            calculation_details: CalculationDetails {
                provider_charge: procedure.cost,
                applied_benefits: vec![],
                deductible_applied: 0.0,
                copay_applied: 0.0,
                coinsurance_applied: 0.0,
                oop_max_limited: false,
                remaining_deductible: 0.0,
                remaining_oop_max: 0.0,
                explanation: "No active coverage - patient pays full amount.".to_string(),
            },
            warnings,
        };
    }

    // 1a. --- Check for Preventive Care (ACA-mandated $0 cost) ---
    if is_preventive_care(&procedure.service_type_codes)
        && matches!(
            network,
            NetworkStatus::InNetwork | NetworkStatus::NotApplicable
        )
    {
        applied_benefits.push(AppliedBenefit {
            benefit_type: "Preventive Care".to_string(),
            amount: 0.0,
            source: "ACA-mandated preventive care - no cost sharing".to_string(),
            network_status: format!("{:?}", network),
        });

        // Get remaining deductible and OOP max for informational purposes
        let deductible_info = get_financial_limit(
            parsed_benefits,
            BenefitType::Deductible,
            network,
            CoverageLevel::Individual,
        );
        let oop_max_info = get_financial_limit(
            parsed_benefits,
            BenefitType::OutOfPocketMax,
            network,
            CoverageLevel::Individual,
        );

        return CalculationResult {
            patient_pays: 0.0,
            insurance_pays: procedure.cost,
            calculation_details: CalculationDetails {
                provider_charge: procedure.cost,
                applied_benefits,
                deductible_applied: 0.0,
                copay_applied: 0.0,
                coinsurance_applied: 0.0,
                oop_max_limited: false,
                remaining_deductible: deductible_info
                    .as_ref()
                    .and_then(|d| d.remaining)
                    .unwrap_or(0.0),
                remaining_oop_max: oop_max_info
                    .as_ref()
                    .and_then(|o| o.remaining)
                    .unwrap_or(0.0),
                explanation:
                    "Preventive care service - covered at 100% with no patient cost sharing (ACA requirement)."
                        .to_string(),
            },
            warnings: vec![],
        };
    }

    let applicable_benefits =
        get_applicable_benefits(parsed_benefits, &procedure.service_type_codes, network);

    // Check if family deductible is already met
    let family_deductible_met =
        is_family_limit_met(parsed_benefits, BenefitType::Deductible, network);

    // Get deductible to use for calculations:
    // 1. If family deductible met -> None (no deductible applies)
    // 2. Otherwise, prefer individual deductible
    // 3. Fallback to family deductible if no individual exists
    let deductible_info = if family_deductible_met {
        None
    } else {
        get_financial_limit(
            parsed_benefits,
            BenefitType::Deductible,
            network,
            CoverageLevel::Individual,
        )
        .or_else(|| {
            get_financial_limit(
                parsed_benefits,
                BenefitType::Deductible,
                network,
                CoverageLevel::Family,
            )
        })
    };

    // Check if family OOP max is already met
    let family_oop_met = is_family_limit_met(parsed_benefits, BenefitType::OutOfPocketMax, network);

    // Get OOP max to use for calculations:
    // 1. If family OOP met -> None (no OOP applies)
    // 2. Otherwise, prefer individual OOP max
    // 3. Fallback to family OOP max if no individual exists
    let oop_max_info = if family_oop_met {
        None
    } else {
        get_financial_limit(
            parsed_benefits,
            BenefitType::OutOfPocketMax,
            network,
            CoverageLevel::Individual,
        )
        .or_else(|| {
            get_financial_limit(
                parsed_benefits,
                BenefitType::OutOfPocketMax,
                network,
                CoverageLevel::Family,
            )
        })
    };

    let mut patient_responsibility = 0.0;
    let mut deductible_applied = 0.0;
    let mut copay_applied = 0.0;
    let mut coinsurance_applied = 0.0;

    // 2. --- Determine Calculation Path (Copay vs. Deductible/Coinsurance) ---
    let copay = get_copay(&applicable_benefits);
    let is_copay_only_path = matches!(
        network,
        NetworkStatus::InNetwork | NetworkStatus::NotApplicable
    ) && copay.is_some();

    if is_copay_only_path {
        // --- PATH A: COPAY ONLY ---
        let copay_amount = copay.unwrap();
        copay_applied = copay_amount;
        patient_responsibility = copay_amount;

        applied_benefits.push(AppliedBenefit {
            benefit_type: "Copayment".to_string(),
            amount: copay_amount,
            source: format!("${:.2} copay per visit", copay_amount),
            network_status: format!("{:?}", network),
        });
        explanation_parts.push(format!("Applied ${:.2} copay.", copay_amount));
    } else {
        // --- PATH B: DEDUCTIBLE & COINSURANCE ---

        // FIX: Use `if let` to borrow `deductible_info` instead of moving it.
        let remaining_deductible = if let Some(info) = &deductible_info {
            // Prefer a direct "remaining" value if available, otherwise calculate it.
            info.remaining
                .unwrap_or_else(|| (info.total - accumulated.individual_deductible_met).max(0.0))
        } else {
            0.0
        };

        if remaining_deductible > 0.0 {
            deductible_applied = procedure.cost.min(remaining_deductible);
            patient_responsibility += deductible_applied;

            // Now we can safely use `deductible_info` again here.
            let total_deductible = deductible_info.as_ref().map(|i| i.total).unwrap_or(0.0);
            applied_benefits.push(AppliedBenefit {
                benefit_type: "Deductible".to_string(),
                amount: deductible_applied,
                source: format!(
                    "Deductible: ${:.2} remaining of ${:.2}",
                    remaining_deductible, total_deductible
                ),
                network_status: format!("{:?}", network),
            });
            explanation_parts.push(format!(
                "Applied ${:.2} toward deductible, leaving ${:.2}.",
                deductible_applied,
                remaining_deductible - deductible_applied
            ));
        }

        let cost_after_deductible = procedure.cost - deductible_applied;
        if cost_after_deductible > 0.0 {
            if let Some(coinsurance_rate) = get_coinsurance_rate(&applicable_benefits) {
                coinsurance_applied = cost_after_deductible * coinsurance_rate;
                patient_responsibility += coinsurance_applied;

                applied_benefits.push(AppliedBenefit {
                    benefit_type: "Coinsurance".to_string(),
                    amount: coinsurance_applied,
                    source: format!("{:.0}% patient coinsurance", coinsurance_rate * 100.0),
                    network_status: format!("{:?}", network),
                });
                explanation_parts.push(format!(
                    "Applied {:.0}% coinsurance (${:.2}) on the remaining ${:.2}.",
                    coinsurance_rate * 100.0,
                    coinsurance_applied,
                    cost_after_deductible
                ));
            } else {
                warnings.push(
                    "No coinsurance found after deductible was met; assuming 100% plan coverage."
                        .to_string(),
                );
                explanation_parts
                    .push("Remaining cost is covered 100% by the insurance plan.".to_string());
            }
        }
    }

    // 3. --- Apply Out-of-Pocket Maximum ---

    // FIX: Apply the same `if let` borrowing pattern to `oop_max_info`.
    let remaining_oop_allowance = if let Some(info) = &oop_max_info {
        info.remaining
            .unwrap_or_else(|| (info.total - accumulated.individual_oop_met).max(0.0))
    } else {
        // If no OOP max is defined, it's effectively infinite.
        f64::MAX
    };

    let mut oop_max_limited = false;
    if patient_responsibility > remaining_oop_allowance {
        let initial_responsibility = patient_responsibility;
        patient_responsibility = remaining_oop_allowance;
        oop_max_limited = true;

        explanation_parts.push(format!(
            "Patient cost was reduced from ${:.2} to ${:.2} by the out-of-pocket maximum.",
            initial_responsibility, patient_responsibility
        ));
    }

    // 4. --- Finalize and Return Result ---
    let insurance_pays = (procedure.cost - patient_responsibility).max(0.0);

    // FIX: Use `if let` here as well for clarity and to prevent the move.
    let final_remaining_deductible = if let Some(info) = &deductible_info {
        (info.total - accumulated.individual_deductible_met - deductible_applied).max(0.0)
    } else {
        0.0
    };

    let final_remaining_oop = (remaining_oop_allowance - patient_responsibility).max(0.0);

    CalculationResult {
        patient_pays: patient_responsibility,
        insurance_pays,
        calculation_details: CalculationDetails {
            provider_charge: procedure.cost,
            applied_benefits,
            deductible_applied,
            copay_applied,
            coinsurance_applied,
            oop_max_limited,
            remaining_deductible: final_remaining_deductible,
            remaining_oop_max: if oop_max_info.is_some() {
                final_remaining_oop
            } else {
                0.0
            },
            explanation: explanation_parts.join(" "),
        },
        warnings,
    }
}

/// Generates a human-readable summary of key plan benefits.
///
/// Creates a convenient overview of the most important benefit amounts,
/// including deductibles and out-of-pocket maximums for both in-network
/// and out-of-network care. This is useful for displaying plan highlights
/// to users or for quick reference during cost calculations.
///
/// # Arguments
///
/// * `parsed_benefits` - Previously parsed insurance benefits to summarize
///
/// # Returns
///
/// A [`HashMap`] with human-readable benefit descriptions. Keys follow the pattern
/// "{[`NetworkStatus`]} {[`BenefitType`]}" (e.g., "InNetwork Individual Deductible").
/// Values include both the total amount and approximate remaining amount.
///
/// # Examples
///
/// ```
/// # use stedi_rs::utilities::benefits_calculator::{get_benefits_summary, ParsedBenefit};
/// let benefits: Vec<ParsedBenefit> = vec![/* ... parsed benefits ... */];
/// let summary = get_benefits_summary(&benefits);
///
/// for (key, value) in &summary {
///     println!("{}: {}", key, value);
/// }
/// // Example output:
/// // "InNetwork Individual Deductible: $1500.00 (approx. $1200.00 remaining)"
/// // "InNetwork Individual OOP Maximum: $5000.00 (approx. $3800.00 remaining)"
/// ```
pub fn get_benefits_summary(parsed_benefits: &[ParsedBenefit]) -> HashMap<String, String> {
    let mut summary = HashMap::new();

    for network in [NetworkStatus::InNetwork, NetworkStatus::OutOfNetwork] {
        if let Some(info) = get_financial_limit(
            parsed_benefits,
            BenefitType::Deductible,
            network,
            CoverageLevel::Individual,
        ) {
            let remaining = info.remaining.unwrap_or(info.total);
            summary.insert(
                format!("{:?} Individual Deductible", network),
                format!("${:.2} (approx. ${:.2} remaining)", info.total, remaining),
            );
        }

        if let Some(info) = get_financial_limit(
            parsed_benefits,
            BenefitType::OutOfPocketMax,
            network,
            CoverageLevel::Individual,
        ) {
            let remaining = info.remaining.unwrap_or(info.total);
            summary.insert(
                format!("{:?} Individual OOP Maximum", network),
                format!("${:.2} (approx. ${:.2} remaining)", info.total, remaining),
            );
        }
    }

    summary
}

// --- Helper Functions ---

/// Checks if service type codes indicate preventive care.
///
/// # Arguments
///
/// * `service_type_codes` - Service type codes for the procedure
///
/// # Returns
///
/// `true` if "preventative" is in the service type codes (case-insensitive).
fn is_preventive_care(service_type_codes: &[String]) -> bool {
    service_type_codes
        .iter()
        .any(|code| code.eq_ignore_ascii_case("preventative"))
}

/// Checks if a family-level financial limit (deductible or OOP max) has been fully met.
///
/// This function looks up the family-level limit and checks if there's no remaining amount left.
/// Used to determine if family deductible/OOP max should override individual limits.
///
/// # Arguments
///
/// * `all_benefits` - Complete list of parsed benefits for the patient
/// * `benefit_type` - Type of limit to check (Deductible or OutOfPocketMax)
/// * `network` - Network status to match
///
/// # Returns
///
/// `true` if family limit exists and has been fully met (remaining ≤ 0), `false` otherwise.
fn is_family_limit_met(
    all_benefits: &[ParsedBenefit],
    benefit_type: BenefitType,
    network: NetworkStatus,
) -> bool {
    if let Some(family_limit) =
        get_financial_limit(all_benefits, benefit_type, network, CoverageLevel::Family)
    {
        // Check if remaining is 0 or negative (fully met)
        if let Some(remaining) = family_limit.remaining {
            return remaining <= 0.0;
        }
    }
    false
}

/// Parses a single raw [`BenefitsInformation`] struct into a [`ParsedBenefit`].
///
/// Converts complex, nested benefit data from payer systems into our standardized
/// internal representation. Benefits without required fields (like benefit type codes)
/// are filtered out by returning `None`.
///
/// # Arguments
///
/// * `benefit` - Raw benefit information from an eligibility response
///
/// # Returns
///
/// `Some(`[`ParsedBenefit`]`)` if the benefit can be successfully parsed,
/// `None` if required fields are missing or invalid.
///
/// # Logic
///
/// 1. Maps benefit type codes to our internal [`BenefitType`] enum
/// 2. Parses amount and percentage values from string fields
/// 3. Determines [`NetworkStatus`], [`CoverageLevel`], and [`TimePeriod`]
/// 4. Extracts service type codes and specialist indicators
fn parse_benefit(benefit: &BenefitsInformation) -> Option<ParsedBenefit> {
    let benefit_type = match &benefit.code {
        Some(BenefitsInformationCode::Deductible) => BenefitType::Deductible,
        Some(BenefitsInformationCode::CoPayment) => BenefitType::Copayment,
        Some(BenefitsInformationCode::CoInsurance) => BenefitType::Coinsurance,
        Some(BenefitsInformationCode::OutOfPocketStopLoss) => BenefitType::OutOfPocketMax,
        Some(BenefitsInformationCode::ActiveCoverage) => BenefitType::ActiveCoverage,
        Some(other) => BenefitType::Other(format!("{:?}", other)),
        None => return None, // Cannot process a benefit without a type code.
    };

    let amount = benefit
        .benefit_amount
        .as_deref()
        .and_then(|a| a.parse().ok());
    let percent = benefit
        .benefit_percent
        .as_deref()
        .and_then(|p| p.parse().ok());

    let network_status = benefit
        .in_plan_network_indicator_code
        .as_ref()
        .map_or(NetworkStatus::Unknown, NetworkStatus::from);

    let coverage_level = match &benefit.coverage_level_code {
        Some(BenefitsInformationCoverageLevelCode::Individual) => CoverageLevel::Individual,
        Some(BenefitsInformationCoverageLevelCode::Family) => CoverageLevel::Family,
        _ => CoverageLevel::Unknown,
    };

    let time_period = match &benefit.time_qualifier_code {
        Some(TimeQualifierCode::CalendarYear) => TimePeriod::CalendarYear,
        Some(TimeQualifierCode::YearToDate) => TimePeriod::YearToDate,
        Some(TimeQualifierCode::Remaining) => TimePeriod::Remaining,
        Some(TimeQualifierCode::Visit) => TimePeriod::Visit,
        Some(other) => TimePeriod::Other(format!("{:?}", other)),
        None => TimePeriod::Other("Unknown".to_string()),
    };

    let service_type_codes = benefit
        .service_type_codes
        .as_ref()
        .map(|codes| codes.iter().map(ToString::to_string).collect())
        .unwrap_or_default();

    let is_specialist_benefit = benefit
        .additional_information
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .any(|info| {
            info.description
                .as_deref()
                .unwrap_or("")
                .to_uppercase()
                .contains("SPECIALIST")
        });

    Some(ParsedBenefit {
        benefit_type,
        amount,
        percent,
        network_status,
        coverage_level,
        time_period,
        service_type_codes,
        is_specialist_benefit,
    })
}

/// Filters the full list of benefits to find those relevant to a specific procedure.
///
/// Applies multiple matching criteria to identify which benefits should be used
/// for calculating costs for a particular procedure and provider combination.
/// This is crucial because insurance plans often have different benefits for
/// different types of services and network statuses.
///
/// # Arguments
///
/// * `all_benefits` - Complete list of parsed benefits for the patient
/// * `service_codes` - Service type codes for the procedure being priced
/// * `network` - [`NetworkStatus`] of the provider
///
/// # Returns
///
/// Vector of references to benefits that match the specified criteria.
///
/// # Matching Logic
///
/// Benefits must match ALL of the following criteria:
/// 1. **Network**: Benefit network status matches provider network or is `NotApplicable`
/// 2. **Service Type**: Benefit applies to the service codes or has no service restriction
///
/// Special handling:
/// - Service code "30" (Health Benefit Plan Coverage) is treated as a universal match
/// - Benefits with empty service codes apply to all services
/// - Service type codes (e.g., "96" for specialist, "98" for primary care) handle benefit distinctions
fn get_applicable_benefits<'a>(
    all_benefits: &'a [ParsedBenefit],
    service_codes: &[String],
    network: NetworkStatus,
) -> Vec<&'a ParsedBenefit> {
    all_benefits
        .iter()
        .filter(|benefit| {
            // Match network status (e.g., InNetwork, OutOfNetwork).
            // A benefit with status `NotApplicable` can apply to any network.
            let network_match = match network {
                NetworkStatus::InNetwork | NetworkStatus::OutOfNetwork => {
                    benefit.network_status == network
                        || benefit.network_status == NetworkStatus::NotApplicable
                }
                NetworkStatus::NotApplicable => {
                    benefit.network_status == NetworkStatus::NotApplicable
                }
                NetworkStatus::Unknown => true, // If network is unknown, assume it could match.
            };

            // Match service type. A benefit with no service codes is generic and applies to all.
            // Service code "30" is a general code for "Health Benefit Plan Coverage".
            // The service type codes (e.g., "96" for specialist, "98" for primary care) handle
            // the distinction between specialist and primary care benefits.
            let service_match = benefit.service_type_codes.is_empty()
                || benefit
                    .service_type_codes
                    .iter()
                    .any(|code| service_codes.contains(code) || code == "30");

            network_match && service_match
        })
        .collect()
}

/// Checks if the patient has active coverage for the given service types.
///
/// Verifies that the patient's insurance plan covers the requested service.
/// This prevents cost calculations for services that aren't covered at all.
///
/// # Arguments
///
/// * `all_benefits` - Complete list of [`ParsedBenefit`] for the patient
/// * `service_codes` - Service type codes to check for coverage
///
/// # Returns
///
/// `true` if active coverage exists for the service types, `false` otherwise.
///
/// # Coverage Logic
///
/// Coverage exists if there's at least one [`BenefitType::ActiveCoverage`] benefit that either:
/// - Has no service type restrictions (covers all services)
/// - Explicitly lists one of the requested service codes
/// - For "preventative" service type: requires ANY active coverage to exist (proves insurance is active)
fn has_active_coverage(all_benefits: &[ParsedBenefit], service_codes: &[String]) -> bool {
    // Special case: "preventative" requires ANY active coverage (proves insurance is active)
    // but doesn't need to match a specific service type in benefits since it's an internal code
    if is_preventive_care(service_codes) {
        return all_benefits
            .iter()
            .any(|b| matches!(b.benefit_type, BenefitType::ActiveCoverage));
    }

    // Normal flow: check for active coverage matching the specific service codes
    all_benefits.iter().any(|b| {
        matches!(b.benefit_type, BenefitType::ActiveCoverage)
            && (b.service_type_codes.is_empty()
                || b.service_type_codes
                    .iter()
                    .any(|code| service_codes.contains(code)))
    })
}

/// A temporary struct to hold financial limit values found while iterating.
///
/// Used internally by [`get_financial_limit`] to accumulate different types of
/// financial information (total, remaining, year-to-date) as it processes benefits.
#[derive(Default, Debug)]
struct FinancialValues {
    /// The total annual limit amount
    total: Option<f64>,
    /// The remaining amount left to meet
    remaining: Option<f64>,
    /// The amount accumulated year-to-date
    year_to_date: Option<f64>,
}

/// The result of [`get_financial_limit`].
///
/// Contains the processed financial limit information with calculated remaining amounts.
#[derive(Debug)]
struct FinancialLimitInfo {
    /// The total annual limit amount
    total: f64,
    /// The remaining amount left to meet (if available or calculable)
    remaining: Option<f64>,
}

/// A generic function to find financial limits like deductibles and OOP maximums.
///
/// Searches through all benefits to find the specified type of financial limit,
/// handling the complex logic of matching [`NetworkStatus`] and [`CoverageLevel`].
/// Prioritizes specific network matches over general [`NetworkStatus::NotApplicable`] benefits.
///
/// # Arguments
///
/// * `all_benefits` - Complete list of [`ParsedBenefit`] for the patient
/// * `benefit_type` - Type of financial limit to find ([`BenefitType::Deductible`], [`BenefitType::OutOfPocketMax`], etc.)
/// * `network` - [`NetworkStatus`] to match
/// * `coverage_level` - [`CoverageLevel`] to match ([`CoverageLevel::Individual`], [`CoverageLevel::Family`])
///
/// # Returns
///
/// `Some(`[`FinancialLimitInfo`]`)` if a matching financial limit is found,
/// `None` if no applicable limit exists.
///
/// # Logic
///
/// 1. Searches for benefits matching the specified type and coverage level
/// 2. Prioritizes benefits with specific network status over [`NetworkStatus::NotApplicable`]
/// 3. Combines different time periods ([`TimePeriod::CalendarYear`], [`TimePeriod::Remaining`], [`TimePeriod::YearToDate`])
/// 4. Calculates remaining amounts when not directly provided
fn get_financial_limit(
    all_benefits: &[ParsedBenefit],
    benefit_type: BenefitType,
    network: NetworkStatus,
    coverage_level: CoverageLevel,
) -> Option<FinancialLimitInfo> {
    let mut specific_network_values = FinancialValues::default();
    let mut fallback_network_values = FinancialValues::default(); // For `NotApplicable` benefits

    for benefit in all_benefits {
        if benefit.benefit_type != benefit_type || benefit.coverage_level != coverage_level {
            continue;
        }

        // We capture values for both the specific network requested and a general "NotApplicable"
        // network, which can serve as a fallback.
        let is_specific_network = benefit.network_status == network;
        let is_fallback_network = benefit.network_status == NetworkStatus::NotApplicable;

        let values_to_update = if is_specific_network {
            &mut specific_network_values
        } else if is_fallback_network {
            &mut fallback_network_values
        } else {
            continue; // This benefit is for a different network.
        };

        // Update the values, taking the max if multiple entries exist (e.g., individual vs. overall).
        match benefit.time_period {
            TimePeriod::CalendarYear => {
                values_to_update.total = Some(
                    values_to_update
                        .total
                        .unwrap_or(0.0)
                        .max(benefit.amount.unwrap_or(0.0)),
                )
            }
            TimePeriod::Remaining => {
                values_to_update.remaining = Some(
                    values_to_update
                        .remaining
                        .unwrap_or(0.0)
                        .max(benefit.amount.unwrap_or(0.0)),
                )
            }
            TimePeriod::YearToDate => {
                values_to_update.year_to_date = Some(
                    values_to_update
                        .year_to_date
                        .unwrap_or(0.0)
                        .max(benefit.amount.unwrap_or(0.0)),
                )
            }
            _ => (),
        }
    }

    // Prioritize the values from the specific network. If they don't exist, use the fallback.
    let final_values = if specific_network_values.total.is_some() {
        specific_network_values
    } else {
        fallback_network_values
    };

    // If we have a total, we can construct the result.
    if let Some(total) = final_values.total {
        // A "remaining" value from the payer is most accurate. If not present,
        // calculate it from the total and year-to-date amount.
        let remaining = final_values
            .remaining
            .or_else(|| final_values.year_to_date.map(|ytd| (total - ytd).max(0.0)));
        Some(FinancialLimitInfo { total, remaining })
    } else {
        None
    }
}

/// Finds the copay amount from a list of applicable benefits.
///
/// Searches through benefits that have already been filtered for applicability
/// to find a fixed copay amount. Returns the first valid copay found.
///
/// # Arguments
///
/// * `benefits` - Pre-filtered list of [`ParsedBenefit`] applicable to the current calculation
///
/// # Returns
///
/// `Some(amount)` if a [`BenefitType::Copayment`] with a positive amount is found,
/// `None` if no applicable copay exists.
fn get_copay(benefits: &[&ParsedBenefit]) -> Option<f64> {
    benefits
        .iter()
        .find(|b| matches!(b.benefit_type, BenefitType::Copayment))
        .and_then(|b| b.amount)
        .filter(|amount| *amount > 0.0)
}

/// Finds the coinsurance rate from a list of applicable benefits.
///
/// Searches through benefits that have already been filtered for applicability
/// to find a coinsurance percentage. Returns the first valid percentage found.
///
/// # Arguments
///
/// * `benefits` - Pre-filtered list of [`ParsedBenefit`] applicable to the current calculation
///
/// # Returns
///
/// `Some(rate)` if a [`BenefitType::Coinsurance`] with a percentage is found,
/// `None` if no applicable coinsurance exists.
///
/// # Note
///
/// The returned rate is expressed as a decimal (e.g., 0.20 for 20% coinsurance).
fn get_coinsurance_rate(benefits: &[&ParsedBenefit]) -> Option<f64> {
    benefits
        .iter()
        .find(|b| matches!(b.benefit_type, BenefitType::Coinsurance))
        .and_then(|b| b.percent)
}
#[cfg(test)]
/// Comprehensive tests for healthcare benefits calculation functionality.
///
/// This test module validates the core calculation logic using real-world eligibility
/// response data. Tests cover various scenarios including:
///
/// - Primary care visits with [`BenefitType::Copayment`]
/// - Specialist visits with different copay amounts
/// - Emergency room visits with special copay rules
/// - Surgical procedures with [`BenefitType::Deductible`] and [`BenefitType::Coinsurance`]
/// - [`NetworkStatus::OutOfNetwork`] services with different benefit structures
/// - [`BenefitType::OutOfPocketMax`] limitations
///
/// The tests use embedded sample eligibility response data that contains
/// typical benefit structures from insurance payers.
mod tests {
    use super::*;
    use crate::models::EligibilityCheckResponseContent; // Assuming your root model is EligibilityResponse

    /// Embedded test data: Sample eligibility response JSON containing benefit information.
    const RESPONSE_JSON: &str = r#"{
  "controlNumber": "000000000",
  "tradingPartnerServiceId": "TEST1",
  "provider": {
    "npi": "0000000000",
    "entityIdentifier": "Provider",
    "providerName": "TEST PROVIDER INC.",
    "entityType": "Non-Person Entity"
  },
  "planInformation": {
    "groupDescription": "TEST GROUP",
    "planNumber": "0000 0000",
    "groupNumber": "000000"
  },
  "subscriber": {
    "lastName": "PATIENT",
    "planNumber": "0000 0000",
    "entityIdentifier": "Insured or Subscriber",
    "groupNumber": "000000",
    "entityType": "Person",
    "groupDescription": "SAMPLE GROUP",
    "address": {
      "address1": "123 TEST ST",
      "city": "ANONYMOUS CITY",
      "postalCode": "00000",
      "state": "CA"
    },
    "gender": "M",
    "firstName": "TEST",
    "memberId": "0000000000",
    "dateOfBirth": "19800101"
  },
  "planStatus": [
    {
      "serviceTypeCodes": [
        "30"
      ],
      "status": "Active Coverage",
      "planDetails": "TEST PLAN PLUS",
      "statusCode": "1"
    },
    {
      "serviceTypeCodes": [
        "86"
      ],
      "statusCode": "1",
      "status": "Active Coverage"
    },
    {
      "serviceTypeCodes": [
        "50"
      ],
      "statusCode": "1",
      "status": "Active Coverage"
    },
    {
      "serviceTypeCodes": [
        "1",
        "33",
        "47",
        "48",
        "50",
        "86",
        "98",
        "MH",
        "PT",
        "UC"
      ],
      "status": "Active Coverage",
      "statusCode": "1"
    },
    {
      "serviceTypeCodes": [
        "96"
      ],
      "statusCode": "1",
      "status": "Active Coverage"
    }
  ],
  "subscriberTraceNumbers": [
    {
      "originatingCompanyIdentifier": "0000000000",
      "traceTypeCode": "1",
      "referenceIdentification": "TEST000000000000000000000000",
      "traceType": "Current Transaction Trace Numbers"
    }
  ],
  "meta": {
    "outboundTraceId": "TEST000000000000000000000000",
    "submitterId": "000000000",
    "applicationMode": "production",
    "senderId": "STEDI",
    "traceId": "TEST000000000000000000000000"
  },
  "payer": {
    "payorIdentification": "00000",
    "contactInformation": {
      "contacts": [
        {
          "communicationNumber": "WWW.EXAMPLE.COM",
          "communicationMode": "Uniform Resource Locator (URL)"
        }
      ]
    },
    "entityIdentifier": "Payer",
    "entityType": "Non-Person Entity",
    "name": "TEST INSURANCE"
  },
  "x12": "",
  "errors": [],
  "eligibilitySearchId": "00000000-0000-0000-0000-000000000000",
  "benefitsInformation": [
    {
      "planCoverage": "TEST PLAN PLUS",
      "serviceTypeCodes": [
        "30"
      ],
      "benefitsRelatedEntities": [
        {
          "entityIdentification": "PI",
          "entityIdentificationValue": "00000",
          "contactInformation": {
            "contacts": [
              {
                "communicationMode": "Uniform Resource Locator (URL)",
                "communicationNumber": "WWW.EXAMPLE.COM"
              }
            ]
          },
          "entityIdentifier": "Payer",
          "entityName": "TEST INSURANCE",
          "entityType": "Non-Person Entity",
          "address": {
            "address1": "P.O. BOX 00000",
            "state": "CA",
            "postalCode": "000000000",
            "city": "ANONYMOUS CITY"
          }
        }
      ],
      "benefitsRelatedEntity": {
        "entityType": "Non-Person Entity",
        "entityName": "TEST INSURANCE",
        "entityIdentification": "PI",
        "entityIdentificationValue": "00000",
        "contactInformation": {
          "contacts": [
            {
              "communicationMode": "Uniform Resource Locator (URL)",
              "communicationNumber": "WWW.EXAMPLE.COM"
            }
          ]
        },
        "address": {
          "postalCode": "000000000",
          "address1": "P.O. BOX 00000",
          "city": "ANONYMOUS CITY",
          "state": "CA"
        },
        "entityIdentifier": "Payer"
      },
      "authOrCertIndicator": "Y",
      "additionalInformation": [
        {
          "description": "FUNDING TYPE = SELF INSURED - LARGE GROUP"
        },
        {
          "description": "A PRIOR AUTHORIZATION OR NOTIFICATION INQUIRY REQUEST MAY BE SUBMITTED"
        }
      ],
      "code": "1",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "name": "Active Coverage",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "insuranceTypeCode": "CO"
    },
    {
      "benefitAmount": "197.07",
      "serviceTypeCodes": [
        "30"
      ],
      "timeQualifierCode": "24",
      "timeQualifier": "Year to Date",
      "inPlanNetworkIndicatorCode": "W",
      "code": "C",
      "coverageLevelCode": "FAM",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "inPlanNetworkIndicator": "Not Applicable",
      "name": "Deductible",
      "coverageLevel": "Family"
    },
    {
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "timeQualifier": "Year to Date",
      "name": "Deductible",
      "coverageLevelCode": "IND",
      "serviceTypeCodes": [
        "30"
      ],
      "coverageLevel": "Individual",
      "inPlanNetworkIndicatorCode": "W",
      "code": "C",
      "timeQualifierCode": "24",
      "benefitAmount": "0",
      "inPlanNetworkIndicator": "Not Applicable"
    },
    {
      "benefitAmount": "638.85",
      "inPlanNetworkIndicator": "Not Applicable",
      "timeQualifier": "Year to Date",
      "code": "G",
      "coverageLevel": "Family",
      "name": "Out of Pocket (Stop Loss)",
      "serviceTypeCodes": [
        "30"
      ],
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "insuranceTypeCode": "CO",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "coverageLevelCode": "FAM",
      "timeQualifierCode": "24",
      "inPlanNetworkIndicatorCode": "W"
    },
    {
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "coverageLevel": "Individual",
      "inPlanNetworkIndicatorCode": "W",
      "timeQualifierCode": "24",
      "code": "G",
      "insuranceTypeCode": "CO",
      "coverageLevelCode": "IND",
      "name": "Out of Pocket (Stop Loss)",
      "inPlanNetworkIndicator": "Not Applicable",
      "serviceTypeCodes": [
        "30"
      ],
      "benefitAmount": "136.21",
      "timeQualifier": "Year to Date",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ]
    },
    {
      "inPlanNetworkIndicatorCode": "N",
      "inPlanNetworkIndicator": "No",
      "timeQualifierCode": "29",
      "insuranceTypeCode": "CO",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "name": "Out of Pocket (Stop Loss)",
      "code": "G",
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "30"
      ],
      "timeQualifier": "Remaining",
      "coverageLevelCode": "IND",
      "benefitAmount": "9863.79",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ]
    },
    {
      "serviceTypeCodes": [
        "30"
      ],
      "name": "Deductible",
      "timeQualifierCode": "29",
      "timeQualifier": "Remaining",
      "inPlanNetworkIndicatorCode": "Y",
      "coverageLevel": "Individual",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "inPlanNetworkIndicator": "Yes",
      "code": "C",
      "coverageLevelCode": "IND",
      "benefitAmount": "1500"
    },
    {
      "name": "Deductible",
      "timeQualifierCode": "23",
      "inPlanNetworkIndicatorCode": "Y",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "timeQualifier": "Calendar Year",
      "coverageLevel": "Family",
      "code": "C",
      "coverageLevelCode": "FAM",
      "serviceTypeCodes": [
        "30"
      ],
      "inPlanNetworkIndicator": "Yes",
      "benefitAmount": "4500"
    },
    {
      "coverageLevelCode": "FAM",
      "name": "Out of Pocket (Stop Loss)",
      "benefitAmount": "20000",
      "serviceTypeCodes": [
        "30"
      ],
      "timeQualifierCode": "23",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "timeQualifier": "Calendar Year",
      "inPlanNetworkIndicator": "No",
      "coverageLevel": "Family",
      "inPlanNetworkIndicatorCode": "N",
      "insuranceTypeCode": "CO",
      "code": "G",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)"
    },
    {
      "benefitAmount": "6000",
      "serviceTypeCodes": [
        "30"
      ],
      "coverageLevelCode": "FAM",
      "coverageLevel": "Family",
      "name": "Deductible",
      "inPlanNetworkIndicator": "No",
      "code": "C",
      "timeQualifierCode": "23",
      "timeQualifier": "Calendar Year",
      "inPlanNetworkIndicatorCode": "N",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ]
    },
    {
      "code": "G",
      "serviceTypeCodes": [
        "30"
      ],
      "insuranceTypeCode": "CO",
      "benefitAmount": "19361.15",
      "coverageLevel": "Family",
      "name": "Out of Pocket (Stop Loss)",
      "coverageLevelCode": "FAM",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "timeQualifierCode": "29",
      "timeQualifier": "Remaining",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "inPlanNetworkIndicatorCode": "N",
      "inPlanNetworkIndicator": "No"
    },
    {
      "code": "G",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "timeQualifierCode": "23",
      "timeQualifier": "Calendar Year",
      "insuranceTypeCode": "CO",
      "benefitAmount": "10000",
      "coverageLevel": "Individual",
      "inPlanNetworkIndicatorCode": "N",
      "inPlanNetworkIndicator": "No",
      "coverageLevelCode": "IND",
      "name": "Out of Pocket (Stop Loss)",
      "serviceTypeCodes": [
        "30"
      ]
    },
    {
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "coverageLevel": "Individual",
      "timeQualifierCode": "29",
      "timeQualifier": "Remaining",
      "benefitAmount": "2000",
      "inPlanNetworkIndicatorCode": "N",
      "name": "Deductible",
      "coverageLevelCode": "IND",
      "serviceTypeCodes": [
        "30"
      ],
      "inPlanNetworkIndicator": "No",
      "code": "C"
    },
    {
      "inPlanNetworkIndicatorCode": "Y",
      "code": "C",
      "serviceTypeCodes": [
        "30"
      ],
      "inPlanNetworkIndicator": "Yes",
      "coverageLevelCode": "FAM",
      "name": "Deductible",
      "timeQualifier": "Remaining",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "benefitAmount": "4302.93",
      "coverageLevel": "Family",
      "timeQualifierCode": "29"
    },
    {
      "name": "Deductible",
      "benefitAmount": "5802.93",
      "inPlanNetworkIndicator": "No",
      "coverageLevel": "Family",
      "timeQualifier": "Remaining",
      "serviceTypeCodes": [
        "30"
      ],
      "coverageLevelCode": "FAM",
      "code": "C",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "timeQualifierCode": "29",
      "inPlanNetworkIndicatorCode": "N"
    },
    {
      "inPlanNetworkIndicator": "Yes",
      "inPlanNetworkIndicatorCode": "Y",
      "coverageLevelCode": "IND",
      "timeQualifierCode": "23",
      "benefitAmount": "1500",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "name": "Deductible",
      "code": "C",
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "30"
      ],
      "timeQualifier": "Calendar Year"
    },
    {
      "inPlanNetworkIndicatorCode": "Y",
      "name": "Out of Pocket (Stop Loss)",
      "timeQualifier": "Calendar Year",
      "inPlanNetworkIndicator": "Yes",
      "benefitAmount": "5000",
      "coverageLevel": "Individual",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "serviceTypeCodes": [
        "30"
      ],
      "code": "G",
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "insuranceTypeCode": "CO",
      "timeQualifierCode": "23"
    },
    {
      "timeQualifierCode": "29",
      "inPlanNetworkIndicatorCode": "Y",
      "inPlanNetworkIndicator": "Yes",
      "coverageLevelCode": "IND",
      "serviceTypeCodes": [
        "30"
      ],
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "timeQualifier": "Remaining",
      "benefitAmount": "4863.79",
      "coverageLevel": "Individual",
      "insuranceTypeCode": "CO",
      "code": "G",
      "name": "Out of Pocket (Stop Loss)",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)"
    },
    {
      "inPlanNetworkIndicatorCode": "Y",
      "name": "Out of Pocket (Stop Loss)",
      "inPlanNetworkIndicator": "Yes",
      "coverageLevelCode": "FAM",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "code": "G",
      "serviceTypeCodes": [
        "30"
      ],
      "coverageLevel": "Family",
      "insuranceTypeCode": "CO",
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "timeQualifierCode": "29",
      "timeQualifier": "Remaining",
      "benefitAmount": "9361.15"
    },
    {
      "code": "C",
      "serviceTypeCodes": [
        "30"
      ],
      "inPlanNetworkIndicator": "No",
      "name": "Deductible",
      "timeQualifier": "Calendar Year",
      "inPlanNetworkIndicatorCode": "N",
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "benefitAmount": "2000",
      "timeQualifierCode": "23"
    },
    {
      "insuranceType": "Consolidated Omnibus Budget Reconciliation Act (COBRA)",
      "coverageLevel": "Family",
      "timeQualifierCode": "23",
      "code": "G",
      "serviceTypeCodes": [
        "30"
      ],
      "insuranceTypeCode": "CO",
      "timeQualifier": "Calendar Year",
      "benefitAmount": "10000",
      "coverageLevelCode": "FAM",
      "serviceTypes": [
        "Health Benefit Plan Coverage"
      ],
      "name": "Out of Pocket (Stop Loss)",
      "inPlanNetworkIndicatorCode": "Y",
      "inPlanNetworkIndicator": "Yes"
    },
    {
      "name": "Active Coverage",
      "serviceTypeCodes": [
        "86"
      ],
      "additionalInformation": [
        {
          "description": "NON-EMERGENCY SERVICES"
        }
      ],
      "code": "1",
      "serviceTypes": [
        "Emergency Services"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable"
    },
    {
      "serviceTypes": [
        "Hospital - Outpatient"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "additionalInformation": [
        {
          "description": "FREESTANDING FACILITY"
        }
      ],
      "name": "Active Coverage",
      "serviceTypeCodes": [
        "50"
      ],
      "code": "1",
      "inPlanNetworkIndicator": "Not Applicable"
    },
    {
      "serviceTypeCodes": [
        "1",
        "33",
        "47",
        "48",
        "50",
        "86",
        "98",
        "MH",
        "PT",
        "UC"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "name": "Active Coverage",
      "code": "1",
      "serviceTypes": [
        "Medical Care",
        "Chiropractic",
        "Hospital",
        "Hospital - Inpatient",
        "Hospital - Outpatient",
        "Emergency Services",
        "Professional (Physician) Visit - Office",
        "Mental Health",
        "Physical Therapy",
        "Urgent Care"
      ]
    },
    {
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "code": "1",
      "name": "Active Coverage",
      "serviceTypeCodes": [
        "96"
      ],
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ]
    },
    {
      "name": "Co-Insurance",
      "serviceTypeCodes": [
        "33",
        "UC",
        "48",
        "PT",
        "98"
      ],
      "coverageLevel": "Individual",
      "coverageLevelCode": "IND",
      "benefitPercent": "0.5",
      "inPlanNetworkIndicatorCode": "N",
      "inPlanNetworkIndicator": "No",
      "timeQualifier": "Visit",
      "serviceTypes": [
        "Chiropractic",
        "Urgent Care",
        "Hospital - Inpatient",
        "Physical Therapy",
        "Professional (Physician) Visit - Office"
      ],
      "timeQualifierCode": "27",
      "code": "A"
    },
    {
      "inPlanNetworkIndicator": "Yes",
      "timeQualifier": "Visit",
      "code": "A",
      "serviceTypes": [
        "Hospital - Inpatient"
      ],
      "benefitPercent": "0.2",
      "timeQualifierCode": "27",
      "inPlanNetworkIndicatorCode": "Y",
      "name": "Co-Insurance",
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "48"
      ]
    },
    {
      "code": "A",
      "serviceTypeCodes": [
        "96"
      ],
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "timeQualifierCode": "27",
      "inPlanNetworkIndicator": "Yes",
      "inPlanNetworkIndicatorCode": "Y",
      "coverageLevel": "Individual",
      "benefitPercent": "0",
      "timeQualifier": "Visit",
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "name": "Co-Insurance",
      "coverageLevelCode": "IND"
    },
    {
      "timeQualifierCode": "27",
      "code": "A",
      "name": "Co-Insurance",
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "PT",
        "UC",
        "33",
        "98"
      ],
      "inPlanNetworkIndicatorCode": "Y",
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Physical Therapy",
        "Urgent Care",
        "Chiropractic",
        "Professional (Physician) Visit - Office"
      ],
      "timeQualifier": "Visit",
      "benefitPercent": "0",
      "inPlanNetworkIndicator": "Yes"
    },
    {
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "timeQualifierCode": "27",
      "name": "Co-Insurance",
      "code": "A",
      "coverageLevel": "Individual",
      "inPlanNetworkIndicator": "No",
      "coverageLevelCode": "IND",
      "serviceTypeCodes": [
        "96"
      ],
      "benefitPercent": "0.5",
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "N"
    },
    {
      "coverageLevelCode": "IND",
      "timeQualifierCode": "27",
      "benefitPercent": "0.2",
      "code": "A",
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "Y",
      "inPlanNetworkIndicator": "Yes",
      "eligibilityAdditionalInformation": {
        "codeListQualifierCode": "ZZ",
        "codeListQualifier": "Mutually Defined",
        "industryCode": "22",
        "industry": "Outpatient Hospital"
      },
      "eligibilityAdditionalInformationList": [
        {
          "industryCode": "22",
          "industry": "Outpatient Hospital",
          "codeListQualifier": "Mutually Defined",
          "codeListQualifierCode": "ZZ"
        }
      ],
      "coverageLevel": "Individual",
      "name": "Co-Insurance",
      "serviceTypes": [
        "Hospital - Outpatient"
      ],
      "serviceTypeCodes": [
        "50"
      ]
    },
    {
      "eligibilityAdditionalInformation": {
        "industry": "Outpatient Hospital",
        "codeListQualifierCode": "ZZ",
        "industryCode": "22",
        "codeListQualifier": "Mutually Defined"
      },
      "inPlanNetworkIndicator": "Not Applicable",
      "serviceTypes": [
        "Emergency Services"
      ],
      "serviceTypeCodes": [
        "86"
      ],
      "timeQualifierCode": "27",
      "eligibilityAdditionalInformationList": [
        {
          "industry": "Outpatient Hospital",
          "codeListQualifierCode": "ZZ",
          "codeListQualifier": "Mutually Defined",
          "industryCode": "22"
        }
      ],
      "name": "Co-Insurance",
      "code": "A",
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "timeQualifier": "Visit",
      "benefitPercent": "0",
      "inPlanNetworkIndicatorCode": "W"
    },
    {
      "name": "Co-Insurance",
      "timeQualifierCode": "27",
      "inPlanNetworkIndicator": "Not Applicable",
      "serviceTypeCodes": [
        "86"
      ],
      "serviceTypes": [
        "Emergency Services"
      ],
      "eligibilityAdditionalInformation": {
        "industryCode": "22",
        "codeListQualifierCode": "ZZ",
        "codeListQualifier": "Mutually Defined",
        "industry": "Outpatient Hospital"
      },
      "eligibilityAdditionalInformationList": [
        {
          "codeListQualifier": "Mutually Defined",
          "industryCode": "22",
          "codeListQualifierCode": "ZZ",
          "industry": "Outpatient Hospital"
        }
      ],
      "code": "A",
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "W",
      "additionalInformation": [
        {
          "description": "NON-EMERGENCY SERVICES"
        }
      ],
      "benefitPercent": "0"
    },
    {
      "inPlanNetworkIndicator": "Yes",
      "eligibilityAdditionalInformationList": [
        {
          "codeListQualifierCode": "ZZ",
          "industry": "Ambulatory Surgical Center",
          "codeListQualifier": "Mutually Defined",
          "industryCode": "24"
        }
      ],
      "code": "A",
      "serviceTypeCodes": [
        "50"
      ],
      "coverageLevel": "Individual",
      "coverageLevelCode": "IND",
      "timeQualifierCode": "27",
      "name": "Co-Insurance",
      "benefitPercent": "0.2",
      "inPlanNetworkIndicatorCode": "Y",
      "eligibilityAdditionalInformation": {
        "codeListQualifierCode": "ZZ",
        "industryCode": "24",
        "industry": "Ambulatory Surgical Center",
        "codeListQualifier": "Mutually Defined"
      },
      "serviceTypes": [
        "Hospital - Outpatient"
      ],
      "additionalInformation": [
        {
          "description": "FREESTANDING FACILITY"
        }
      ],
      "timeQualifier": "Visit"
    },
    {
      "timeQualifierCode": "27",
      "timeQualifier": "Visit",
      "code": "A",
      "serviceTypes": [
        "Hospital - Outpatient"
      ],
      "benefitPercent": "0.5",
      "inPlanNetworkIndicatorCode": "N",
      "serviceTypeCodes": [
        "50"
      ],
      "name": "Co-Insurance",
      "eligibilityAdditionalInformation": {
        "industry": "Ambulatory Surgical Center",
        "codeListQualifierCode": "ZZ",
        "industryCode": "24",
        "codeListQualifier": "Mutually Defined"
      },
      "coverageLevel": "Individual",
      "coverageLevelCode": "IND",
      "additionalInformation": [
        {
          "description": "FREESTANDING FACILITY"
        }
      ],
      "eligibilityAdditionalInformationList": [
        {
          "industryCode": "24",
          "codeListQualifierCode": "ZZ",
          "codeListQualifier": "Mutually Defined",
          "industry": "Ambulatory Surgical Center"
        }
      ],
      "inPlanNetworkIndicator": "No"
    },
    {
      "serviceTypeCodes": [
        "50"
      ],
      "benefitPercent": "0.5",
      "inPlanNetworkIndicator": "No",
      "timeQualifier": "Visit",
      "serviceTypes": [
        "Hospital - Outpatient"
      ],
      "inPlanNetworkIndicatorCode": "N",
      "coverageLevel": "Individual",
      "timeQualifierCode": "27",
      "eligibilityAdditionalInformation": {
        "codeListQualifier": "Mutually Defined",
        "industryCode": "22",
        "codeListQualifierCode": "ZZ",
        "industry": "Outpatient Hospital"
      },
      "code": "A",
      "eligibilityAdditionalInformationList": [
        {
          "industry": "Outpatient Hospital",
          "codeListQualifier": "Mutually Defined",
          "codeListQualifierCode": "ZZ",
          "industryCode": "22"
        }
      ],
      "coverageLevelCode": "IND",
      "name": "Co-Insurance"
    },
    {
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "98",
        "33",
        "PT"
      ],
      "timeQualifier": "Visit",
      "code": "B",
      "timeQualifierCode": "27",
      "benefitAmount": "25",
      "inPlanNetworkIndicatorCode": "Y",
      "serviceTypes": [
        "Professional (Physician) Visit - Office",
        "Chiropractic",
        "Physical Therapy"
      ],
      "name": "Co-Payment",
      "coverageLevelCode": "IND",
      "inPlanNetworkIndicator": "Yes"
    },
    {
      "serviceTypes": [
        "Hospital - Inpatient"
      ],
      "inPlanNetworkIndicator": "Not Applicable",
      "coverageLevel": "Individual",
      "code": "B",
      "timeQualifierCode": "27",
      "benefitAmount": "0",
      "inPlanNetworkIndicatorCode": "W",
      "coverageLevelCode": "IND",
      "name": "Co-Payment",
      "serviceTypeCodes": [
        "48"
      ],
      "timeQualifier": "Visit"
    },
    {
      "serviceTypeCodes": [
        "33",
        "PT",
        "98"
      ],
      "inPlanNetworkIndicator": "No",
      "inPlanNetworkIndicatorCode": "N",
      "coverageLevel": "Individual",
      "code": "B",
      "name": "Co-Payment",
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Chiropractic",
        "Physical Therapy",
        "Professional (Physician) Visit - Office"
      ],
      "timeQualifierCode": "27",
      "timeQualifier": "Visit",
      "benefitAmount": "0"
    },
    {
      "timeQualifier": "Visit",
      "code": "B",
      "timeQualifierCode": "27",
      "coverageLevelCode": "IND",
      "serviceTypeCodes": [
        "96"
      ],
      "coverageLevel": "Individual",
      "name": "Co-Payment",
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "benefitAmount": "50",
      "inPlanNetworkIndicator": "Yes",
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "inPlanNetworkIndicatorCode": "Y"
    },
    {
      "coverageLevelCode": "IND",
      "benefitAmount": "60",
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "code": "B",
      "coverageLevel": "Individual",
      "name": "Co-Payment",
      "serviceTypeCodes": [
        "UC"
      ],
      "serviceTypes": [
        "Urgent Care"
      ],
      "timeQualifierCode": "27"
    },
    {
      "name": "Co-Payment",
      "inPlanNetworkIndicatorCode": "N",
      "code": "B",
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "timeQualifier": "Visit",
      "inPlanNetworkIndicator": "No",
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "timeQualifierCode": "27",
      "benefitAmount": "0",
      "serviceTypeCodes": [
        "96"
      ]
    },
    {
      "timeQualifier": "Visit",
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "benefitAmount": "0",
      "timeQualifierCode": "27",
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "additionalInformation": [
        {
          "description": "FREESTANDING FACILITY"
        }
      ],
      "eligibilityAdditionalInformation": {
        "codeListQualifierCode": "ZZ",
        "codeListQualifier": "Mutually Defined",
        "industry": "Ambulatory Surgical Center",
        "industryCode": "24"
      },
      "eligibilityAdditionalInformationList": [
        {
          "industry": "Ambulatory Surgical Center",
          "codeListQualifier": "Mutually Defined",
          "codeListQualifierCode": "ZZ",
          "industryCode": "24"
        }
      ],
      "serviceTypeCodes": [
        "50"
      ],
      "name": "Co-Payment",
      "code": "B",
      "serviceTypes": [
        "Hospital - Outpatient"
      ]
    },
    {
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "86"
      ],
      "benefitAmount": "300",
      "code": "B",
      "timeQualifierCode": "27",
      "serviceTypes": [
        "Emergency Services"
      ],
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "additionalInformation": [
        {
          "description": "NON-EMERGENCY SERVICES"
        }
      ],
      "eligibilityAdditionalInformation": {
        "industry": "Outpatient Hospital",
        "industryCode": "22",
        "codeListQualifierCode": "ZZ",
        "codeListQualifier": "Mutually Defined"
      },
      "name": "Co-Payment",
      "eligibilityAdditionalInformationList": [
        {
          "codeListQualifierCode": "ZZ",
          "industryCode": "22",
          "industry": "Outpatient Hospital",
          "codeListQualifier": "Mutually Defined"
        }
      ],
      "coverageLevelCode": "IND"
    },
    {
      "eligibilityAdditionalInformationList": [
        {
          "codeListQualifier": "Mutually Defined",
          "industry": "Outpatient Hospital",
          "industryCode": "22",
          "codeListQualifierCode": "ZZ"
        }
      ],
      "serviceTypeCodes": [
        "50"
      ],
      "benefitAmount": "0",
      "code": "B",
      "timeQualifierCode": "27",
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "name": "Co-Payment",
      "coverageLevel": "Individual",
      "eligibilityAdditionalInformation": {
        "codeListQualifier": "Mutually Defined",
        "industry": "Outpatient Hospital",
        "industryCode": "22",
        "codeListQualifierCode": "ZZ"
      },
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Hospital - Outpatient"
      ]
    },
    {
      "timeQualifier": "Visit",
      "serviceTypeCodes": [
        "86"
      ],
      "serviceTypes": [
        "Emergency Services"
      ],
      "code": "B",
      "benefitAmount": "300",
      "eligibilityAdditionalInformation": {
        "codeListQualifierCode": "ZZ",
        "industryCode": "22",
        "codeListQualifier": "Mutually Defined",
        "industry": "Outpatient Hospital"
      },
      "eligibilityAdditionalInformationList": [
        {
          "codeListQualifierCode": "ZZ",
          "industryCode": "22",
          "industry": "Outpatient Hospital",
          "codeListQualifier": "Mutually Defined"
        }
      ],
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "inPlanNetworkIndicatorCode": "W",
      "name": "Co-Payment",
      "inPlanNetworkIndicator": "Not Applicable",
      "timeQualifierCode": "27"
    },
    {
      "coverageLevel": "Family",
      "coverageLevelCode": "FAM",
      "timeQualifierCode": "23",
      "timeQualifier": "Calendar Year",
      "serviceTypes": [
        "Emergency Services"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "benefitAmount": "0",
      "code": "C",
      "name": "Deductible",
      "serviceTypeCodes": [
        "86"
      ]
    },
    {
      "coverageLevel": "Family",
      "serviceTypes": [
        "Urgent Care",
        "Professional (Physician) Visit - Office",
        "Chiropractic",
        "Physical Therapy"
      ],
      "benefitAmount": "0",
      "inPlanNetworkIndicator": "Yes",
      "serviceTypeCodes": [
        "UC",
        "98",
        "33",
        "PT"
      ],
      "timeQualifier": "Remaining",
      "code": "C",
      "timeQualifierCode": "29",
      "inPlanNetworkIndicatorCode": "Y",
      "coverageLevelCode": "FAM",
      "name": "Deductible"
    },
    {
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "96"
      ],
      "benefitAmount": "0",
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "inPlanNetworkIndicatorCode": "Y",
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "name": "Deductible",
      "timeQualifierCode": "23",
      "inPlanNetworkIndicator": "Yes",
      "code": "C",
      "coverageLevelCode": "IND",
      "timeQualifier": "Calendar Year"
    },
    {
      "code": "C",
      "timeQualifierCode": "23",
      "additionalInformation": [
        {
          "description": "NON-EMERGENCY SERVICES"
        }
      ],
      "serviceTypes": [
        "Emergency Services"
      ],
      "benefitAmount": "0",
      "inPlanNetworkIndicator": "Not Applicable",
      "coverageLevel": "Family",
      "serviceTypeCodes": [
        "86"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "name": "Deductible",
      "coverageLevelCode": "FAM",
      "timeQualifier": "Calendar Year"
    },
    {
      "serviceTypeCodes": [
        "UC",
        "PT",
        "98",
        "33"
      ],
      "coverageLevelCode": "IND",
      "inPlanNetworkIndicator": "Yes",
      "code": "C",
      "coverageLevel": "Individual",
      "name": "Deductible",
      "serviceTypes": [
        "Urgent Care",
        "Physical Therapy",
        "Professional (Physician) Visit - Office",
        "Chiropractic"
      ],
      "benefitAmount": "0",
      "inPlanNetworkIndicatorCode": "Y",
      "timeQualifierCode": "23",
      "timeQualifier": "Calendar Year"
    },
    {
      "name": "Deductible",
      "code": "C",
      "serviceTypes": [
        "Emergency Services"
      ],
      "inPlanNetworkIndicatorCode": "W",
      "serviceTypeCodes": [
        "86"
      ],
      "timeQualifier": "Remaining",
      "inPlanNetworkIndicator": "Not Applicable",
      "benefitAmount": "0",
      "coverageLevel": "Individual",
      "coverageLevelCode": "IND",
      "timeQualifierCode": "29"
    },
    {
      "coverageLevelCode": "IND",
      "timeQualifier": "Calendar Year",
      "inPlanNetworkIndicator": "Not Applicable",
      "serviceTypes": [
        "Emergency Services"
      ],
      "name": "Deductible",
      "inPlanNetworkIndicatorCode": "W",
      "coverageLevel": "Individual",
      "benefitAmount": "0",
      "additionalInformation": [
        {
          "description": "NON-EMERGENCY SERVICES"
        }
      ],
      "serviceTypeCodes": [
        "86"
      ],
      "code": "C",
      "timeQualifierCode": "23"
    },
    {
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "timeQualifierCode": "29",
      "coverageLevelCode": "IND",
      "timeQualifier": "Remaining",
      "inPlanNetworkIndicator": "Yes",
      "code": "C",
      "serviceTypeCodes": [
        "96"
      ],
      "coverageLevel": "Individual",
      "name": "Deductible",
      "inPlanNetworkIndicatorCode": "Y",
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "benefitAmount": "0"
    },
    {
      "serviceTypeCodes": [
        "86"
      ],
      "timeQualifierCode": "23",
      "code": "C",
      "name": "Deductible",
      "timeQualifier": "Calendar Year",
      "benefitAmount": "0",
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Emergency Services"
      ],
      "coverageLevel": "Individual",
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable"
    },
    {
      "serviceTypes": [
        "Chiropractic",
        "Professional (Physician) Visit - Office",
        "Physical Therapy",
        "Urgent Care"
      ],
      "coverageLevel": "Individual",
      "timeQualifier": "Remaining",
      "code": "C",
      "inPlanNetworkIndicatorCode": "Y",
      "serviceTypeCodes": [
        "33",
        "98",
        "PT",
        "UC"
      ],
      "timeQualifierCode": "29",
      "name": "Deductible",
      "coverageLevelCode": "IND",
      "benefitAmount": "0",
      "inPlanNetworkIndicator": "Yes"
    },
    {
      "name": "Deductible",
      "code": "C",
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "inPlanNetworkIndicator": "Yes",
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "serviceTypeCodes": [
        "96"
      ],
      "coverageLevel": "Family",
      "timeQualifierCode": "23",
      "timeQualifier": "Calendar Year",
      "benefitAmount": "0",
      "inPlanNetworkIndicatorCode": "Y",
      "coverageLevelCode": "FAM"
    },
    {
      "additionalInformation": [
        {
          "description": "SPECIALIST"
        }
      ],
      "serviceTypeCodes": [
        "96"
      ],
      "coverageLevelCode": "FAM",
      "timeQualifier": "Remaining",
      "code": "C",
      "benefitAmount": "0",
      "coverageLevel": "Family",
      "name": "Deductible",
      "timeQualifierCode": "29",
      "serviceTypes": [
        "Professional (Physician)"
      ],
      "inPlanNetworkIndicatorCode": "Y",
      "inPlanNetworkIndicator": "Yes"
    },
    {
      "timeQualifier": "Calendar Year",
      "timeQualifierCode": "23",
      "name": "Deductible",
      "coverageLevelCode": "FAM",
      "coverageLevel": "Family",
      "code": "C",
      "inPlanNetworkIndicator": "Yes",
      "serviceTypes": [
        "Physical Therapy",
        "Urgent Care",
        "Chiropractic",
        "Professional (Physician) Visit - Office"
      ],
      "serviceTypeCodes": [
        "PT",
        "UC",
        "33",
        "98"
      ],
      "inPlanNetworkIndicatorCode": "Y",
      "benefitAmount": "0"
    },
    {
      "serviceTypeCodes": [
        "86"
      ],
      "serviceTypes": [
        "Emergency Services"
      ],
      "timeQualifierCode": "29",
      "inPlanNetworkIndicatorCode": "W",
      "name": "Deductible",
      "code": "C",
      "coverageLevel": "Family",
      "benefitAmount": "0",
      "timeQualifier": "Remaining",
      "inPlanNetworkIndicator": "Not Applicable",
      "coverageLevelCode": "FAM"
    },
    {
      "benefitQuantity": "20",
      "coverageLevel": "Individual",
      "serviceTypeCodes": [
        "33"
      ],
      "serviceTypes": [
        "Chiropractic"
      ],
      "code": "F",
      "quantityQualifierCode": "VS",
      "inPlanNetworkIndicatorCode": "W",
      "additionalInformation": [
        {
          "description": "REHABILITATIVE"
        }
      ],
      "coverageLevelCode": "IND",
      "timeQualifierCode": "29",
      "timeQualifier": "Remaining",
      "name": "Limitations",
      "quantityQualifier": "Visits",
      "inPlanNetworkIndicator": "Not Applicable"
    },
    {
      "timeQualifier": "Remaining",
      "name": "Limitations",
      "inPlanNetworkIndicator": "Not Applicable",
      "additionalInformation": [
        {
          "description": "LIMITATION IS COMBINED FOR PT, AD, AND AF FOR BOTH IN AND OUT OF NETWORK"
        },
        {
          "description": "REHABILITATIVE"
        }
      ],
      "quantityQualifierCode": "VS",
      "coverageLevel": "Individual",
      "serviceTypes": [
        "Physical Therapy"
      ],
      "code": "F",
      "timeQualifierCode": "29",
      "benefitQuantity": "60",
      "quantityQualifier": "Visits",
      "inPlanNetworkIndicatorCode": "W",
      "serviceTypeCodes": [
        "PT"
      ],
      "coverageLevelCode": "IND"
    },
    {
      "coverageLevelCode": "IND",
      "coverageLevel": "Individual",
      "serviceTypes": [
        "Physical Therapy",
        "Chiropractic"
      ],
      "quantityQualifier": "Visits",
      "benefitQuantity": "3",
      "inPlanNetworkIndicatorCode": "Y",
      "code": "F",
      "serviceTypeCodes": [
        "PT",
        "33"
      ],
      "timeQualifierCode": "29",
      "additionalInformation": [
        {
          "description": "REHABILITATIVE"
        },
        {
          "description": "ADDITIONAL BENEFIT FOR MUSCULOSKELETAL PAIN MANAGEMENT PROGRAM"
        }
      ],
      "timeQualifier": "Remaining",
      "name": "Limitations",
      "inPlanNetworkIndicator": "Yes",
      "quantityQualifierCode": "VS"
    },
    {
      "timeQualifierCode": "23",
      "quantityQualifierCode": "VS",
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Chiropractic"
      ],
      "name": "Limitations",
      "serviceTypeCodes": [
        "33"
      ],
      "timeQualifier": "Calendar Year",
      "quantityQualifier": "Visits",
      "benefitQuantity": "20",
      "coverageLevel": "Individual",
      "additionalInformation": [
        {
          "description": "REHABILITATIVE"
        }
      ],
      "inPlanNetworkIndicator": "Not Applicable",
      "inPlanNetworkIndicatorCode": "W",
      "code": "F"
    },
    {
      "serviceTypes": [
        "Physical Therapy"
      ],
      "coverageLevelCode": "IND",
      "timeQualifierCode": "23",
      "serviceTypeCodes": [
        "PT"
      ],
      "quantityQualifier": "Visits",
      "additionalInformation": [
        {
          "description": "LIMITATION IS COMBINED FOR PT, AD, AND AF FOR BOTH IN AND OUT OF NETWORK"
        },
        {
          "description": "REHABILITATIVE"
        }
      ],
      "name": "Limitations",
      "timeQualifier": "Calendar Year",
      "inPlanNetworkIndicatorCode": "W",
      "inPlanNetworkIndicator": "Not Applicable",
      "quantityQualifierCode": "VS",
      "benefitQuantity": "60",
      "coverageLevel": "Individual",
      "code": "F"
    },
    {
      "name": "Limitations",
      "timeQualifier": "Visit",
      "inPlanNetworkIndicatorCode": "Y",
      "inPlanNetworkIndicator": "Yes",
      "serviceTypeCodes": [
        "48",
        "50",
        "50"
      ],
      "code": "F",
      "coverageLevelCode": "IND",
      "serviceTypes": [
        "Hospital - Inpatient",
        "Hospital - Outpatient",
        "Hospital - Outpatient"
      ],
      "coverageLevel": "Individual",
      "timeQualifierCode": "27",
      "benefitAmount": "0"
    },
    {
      "benefitAmount": "0",
      "inPlanNetworkIndicatorCode": "Y",
      "serviceTypes": [
        "Hospital - Outpatient"
      ],
      "additionalInformation": [
        {
          "description": "ADDITIONAL COVERED PER OCCURRENCE"
        },
        {
          "description": "FREESTANDING FACILITY"
        }
      ],
      "name": "Limitations",
      "code": "F",
      "inPlanNetworkIndicator": "Yes",
      "serviceTypeCodes": [
        "50"
      ],
      "coverageLevel": "Individual",
      "coverageLevelCode": "IND",
      "timeQualifierCode": "27",
      "timeQualifier": "Visit"
    },
    {
      "timeQualifier": "Visit",
      "code": "F",
      "name": "Limitations",
      "inPlanNetworkIndicator": "Yes",
      "serviceTypes": [
        "Hospital - Inpatient",
        "Hospital - Outpatient"
      ],
      "timeQualifierCode": "27",
      "benefitAmount": "0",
      "additionalInformation": [
        {
          "description": "ADDITIONAL COVERED PER OCCURRENCE"
        }
      ],
      "coverageLevelCode": "IND",
      "inPlanNetworkIndicatorCode": "Y",
      "serviceTypeCodes": [
        "48",
        "50"
      ],
      "coverageLevel": "Individual"
    },
    {
      "serviceTypeCodes": [
        "88"
      ],
      "serviceTypes": [
        "Pharmacy"
      ],
      "code": "U",
      "benefitsRelatedEntity": {
        "entityIdentifier": "Vendor",
        "entityType": "Non-Person Entity",
        "contactInformation": {
          "contacts": [
            {
              "communicationNumber": "WWW.EXAMPLE.COM",
              "communicationMode": "Uniform Resource Locator (URL)"
            }
          ]
        },
        "entityName": "TEST PHARMACY"
      },
      "benefitsRelatedEntities": [
        {
          "entityName": "TEST PHARMACY",
          "contactInformation": {
            "contacts": [
              {
                "communicationMode": "Uniform Resource Locator (URL)",
                "communicationNumber": "WWW.EXAMPLE.COM"
              }
            ]
          },
          "entityType": "Non-Person Entity",
          "entityIdentifier": "Vendor"
        }
      ],
      "name": "Contact Following Entity for Eligibility or Benefit Information"
    },
    {
      "name": "Contact Following Entity for Eligibility or Benefit Information",
      "benefitsRelatedEntity": {
        "contactInformation": {
          "contacts": [
            {
              "communicationMode": "Uniform Resource Locator (URL)",
              "communicationNumber": "WWW.EXAMPLE.COM"
            }
          ]
        },
        "entityType": "Non-Person Entity",
        "entityIdentifier": "Vendor",
        "entityName": "TEST DENTAL"
      },
      "serviceTypeCodes": [
        "35"
      ],
      "serviceTypes": [
        "Dental Care"
      ],
      "benefitsRelatedEntities": [
        {
          "entityIdentifier": "Vendor",
          "entityName": "TEST DENTAL",
          "contactInformation": {
            "contacts": [
              {
                "communicationNumber": "WWW.EXAMPLE.COM",
                "communicationMode": "Uniform Resource Locator (URL)"
              }
            ]
          },
          "entityType": "Non-Person Entity"
        }
      ],
      "code": "U"
    },
    {
      "benefitsRelatedEntities": [
        {
          "entityIdentifier": "Provider",
          "entityName": "TEST PROVIDER INC.",
          "entityIdentification": "XX",
          "entityIdentificationValue": "0000000000",
          "entityType": "Non-Person Entity"
        }
      ],
      "code": "X",
      "name": "Health Care Facility",
      "benefitsRelatedEntity": {
        "entityName": "TEST PROVIDER INC.",
        "entityIdentifier": "Provider",
        "entityType": "Non-Person Entity",
        "entityIdentification": "XX",
        "entityIdentificationValue": "0000000000"
      }
    }
  ],
  "planDateInformation": {
    "plan": "20250801-20251231",
    "premiumPaidToDateEnd": "99991231"
  },
  "reassociationKey": "0000000000"
}"#;

    /// Embedded test data: Sample provider cost information (currently unused but available for future tests).
    #[allow(dead_code)]
    const RESPONSE_COST_JSON: &str = r#"[{"npi": "1316263759", "name": "Lavanya P Krishnan", "specialty": "General Surgery", "rating": 9.02, "review_count": 56, "location": "459 Geary St, San Francisco, CA 94102, US", "distance": "1.2 miles", "phone": "4153295100", "credentials": ["MD", "Dermatology"], "languages": ["english"], "profile_url": "https://www.doximity.com/pub/lavanya-krishnan-md", "ranking_score": 0.2329, "mrf_average_cost": 96.41, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 96.41}]}, {"npi": "1235162447", "name": "David J Macgregor", "specialty": "Internal Medicine", "rating": 9.07, "review_count": 30, "location": "450 Sutter St # 1824, San Francisco, CA 941084111, US", "distance": "1.2 miles", "phone": "4159899400", "credentials": ["MD", "Dermatology"], "languages": ["english", "spanish", "tagalog", "castilian"], "profile_url": "https://www.vitals.com/doctors/Dr_David_Macgregor.html", "ranking_score": 0.2057, "mrf_average_cost": 516.98, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 516.98}]}, {"npi": "1760474894", "name": "Lucia R Tuffanelli", "specialty": "Internal Medicine", "rating": 5.19, "review_count": 150, "location": "450 Sutter St # 4002, San Francisco, CA 94108, US", "distance": "1.2 miles", "phone": "4157814083", "credentials": ["MD", "Dermatology"], "languages": ["english", "spanish", "cantonese", "french", "mandarin (chinese)"], "profile_url": "https://www.doximity.com/pub/lucia-tuffanelli-md", "ranking_score": 0.1947, "mrf_average_cost": 192.05, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 192.05}]}, {"npi": "1235213208", "name": "Michael J Dans", "specialty": "Dermatology", "rating": 8.0, "review_count": 64, "location": "490 Post St # 1404, San Francisco, CA 94102, US", "distance": "1.2 miles", "phone": "4157811932", "credentials": ["MD", "PHD", "Dermatology"], "languages": ["english", "russian"], "profile_url": "https://www.yelp.com/biz/michael-dans-md-san-francisco-2?adjust_creative=1QstiIgnJKcjP4BufizP_A&utm_campaign=yelp_api_v3&utm_medium=api_v3_business_search&utm_source=1QstiIgnJKcjP4BufizP_A", "ranking_score": 0.1519, "mrf_average_cost": 192.05, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 192.05}]}, {"npi": "1770700908", "name": "Jennifer J Lee", "specialty": "Dermatology", "rating": 8.0, "review_count": 25, "location": "929 Clay St, San Francisco, CA 94108, US", "distance": "1.4 miles", "phone": "4159829877", "credentials": ["MD"], "languages": ["english", "spanish", "korean", "cantonese", "chinese"], "profile_url": "https://www.doximity.com/pub/jennifer-lee-md-3813", "ranking_score": 0.1444, "mrf_average_cost": 192.05, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 192.05}]}, {"npi": "1093960643", "name": "Aleda A Longwell", "specialty": "General Surgery", "rating": 7.14, "review_count": 14, "location": "1 Daniel Burnham Ct # 350C, San Francisco, CA 94109, US", "distance": "1.8 miles", "phone": "4157716300", "credentials": ["MD", "Dermatology"], "languages": ["english", "spanish", "mandarin (chinese)", "farsi", "persian (farsi)"], "profile_url": "https://www.doximity.com/pub/aleda-longwell-md", "ranking_score": 0.1257, "mrf_average_cost": 192.05, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 192.05}]}, {"npi": "1235660234", "name": "Allison S Dobry", "specialty": "Dermatology", "rating": 0.0, "review_count": 0, "location": "185 Berry St # 1000, San Francisco, CA 94107, US", "distance": "0.2 miles", "phone": "4153537800", "credentials": ["MD", "Dermatology"], "languages": ["english"], "profile_url": "https://www.doximity.com/pub/allison-dobry-md", "ranking_score": 0.0968, "mrf_average_cost": 841.91, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 841.91}]}, {"npi": "1699865741", "name": "Kelly K Wong", "specialty": "Family Medicine", "rating": 8.12, "review_count": 16, "location": "728 Pacific Ave # 6008, San Francisco, CA 94133, US", "distance": "1.5 miles", "phone": "4158057782", "credentials": ["MD", "Pediatrics"], "languages": ["english", "cantonese", "mandarin (chinese)", "chinese", "spanish"], "profile_url": "https://www.doximity.com/pub/kelly-wong-md", "ranking_score": 0.0382, "mrf_average_cost": 192.05, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 192.05}]}, {"npi": "1982625349", "name": "Assad A Hassoun", "specialty": "Transplant Surgery", "rating": 9.04, "review_count": 25, "location": "1100 Van Ness Ave FL 5, San Francisco, CA 94109, US", "distance": "1.7 miles", "phone": "4156001010", "credentials": ["MD", "Surgery"], "languages": ["english", "arabic", "persian (farsi)", "farsi"], "profile_url": "https://www.doximity.com/pub/assad-hassoun-md", "ranking_score": 0.0297, "mrf_average_cost": 593.5, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 593.5}]}, {"npi": "1023570041", "name": "Malcolm N Pyles", "specialty": "Internal Medicine", "rating": 0.0, "review_count": 0, "location": "1 Daniel Burnham Ct # 350C, San Francisco, CA 94109, US", "distance": "1.8 miles", "phone": "4157716300", "credentials": ["MD", "Dermatology"], "languages": ["english", "spanish", "castilian"], "profile_url": "https://www.vitals.com/doctors/Malcolm_N_Pyles.html", "ranking_score": 0.0257, "mrf_average_cost": 192.05, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 192.05}]}, {"npi": "1699201988", "name": "Cristian Daniel Gonzalez", "specialty": "Internal Medicine", "rating": 10.0, "review_count": 4, "location": "2100 Webster St # 212, San Francisco, CA 94115, US", "distance": "2.4 miles", "phone": "4152021540", "credentials": ["MD", "Dermatology"], "languages": ["english", "spanish", "castilian"], "profile_url": "https://www.vitals.com/doctors/1pwd8s/cristian-gonzalez", "ranking_score": 0.0, "mrf_average_cost": 280.77, "procedure_costs": [{"procedure_code": "10040", "procedure_name": "Acne surgery", "procedure_code_type": "CPT", "cost": 280.77}]}]"#;

    /// Helper function to load and parse benefits from embedded test data.
    ///
    /// Parses the embedded sample eligibility response and converts it into parsed benefits
    /// for use in test scenarios. This provides realistic test data that matches
    /// the structure of actual payer responses.
    ///
    /// # Returns
    ///
    /// Vector of [`ParsedBenefit`] structures ready for calculation testing.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON cannot be parsed.
    fn setup_benefits() -> Vec<ParsedBenefit> {
        let response: EligibilityCheckResponseContent =
            serde_json::from_str(RESPONSE_JSON).expect("JSON was not well-formatted");

        parse_benefits(&response.benefits_information.unwrap_or_default())
    }

    /// Tests calculation for a typical primary care office visit with copay.
    ///
    /// Validates that [`NetworkStatus::InNetwork`] primary care visits use the appropriate copay
    /// amount rather than applying deductible and coinsurance. This is the most
    /// common healthcare cost calculation scenario.
    ///
    /// # Expected Behavior
    ///
    /// - Patient pays the primary care copay amount ($25.00)
    /// - Insurance covers the remainder
    /// - No deductible is applied (copay takes precedence)
    #[test]
    fn test_primary_care_visit_copay() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "99213".to_string(),
            name: "Office Visit".to_string(),
            cost: 150.0,
            service_type_codes: vec!["98".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);

        assert_eq!(result.patient_pays, 25.0);
        assert_eq!(result.insurance_pays, 125.0);
        assert_eq!(result.calculation_details.copay_applied, 25.0);
        assert_eq!(result.calculation_details.deductible_applied, 0.0);
    }

    /// Tests calculation for a specialist office visit with higher copay.
    ///
    /// Validates that visits to specialist providers use the specialist-specific
    /// copay amount, which is typically higher than primary care copays.
    ///
    /// # Expected Behavior
    ///
    /// - Patient pays the specialist copay amount ($50.00)
    /// - Insurance covers the remainder
    /// - Provider specialty flag correctly triggers specialist benefits
    #[test]
    fn test_specialist_visit_copay() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "99214".to_string(),
            name: "Specialist Office Visit".to_string(),
            cost: 250.0,
            service_type_codes: vec!["96".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: true,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);

        assert_eq!(result.patient_pays, 50.0);
        assert_eq!(result.insurance_pays, 200.0);
        assert_eq!(result.calculation_details.copay_applied, 50.0);
        assert_eq!(result.calculation_details.deductible_applied, 0.0);
    }

    /// Tests calculation for an emergency room visit.
    ///
    /// Emergency room visits often have special copay rules and network status
    /// handling, as they're typically covered regardless of network status.
    ///
    /// # Expected Behavior
    ///
    /// - Patient pays the ER copay amount ($300.00)
    /// - Network status is [`NetworkStatus::NotApplicable`] for emergency services
    /// - ER copay doesn't count toward regular deductible
    #[test]
    fn test_emergency_room_visit() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "99284".to_string(),
            name: "Emergency Room Visit".to_string(),
            cost: 2500.0,
            service_type_codes: vec!["86".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result = calculate_out_of_pocket(
            &benefits,
            &procedure,
            NetworkStatus::NotApplicable,
            &provider,
        );
        println!("{:?}", result.calculation_details);

        // Based on the JSON, the ER visit has a $300 copay
        assert_eq!(result.patient_pays, 300.0);
        assert_eq!(result.insurance_pays, 2200.0);
        assert_eq!(result.calculation_details.copay_applied, 300.0);
        // The ER copay should not apply to the main deductible
        assert_eq!(result.calculation_details.deductible_applied, 0.0);
    }

    /// Tests calculation for expensive inpatient surgery with deductible and coinsurance.
    ///
    /// This scenario validates the full deductible and coinsurance calculation path
    /// for high-cost procedures that exceed typical copay thresholds.
    ///
    /// # Expected Behavior
    ///
    /// - Full remaining deductible is applied first ($1,500)
    /// - 20% coinsurance applies to remaining amount after deductible
    /// - Total patient responsibility is deductible + coinsurance
    #[test]
    fn test_surgery_with_deductible_and_coinsurance() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "63030".to_string(),
            name: "Inpatient Surgery".to_string(),
            cost: 15000.0,
            service_type_codes: vec!["48".to_string()], // Hospital - Inpatient
        };
        let provider = ProviderInfo {
            is_specialist: true,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);

        // Step 1: Apply remaining deductible
        // The full $1500 deductible should be applied.
        let deductible_applied = 1500.0;

        // Step 2: Apply coinsurance on the rest
        // Remaining cost is $15000 - $1500 = $13500.
        // In-network coinsurance for inpatient is 20%.
        let coinsurance_applied = 13500.0 * 0.20; // $2700

        let expected_patient_pays = deductible_applied + coinsurance_applied; // 1500 + 2700 = 4200
        println!("{:?}", result.calculation_details);

        assert_eq!(result.calculation_details.deductible_applied, 1500.0);
        assert_eq!(result.calculation_details.coinsurance_applied, 2700.0);
        assert_eq!(result.patient_pays, expected_patient_pays);
        assert_eq!(result.insurance_pays, 15000.0 - expected_patient_pays);
    }

    /// Tests calculation for [`NetworkStatus::OutOfNetwork`] provider visit.
    ///
    /// Out-of-network visits typically don't have copays and go straight to
    /// the out-of-network deductible and coinsurance structure.
    ///
    /// # Expected Behavior
    ///
    /// - No copay applied (out-of-network typically doesn't have copays)
    /// - Cost applies to out-of-network deductible ($2,000)
    /// - Higher coinsurance rates typically apply after deductible
    #[test]
    fn test_out_of_network_visit_deductible() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "99213".to_string(),
            name: "Office Visit".to_string(),
            cost: 400.0,
            service_type_codes: vec!["98".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result = calculate_out_of_pocket(
            &benefits,
            &procedure,
            NetworkStatus::OutOfNetwork,
            &provider,
        );
        println!("{:?}", result.calculation_details);

        // Out-of-network visits go straight to the OON deductible ($2000 in the JSON)
        assert_eq!(result.patient_pays, 400.0);
        assert_eq!(result.insurance_pays, 0.0);
        assert_eq!(result.calculation_details.deductible_applied, 400.0);
        assert_eq!(result.calculation_details.copay_applied, 0.0);
        assert_eq!(result.calculation_details.remaining_deductible, 1600.0); // 2000 - 400
    }

    /// Tests out-of-pocket maximum limitation on very expensive procedures.
    ///
    /// Validates that patient costs are capped by the annual out-of-pocket
    /// maximum, even for procedures that would normally result in higher costs.
    ///
    /// # Expected Behavior
    ///
    /// - Normal calculation would exceed remaining OOP max
    /// - Patient cost is limited to remaining OOP max amount
    /// - `oop_max_limited` flag is set to true
    /// - Remaining OOP max drops to zero
    #[test]
    fn test_oop_max_limit() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "63030".to_string(),
            name: "Very Expensive Surgery".to_string(),
            cost: 50000.0,
            service_type_codes: vec!["48".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: true,
        };

        // From the JSON, the remaining OOP max is $4863.79
        let remaining_oop_max = 4863.79;

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);
        // The patient should only pay the remaining out-of-pocket max, not the full calculated amount.
        assert_eq!(result.patient_pays, remaining_oop_max);
        assert!(result.calculation_details.oop_max_limited);
        assert_eq!(result.calculation_details.remaining_oop_max, 0.0);
    }

    /// Tests calculation for preventive care with "preventative" service type code.
    ///
    /// Validates that procedures marked with the "preventative" service type code
    /// are covered at $0 for in-network providers, per ACA requirements.
    ///
    /// # Expected Behavior
    ///
    /// - Patient pays: $0
    /// - Insurance covers: 100% of procedure cost
    /// - Deductible is NOT applied
    /// - Explanation indicates ACA-mandated preventive care
    #[test]
    fn test_preventive_care_zero_cost() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "99395".to_string(),
            name: "Annual Preventive Exam".to_string(),
            cost: 200.0,
            service_type_codes: vec!["preventative".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);

        // Patient should pay nothing for preventive care
        assert_eq!(result.patient_pays, 0.0);
        assert_eq!(result.insurance_pays, 200.0);

        // No cost sharing mechanisms should be applied
        assert_eq!(result.calculation_details.deductible_applied, 0.0);
        assert_eq!(result.calculation_details.copay_applied, 0.0);
        assert_eq!(result.calculation_details.coinsurance_applied, 0.0);

        // Should have preventive care benefit applied
        assert_eq!(result.calculation_details.applied_benefits.len(), 1);
        assert_eq!(
            result.calculation_details.applied_benefits[0].benefit_type,
            "Preventive Care"
        );
        assert_eq!(result.calculation_details.applied_benefits[0].amount, 0.0);

        // Explanation should mention preventive care
        assert!(
            result
                .calculation_details
                .explanation
                .contains("Preventive care")
        );
    }

    /// Tests that preventive care out-of-network still goes through normal calculation.
    ///
    /// Validates that the preventive care exception only applies to in-network providers.
    /// Out-of-network preventive care should still be subject to deductible/coinsurance.
    ///
    /// # Expected Behavior
    ///
    /// - Patient pays according to out-of-network rules (deductible/coinsurance)
    /// - NOT covered at $0
    #[test]
    fn test_preventive_care_out_of_network() {
        let benefits = setup_benefits();
        let procedure = ProcedureDetails {
            code: "99395".to_string(),
            name: "Annual Preventive Exam".to_string(),
            cost: 200.0,
            service_type_codes: vec!["preventative".to_string()],
        };
        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result = calculate_out_of_pocket(
            &benefits,
            &procedure,
            NetworkStatus::OutOfNetwork,
            &provider,
        );
        println!("{:?}", result.calculation_details);

        // Patient should pay according to OON rules, NOT $0
        assert!(result.patient_pays > 0.0);

        // Should apply deductible since it's OON
        assert!(result.calculation_details.deductible_applied > 0.0);
    }

    /// Tests family deductible met scenario - should not apply any deductible.
    ///
    /// When family deductible is fully met, no deductible should be applied even if
    /// individual deductible has remaining amount.
    ///
    /// # Expected Behavior
    ///
    /// - Family deductible met -> Skip deductible entirely
    /// - Apply coinsurance directly to full cost
    #[test]
    fn test_family_deductible_met() {
        // Create custom benefits with family deductible met
        let benefits = vec![
            ParsedBenefit {
                benefit_type: BenefitType::ActiveCoverage,
                amount: None,
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            // Individual deductible with $1000 remaining
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(1500.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(1000.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::Remaining,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            // Family deductible FULLY MET ($0 remaining)
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(3000.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Family,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(0.0), // ← Family deductible MET
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Family,
                time_period: TimePeriod::Remaining,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            // 20% coinsurance
            ParsedBenefit {
                benefit_type: BenefitType::Coinsurance,
                amount: None,
                percent: Some(0.20),
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
        ];

        let procedure = ProcedureDetails {
            code: "99213".to_string(),
            name: "Office Visit".to_string(),
            cost: 200.0,
            service_type_codes: vec!["98".to_string()],
        };

        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);

        // Family deductible met, so NO deductible should be applied
        assert_eq!(result.calculation_details.deductible_applied, 0.0);

        // Should go straight to 20% coinsurance on full $200
        assert_eq!(result.calculation_details.coinsurance_applied, 40.0);
        assert_eq!(result.patient_pays, 40.0);
        assert_eq!(result.insurance_pays, 160.0);
    }

    /// Tests fallback to family deductible when no individual deductible exists.
    ///
    /// If there's no individual deductible but family deductible exists,
    /// should use the family deductible for calculations.
    ///
    /// # Expected Behavior
    ///
    /// - No individual deductible -> Use family deductible
    /// - Apply family deductible remaining amount
    #[test]
    fn test_no_individual_use_family_deductible() {
        let benefits = vec![
            ParsedBenefit {
                benefit_type: BenefitType::ActiveCoverage,
                amount: None,
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            // NO individual deductible
            // Family deductible with $500 remaining
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(3000.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Family,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(500.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Family,
                time_period: TimePeriod::Remaining,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Coinsurance,
                amount: None,
                percent: Some(0.20),
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
        ];

        let procedure = ProcedureDetails {
            code: "99213".to_string(),
            name: "Office Visit".to_string(),
            cost: 200.0,
            service_type_codes: vec!["98".to_string()],
        };

        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);

        // Should use family deductible of $500 remaining
        assert_eq!(result.calculation_details.deductible_applied, 200.0);

        // No coinsurance since all cost went to deductible
        assert_eq!(result.calculation_details.coinsurance_applied, 0.0);
        assert_eq!(result.patient_pays, 200.0);
        assert_eq!(result.insurance_pays, 0.0);
    }

    /// Tests preferring individual deductible when both individual and family exist.
    ///
    /// When both individual and family deductibles exist (and family not met),
    /// should prefer individual deductible for calculations.
    ///
    /// # Expected Behavior
    ///
    /// - Individual deductible exists -> Use individual
    /// - Ignore family deductible (unless met)
    #[test]
    fn test_prefer_individual_over_family() {
        let benefits = vec![
            ParsedBenefit {
                benefit_type: BenefitType::ActiveCoverage,
                amount: None,
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            // Individual deductible: $1500 total, $800 remaining
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(1500.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(800.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::Remaining,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            // Family deductible: $3000 total, $500 remaining (closer to met, but should NOT use)
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(3000.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Family,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Deductible,
                amount: Some(500.0),
                percent: None,
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Family,
                time_period: TimePeriod::Remaining,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
            ParsedBenefit {
                benefit_type: BenefitType::Coinsurance,
                amount: None,
                percent: Some(0.20),
                network_status: NetworkStatus::InNetwork,
                coverage_level: CoverageLevel::Individual,
                time_period: TimePeriod::CalendarYear,
                service_type_codes: vec![],
                is_specialist_benefit: false,
            },
        ];

        let procedure = ProcedureDetails {
            code: "99213".to_string(),
            name: "Office Visit".to_string(),
            cost: 200.0,
            service_type_codes: vec!["98".to_string()],
        };

        let provider = ProviderInfo {
            is_specialist: false,
        };

        let result =
            calculate_out_of_pocket(&benefits, &procedure, NetworkStatus::InNetwork, &provider);
        println!("{:?}", result.calculation_details);

        // Should use INDIVIDUAL deductible ($800 remaining), not family ($500 remaining)
        assert_eq!(result.calculation_details.deductible_applied, 200.0);

        // Remaining individual deductible should be $1300 ($1500 total - $200 applied)
        // Note: The calculator uses total - applied, not the remaining field for final calculation
        assert_eq!(result.calculation_details.remaining_deductible, 1300.0);

        assert_eq!(result.patient_pays, 200.0);
    }
}
