//! Input validation exception handling.
//!
//! This module contains structures for handling HTTP 400 validation errors that occur
//! when request input fails validation against the API specification.
//!
//! ## Common Validation Errors
//!
//! Validation exceptions typically occur due to:
//! - Missing required fields (e.g., `controlNumber`, `tradingPartnerServiceId`)
//! - Invalid data formats (e.g., incorrect date format, invalid NPI)
//! - Values outside allowed ranges or constraints
//! - Invalid enum values for structured fields
//!
//! ## Error Structure
//!
//! Validation exceptions include:
//! - A general error message describing the validation failure
//! - A detailed list of specific field validation errors
//! - Each field error includes the field name and constraint that was violated
//!
//! ## Resolution
//!
//! To resolve validation errors:
//! 1. Review the `fieldList` for specific field validation failures
//! 2. Ensure all required fields are provided
//! 3. Verify data formats match the API specification
//! 4. Check that enum values are from the allowed set
//!
//! See the [Stedi API Reference](https://www.stedi.com/docs/api-reference/healthcare/post-healthcare-eligibility)
//! for complete field requirements and constraints.

use serde::{Deserialize, Serialize};

/// ValidationException : A standard error for input validation failures. This should be thrown by services when a member of the input structure falls outside of the modeled or documented constraints.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationException {
    /// A list of specific failures encountered while validating the input. A member can appear in this list more than once if it failed to satisfy multiple constraints.
    #[serde(rename = "fieldList", skip_serializing_if = "Option::is_none")]
    pub field_list: Option<Vec<super::ValidationExceptionField>>,
    /// A summary of the validation failure.
    #[serde(rename = "message")]
    pub message: String,
}

impl ValidationException {
    /// A standard error for input validation failures. This should be thrown by services when a member of the input structure falls outside of the modeled or documented constraints.
    pub fn new(message: String) -> ValidationException {
        ValidationException {
            field_list: None,
            message,
        }
    }
}
