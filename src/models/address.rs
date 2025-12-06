//! Address information for healthcare eligibility requests.
//!
//! This module contains address structures used for provider and patient
//! location information in healthcare eligibility checks.

use serde::{Deserialize, Serialize};

/// Represents a physical address for healthcare providers or patients.
///
/// This struct is used to capture location information required for healthcare eligibility
/// verification processes. It includes standard address components that help identify
/// the geographical location of entities involved in healthcare transactions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The first line of the address.
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// The second line of the address.
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The two-letter country code from [Part 1 of ISO 3166](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The country subdivision code from [Part 2 of ISO 3166](https://en.wikipedia.org/wiki/ISO_3166-2).
    #[serde(
        rename = "countrySubDivisionCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_sub_division_code: Option<String>,
    /// The United States or Canadian postal code, excluding punctuation and blanks.
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state or province code for the address.
    ///
    /// This field uses the `InfallibleStateOrProvinceCode` type which ensures
    /// that only valid state or province codes are accepted. The infallible nature
    /// of this type means it cannot contain invalid values, providing data integrity
    /// for healthcare eligibility requests.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<super::InfallibleStateOrProvinceCode>,
}

impl Address {
    /// Creates a new `Address` instance with all fields set to `None`.
    ///
    /// This constructor initializes an empty address structure that can be
    /// populated with location data for healthcare providers or patients.
    /// It's typically used as a starting point when building eligibility
    /// request payloads.
    pub fn new() -> Address {
        Address {
            address1: None,
            address2: None,
            city: None,
            country_code: None,
            country_sub_division_code: None,
            postal_code: None,
            state: None,
        }
    }
}
