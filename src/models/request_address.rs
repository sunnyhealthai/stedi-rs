use serde::{Deserialize, Serialize};

/// Address information for healthcare entities in eligibility check requests.
///
/// This struct represents physical address details for healthcare entities such as
/// providers, subscribers, or dependents in eligibility requests. Address information
/// helps payers identify entities and may be used for network matching, benefit
/// determination, or administrative purposes.
///
/// ## Required Fields
///
/// The following fields are required:
/// - **`address1`**: The first line of the address (street address)
/// - **`city`**: The city name
///
/// All other fields are optional but may be required by specific payers or for
/// certain use cases.
///
/// ## Address Components
///
/// The address structure includes:
///
/// - **Street Address**: `address1` (required) and `address2` (optional) for apartment
///   numbers, suite numbers, or additional address lines
/// - **City**: `city` (required)
/// - **State/Province**: `state` for US states or Canadian provinces (optional but
///   commonly used)
/// - **Postal Code**: `postalCode` for US ZIP codes or Canadian postal codes (optional
///   but commonly used)
/// - **Country**: `countryCode` for international addresses (optional, defaults to US)
/// - **Country Subdivision**: `countrySubDivisionCode` for additional country subdivision
///   information (optional)
///
/// ## Usage Context
///
/// Address information is used in eligibility requests to:
///
/// - **Entity identification**: Help payers identify and verify entities
/// - **Network matching**: Determine if providers are in-network based on location
/// - **Benefit determination**: Some benefits may be location-specific
/// - **Administrative purposes**: Address verification, mailings, or other administrative
///   functions
/// - **Multi-location providers**: Identify specific provider locations when providers
///   have multiple locations
///
/// ## Address Format
///
/// - **US Addresses**: Typically include `address1`, `city`, `state`, and `postalCode`
/// - **Canadian Addresses**: Include `address1`, `city`, `state` (province), and
///   `postalCode` (Canadian postal code format)
/// - **International Addresses**: Include `countryCode` and potentially
///   `countrySubDivisionCode`
///
/// ## Postal Code Format
///
/// Postal codes should be provided without punctuation or blanks:
/// - US ZIP codes: `12345` or `123456789` (5-digit or 9-digit format)
/// - Canadian postal codes: `K1A0B1` (6 characters, no spaces)
///
/// ## Best Practices
///
/// - **Always include required fields**: `address1` and `city` are mandatory
/// - **Include state and postal code**: Most US/Canadian addresses should include these
/// - **Use address2 for additional info**: Suite numbers, apartment numbers, or building
///   names should go in `address2`
/// - **Avoid punctuation in postal codes**: Provide postal codes without dashes or spaces
/// - **Use standard abbreviations**: Use standard state/province codes
///
/// ## X12 HIPAA
///
/// Maps to address segments in X12 270 transactions, including:
/// - N3 segments for street address (address1, address2)
/// - N4 segments for city, state, postal code, and country
///
/// ## Examples
///
/// US address:
/// ```rust
/// use stedi_rs::models::{RequestAddress, StateOrProvinceCode};
///
/// let address = RequestAddress {
///     address1: "123 Main Street".to_string(),
///     address2: Some("Suite 100".to_string()),
///     city: "San Francisco".to_string(),
///     state: Some(StateOrProvinceCode::Ca),
///     postal_code: Some("94102".to_string()),
///     country_code: None,
///     country_sub_division_code: None,
/// };
/// ```
///
/// Canadian address:
/// ```rust
/// use stedi_rs::models::{RequestAddress, StateOrProvinceCode};
///
/// let address = RequestAddress {
///     address1: "456 Queen Street".to_string(),
///     address2: None,
///     city: "Toronto".to_string(),
///     state: Some(StateOrProvinceCode::On),
///     postal_code: Some("M5H2M9".to_string()),
///     country_code: Some("CA".to_string()),
///     country_sub_division_code: None,
/// };
/// ```
///
/// ## Stedi Notes
///
/// Address information is typically required for providers when they have multiple
/// locations and you need to identify the specific location making the request. For
/// subscribers and dependents, address information may be optional depending on the
/// payer's requirements.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestAddress {
    /// The first line of the address (street address).
    ///
    /// This is the primary street address line, containing the street number and name.
    /// Examples: "123 Main Street", "456 Oak Avenue", "789 Broadway". This field is
    /// required and must be provided.
    #[serde(rename = "address1")]
    pub address1: String,
    /// The second line of the address (optional additional address information).
    ///
    /// Used for additional address information such as apartment numbers, suite numbers,
    /// unit numbers, building names, or floor numbers. Examples: "Suite 100", "Apt 5B",
    /// "Building 2", "Floor 3". This field is optional.
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city name.
    ///
    /// The city or municipality name for the address. Examples: "San Francisco",
    /// "New York", "Toronto", "Vancouver". This field is required and must be provided.
    #[serde(rename = "city")]
    pub city: String,
    /// The two-letter country code from [Part 1 of ISO 3166](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    ///
    /// Specifies the country using ISO 3166-1 alpha-2 codes. Common codes include "US"
    /// for United States, "CA" for Canada, "MX" for Mexico. This field is optional and
    /// defaults to "US" for most US addresses. Include when the address is outside the
    /// United States or when explicitly required.
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The country subdivision code from [Part 2 of ISO 3166](https://en.wikipedia.org/wiki/ISO_3166-2).
    ///
    /// Specifies additional country subdivision information using ISO 3166-2 codes. This
    /// field is optional and is typically used for international addresses or when
    /// additional subdivision information is needed beyond the state/province code.
    #[serde(
        rename = "countrySubDivisionCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub country_sub_division_code: Option<String>,
    /// The United States or Canadian postal code, excluding punctuation and blanks.
    ///
    /// For US addresses, provide ZIP codes in 5-digit (e.g., "12345") or 9-digit format
    /// (e.g., "123456789") without dashes. For Canadian addresses, provide postal codes
    /// in 6-character format (e.g., "K1A0B1") without spaces.
    ///
    /// This field is optional but commonly included for US and Canadian addresses to help
    /// with address verification and network matching.
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The US state or Canadian province code.
    ///
    /// Specifies the state (for US addresses) or province (for Canadian addresses) using
    /// standardized codes. This field is optional but commonly included for US and
    /// Canadian addresses to help with address verification, network matching, and
    /// benefit determination.
    ///
    /// Use the `StateOrProvinceCode` enum to specify valid state or province codes.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<super::StateOrProvinceCode>,
}

impl RequestAddress {
    /// Creates a new `RequestAddress` instance with the required address fields.
    ///
    /// This constructor initializes an address object with the minimum required fields
    /// (`address1` and `city`). All optional fields are initialized to `None` and can
    /// be set individually if needed.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{RequestAddress, StateOrProvinceCode};
    ///
    /// // Create a basic address with required fields
    /// let mut address = RequestAddress::new(
    ///     "123 Main Street".to_string(),
    ///     "San Francisco".to_string(),
    /// );
    ///
    /// // Optionally add additional address information
    /// address.address2 = Some("Suite 100".to_string());
    /// address.state = Some(StateOrProvinceCode::Ca);
    /// address.postal_code = Some("94102".to_string());
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Required fields**: `address1` and `city` must be provided
    /// - **Optional fields**: `address2`, `state`, `postalCode`, `countryCode`, and
    ///   `countrySubDivisionCode` are optional but commonly used
    /// - **Postal code format**: Provide postal codes without punctuation or spaces
    /// - **State/Province**: Use `StateOrProvinceCode` enum for valid codes
    /// - **Multi-line addresses**: Use `address2` for suite numbers, apartment numbers,
    ///   or additional address lines
    ///
    /// # Arguments
    ///
    /// * `address1` - The first line of the address (street address)
    /// * `city` - The city name
    ///
    /// # Returns
    ///
    /// A new `RequestAddress` instance with the specified address and city, and all
    /// optional fields set to `None`.
    pub fn new(address1: String, city: String) -> RequestAddress {
        RequestAddress {
            address1,
            address2: None,
            city,
            country_code: None,
            country_sub_division_code: None,
            postal_code: None,
            state: None,
        }
    }
}
