use serde::{Deserialize, Serialize};

/// Entity identifier code specifying the type of payer entity in healthcare eligibility transactions.
///
/// This enum identifies the role or type of payer entity in healthcare eligibility responses.
/// It helps distinguish between different types of organizations that may act as payers or
/// payer-related entities, such as insurance companies, third-party administrators, employers,
/// or plan sponsors.
///
/// ## Entity Types
///
/// ### Payer
/// The primary insurance company or healthcare payer organization that provides coverage
/// and processes claims. This is the most common entity type in eligibility responses.
///
/// ### Third-Party Administrator (TPA)
/// An organization that administers health insurance plans on behalf of employers or
/// other plan sponsors. TPAs handle claims processing, eligibility verification, and
/// other administrative functions but are not the actual payer.
///
/// ### Employer
/// The employer organization that sponsors a group health insurance plan. Employers
/// may be identified as payer entities when they self-insure or when they are the
/// primary contact for plan administration.
///
/// ### Plan Sponsor
/// The organization that establishes and maintains a health insurance plan, typically
/// an employer, union, or other group. Plan sponsors are responsible for plan design
/// and may work with TPAs or insurance companies to administer benefits.
///
/// ### Gateway Provider
/// An intermediary organization that facilitates communication between healthcare
/// providers and multiple payers. Gateway providers route eligibility requests to
/// the appropriate payer and aggregate responses.
///
/// ## Usage Context
///
/// The payer entity identifier is used in:
/// - **Eligibility Responses**: Identifying the type of entity providing the eligibility
///   information in `payer.entityIdentifier`
/// - **Coordination of Benefits**: Understanding the relationship between different
///   payer entities when multiple insurance policies are involved
/// - **Administrative Routing**: Determining how to route follow-up communications or
///   claims based on the entity type
/// - **Regulatory Compliance**: Identifying entities subject to different regulatory
///   requirements based on their role
///
/// ## Examples
///
/// - A traditional insurance company would typically be identified as `Payer`
/// - An employer that self-insures might be identified as `Employer` or `PlanSponsor`
/// - A benefits administration company handling claims for multiple employers would be
///   identified as `ThirdPartyAdministrator`
/// - A clearinghouse routing requests to multiple payers would be identified as
///   `GatewayProvider`
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum PayerEntityIdentifier {
    /// Third-Party Administrator (TPA) - Organization that administers health insurance plans on behalf of employers or plan sponsors.
    ///
    /// TPAs handle claims processing, eligibility verification, and administrative functions
    /// but are not the actual insurance payer. They act as intermediaries between employers,
    /// plan sponsors, and insurance companies.
    #[serde(rename = "Third-Party Administrator")]
    #[default]
    ThirdPartyAdministrator,
    /// Employer - The employer organization that sponsors a group health insurance plan.
    ///
    /// Employers may be identified as payer entities when they self-insure or when they are
    /// the primary contact for plan administration. This is common in self-funded employer
    /// health plans.
    #[serde(rename = "Employer")]
    Employer,
    /// Gateway Provider - Intermediary organization that facilitates communication between providers and multiple payers.
    ///
    /// Gateway providers route eligibility requests to the appropriate payer and aggregate
    /// responses. They act as clearinghouses or intermediaries in the healthcare transaction
    /// process.
    #[serde(rename = "Gateway Provider")]
    GatewayProvider,
    /// Plan Sponsor - Organization that establishes and maintains a health insurance plan.
    ///
    /// Plan sponsors are typically employers, unions, or other groups responsible for plan
    /// design and may work with TPAs or insurance companies to administer benefits. They
    /// are the entity that sponsors the health plan.
    #[serde(rename = "Plan Sponsor")]
    PlanSponsor,
    /// Payer - The primary insurance company or healthcare payer organization.
    ///
    /// This is the most common entity type, representing the insurance company or healthcare
    /// payer that provides coverage and processes claims. This is the standard payer entity
    /// in most eligibility responses.
    #[serde(rename = "Payer")]
    Payer,
}

impl std::fmt::Display for PayerEntityIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ThirdPartyAdministrator => write!(f, "Third-Party Administrator"),
            Self::Employer => write!(f, "Employer"),
            Self::GatewayProvider => write!(f, "Gateway Provider"),
            Self::PlanSponsor => write!(f, "Plan Sponsor"),
            Self::Payer => write!(f, "Payer"),
        }
    }
}
