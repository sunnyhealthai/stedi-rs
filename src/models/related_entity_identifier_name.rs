use serde::{Deserialize, Serialize};

/// Human-readable name for related entity identifier codes used in benefit-related entity information.
///
/// This enum provides descriptive names for identifying organizational entities, physical
/// locations, properties, or individuals that are related to benefits information. Related
/// entities include providers, payers, administrators, and other organizations involved in
/// benefit delivery or administration.
///
/// The corresponding code version contains standardized codes for programmatic use, while
/// this enum provides human-readable names for display and understanding.
///
/// ## Related Entity Categories
///
/// Related entities can be grouped into several categories:
///
/// ### Healthcare Providers
/// - **Provider**: General healthcare provider
/// - **Primary Care Provider**: Member's primary care physician
/// - **Other Physician**: Other physician providers
/// - **Contracted Service Provider**: Provider contracted to provide specific services
/// - **Facility**: Healthcare facility such as hospitals or clinics
///
/// ### Provider Organizations
/// - **Group**: Provider group or medical group
/// - **Independent Physicians Association (IPA)**: IPA organization
/// - **Preferred Provider Organization (PPO)**: PPO network (also used to identify networks
///   for in-network benefit restrictions)
/// - **Managed Care Organization**: MCO managing care delivery
///
/// ### Payers and Insurance Carriers
/// - **Payer**: Insurance company or healthcare payer
/// - **Primary Payer**: Primary insurance payer in coordination of benefits
/// - **Secondary Payer**: Secondary insurance payer in coordination of benefits
/// - **Tertiary Payer**: Tertiary insurance payer in coordination of benefits
/// - **Origin Carrier**: Original insurance carrier
/// - **Prior Insurance Carrier**: Previous insurance carrier
///
/// ### Administrative Entities
/// - **Third-Party Administrator**: TPA administering health plans
/// - **Plan Sponsor**: Organization sponsoring the health plan
/// - **Employer**: Employer sponsoring group health plans
/// - **Gateway Provider**: Intermediary facilitating transactions
///
/// ### Management Organizations
/// - **Utilization Management Organization**: UMO managing utilization
/// - **Organization Completing Configuration Change**: Organization handling configuration
/// - **Party Performing Verification**: Entity performing eligibility verification
///
/// ### Other Entities
/// - **Insured or Subscriber**: The insured member or subscriber
/// - **Legal Representative**: Legal representative of the member
/// - **Vendor**: Vendor providing services or products
///
/// ## Usage Context
///
/// Related entity identifiers are used in benefit-related entity information to:
///
/// - **Identify benefit administrators**: Identify PBMs, carveout administrators, or other
///   entities managing specific benefits
/// - **Network identification**: Identify provider networks (especially PPOs) that benefits
///   are restricted to for in-network coverage
/// - **Coordination of benefits**: Identify primary, secondary, and tertiary payers
/// - **Provider relationships**: Identify primary care providers, specialists, or facilities
///   associated with benefits
/// - **Administrative routing**: Route benefits to appropriate administrators or vendors
/// - **Care coordination**: Identify entities involved in care delivery and coordination
///
/// ## PPO Network Identification
///
/// The `Preferred Provider Organization (PPO)` identifier is used to:
/// - Identify a PPO by name or identification number
/// - Identify the network that benefits are restricted to for in-network benefits
/// - Specify which provider network applies to benefit coverage
///
/// This is particularly important for plans with network restrictions, where benefits may
/// differ based on whether services are received from in-network or out-of-network providers.
///
/// ## Examples
///
/// - A pharmacy benefit might identify a PBM as `ThirdPartyAdministrator`
/// - An in-network benefit might identify the network as `PreferredProviderOrganization`
/// - A coordination of benefits scenario might identify `PrimaryPayer` and `SecondaryPayer`
/// - A benefit might identify the member's `PrimaryCareProvider`
///
/// ## X12 HIPAA
///
/// Maps to related entity identifier codes in X12 271 benefit information segments,
/// including entity identification in benefit-related entity loops.
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum RelatedEntityIdentifierName {
    /// Contracted Service Provider - Provider contracted to provide specific services.
    ///
    /// Used to identify providers who are contracted to deliver specific services under
    /// a benefit. Common for carveout benefits or specialized service providers.
    #[serde(rename = "Contracted Service Provider")]
    #[default]
    ContractedServiceProvider,
    /// Preferred Provider Organization (PPO) - PPO network identifier.
    ///
    /// Used to identify a PPO by name or identification number. Also used to identify
    /// the network that benefits are restricted to for in-network benefits. Important
    /// for plans with network restrictions where benefits differ based on provider network.
    #[serde(rename = "Preferred Provider Organization (PPO)")]
    PreferredProviderOrganizationLeftParenthesisPpoRightParenthesis,
    /// Provider - General healthcare provider.
    ///
    /// Used to identify general healthcare providers associated with benefits. Common
    /// for identifying providers involved in benefit delivery or administration.
    #[serde(rename = "Provider")]
    Provider,
    /// Third-Party Administrator - TPA administering health plans.
    ///
    /// Used to identify third-party administrators that handle claims processing,
    /// eligibility verification, and administrative functions for health plans. Common
    /// for PBMs, carveout administrators, and other administrative entities.
    #[serde(rename = "Third-Party Administrator")]
    ThirdPartyAdministrator,
    /// Employer - Employer sponsoring group health plans.
    ///
    /// Used to identify employers that sponsor group health insurance plans. Common
    /// for self-insured employer plans or when employers are involved in plan administration.
    #[serde(rename = "Employer")]
    Employer,
    /// Other Physician - Other physician providers.
    ///
    /// Used to identify physician providers other than the primary care provider.
    /// Common for specialists or other physicians involved in care delivery.
    #[serde(rename = "Other Physician")]
    OtherPhysician,
    /// Facility - Healthcare facility such as hospitals or clinics.
    ///
    /// Used to identify healthcare facilities associated with benefits. Common for
    /// facility-based benefits or when facilities are involved in benefit delivery.
    #[serde(rename = "Facility")]
    Facility,
    /// Gateway Provider - Intermediary facilitating transactions.
    ///
    /// Used to identify gateway providers or clearinghouses that facilitate communication
    /// between providers and payers. Common for transaction routing entities.
    #[serde(rename = "Gateway Provider")]
    GatewayProvider,
    /// Group - Provider group or medical group.
    ///
    /// Used to identify provider groups or medical groups associated with benefits.
    /// Common for group practices or medical organizations.
    #[serde(rename = "Group")]
    Group,
    /// Independent Physicians Association (IPA) - IPA organization.
    ///
    /// Used to identify Independent Physicians Associations that are involved in benefit
    /// delivery or network management. Common for IPA-based provider networks.
    #[serde(rename = "Independent Physicians Association (IPA)")]
    IndependentPhysiciansAssociationLeftParenthesisIpaRightParenthesis,
    /// Insured or Subscriber - The insured member or subscriber.
    ///
    /// Used to identify the insured member or subscriber as a related entity. Common
    /// when the member themselves are referenced in benefit information.
    #[serde(rename = "Insured or Subscriber")]
    InsuredOrSubscriber,
    /// Legal Representative - Legal representative of the member.
    ///
    /// Used to identify legal representatives acting on behalf of members. Common for
    /// guardians, conservators, or other legal representatives.
    #[serde(rename = "Legal Representative")]
    LegalRepresentative,
    /// Origin Carrier - Original insurance carrier.
    ///
    /// Used to identify the original insurance carrier in coordination of benefits
    /// scenarios or when tracking insurance history.
    #[serde(rename = "Origin Carrier")]
    OriginCarrier,
    /// Primary Care Provider - Member's primary care physician.
    ///
    /// Used to identify the member's primary care provider. Common for HMO plans or
    /// plans requiring PCP assignment for benefit access.
    #[serde(rename = "Primary Care Provider")]
    PrimaryCareProvider,
    /// Prior Insurance Carrier - Previous insurance carrier.
    ///
    /// Used to identify previous insurance carriers in coordination of benefits scenarios
    /// or when tracking insurance history and transitions.
    #[serde(rename = "Prior Insurance Carrier")]
    PriorInsuranceCarrier,
    /// Plan Sponsor - Organization sponsoring the health plan.
    ///
    /// Used to identify plan sponsors, typically employers, unions, or other groups
    /// responsible for establishing and maintaining health insurance plans.
    #[serde(rename = "Plan Sponsor")]
    PlanSponsor,
    /// Payer - Insurance company or healthcare payer.
    ///
    /// Used to identify insurance companies or healthcare payers. Common for identifying
    /// the payer providing benefits or involved in benefit administration.
    #[serde(rename = "Payer")]
    Payer,
    /// Primary Payer - Primary insurance payer in coordination of benefits.
    ///
    /// Used to identify the primary payer in coordination of benefits scenarios where
    /// multiple insurance policies are involved. The primary payer pays first.
    #[serde(rename = "Primary Payer")]
    PrimaryPayer,
    /// Secondary Payer - Secondary insurance payer in coordination of benefits.
    ///
    /// Used to identify the secondary payer in coordination of benefits scenarios. The
    /// secondary payer pays after the primary payer has processed the claim.
    #[serde(rename = "Secondary Payer")]
    SecondaryPayer,
    /// Tertiary Payer - Tertiary insurance payer in coordination of benefits.
    ///
    /// Used to identify the tertiary payer in coordination of benefits scenarios where
    /// three or more insurance policies are involved. The tertiary payer pays after
    /// primary and secondary payers.
    #[serde(rename = "Tertiary Payer")]
    TertiaryPayer,
    /// Party Performing Verification - Entity performing eligibility verification.
    ///
    /// Used to identify the entity that performed the eligibility verification. Common
    /// for tracking which organization or system verified eligibility information.
    #[serde(rename = "Party Performing Verification")]
    PartyPerformingVerification,
    /// Vendor - Vendor providing services or products.
    ///
    /// Used to identify vendors that provide services or products related to benefits.
    /// Common for DME suppliers, transportation services, or other benefit vendors.
    #[serde(rename = "Vendor")]
    Vendor,
    /// Organization Completing Configuration Change - Organization handling configuration.
    ///
    /// Used to identify organizations that completed configuration changes related to
    /// benefits or plan administration. Common for tracking administrative changes.
    #[serde(rename = "Organization Completing Configuration Change")]
    OrganizationCompletingConfigurationChange,
    /// Utilization Management Organization - UMO managing utilization.
    ///
    /// Used to identify utilization management organizations that manage care utilization,
    /// prior authorizations, or utilization review processes. Common for UM entities
    /// involved in benefit administration.
    #[serde(rename = "Utilization Management Organization")]
    UtilizationManagementOrganization,
    /// Managed Care Organization - MCO managing care delivery.
    ///
    /// Used to identify managed care organizations that manage care delivery and benefit
    /// administration. Common for HMOs, PPOs, or other managed care entities.
    #[serde(rename = "Managed Care Organization")]
    ManagedCareOrganization,
}

impl std::fmt::Display for RelatedEntityIdentifierName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ContractedServiceProvider => write!(f, "Contracted Service Provider"),
            Self::PreferredProviderOrganizationLeftParenthesisPpoRightParenthesis => {
                write!(f, "Preferred Provider Organization (PPO)")
            }
            Self::Provider => write!(f, "Provider"),
            Self::ThirdPartyAdministrator => write!(f, "Third-Party Administrator"),
            Self::Employer => write!(f, "Employer"),
            Self::OtherPhysician => write!(f, "Other Physician"),
            Self::Facility => write!(f, "Facility"),
            Self::GatewayProvider => write!(f, "Gateway Provider"),
            Self::Group => write!(f, "Group"),
            Self::IndependentPhysiciansAssociationLeftParenthesisIpaRightParenthesis => {
                write!(f, "Independent Physicians Association (IPA)")
            }
            Self::InsuredOrSubscriber => write!(f, "Insured or Subscriber"),
            Self::LegalRepresentative => write!(f, "Legal Representative"),
            Self::OriginCarrier => write!(f, "Origin Carrier"),
            Self::PrimaryCareProvider => write!(f, "Primary Care Provider"),
            Self::PriorInsuranceCarrier => write!(f, "Prior Insurance Carrier"),
            Self::PlanSponsor => write!(f, "Plan Sponsor"),
            Self::Payer => write!(f, "Payer"),
            Self::PrimaryPayer => write!(f, "Primary Payer"),
            Self::SecondaryPayer => write!(f, "Secondary Payer"),
            Self::TertiaryPayer => write!(f, "Tertiary Payer"),
            Self::PartyPerformingVerification => write!(f, "Party Performing Verification"),
            Self::Vendor => write!(f, "Vendor"),
            Self::OrganizationCompletingConfigurationChange => {
                write!(f, "Organization Completing Configuration Change")
            }
            Self::UtilizationManagementOrganization => {
                write!(f, "Utilization Management Organization")
            }
            Self::ManagedCareOrganization => write!(f, "Managed Care Organization"),
        }
    }
}
