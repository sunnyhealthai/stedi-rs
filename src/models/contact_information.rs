use serde::{Deserialize, Serialize};

/// Contact information for entities involved in healthcare eligibility verification processes.
///
/// This struct represents the contact details for various parties participating in healthcare
/// eligibility and benefits inquiries, including insurance payers, healthcare providers,
/// and other relevant organizations. It is used to store and transmit contact information
/// that facilitates communication between these entities during the eligibility checking workflow.
///
/// ## Usage Context
///
/// ContactInformation is typically used when:
/// - Submitting eligibility inquiries to insurance payers
/// - Receiving response information that includes contact details for follow-up
/// - Storing provider contact information for billing and administrative purposes
/// - Maintaining records of various healthcare entities involved in patient care coordination
///
/// ## X12 HIPAA Relationship
///
/// In X12 HIPAA transactions, particularly the 270/271 eligibility inquiry and response
/// transactions, ContactInformation corresponds to segments like PER (Contact Information)
/// and NM1 (Name) combined. The `contacts` field maps to PER segment data containing
/// phone numbers, email addresses, and other communication methods, while the `name` field
/// corresponds to the entity name from NM1 segments.
///
/// This struct helps standardize the representation of contact data across different
/// healthcare transaction types, ensuring compliance with HIPAA formatting requirements
/// while providing a clean API for developers to work with.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactInformation {
    /// The contact information.
    ///
    /// Contains a list of contact methods for the entity, such as phone numbers,
    /// fax numbers, email addresses, or other communication channels. Each contact
    /// method is represented as a Contacts object with type and value information.
    ///
    /// This field is optional and may be None when no contact methods are available
    /// or when the contact information is not required for the specific transaction.
    #[serde(rename = "contacts", skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<super::Contacts>>,
    /// The name of the contact person.
    ///
    /// Represents the full name of the individual or organization contact. In payer
    /// contexts, this might be the name of the insurance company. For provider contacts,
    /// this would be the healthcare provider's name or the name of a specific contact
    /// person at that organization.
    ///
    /// This field is optional and may be None when only organizational contact methods
    /// are needed without specifying an individual contact person.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ContactInformation {
    /// Creates a new ContactInformation instance with all fields set to None.
    ///
    /// This function initializes an empty ContactInformation struct using default values.
    /// It's typically used as a starting point when building contact information for
    /// healthcare entities, allowing fields to be populated as needed.
    ///
    /// ## Returns
    ///
    /// A ContactInformation struct with `contacts` and `name` fields initialized to None.
    pub fn new() -> ContactInformation {
        ContactInformation {
            contacts: None,
            name: None,
        }
    }
}
