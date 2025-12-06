use serde::{Deserialize, Serialize};

/// Human-readable names for coverage level codes.
///
/// This enum serves as the companion to `BenefitsInformationCoverageLevelCode`,
/// providing descriptive names for each coverage level. While the code enum
/// contains abbreviated or coded representations, this enum contains the full
/// names that correspond to those codes.
///
/// Payers may sometimes return non-compliant values that don't match these
/// predefined variants.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BenefitsInformationCoverageLevelName {
    /// Coverage for children only
    #[serde(rename = "Children Only")]
    ChildrenOnly,
    /// Coverage for dependents only
    #[serde(rename = "Dependents Only")]
    DependentsOnly,
    /// Coverage for employee and their children
    #[serde(rename = "Employee and Children")]
    EmployeeAndChildren,
    /// Coverage for employee only
    #[serde(rename = "Employee Only")]
    EmployeeOnly,
    /// Coverage for employee and spouse
    #[serde(rename = "Employee and Spouse")]
    EmployeeAndSpouse,
    /// Coverage for entire family
    #[serde(rename = "Family")]
    Family,
    /// Coverage for individual
    #[serde(rename = "Individual")]
    Individual,
    /// Coverage for spouse and children
    #[serde(rename = "Spouse and Children")]
    SpouseAndChildren,
    /// Coverage for spouse only
    #[serde(rename = "Spouse Only")]
    SpouseOnly,
}

impl std::fmt::Display for BenefitsInformationCoverageLevelName {
    /// Formats the coverage level name as its full string representation.
    ///
    /// This implementation returns the human-readable name that corresponds
    /// to the coverage level code, matching the serde rename attributes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChildrenOnly => write!(f, "Children Only"),
            Self::DependentsOnly => write!(f, "Dependents Only"),
            Self::EmployeeAndChildren => write!(f, "Employee and Children"),
            Self::EmployeeOnly => write!(f, "Employee Only"),
            Self::EmployeeAndSpouse => write!(f, "Employee and Spouse"),
            Self::Family => write!(f, "Family"),
            Self::Individual => write!(f, "Individual"),
            Self::SpouseAndChildren => write!(f, "Spouse and Children"),
            Self::SpouseOnly => write!(f, "Spouse Only"),
        }
    }
}

impl Default for BenefitsInformationCoverageLevelName {
    /// Returns the default coverage level name.
    ///
    /// The default variant is `ChildrenOnly`, representing coverage for
    /// children only.
    fn default() -> BenefitsInformationCoverageLevelName {
        Self::ChildrenOnly
    }
}
