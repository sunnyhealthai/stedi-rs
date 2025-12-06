//! Healthcare encounter information for eligibility checks.
//!
//! This module contains the structure for specifying encounter details in eligibility
//! check requests, including service dates, service types, and procedure codes.
//!
//! ## Service Dates
//!
//! You can specify service dates in two ways:
//! - Single date: Use `dateOfService` for a specific date
//! - Date range: Use `beginningDateOfService` and `endDateOfService` together
//!
//! If no dates are provided, the payer uses the current date in their timezone.
//!
//! ### Date Recommendations
//!
//! - For eligibility checks for "today", omit date fields for consistent behavior
//! - Use dates up to 12 months in the past or up to the current month end
//! - Some payers (like CMS) support future dates, especially the next calendar month
//! - Always use YYYYMMDD format for dates
//!
//! ## Service Type Codes
//!
//! Service Type Codes (STCs) specify what type of benefits information to request:
//! - If no service types are specified, defaults to "30" (Plan coverage and general benefits)
//! - Include only one service type code per request for best results
//! - See [Service Type Codes documentation](https://www.stedi.com/docs/healthcare/eligibility-stc-procedure-codes)
//!
//! ## Procedure Codes
//!
//! For specific procedures, provide:
//! - `procedureCode`: The specific CPT, HCPCS, or other procedure code
//! - `productOrServiceIDQualifier`: The type of procedure code being used

use serde::{Deserialize, Serialize};

/// Details about the eligibility or benefit information you are requesting for the patient.
///
/// The encounter object specifies what types of benefits information to retrieve and when
/// the services are expected to occur. This information helps payers return relevant
/// benefits data for specific service types and dates.
///
/// ## Service Type Codes and Procedure Codes
///
/// You must specify either:
/// - **Service Type Codes** (`serviceTypeCodes`): Two-character codes that group similar
///   healthcare services into standard categories (e.g., `30` for Plan coverage, `47` for
///   Hospital, `UC` for Urgent Care). If you don't specify either service type codes or
///   procedure codes, Stedi defaults to using `30` (Plan coverage and general benefits).
/// - **Procedure Codes** (`procedureCode` + `productOrServiceIDQualifier`): Specific CPT,
///   HCPCS, ADA, or other procedure codes for procedure-specific eligibility checks. Many
///   payers don't support procedure code-specific eligibility checks.
///
/// Visit [STCs and procedure codes](https://www.stedi.com/docs/healthcare/eligibility-stc-procedure-codes)
/// for detailed guidance on using service type codes and procedure codes.
///
/// ## Service Dates
///
/// You can specify service dates in two ways:
/// - **Single date**: Use `dateOfService` for a specific date (e.g., a doctor's visit)
/// - **Date range**: Use `beginningDateOfService` and `endDateOfService` together for a
///   range of dates
///
/// If no dates are provided, the payer defaults to using the current date in their timezone.
/// When checking eligibility for today, omit the `dateOfService` property to ensure consistent
/// behavior across payers.
///
/// ### Date Recommendations
///
/// - Submit dates up to 12 months in the past or up to the end of the current month
/// - Some payers (like CMS) support future dates, especially the next calendar month
/// - Always use `YYYYMMDD` format for dates
/// - Dates must be valid and not in the far future (most payers only support dates through
///   the current calendar month)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encounter {
    /// The beginning date of service. If you include this value, you must also include the `endDateOfService`.
    #[serde(
        rename = "beginningDateOfService",
        skip_serializing_if = "Option::is_none"
    )]
    pub beginning_date_of_service: Option<String>,
    /// The date of service. You can use this value to specify a single occasion, such as a doctor's visit. If you don't specify a service date (either a single day or a range of dates), the payer defaults to using the current date in their timezone.
    #[serde(rename = "dateOfService", skip_serializing_if = "Option::is_none")]
    pub date_of_service: Option<String>,
    /// Diagnosis code pointers in order of importance to the service. These pointers are an index to the ICD-10 codes you included in the `subscriber.healthCareCodeInformation` or `dependents.healthCareCodeInformation` object arrays. The pointer values can be from 1 - 8 (integer numbers). If you are including diagnosis codes, you **must** set at least one pointer here for the primary diagnosis. Then, you can add up to three additional pointers (up to four in total). Don't put ICD-10 codes here.
    #[serde(
        rename = "diagnosisCodePointer",
        skip_serializing_if = "Option::is_none"
    )]
    pub diagnosis_code_pointer: Option<Vec<String>>,
    /// The end date of service. If you include this value, you must also include the `beginningDateOfService`.
    #[serde(rename = "endDateOfService", skip_serializing_if = "Option::is_none")]
    pub end_date_of_service: Option<String>,
    /// The type of facility where the service was provided (place of service code).
    ///
    /// This field specifies the location where the healthcare service will be or was performed,
    /// which can affect benefit coverage and reimbursement rates. Place of service codes are
    /// standardized by the Centers for Medicare and Medicaid Services (CMS).
    ///
    /// Common place of service codes include:
    /// - `11` - Office
    /// - `12` - Home
    /// - `20` - Urgent Care Facility
    /// - `21` - Inpatient Hospital
    /// - `22` - On Campus - Outpatient Hospital
    /// - `23` - Emergency Room - Hospital
    /// - `24` - Ambulatory Surgical Center
    /// - `31` - Skilled Nursing Facility
    /// - `34` - Hospice
    ///
    /// Visit the [CMS Place of Service Code Set](https://www.cms.gov/medicare/coding-billing/place-of-service-codes/code-sets)
    /// for a complete list of place of service codes and their descriptions.
    ///
    /// ## Usage
    ///
    /// Include this field when:
    /// - The payer requires place of service information for benefit determination
    /// - Different benefit levels apply based on where services are provided
    /// - You need to specify a non-standard location (e.g., home, school, mobile unit)
    ///
    /// Many payers can determine benefits without this field, so it's typically optional unless
    /// specifically required by the payer or when checking eligibility for services in non-standard
    /// locations.
    #[serde(rename = "industryCode", skip_serializing_if = "Option::is_none")]
    pub industry_code: Option<super::IndustryCode>,
    /// Use only when you need to send multiple procedure codes in a single request. Otherwise, use the `encounter.procedureCode` and `encounter.productOrServiceIDQualifier` properties.
    #[serde(rename = "medicalProcedures", skip_serializing_if = "Option::is_none")]
    pub medical_procedures: Option<Vec<super::MedicalProcedure>>,
    /// The prior authorization or referral number for a particular benefit or procedure.
    #[serde(
        rename = "priorAuthorizationOrReferralNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub prior_authorization_or_referral_number: Option<String>,
    /// The procedure code identifying the specific medical procedure or service.
    ///
    /// This field contains the actual procedure code value (e.g., "99213" for a CPT code,
    /// "D0120" for an ADA dental code). The type of procedure code is specified by the
    /// `productOrServiceIDQualifier` field, which indicates whether this is a CPT, HCPCS,
    /// ADA, or other type of code.
    ///
    /// ## Common Procedure Code Types
    ///
    /// - **CPT Codes** (`CJ`): Current Procedural Terminology codes used for medical procedures
    /// - **HCPCS Codes** (`HC`): Healthcare Common Procedure Coding System codes for services,
    ///   supplies, and equipment
    /// - **ADA Codes** (`AD`): American Dental Association codes for dental procedures
    /// - **NDC Codes** (`N4`): National Drug Code in 5-4-2 format for pharmacy/drug services
    ///
    /// ## Usage
    ///
    /// Use procedure codes when:
    /// - You need procedure-specific eligibility information (e.g., whether a specific surgery
    ///   is covered)
    /// - The payer supports procedure code-based eligibility checks (many payers don't)
    /// - You want to check coverage for a specific service rather than a service category
    ///
    /// Note: Many payers don't support procedure code-specific eligibility checks and may ignore
    /// this field. In such cases, use service type codes (`serviceTypeCodes`) instead. If you
    /// need to send multiple procedure codes, use the `medicalProcedures` array instead of
    /// `procedureCode`.
    #[serde(rename = "procedureCode", skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<String>,
    /// The procedure modifier that provides additional information related to the performance of the service.
    #[serde(rename = "procedureModifiers", skip_serializing_if = "Option::is_none")]
    pub procedure_modifiers: Option<Vec<String>>,
    /// Code identifying the type or source of the procedure code.
    ///
    /// This qualifier specifies what coding system the `procedureCode` belongs to. It's required
    /// when providing a `procedureCode` to ensure the payer interprets the code correctly.
    ///
    /// ## Common Qualifiers
    ///
    /// - **`CJ` - Current Procedural Terminology (CPT) Codes**: Most common for medical procedures
    /// - **`HC` - HCPCS Codes**: Healthcare Common Procedure Coding System codes for services,
    ///   supplies, and equipment
    /// - **`AD` - American Dental Association Codes**: Used for dental procedures
    /// - **`N4` - National Drug Code (5-4-2 Format)**: Used for pharmacy/drug services
    /// - **`ID` - ICD-9-CM Procedure**: International Classification of Diseases procedure codes
    /// - **`IV` - HIEC Product/Service Code**: Home Infusion EDI Coalition codes
    /// - **`ZZ` - Mutually Defined**: Used when the code type is defined between payer and provider
    ///
    /// ## Usage
    ///
    /// This field is required when `procedureCode` is provided. It tells the payer which coding
    /// system to use when interpreting the procedure code. For example, if you provide a CPT code
    /// like "99213", set this to `CJ` (Current Procedural Terminology Codes).
    ///
    /// If you're using service type codes instead of procedure codes, you don't need to provide
    /// this field.
    #[serde(
        rename = "productOrServiceIDQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_or_service_id_qualifier: Option<super::EncounterProductOrServiceIdQualifier>,
    /// The type of information provided in the `priorAuthorizationOrReferralNumber` field.
    ///
    /// This qualifier specifies whether the `priorAuthorizationOrReferralNumber` contains a
    /// referral number or a prior authorization number. It's required when providing a
    /// `priorAuthorizationOrReferralNumber` to ensure the payer interprets the number correctly.
    ///
    /// ## Qualifier Values
    ///
    /// - **`9F` - Referral Number**: Indicates that `priorAuthorizationOrReferralNumber` contains
    ///   a referral number from a referring provider
    /// - **`G1` - Prior Authorization Number**: Indicates that `priorAuthorizationOrReferralNumber`
    ///   contains a prior authorization number that was previously issued by the payer
    ///
    /// ## Usage
    ///
    /// This field is required when `priorAuthorizationOrReferralNumber` is provided. It tells the
    /// payer what type of authorization or referral information you're submitting, which helps
    /// them verify the authorization and determine benefit coverage.
    ///
    /// Some payers require prior authorization numbers for certain services or procedures. If you
    /// have a prior authorization number, include it along with this qualifier set to `G1`.
    #[serde(
        rename = "referenceIdentificationQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_identification_qualifier:
        Option<super::EncounterReferenceIdentificationQualifier>,
    /// One or more codes classifying the type of services for which you want to receive benefits information.  If you don't specify a service type code or a `procedureCode` and `productOrServiceIDQualifier`, Stedi defaults to using `30` - Health Benefit Plan Coverage. Visit [Service Type Codes](https://www.stedi.com/docs/healthcare/eligibility-stc-procedure-codes#full-stc-list) for a complete list.  Not all payers support all service type codes, and not all payers support multiple service type codes in the same request. We recommend including one service type code per request unless you're sure the payer supports multiple.  Payers aren't required to respond with exactly the same STC(s) in the response, so you may receive benefits information for STCs you didn't request. However, receiving different STCs can mean that the payer is ignoring the STC you sent, which is why we recommend [testing payers](https://www.stedi.com/docs/healthcare/eligibility-stc-procedure-codes#test-payer-stc-support) to determine their support for specific STCs.
    #[serde(rename = "serviceTypeCodes", skip_serializing_if = "Option::is_none")]
    pub service_type_codes: Option<Vec<super::RequestEligibilityServiceTypeCode>>,
}

impl Encounter {
    /// Creates a new `Encounter` instance with all fields initialized to `None`.
    ///
    /// This constructor provides a clean starting point for building encounter information
    /// for eligibility check requests. All optional fields are set to `None` and can be
    /// populated individually based on your needs.
    ///
    /// ## Default Behavior
    ///
    /// If you don't specify either `serviceTypeCodes` or a `procedureCode` with
    /// `productOrServiceIDQualifier`, Stedi defaults to using `30` (Plan coverage and
    /// general benefits) as the only `serviceTypeCodes` value.
    ///
    /// If you don't specify service dates, the payer defaults to using the current date
    /// in their timezone.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{Encounter, RequestEligibilityServiceTypeCode};
    ///
    /// let mut encounter = Encounter::new();
    /// encounter.service_type_codes = Some(vec![RequestEligibilityServiceTypeCode::HealthBenefitPlanCoverage]);
    /// encounter.date_of_service = Some("20240115".to_string());
    /// ```
    ///
    /// # Returns
    ///
    /// A new `Encounter` instance with all optional fields set to `None`.
    pub fn new() -> Encounter {
        Encounter {
            beginning_date_of_service: None,
            date_of_service: None,
            diagnosis_code_pointer: None,
            end_date_of_service: None,
            industry_code: None,
            medical_procedures: None,
            prior_authorization_or_referral_number: None,
            procedure_code: None,
            procedure_modifiers: None,
            product_or_service_id_qualifier: None,
            reference_identification_qualifier: None,
            service_type_codes: None,
        }
    }
}
