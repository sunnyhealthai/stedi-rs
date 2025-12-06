use serde::{Deserialize, Serialize};

/// Entity type qualifier indicating whether an entity is a person or an organization.
///
/// This enum distinguishes between individual persons and organizational entities in healthcare
/// eligibility transactions. It's used throughout eligibility responses to classify various
/// entities such as providers, subscribers, dependents, payers, and related entities.
///
/// ## Usage Context
///
/// The entity type qualifier is used in multiple contexts:
///
/// - **Providers** (`responseProvider.entityType`): Indicates whether the provider is an
///   individual practitioner (`Person`) or an organization like a hospital or medical group
///   (`NonPersonEntity`)
///
/// - **Subscribers** (`responseSubscriber.entityType`): Typically `Person` for individual
///   policyholders, though organizations can sometimes be subscribers
///
/// - **Dependents** (`responseDependent.entityType`): Always `Person` for individual dependents
///
/// - **Payers** (`payer.entityType`): Typically `NonPersonEntity` for insurance companies and
///   other organizational payers, though individual payers are possible
///
/// - **Related Entities** (`benefitsRelatedEntity.entityType`): Distinguishes between individual
///   providers (like a primary care physician) and organizational entities (like PBMs, carveout
///   administrators, or hospitals)
///
/// ## Entity Types
///
/// - **`Person`**: Individual entities such as:
///   - Individual healthcare providers (doctors, nurses, therapists)
///   - Primary care providers
///   - Subscribers and dependents
///   - Individual practitioners
///
/// - **`NonPersonEntity`**: Organizational entities such as:
///   - Insurance companies and payers
///   - Hospitals and medical groups
///   - Pharmacy Benefits Managers (PBMs)
///   - Carveout administrators
///   - Healthcare organizations
///   - Provider networks
///
/// ## Examples
///
/// - A patient's primary care provider would typically be `Person` (an individual doctor)
/// - A carveout administrator or PBM would be `NonPersonEntity` (an organization)
/// - A hospital system would be `NonPersonEntity`
/// - A subscriber or dependent would be `Person`
///
/// This distinction helps systems properly process and display entity information, as person
/// entities typically have name fields (`firstName`, `lastName`) while non-person entities
/// typically have organization name fields (`name`, `organizationName`).
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EntityTypeQualifier {
    /// An individual person entity.
    ///
    /// Used for individual healthcare providers, subscribers, dependents, and other
    /// person-level entities. Person entities typically have name fields like `firstName`
    /// and `lastName` rather than organization names.
    #[serde(rename = "Person")]
    #[default]
    Person,
    /// An organizational or non-person entity.
    ///
    /// Used for organizations such as insurance companies, hospitals, medical groups,
    /// PBMs, carveout administrators, and other healthcare organizations. Non-person
    /// entities typically have organization name fields like `name` or `organizationName`
    /// rather than individual name fields.
    #[serde(rename = "Non-Person Entity")]
    NonPersonEntity,
}

impl std::fmt::Display for EntityTypeQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Person => write!(f, "Person"),
            Self::NonPersonEntity => write!(f, "Non-Person Entity"),
        }
    }
}
