use serde::{Deserialize, Serialize};

/// Provider type classification identifying the type of healthcare provider entity.
///
/// This enum classifies different types of healthcare providers and related entities
/// in eligibility transactions. The provider type helps payers understand the role and
/// nature of the entity requesting eligibility information or providing healthcare
/// services, which can affect benefit determination, authorization requirements, and
/// claims processing.
///
/// ## Provider Type Categories
///
/// Provider types can be grouped into several categories:
///
/// ### Healthcare Providers
/// - **Provider**: General healthcare provider, typically an individual practitioner
///   or medical group providing direct patient care
/// - **Hospital**: Hospital facility providing inpatient and outpatient services
/// - **Facility**: Healthcare facility such as clinics, ambulatory surgery centers,
///   or other medical facilities
///
/// ### Payer-Related Entities
/// - **Payer**: Insurance company or healthcare payer organization
/// - **Third-Party Administrator**: Organization that administers health plans on
///   behalf of employers or plan sponsors
/// - **Plan Sponsor**: Organization that establishes and maintains a health plan
/// - **Employer**: Employer organization sponsoring a group health plan
///
/// ### Intermediary Entities
/// - **Gateway Provider**: Intermediary organization that facilitates communication
///   between providers and multiple payers
///
/// ## Usage Context
///
/// Provider type is used in eligibility transactions to:
///
/// - **Identify entity roles**: Clarify what type of entity is making the request
///   or providing services
/// - **Benefit determination**: Help payers determine appropriate benefits based on
///   provider type and network status
/// - **Authorization requirements**: Different provider types may have different
///   prior authorization requirements
/// - **Network matching**: Determine if providers are in-network or out-of-network
///   for the specific provider type
/// - **Claims routing**: Route claims to the appropriate payer or administrator
///   based on provider type
/// - **Enrollment verification**: Verify that providers are properly enrolled with
///   payers for their specific provider type
///
/// ## Common Usage Patterns
///
/// - **Individual practitioners**: Typically use `Provider` for individual doctors,
///   nurses, or therapists
/// - **Healthcare facilities**: Use `Hospital` for hospitals, `Facility` for clinics
///   and other facilities
/// - **Billing entities**: May use `Provider` or `Facility` depending on the billing
///   structure
/// - **Payer entities**: Use `Payer`, `ThirdPartyAdministrator`, or `PlanSponsor`
///   when the entity is payer-related
///
/// ## Provider Enrollment
///
/// Different provider types may have different enrollment requirements with specific
/// payers. It's important to use the correct provider type classification to ensure:
///
/// - Proper enrollment verification
/// - Accurate benefit determination
/// - Correct authorization requirements
/// - Appropriate network matching
///
/// ## X12 HIPAA
///
/// Maps to provider type qualifiers in X12 270/271 transactions, including:
/// - Provider type codes in provider identification segments
/// - Entity type qualifiers for provider entities
/// - Provider role indicators in benefit information
///
/// ## Examples
///
/// - A primary care physician would typically be classified as `Provider`
/// - A hospital system would be classified as `Hospital`
/// - An insurance company would be classified as `Payer`
/// - A benefits administration company would be classified as `ThirdPartyAdministrator`
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ProviderType {
    /// Payer - Insurance company or healthcare payer organization.
    ///
    /// Used when the entity is an insurance company or healthcare payer that provides
    /// coverage and processes claims. This is the default provider type and is commonly
    /// used for payer entities in eligibility transactions.
    #[serde(rename = "payer")]
    #[default]
    Payer,
    /// Third-Party Administrator - Organization that administers health plans on behalf of employers or plan sponsors.
    ///
    /// Used for TPAs that handle claims processing, eligibility verification, and
    /// administrative functions for health plans. TPAs act as intermediaries between
    /// employers, plan sponsors, and insurance companies.
    #[serde(rename = "third-party administrator")]
    ThirdPartyAdministrator,
    /// Employer - Employer organization sponsoring a group health plan.
    ///
    /// Used when the entity is an employer that sponsors a group health insurance plan.
    /// This is common for self-insured employer plans or when employers are the primary
    /// contact for plan administration.
    #[serde(rename = "employer")]
    Employer,
    /// Hospital - Hospital facility providing inpatient and outpatient services.
    ///
    /// Used for hospital facilities that provide comprehensive healthcare services
    /// including inpatient care, emergency services, and outpatient procedures. Hospitals
    /// are distinct from other facilities due to their ability to provide inpatient care.
    #[serde(rename = "hospital")]
    Hospital,
    /// Facility - Healthcare facility such as clinics, ambulatory surgery centers, or other medical facilities.
    ///
    /// Used for healthcare facilities that provide outpatient services, such as clinics,
    /// ambulatory surgery centers, urgent care centers, imaging centers, or other
    /// medical facilities. This is a broader category than hospitals and includes various
    /// types of healthcare facilities.
    #[serde(rename = "facility")]
    Facility,
    /// Gateway Provider - Intermediary organization that facilitates communication between providers and multiple payers.
    ///
    /// Used for gateway providers or clearinghouses that route eligibility requests to
    /// multiple payers and aggregate responses. Gateway providers act as intermediaries
    /// in the healthcare transaction process.
    #[serde(rename = "gateway provider")]
    GatewayProvider,
    /// Plan Sponsor - Organization that establishes and maintains a health plan.
    ///
    /// Used for plan sponsors, which are typically employers, unions, or other groups
    /// responsible for establishing and maintaining health insurance plans. Plan sponsors
    /// work with TPAs or insurance companies to administer benefits.
    #[serde(rename = "plan sponsor")]
    PlanSponsor,
    /// Provider - General healthcare provider, typically an individual practitioner or medical group.
    ///
    /// Used for general healthcare providers, including individual practitioners (doctors,
    /// nurses, therapists) and medical groups providing direct patient care. This is the
    /// most common provider type for healthcare providers delivering medical services.
    #[serde(rename = "provider")]
    Provider,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Payer => write!(f, "payer"),
            Self::ThirdPartyAdministrator => write!(f, "third-party administrator"),
            Self::Employer => write!(f, "employer"),
            Self::Hospital => write!(f, "hospital"),
            Self::Facility => write!(f, "facility"),
            Self::GatewayProvider => write!(f, "gateway provider"),
            Self::PlanSponsor => write!(f, "plan sponsor"),
            Self::Provider => write!(f, "provider"),
        }
    }
}
