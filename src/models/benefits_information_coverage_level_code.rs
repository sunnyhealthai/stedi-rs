use serde::{Deserialize, Serialize};

/// Healthcare coverage level codes for eligibility checks.
///
/// These codes indicate the level of coverage for a patient under an insurance policy
/// and are commonly used in healthcare eligibility verification processes.
/// The codes include standard values like `CHD` for Children Only and `FAM` for Family,
/// though payers may sometimes return other non-compliant values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BenefitsInformationCoverageLevelCode {
    /// Children Only (CHD) - Coverage applies to children only.
    #[serde(rename = "CHD")]
    ChildrenOnly,
    /// Dependents Only (DEP) - Coverage applies to dependents only.
    #[serde(rename = "DEP")]
    DependentsOnly,
    /// Employee and Children (ECH) - Coverage applies to employee and their children.
    #[serde(rename = "ECH")]
    EmployeeAndChildren,
    /// Employee Only (EMP) - Coverage applies to employee only.
    #[serde(rename = "EMP")]
    EmployeeOnly,
    /// Employee and Spouse (ESP) - Coverage applies to employee and their spouse.
    #[serde(rename = "ESP")]
    EmployeeAndSpouse,
    /// Family (FAM) - Coverage applies to the entire family.
    #[serde(rename = "FAM")]
    Family,
    /// Individual (IND) - Coverage applies to an individual.
    #[serde(rename = "IND")]
    Individual,
    /// Spouse and Children (SPC) - Coverage applies to spouse and children.
    #[serde(rename = "SPC")]
    SpouseAndChildren,
    /// Spouse Only (SPO) - Coverage applies to spouse only.
    #[serde(rename = "SPO")]
    SpouseOnly,
}

impl std::fmt::Display for BenefitsInformationCoverageLevelCode {
    /// Formats the coverage level code as its standard string representation.
    ///
    /// This implementation returns the exact code value that would be used in
    /// healthcare eligibility transactions (e.g., "CHD", "FAM", "EMP").
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChildrenOnly => write!(f, "CHD"),
            Self::DependentsOnly => write!(f, "DEP"),
            Self::EmployeeAndChildren => write!(f, "ECH"),
            Self::EmployeeOnly => write!(f, "EMP"),
            Self::EmployeeAndSpouse => write!(f, "ESP"),
            Self::Family => write!(f, "FAM"),
            Self::Individual => write!(f, "IND"),
            Self::SpouseAndChildren => write!(f, "SPC"),
            Self::SpouseOnly => write!(f, "SPO"),
        }
    }
}

impl Default for BenefitsInformationCoverageLevelCode {
    /// Returns the default coverage level code.
    ///
    /// The default is set to `ChildrenOnly` (`CHD`) as a fallback value.
    fn default() -> BenefitsInformationCoverageLevelCode {
        Self::ChildrenOnly
    }
}
