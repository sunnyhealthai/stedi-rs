use serde::{Deserialize, Serialize};

/// Entity identifier code specifying the type of provider entity in eligibility responses.
///
/// This enum identifies the role or type of provider entity returned by payers in eligibility
/// responses. It helps distinguish between different types of organizations that may act as
/// providers or provider-related entities, such as healthcare providers, hospitals, facilities,
/// or administrative entities.
///
/// This enum is used in `ResponseProvider` to identify the type of provider entity that is
/// associated with benefits or benefit-related information in eligibility responses.
///
/// ## Entity Types
///
/// ### Healthcare Providers
/// - **Provider**: General healthcare provider, typically an individual practitioner or medical
///   group providing direct patient care
/// - **Hospital**: Hospital facility providing inpatient and outpatient services
/// - **Facility**: Healthcare facility such as clinics, ambulatory surgery centers, or other
///   medical facilities
///
/// ### Payer-Related Entities
/// - **Payer**: Insurance company or healthcare payer organization
/// - **Third-Party Administrator**: Organization that administers health plans on behalf of
///   employers or plan sponsors
/// - **Plan Sponsor**: Organization that establishes and maintains a health plan
/// - **Employer**: Employer organization sponsoring a group health plan
///
/// ### Intermediary Entities
/// - **Gateway Provider**: Intermediary organization that facilitates communication between
///   providers and multiple payers
///
/// ## Usage Context
///
/// Provider entity identifiers are used in eligibility responses to:
///
/// - **Identify provider roles**: Clarify what type of entity is providing services or
///   administering benefits
/// - **Benefit determination**: Help understand provider-specific benefits or network restrictions
/// - **Network matching**: Determine if providers are in-network or out-of-network for the
///   specific provider type
/// - **Administrative routing**: Route benefits to appropriate administrators or vendors
/// - **Care coordination**: Identify entities involved in care delivery and coordination
///
/// ## Common Usage Patterns
///
/// - **Individual practitioners**: Typically identified as `Provider` for individual doctors,
///   nurses, or therapists
/// - **Healthcare facilities**: Use `Hospital` for hospitals, `Facility` for clinics and
///   other facilities
/// - **Payer entities**: Use `Payer`, `ThirdPartyAdministrator`, or `PlanSponsor` when the
///   entity is payer-related
/// - **Benefit administrators**: Use `ThirdPartyAdministrator` for PBMs, carveout administrators,
///   or other administrative entities
///
/// ## X12 HIPAA
///
/// Maps to provider entity identifier codes in X12 271 transactions, including entity
/// identification codes in provider segments and benefit-related entity segments.
///
/// ## Examples
///
/// - A primary care physician would typically be identified as `Provider`
/// - A hospital system would be identified as `Hospital`
/// - A pharmacy benefit manager would be identified as `ThirdPartyAdministrator`
/// - An insurance company would be identified as `Payer`
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ResponseProviderEntityIdentifier {
    /// Provider - General healthcare provider entity.
    ///
    /// Used for general healthcare providers, including individual practitioners (doctors,
    /// nurses, therapists) and medical groups providing direct patient care. This is the
    /// most common provider entity type for healthcare providers delivering medical services.
    #[serde(rename = "Provider")]
    #[default]
    Provider,
    /// Third-Party Administrator - TPA organization administering health plans.
    ///
    /// Used for third-party administrators that handle claims processing, eligibility
    /// verification, and administrative functions for health plans. TPAs act as intermediaries
    /// between employers, plan sponsors, and insurance companies. Common for PBMs, carveout
    /// administrators, and other administrative entities.
    #[serde(rename = "Third-Party Administrator")]
    ThirdPartyAdministrator,
    /// Employer - Employer organization sponsoring a group health plan.
    ///
    /// Used when the entity is an employer that sponsors a group health insurance plan.
    /// This is common for self-insured employer plans or when employers are the primary
    /// contact for plan administration.
    #[serde(rename = "Employer")]
    Employer,
    /// Hospital - Hospital facility providing inpatient and outpatient services.
    ///
    /// Used for hospital facilities that provide comprehensive healthcare services including
    /// inpatient care, emergency services, and outpatient procedures. Hospitals are distinct
    /// from other facilities due to their ability to provide inpatient care.
    #[serde(rename = "Hospital")]
    Hospital,
    /// Facility - Healthcare facility such as clinics, ambulatory surgery centers, or other medical facilities.
    ///
    /// Used for healthcare facilities that provide outpatient services, such as clinics,
    /// ambulatory surgery centers, urgent care centers, imaging centers, or other medical
    /// facilities. This is a broader category than hospitals and includes various types
    /// of healthcare facilities.
    #[serde(rename = "Facility")]
    Facility,
    /// Gateway Provider - Intermediary organization facilitating communication between providers and multiple payers.
    ///
    /// Used for gateway providers or clearinghouses that route eligibility requests to
    /// multiple payers and aggregate responses. Gateway providers act as intermediaries
    /// in the healthcare transaction process.
    #[serde(rename = "Gateway Provider")]
    GatewayProvider,
    /// Plan Sponsor - Organization that establishes and maintains a health plan.
    ///
    /// Used for plan sponsors, which are typically employers, unions, or other groups
    /// responsible for establishing and maintaining health insurance plans. Plan sponsors
    /// work with TPAs or insurance companies to administer benefits.
    #[serde(rename = "Plan Sponsor")]
    PlanSponsor,
    /// Payer - Insurance company or healthcare payer organization.
    ///
    /// Used when the entity is an insurance company or healthcare payer that provides
    /// coverage and processes claims. This is commonly used for payer entities in
    /// eligibility responses.
    #[serde(rename = "Payer")]
    Payer,
}

impl std::fmt::Display for ResponseProviderEntityIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Provider => write!(f, "Provider"),
            Self::ThirdPartyAdministrator => write!(f, "Third-Party Administrator"),
            Self::Employer => write!(f, "Employer"),
            Self::Hospital => write!(f, "Hospital"),
            Self::Facility => write!(f, "Facility"),
            Self::GatewayProvider => write!(f, "Gateway Provider"),
            Self::PlanSponsor => write!(f, "Plan Sponsor"),
            Self::Payer => write!(f, "Payer"),
        }
    }
}
