use serde::{Deserialize, Serialize};

/// BenefitsRelatedEntity : Identify another entity associated with the eligibility or benefits. This could be a provider, an individual, an organization, or another payer.  This is where information for a crossover carrier such as Medicaid or Medicare is provided, if it's applicable to the patient and the payer supports it.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BenefitsRelatedEntity {
    /// The address of the entity, such as a provider or organization.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::Address>,
    /// The contact information for the entity, such as a phone number or email address.
    #[serde(rename = "contactInformation", skip_serializing_if = "Option::is_none")]
    pub contact_information: Option<super::ContactInformation>,
    /// The first name of the entity, if the entity is a person.
    #[serde(rename = "entityFirstname", skip_serializing_if = "Option::is_none")]
    pub entity_firstname: Option<String>,
    /// Code identifying the type of identifier in the `entityIdentificationValue` property. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#identification-code-qualifiers) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "entityIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_identification: Option<super::BenefitRelatedEntityIdentification>,
    /// The identification number for the entity, qualified by the code in `entityIdentification`.
    #[serde(
        rename = "entityIdentificationValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_identification_value: Option<String>,
    /// Code identifying an organizational entity, a physical location, property, or individual. When set to `Party Performing Verification` for a BCBS payer, this is the patient's home plan.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityIdentifier", skip_serializing_if = "Option::is_none")]
    pub entity_identifier: Option<super::RelatedEntityIdentifierName>,
    /// The middle name or initial of the entity, if the entity is a person.
    #[serde(rename = "entityMiddlename", skip_serializing_if = "Option::is_none")]
    pub entity_middlename: Option<String>,
    /// The last name (if the entity is a person) or the business name (if the entity is an organization).
    #[serde(rename = "entityName", skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    /// Code specifying the relationship between the entity and the patient. Can be `01` - Parent, `02` - Child, `27` - Domestic Partner, `41` - Spouse, `48` - Employee, `65` - Other, or `72` - Unknown.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityRelationship", skip_serializing_if = "Option::is_none")]
    pub entity_relationship: Option<super::BenefitsRelatedEntityRelationshipCode>,
    /// The name suffix, such as Sr. Jr. or III.
    #[serde(rename = "entitySuffix", skip_serializing_if = "Option::is_none")]
    pub entity_suffix: Option<String>,
    /// The type of entity, indicating whether the related entity is a person or an organization. Can be `Person` (for individual entities like a primary care provider) or `Non-Person Entity` (for organizations like payers, Pharmacy Benefits Managers (PBMs), carveout administrators, hospitals, or other healthcare organizations). This field helps distinguish between individual practitioners and organizational entities when processing related entity information. For example, a patient's primary care provider would typically be `Person`, while a carveout administrator or PBM would be `Non-Person Entity`. Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<super::EntityTypeQualifier>,
    /// Additional provider-specific information when the related entity is a healthcare provider. Contains the provider's role code (indicating their role in the type of benefits information, such as Primary Care Provider, Referring Provider, etc.) and their taxonomy code (a standardized code that classifies the provider's specialty or type of practice). This information is typically included when the related entity represents a healthcare provider associated with the patient's benefits, such as a primary care provider (PCP) or a specialist. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#provider-codes) for provider code values.
    #[serde(
        rename = "providerInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_information: Option<super::ProviderInformation>,
}

impl BenefitsRelatedEntity {
    /// Identify another entity associated with the eligibility or benefits. This could be a provider, an individual, an organization, or another payer.  This is where information for a crossover carrier such as Medicaid or Medicare is provided, if it's applicable to the patient and the payer supports it.
    pub fn new() -> BenefitsRelatedEntity {
        BenefitsRelatedEntity {
            address: None,
            contact_information: None,
            entity_firstname: None,
            entity_identification: None,
            entity_identification_value: None,
            entity_identifier: None,
            entity_middlename: None,
            entity_name: None,
            entity_relationship: None,
            entity_suffix: None,
            entity_type: None,
            provider_information: None,
        }
    }
}
