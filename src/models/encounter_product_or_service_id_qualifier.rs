use serde::{Deserialize, Serialize};

/// EncounterProductOrServiceIdQualifier : Code identifying the type/source of the `procedureCode`. You can set this to `AD` - American Dental Association Codes, `CJ` - Current Procedural Terminology (CPT) Codes, `HC` - Health Care Financing Administration Common Procedural Coding System (HCPCS) Codes, `ID` - International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Procedure, `IV` - Home Infusion EDI Coalition (HIEC) Product/Service Code, `N4` - National Drug Code in 5-4-2 Format, or `ZZ` - Mutually Defined.
/// Code identifying the type/source of the `procedureCode`. You can set this to `AD` - American Dental Association Codes, `CJ` - Current Procedural Terminology (CPT) Codes, `HC` - Health Care Financing Administration Common Procedural Coding System (HCPCS) Codes, `ID` - International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Procedure, `IV` - Home Infusion EDI Coalition (HIEC) Product/Service Code, `N4` - National Drug Code in 5-4-2 Format, or `ZZ` - Mutually Defined.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum EncounterProductOrServiceIdQualifier {
    /// American Dental Association Codes
    #[serde(rename = "AD")]
    #[default]
    AmericanDentalAssociationCodes,
    /// Current Procedural Terminology (CPT) Codes
    #[serde(rename = "CJ")]
    CurrentProceduralTerminologyCodes,
    /// Health Care Financing Administration Common Procedural Coding System (HCPCS) Codes
    #[serde(rename = "HC")]
    HcpcsCodes,
    /// International Classification of Diseases, 9th Revision, Clinical Modification (ICD-9-CM) - Procedure
    #[serde(rename = "ID")]
    Icd9CmProcedure,
    /// Home Infusion EDI Coalition (HIEC) Product/Service Code
    #[serde(rename = "IV")]
    HomeInfusionEdiCoalitionProductServiceCode,
    /// National Drug Code in 5-4-2 Format
    #[serde(rename = "N4")]
    NationalDrugCode542Format,
    /// Mutually Defined
    #[serde(rename = "ZZ")]
    MutuallyDefined,
}

impl std::fmt::Display for EncounterProductOrServiceIdQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AmericanDentalAssociationCodes => write!(f, "AD"),
            Self::CurrentProceduralTerminologyCodes => write!(f, "CJ"),
            Self::HcpcsCodes => write!(f, "HC"),
            Self::Icd9CmProcedure => write!(f, "ID"),
            Self::HomeInfusionEdiCoalitionProductServiceCode => write!(f, "IV"),
            Self::NationalDrugCode542Format => write!(f, "N4"),
            Self::MutuallyDefined => write!(f, "ZZ"),
        }
    }
}
