//! Healthcare eligibility response models and utilities.
//!
//! This module contains data structures used to represent additional information
//! returned in healthcare eligibility check responses. These structures help
//! convey supplementary details about benefit coverage that may be relevant
//! to healthcare providers and patients.

use serde::{Deserialize, Serialize};

/// Contains supplementary information about healthcare benefits eligibility.
///
/// This struct represents additional descriptive information that may be
/// included in an eligibility response to provide context about benefit
/// coverage, limitations, or other relevant details that don't fit into
/// the structured benefit fields.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdditionalInformation {
    /// A free-form message containing additional information about the benefits in the response.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AdditionalInformation {
    /// Creates a new `AdditionalInformation` instance with no description.
    ///
    /// This constructor initializes an empty AdditionalInformation struct
    /// with all fields set to their default values. The description field
    /// will be initialized as `None`.
    ///
    /// # Returns
    ///
    /// A new `AdditionalInformation` instance with default values.
    pub fn new() -> AdditionalInformation {
        AdditionalInformation { description: None }
    }
}
