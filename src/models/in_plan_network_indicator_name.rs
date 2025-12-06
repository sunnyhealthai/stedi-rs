use serde::{Deserialize, Serialize};

/// Human-readable name indicating whether benefits apply to in-network or out-of-network providers.
///
/// This enum provides the human-readable name for the in-plan network indicator, which specifies
/// whether a particular benefit applies to in-network providers, out-of-network providers, or both.
/// It is used in `benefitsInformation.inPlanNetworkIndicator` to indicate network status for
/// benefit coverage.
///
/// The corresponding code version (`InPlanNetworkIndicatorCode`) contains the standardized codes:
/// - `Y` - Yes (in-network)
/// - `N` - No (out-of-network)
/// - `U` - Unknown
/// - `W` - Not Applicable (applies to both)
///
/// ## Network Status and Patient Costs
///
/// Network status significantly affects patient cost-sharing amounts:
/// - **In-network providers** (`Yes`): Typically have lower patient costs, including lower
///   co-pays, deductibles, and coinsurance rates. Patients pay less out-of-pocket when using
///   in-network providers.
/// - **Out-of-network providers** (`No`): Typically have higher patient costs, including higher
///   co-pays, deductibles, and coinsurance rates. Patients may also be responsible for balance
///   billing (the difference between what the provider charges and what the plan pays).
/// - **Not Applicable**: The benefit applies to both in-network and out-of-network providers
///   equally, or network status doesn't apply to this benefit type.
///
/// ## Important Note
///
/// **This indicator does NOT tell you whether the provider requesting the eligibility check is
/// in-network for the health plan.** It only indicates whether the benefits being returned apply
/// to in-network or out-of-network scenarios. To determine if a specific provider is in-network,
/// you must verify network status separately with the payer.
///
/// Payers may return benefits separately for in-network and out-of-network scenarios, allowing
/// you to compare costs and help patients make informed decisions about where to receive care.
///
/// ## Usage Context
///
/// This enum is used in `benefitsInformation.inPlanNetworkIndicator` to provide human-readable
/// network status information. When processing benefits, check this field along with the
/// corresponding `inPlanNetworkIndicatorCode` to understand network restrictions and cost
/// implications.
///
/// Visit [Provider network status, authorizations, referrals](https://www.stedi.com/docs/healthcare/eligibility-network-status-authorization-referrals)
/// for more details about network status and how to verify provider network participation.
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InPlanNetworkIndicatorName {
    /// Benefits apply to in-network providers.
    ///
    /// Indicates that the benefits information applies specifically to in-network providers.
    /// Patients using in-network providers will typically have lower cost-sharing amounts
    /// (co-pays, deductibles, coinsurance) compared to out-of-network providers.
    #[serde(rename = "Yes")]
    #[default]
    Yes,
    /// Benefits apply to out-of-network providers.
    ///
    /// Indicates that the benefits information applies specifically to out-of-network providers.
    /// Patients using out-of-network providers will typically have higher cost-sharing amounts
    /// and may be subject to balance billing.
    #[serde(rename = "No")]
    No,
    /// Network status is unknown or cannot be determined.
    ///
    /// The payer cannot determine whether the benefits apply to in-network or out-of-network
    /// providers. This may occur when network information is unavailable or when the benefit
    /// type doesn't have clear network distinctions.
    #[serde(rename = "Unknown")]
    Unknown,
    /// Network status does not apply to this benefit type.
    ///
    /// Indicates that the benefit applies to both in-network and out-of-network providers
    /// equally, or that network status is not relevant for this particular benefit type.
    /// Some benefits (like certain preventive services) may not have network restrictions.
    #[serde(rename = "Not Applicable")]
    NotApplicable,
}

impl std::fmt::Display for InPlanNetworkIndicatorName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "Yes"),
            Self::No => write!(f, "No"),
            Self::Unknown => write!(f, "Unknown"),
            Self::NotApplicable => write!(f, "Not Applicable"),
        }
    }
}
