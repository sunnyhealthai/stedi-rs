use serde::{Deserialize, Serialize};

/// DiagnosisTypeCode : The type of diagnosis code you are providing. You can set to `BK` - International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Diagnosis, `ABK` - International Classification of Diseases Clinical Modification (ICD-10-CM) Principal Diagnosis, `BF`- International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis, or `ABF`- International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis.    Note that ICD-9 codes are **deprecated** and should no longer be used in eligibility checks.
/// The type of diagnosis code you are providing. You can set to `BK` - International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Diagnosis, `ABK` - International Classification of Diseases Clinical Modification (ICD-10-CM) Principal Diagnosis, `BF`- International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis, or `ABF`- International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis.    Note that ICD-9 codes are **deprecated** and should no longer be used in eligibility checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
#[derive(Default)]
pub enum DiagnosisTypeCode {
    /// International Classification of Diseases Clinical Modification (ICD-9-CM) Principal Diagnosis
    #[serde(rename = "BK")]
    #[default]
    Icd9CmPrincipalDiagnosis,
    /// International Classification of Diseases Clinical Modification (ICD-10-CM) Principal Diagnosis
    #[serde(rename = "ABK")]
    Icd10CmPrincipalDiagnosis,
    /// International Classification of Diseases Clinical Modification (ICD-9-CM) Diagnosis
    #[serde(rename = "BF")]
    Icd9CmDiagnosis,
    /// International Classification of Diseases Clinical Modification (ICD-10-CM) Diagnosis
    #[serde(rename = "ABF")]
    Icd10CmDiagnosis,
}

impl std::fmt::Display for DiagnosisTypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Icd9CmPrincipalDiagnosis => write!(f, "BK"),
            Self::Icd10CmPrincipalDiagnosis => write!(f, "ABK"),
            Self::Icd9CmDiagnosis => write!(f, "BF"),
            Self::Icd10CmDiagnosis => write!(f, "ABF"),
        }
    }
}
