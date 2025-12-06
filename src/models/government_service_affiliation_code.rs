use serde::{Deserialize, Serialize};

/// Government service affiliation codes indicating the government service affiliation
/// of the subscriber or dependent as it relates to military service.
///
/// These codes are returned in the `subscriber.governmentServiceAffiliationCode` and
/// `dependent.governmentServiceAffiliationCode` properties.
///
/// Note: Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum GovernmentServiceAffiliationCode {
    /// Air Force
    #[serde(rename = "A")]
    #[default]
    AirForce,

    /// Air Force Reserves
    #[serde(rename = "B")]
    AirForceReserves,

    /// Army
    #[serde(rename = "C")]
    Army,

    /// Army Reserves
    #[serde(rename = "D")]
    ArmyReserves,

    /// Coast Guard
    #[serde(rename = "E")]
    CoastGuard,

    /// Marine Corps
    #[serde(rename = "F")]
    MarineCorps,

    /// Marine Corps Reserves
    #[serde(rename = "G")]
    MarineCorpsReserves,

    /// National Guard
    #[serde(rename = "H")]
    NationalGuard,

    /// Navy
    #[serde(rename = "I")]
    Navy,

    /// Navy Reserves
    #[serde(rename = "J")]
    NavyReserves,

    /// Other
    #[serde(rename = "K")]
    Other,

    /// Peace Corp
    #[serde(rename = "L")]
    PeaceCorp,

    /// Regular Armed Forces
    #[serde(rename = "M")]
    RegularArmedForces,

    /// Reserves
    #[serde(rename = "N")]
    Reserves,

    /// U.S. Public Health Service
    #[serde(rename = "O")]
    PublicHealthService,

    /// Foreign Military
    #[serde(rename = "Q")]
    ForeignMilitary,

    /// American Red Cross
    #[serde(rename = "R")]
    AmericanRedCross,

    /// Department of Defense
    #[serde(rename = "S")]
    DepartmentOfDefense,

    /// United Services Organization
    #[serde(rename = "U")]
    UnitedServicesOrganization,

    /// Military Sealift Command
    #[serde(rename = "W")]
    MilitarySealiftCommand,
}

impl std::fmt::Display for GovernmentServiceAffiliationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AirForce => write!(f, "A"),
            Self::AirForceReserves => write!(f, "B"),
            Self::Army => write!(f, "C"),
            Self::ArmyReserves => write!(f, "D"),
            Self::CoastGuard => write!(f, "E"),
            Self::MarineCorps => write!(f, "F"),
            Self::MarineCorpsReserves => write!(f, "G"),
            Self::NationalGuard => write!(f, "H"),
            Self::Navy => write!(f, "I"),
            Self::NavyReserves => write!(f, "J"),
            Self::Other => write!(f, "K"),
            Self::PeaceCorp => write!(f, "L"),
            Self::RegularArmedForces => write!(f, "M"),
            Self::Reserves => write!(f, "N"),
            Self::PublicHealthService => write!(f, "O"),
            Self::ForeignMilitary => write!(f, "Q"),
            Self::AmericanRedCross => write!(f, "R"),
            Self::DepartmentOfDefense => write!(f, "S"),
            Self::UnitedServicesOrganization => write!(f, "U"),
            Self::MilitarySealiftCommand => write!(f, "W"),
        }
    }
}
