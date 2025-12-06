use serde::{Deserialize, Serialize};

/// Healthcare diagnosis information for condition-specific eligibility checks.
///
/// This struct contains diagnosis codes (ICD-9-CM or ICD-10-CM) that specify the patient's
/// medical condition or diagnosis. Diagnosis codes are used in eligibility check requests to
/// help payers determine condition-specific benefit coverage, prior authorization requirements,
/// and medical necessity for certain procedures or services.
///
/// ## Usage Context
///
/// `HealthCareInformation` is used in:
/// - `requestSubscriber.healthCareCodeInformation` - Diagnosis codes for the subscriber
/// - `requestDependent.healthCareCodeInformation` - Diagnosis codes for dependents
///
/// When diagnosis codes are included in an eligibility check request, they are referenced
/// by diagnosis code pointers in the `encounter.diagnosisCodePointer` field. The pointers
/// (1-8) correspond to the order of diagnosis codes in the `healthCareCodeInformation` array,
/// with the primary diagnosis typically being the first one.
///
/// ## Diagnosis Code Format
///
/// - **Omit decimal points**: Diagnosis codes should be provided without decimal points
///   (e.g., "E119" not "E11.9"). The decimal point is assumed and will be added by the payer.
/// - **ICD-10-CM**: The current standard (use `ABK` or `ABF` for ICD-10-CM codes)
/// - **ICD-9-CM**: Deprecated and should no longer be used (use `BK` or `BF` only if required)
///
/// ## When to Include Diagnosis Codes
///
/// Include diagnosis codes when:
/// - Checking eligibility for condition-specific benefits or services
/// - The payer requires diagnosis codes for certain procedure codes
/// - You need to verify medical necessity or prior authorization requirements
/// - Checking coverage for treatments related to specific conditions
///
/// Many payers can return benefits information without diagnosis codes, so they're typically
/// optional unless specifically required by the payer or when checking condition-specific coverage.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthCareInformation {
    /// The diagnosis code. Omit the decimal points in diagnosis codes - the decimal point is assumed.
    ///
    /// This field contains the actual diagnosis code value (e.g., "E119" for Type 2 diabetes
    /// mellitus without complications in ICD-10-CM). The code should be provided without
    /// decimal points, as the payer will add them automatically.
    ///
    /// ## Format Requirements
    ///
    /// - **ICD-10-CM codes**: Typically 3-7 characters (e.g., "E119", "I10", "M79611")
    /// - **ICD-9-CM codes**: Typically 3-5 characters (deprecated, avoid if possible)
    /// - **No decimal points**: Provide codes without decimal separators
    /// - **Alphanumeric**: Codes may contain both letters and numbers
    ///
    /// The type of diagnosis code (ICD-9 vs ICD-10, principal vs regular) is specified by
    /// the `diagnosisTypeCode` field.
    #[serde(rename = "diagnosisCode")]
    pub diagnosis_code: String,
    /// The type of diagnosis code indicating the coding system and whether it's a principal diagnosis.
    ///
    /// This field specifies which diagnosis coding system is being used and whether the diagnosis
    /// code represents a principal diagnosis (the main condition being treated) or a secondary
    /// diagnosis (additional conditions).
    ///
    /// ## Diagnosis Type Values
    ///
    /// - **`ABK` - ICD-10-CM Principal Diagnosis**: The primary diagnosis using ICD-10-CM codes
    ///   (recommended). Use this for the main condition being treated.
    /// - **`ABF` - ICD-10-CM Diagnosis**: A secondary or additional diagnosis using ICD-10-CM
    ///   codes (recommended). Use this for additional conditions beyond the principal diagnosis.
    /// - **`BK` - ICD-9-CM Principal Diagnosis**: The primary diagnosis using ICD-9-CM codes
    ///   (deprecated, avoid if possible)
    /// - **`BF` - ICD-9-CM Diagnosis**: A secondary diagnosis using ICD-9-CM codes (deprecated,
    ///   avoid if possible)
    ///
    /// ## Recommendations
    ///
    /// - **Use ICD-10-CM codes**: ICD-9 codes are deprecated and should no longer be used in
    ///   eligibility checks. Always prefer `ABK` or `ABF` over `BK` or `BF`.
    /// - **Principal vs Secondary**: If you're including multiple diagnosis codes, mark the
    ///   primary condition as a principal diagnosis (`ABK`) and additional conditions as
    ///   regular diagnoses (`ABF`).
    /// - **Order matters**: When including multiple diagnosis codes, the first one in the array
    ///   should typically be the principal diagnosis, and it should be referenced first in
    ///   `encounter.diagnosisCodePointer`.
    #[serde(rename = "diagnosisTypeCode")]
    pub diagnosis_type_code: super::DiagnosisTypeCode,
}

impl HealthCareInformation {
    /// Creates a new `HealthCareInformation` instance with the required diagnosis information.
    ///
    /// This constructor initializes a healthcare information object with a diagnosis code and
    /// its type. Both fields are required to properly identify and classify the patient's
    /// medical condition for eligibility checking purposes.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{HealthCareInformation, DiagnosisTypeCode};
    ///
    /// // Create a principal diagnosis for Type 2 diabetes (ICD-10-CM: E11.9)
    /// let diagnosis = HealthCareInformation::new(
    ///     "E119".to_string(),  // Note: no decimal point
    ///     DiagnosisTypeCode::Icd10CmPrincipalDiagnosis,
    /// );
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **No decimal points**: Provide diagnosis codes without decimal separators
    /// - **Use ICD-10-CM**: Prefer `Icd10CmPrincipalDiagnosis` or `Icd10CmDiagnosis` over
    ///   ICD-9-CM variants, as ICD-9 codes are deprecated
    /// - **Principal diagnosis**: Use principal diagnosis type (`ABK`) for the primary condition
    ///
    /// # Arguments
    ///
    /// * `diagnosis_code` - The diagnosis code without decimal points (e.g., "E119" for E11.9)
    /// * `diagnosis_type_code` - The type of diagnosis code (ICD-9 vs ICD-10, principal vs regular)
    ///
    /// # Returns
    ///
    /// A new `HealthCareInformation` instance with the specified diagnosis code and type.
    pub fn new(
        diagnosis_code: String,
        diagnosis_type_code: super::DiagnosisTypeCode,
    ) -> HealthCareInformation {
        HealthCareInformation {
            diagnosis_code,
            diagnosis_type_code,
        }
    }
}
