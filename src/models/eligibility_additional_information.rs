use serde::{Deserialize, Serialize};

/// Additional eligibility information for benefits, including nature of injury codes and place of service codes.
///
/// This struct provides supplementary information about benefits that may include nature of injury
/// codes, facility type codes (place of service codes), and descriptions of injured body parts.
/// It is used in `benefitsInformation.eligibilityAdditionalInformation` and
/// `benefitsInformation.eligibilityAdditionalInformationList` when there are multiple nature of
/// injury codes or facility type codes included in the response.
///
/// ## Primary Use Cases
///
/// - **Nature of Injury Codes**: Used primarily for workers' compensation claims to specify the
///   type of injury or condition (e.g., sprain, fracture, laceration). The code category is
///   always set to `44` - Nature of Injury.
/// - **Place of Service Codes**: When `codeListQualifierCode` is set to `ZZ` (Mutually Defined),
///   the `industryCode` property contains a place of service code indicating where healthcare
///   services are provided (e.g., office, hospital, skilled nursing facility).
/// - **Injured Body Parts**: Descriptions of specific body parts affected by an injury, often
///   used in conjunction with nature of injury codes.
///
/// ## Code Lists
///
/// The struct supports multiple code list qualifiers:
/// - `GR` - National Council on Compensation Insurance (NCCI) Nature of Injury Code
/// - `NI` - Nature of Injury Code
/// - `ZZ` - Mutually Defined (used for place of service codes)
///
/// When `codeListQualifierCode` is `ZZ`, the `industryCode` will be set to a place of service
/// code from the CMS Place of Service Code Set. Visit the [Place of Service Code Set](https://www.cms.gov/medicare/coding-billing/place-of-service-codes/code-sets)
/// for a complete list of these codes and their descriptions.
///
/// ## Multiple Entries
///
/// When multiple nature of injury codes or facility type codes are present in a response,
/// payers use `benefitsInformation.eligibilityAdditionalInformationList` (an array) instead
/// of the single `eligibilityAdditionalInformation` object. This allows multiple codes to be
/// returned for a single benefit entry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibilityAdditionalInformation {
    /// The code category. Always set to `44` - Nature of Injury.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "codeCategory", skip_serializing_if = "Option::is_none")]
    pub code_category: Option<super::InjuryCodeCategory>,
    /// The name of the `codeListQualifierCode`. For example `Mutually Defined` when the code is set to `ZZ`.
    #[serde(rename = "codeListQualifier", skip_serializing_if = "Option::is_none")]
    pub code_list_qualifier: Option<String>,
    /// Identifies a specific industry code list. Can be `GR` - National Council on Compensation Insurance (NCCI) Nature of Injury Code, `NI` - Nature of Injury Code, or `ZZ` - Mutually Defined.   When this is set to `ZZ`, the `industryCode` property will be set to a place of service code.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "codeListQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub code_list_qualifier_code: Option<super::CodeListQualifierCode>,
    /// The name of the `industryCode`. For example `Pharmacy` when the code is `01`.
    #[serde(rename = "industry", skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// The specific industry code. When `codeListQualifierCode` is set to `ZZ` - Mutually Defined, this property will be set to a place of service code. Visit the [Place of Service Code Set](https://www.cms.gov/medicare/coding-billing/place-of-service-codes/code-sets) for a complete list of these codes and their descriptions.
    #[serde(rename = "industryCode", skip_serializing_if = "Option::is_none")]
    pub industry_code: Option<String>,
    /// Description of injured body parts.
    #[serde(
        rename = "injuredBodyPartName",
        skip_serializing_if = "Option::is_none"
    )]
    pub injured_body_part_name: Option<String>,
}

impl EligibilityAdditionalInformation {
    /// Creates a new instance of `EligibilityAdditionalInformation` with all fields initialized to `None`.
    ///
    /// This constructor provides a clean starting point for building eligibility additional
    /// information data structures, typically used when processing eligibility response data
    /// where specific fields will be populated based on the received information.
    ///
    /// # Returns
    ///
    /// A new `EligibilityAdditionalInformation` instance with all optional fields set to `None`.
    pub fn new() -> EligibilityAdditionalInformation {
        EligibilityAdditionalInformation {
            code_category: None,
            code_list_qualifier: None,
            code_list_qualifier_code: None,
            industry: None,
            industry_code: None,
            injured_body_part_name: None,
        }
    }
}
