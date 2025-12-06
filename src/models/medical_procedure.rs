use serde::{Deserialize, Serialize};

/// Medical procedure information for eligibility checks with multiple procedure codes.
///
/// This struct represents a single medical procedure with its associated codes, modifiers,
/// and diagnosis pointers. It is used in `encounter.medicalProcedures` when you need to send
/// multiple procedure codes in a single eligibility check request.
///
/// ## When to Use MedicalProcedure
///
/// Use `MedicalProcedure` in the `medicalProcedures` array when:
/// - You need to check eligibility for multiple procedures in a single request
/// - Each procedure requires its own diagnosis code pointers or modifiers
/// - You want to specify procedure-specific information beyond what's available in the
///   single `encounter.procedureCode` field
///
/// For single procedure codes, use `encounter.procedureCode` and
/// `encounter.productOrServiceIDQualifier` instead.
///
/// ## Procedure Code Support
///
/// Many payers do not support procedure code-specific eligibility checks. If a payer doesn't
/// support procedure codes, they may return a generic benefits response for service type code
/// `30` (Plan coverage and general benefits) instead of procedure-specific information.
///
/// ## Required Fields
///
/// - `procedureCode`: The actual procedure code value (required)
/// - `productOrServiceIDQualifier`: The type of procedure code (required)
///
/// ## Optional Fields
///
/// - `diagnosisCodePointer`: References to diagnosis codes for this procedure
/// - `procedureModifiers`: Additional modifiers that provide context about the procedure
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MedicalProcedure {
    /// Diagnosis code pointers in order of importance to this procedure.
    ///
    /// These pointers are an index to the ICD-10 codes you included in the
    /// `subscriber.healthCareCodeInformation` or `dependents.healthCareCodeInformation` object
    /// arrays. The pointer values can be from 1-8 (integer numbers as strings).
    ///
    /// If you are including diagnosis codes, you **must** set at least one pointer here for
    /// the primary diagnosis. Then, you can add up to three additional pointers (up to four
    /// in total). Don't put ICD-10 codes here - use the pointers to reference codes in the
    /// `healthCareCodeInformation` array.
    ///
    /// ## Example
    ///
    /// If you have diagnosis codes in positions 1, 2, and 3 of your `healthCareCodeInformation`
    /// array, and this procedure is primarily related to diagnosis 1 with secondary diagnoses
    /// 2 and 3, you would set this field to `Some(vec!["1".to_string(), "2".to_string(), "3".to_string()])`.
    #[serde(
        rename = "diagnosisCodePointer",
        skip_serializing_if = "Option::is_none"
    )]
    pub diagnosis_code_pointer: Option<Vec<String>>,
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
    /// ## Format
    ///
    /// Provide the procedure code exactly as it appears in the coding system, without any
    /// formatting or separators. For example, use "99213" not "992.13" for CPT codes.
    #[serde(rename = "procedureCode")]
    pub procedure_code: String,
    /// Procedure modifiers that provide additional information related to the service.
    ///
    /// Modifiers are two-character codes that provide additional context about how, where,
    /// or why a procedure was performed. They can affect reimbursement and are often required
    /// for accurate claims processing.
    ///
    /// ## Common Modifier Examples
    ///
    /// - **CPT Modifiers**: `25` (Significant, separately identifiable evaluation and management
    ///   service), `59` (Distinct procedural service), `LT`/`RT` (Left/Right side)
    /// - **HCPCS Modifiers**: `E1`-`E4` (Upper/lower left/right eyelid), `F1`-`F9` (Left hand,
    ///   right hand, etc.), `TA`-`T9` (Left foot, right foot, etc.)
    ///
    /// Include modifiers when they are relevant to the procedure and may affect benefit
    /// determination or coverage.
    #[serde(rename = "procedureModifiers", skip_serializing_if = "Option::is_none")]
    pub procedure_modifiers: Option<Vec<String>>,
    /// Code identifying the type or source of the procedure code.
    ///
    /// This qualifier specifies what coding system the `procedureCode` belongs to. It's required
    /// to ensure the payer interprets the code correctly.
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
    /// This field is required and tells the payer which coding system to use when interpreting
    /// the procedure code. For example, if you provide a CPT code like "99213", set this to
    /// `CurrentProceduralTerminologyCodes` (`CJ`).
    #[serde(rename = "productOrServiceIDQualifier")]
    pub product_or_service_id_qualifier: super::MedicalProcedureProductOrServiceIdQualifier,
}

impl MedicalProcedure {
    /// Creates a new `MedicalProcedure` instance with the required procedure information.
    ///
    /// This constructor initializes a medical procedure object with a procedure code and its
    /// qualifier. Both fields are required to properly identify and classify the medical
    /// procedure for eligibility checking purposes.
    ///
    /// Optional fields (`diagnosisCodePointer` and `procedureModifiers`) are initialized to
    /// `None` and can be set individually if needed.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{MedicalProcedure, MedicalProcedureProductOrServiceIdQualifier};
    ///
    /// // Create a medical procedure for a CPT code (99213 - Office visit)
    /// let procedure = MedicalProcedure::new(
    ///     "99213".to_string(),
    ///     MedicalProcedureProductOrServiceIdQualifier::CurrentProceduralTerminologyCodes,
    /// );
    ///
    /// // Optionally add diagnosis pointers and modifiers
    /// let mut procedure_with_modifiers = MedicalProcedure::new(
    ///     "99213".to_string(),
    ///     MedicalProcedureProductOrServiceIdQualifier::CurrentProceduralTerminologyCodes,
    /// );
    /// procedure_with_modifiers.procedure_modifiers = Some(vec!["25".to_string()]);
    /// procedure_with_modifiers.diagnosis_code_pointer = Some(vec!["1".to_string()]);
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Procedure code format**: Provide codes without formatting (e.g., "99213" not "992.13")
    /// - **Qualifier required**: Always specify the correct qualifier for the procedure code type
    /// - **Multiple procedures**: Use this in the `encounter.medicalProcedures` array when you
    ///   need to send multiple procedure codes
    /// - **Payer support**: Many payers don't support procedure code-specific eligibility checks
    ///
    /// # Arguments
    ///
    /// * `procedure_code` - The procedure code value (e.g., "99213" for CPT, "D0120" for ADA)
    /// * `product_or_service_id_qualifier` - The type of procedure code (CPT, HCPCS, ADA, etc.)
    ///
    /// # Returns
    ///
    /// A new `MedicalProcedure` instance with the specified procedure code and qualifier, and
    /// all optional fields set to `None`.
    pub fn new(
        procedure_code: String,
        product_or_service_id_qualifier: super::MedicalProcedureProductOrServiceIdQualifier,
    ) -> MedicalProcedure {
        MedicalProcedure {
            diagnosis_code_pointer: None,
            procedure_code,
            procedure_modifiers: None,
            product_or_service_id_qualifier,
        }
    }
}
