use serde::{Deserialize, Serialize};

/// Military service rank code identifying the rank of a service member.
///
/// This enum provides standardized codes for military service ranks across different branches
/// of the United States Armed Forces. It is used in eligibility transactions to identify the
/// rank of military personnel, which may be relevant for certain benefit determinations,
/// coverage types, or administrative purposes.
///
/// ## Military Rank Categories
///
/// The ranks are organized by branch and level:
///
/// ### Navy Ranks
/// - **Admiral** (`A1`): Highest flag officer rank
/// - **Fleet Admiral** (`F4`): Five-star admiral rank
/// - **Vice Admiral** (`V1`): Three-star admiral rank
/// - **Rear Admiral** (`R1`): Two-star admiral rank
/// - **Commodore** (`C7`): One-star admiral rank
/// - **Captain** (`C1`): O-6 rank
/// - **Commander** (`C6`): O-5 rank
/// - **Lieutenant Commander** (`L4`): O-4 rank
/// - **Lieutenant** (`L2`): O-3 rank
/// - **Lieutenant Junior Grade** (`L6`): O-2 rank
/// - **Ensign** (`E1`): O-1 rank
/// - **Master Chief Petty Officer** (`M3`): E-9 rank
/// - **Senior Chief Petty Officer** (`S5`): E-8 rank
/// - **Chief Petty Officer** (`C3`): E-7 rank
/// - **Petty Officer First Class** (`P1`): E-6 rank
/// - **Petty Officer Second Class** (`P2`): E-5 rank
/// - **Petty Officer Third Class** (`P3`): E-4 rank
/// - **Seaman** (`S1`): E-3 rank
/// - **Seaman Apprentice** (`S2`): E-2 rank
/// - **Seaman Recruit** (`S3`): E-1 rank
///
/// ### Army Ranks
/// - **General** (`G1`): O-10 rank
/// - **Lieutenant General** (`L5`): O-9 rank
/// - **Major General** (`M2`): O-8 rank
/// - **Brigadier General** (`B2`): O-7 rank
/// - **Colonel** (`C5`): O-6 rank
/// - **Lieutenant Colonel** (`L3`): O-5 rank
/// - **Major** (`M1`): O-4 rank
/// - **Captain** (`C1`): O-3 rank
/// - **First Lieutenant** (`F1`): O-2 rank
/// - **Second Lieutenant** (`S4`): O-1 rank
/// - **Sergeant Major Specialist 9** (`S9`): E-9 rank
/// - **Master Sergeant Specialist 8** (`M6`): E-8 rank
/// - **Sergeant First Class Specialist 7** (`S8`): E-7 rank
/// - **Staff Sergeant Specialist 6** (`SC`): E-6 rank
/// - **Sergeant Specialist 5** (`SA`): E-5 rank
/// - **Corporal Specialist 4** (`C9`): E-4 rank
/// - **Corporal** (`C8`): E-4 rank
/// - **Private First Class** (`P5`): E-3 rank
/// - **Private** (`P4`): E-2 rank
/// - **Recruit** (`R2`): E-1 rank
///
/// ### Air Force Ranks
/// - **General** (`G1`): O-10 rank
/// - **Lieutenant General** (`L5`): O-9 rank
/// - **Major General** (`M2`): O-8 rank
/// - **Brigadier General** (`B2`): O-7 rank
/// - **Colonel** (`C5`): O-6 rank
/// - **Lieutenant Colonel** (`L3`): O-5 rank
/// - **Major** (`M1`): O-4 rank
/// - **Captain** (`C1`): O-3 rank
/// - **First Lieutenant** (`F1`): O-2 rank
/// - **Second Lieutenant** (`S4`): O-1 rank
/// - **Chief Master Sergeant** (`C2`): E-9 rank
/// - **Senior Master Sergeant** (`S6`): E-8 rank
/// - **Master Sergeant** (`M5`): E-7 rank
/// - **Technical Sergeant** (`T1`): E-6 rank
/// - **Staff Sergeant** (`SB`): E-5 rank
/// - **Sergeant** (`S7`): E-4 rank
/// - **Airman First Class** (`A3`): E-3 rank
/// - **Airman** (`A2`): E-2 rank
/// - **Basic Airman** (`B1`): E-1 rank
///
/// ### Marine Corps Ranks
/// - **General** (`G1`): O-10 rank
/// - **Lieutenant General** (`L5`): O-9 rank
/// - **Major General** (`M2`): O-8 rank
/// - **Brigadier General** (`B2`): O-7 rank
/// - **Colonel** (`C5`): O-6 rank
/// - **Lieutenant Colonel** (`L3`): O-5 rank
/// - **Major** (`M1`): O-4 rank
/// - **Captain** (`C1`): O-3 rank
/// - **First Lieutenant** (`F1`): O-2 rank
/// - **Second Lieutenant** (`S4`): O-1 rank
/// - **Sergeant Major Specialist 9** (`S9`): E-9 rank
/// - **Master Gunnery Sergeant Major** (`M4`): E-9 rank
/// - **First Sergeant Master Sergeant** (`F3`): E-8 rank
/// - **Master Sergeant** (`M5`): E-8 rank
/// - **Gunnery Sergeant** (`G4`): E-7 rank
/// - **Staff Sergeant** (`SB`): E-6 rank
/// - **Sergeant** (`S7`): E-5 rank
/// - **Corporal** (`C8`): E-4 rank
/// - **Lance Corporal** (`L1`): E-3 rank
/// - **Private First Class** (`P5`): E-2 rank
/// - **Private** (`P4`): E-1 rank
///
/// ### Warrant Officers
/// - **Chief Warrant** (`C4`): W-4/W-5 rank
/// - **Warrant Officer** (`W1`): W-1/W-2/W-3 rank
///
/// ## Usage Context
///
/// Military service rank codes may be used in eligibility transactions when:
/// - Identifying military personnel for TRICARE or other military healthcare programs
/// - Determining eligibility for certain military-specific benefits
/// - Administrative purposes in military healthcare systems
/// - Coordinating benefits between military and civilian healthcare systems
///
/// ## Code Format
///
/// The codes follow a pattern where:
/// - Letter prefixes indicate rank categories (A=Admiral, B=Brigadier/Basic, C=Captain/Chief/Colonel, etc.)
/// - Numbers indicate specific ranks within categories
/// - Some codes use letters (SA, SB, SC) for specialist ranks
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum MilitaryServiceRankCode {
    /// Admiral (A1) - Navy flag officer rank, O-10
    #[serde(rename = "A1")]
    #[default]
    Admiral,
    /// Airman (A2) - Air Force enlisted rank, E-2
    #[serde(rename = "A2")]
    Airman,
    /// Airman First Class (A3) - Air Force enlisted rank, E-3
    #[serde(rename = "A3")]
    AirmanFirstClass,
    /// Basic Airman (B1) - Air Force enlisted rank, E-1
    #[serde(rename = "B1")]
    BasicAirman,
    /// Brigadier General (B2) - Army/Air Force/Marine Corps flag officer rank, O-7
    #[serde(rename = "B2")]
    BrigadierGeneral,
    /// Captain (C1) - Navy/Army/Air Force/Marine Corps officer rank, O-6
    #[serde(rename = "C1")]
    Captain,
    /// Chief Master Sergeant (C2) - Air Force enlisted rank, E-9
    #[serde(rename = "C2")]
    ChiefMasterSergeant,
    /// Chief Petty Officer (C3) - Navy enlisted rank, E-7
    #[serde(rename = "C3")]
    ChiefPettyOfficer,
    /// Chief Warrant (C4) - Warrant officer rank, W-4/W-5
    #[serde(rename = "C4")]
    ChiefWarrant,
    /// Colonel (C5) - Army/Air Force/Marine Corps officer rank, O-6
    #[serde(rename = "C5")]
    Colonel,
    /// Commander (C6) - Navy officer rank, O-5
    #[serde(rename = "C6")]
    Commander,
    /// Commodore (C7) - Navy flag officer rank, O-7
    #[serde(rename = "C7")]
    Commodore,
    /// Corporal (C8) - Army/Marine Corps enlisted rank, E-4
    #[serde(rename = "C8")]
    Corporal,
    /// Corporal Specialist 4 (C9) - Army enlisted rank, E-4
    #[serde(rename = "C9")]
    CorporalSpecialist4,
    /// Ensign (E1) - Navy officer rank, O-1
    #[serde(rename = "E1")]
    Ensign,
    /// First Lieutenant (F1) - Army/Air Force/Marine Corps officer rank, O-2
    #[serde(rename = "F1")]
    FirstLieutenant,
    /// First Sergeant (F2) - Army/Marine Corps enlisted rank, E-8
    #[serde(rename = "F2")]
    FirstSergeant,
    /// First Sergeant Master Sergeant (F3) - Marine Corps enlisted rank, E-8
    #[serde(rename = "F3")]
    FirstSergeantMasterSergeant,
    /// Fleet Admiral (F4) - Navy flag officer rank, O-11 (five-star)
    #[serde(rename = "F4")]
    FleetAdmiral,
    /// General (G1) - Army/Air Force/Marine Corps flag officer rank, O-10
    #[serde(rename = "G1")]
    General,
    /// Gunnery Sergeant (G4) - Marine Corps enlisted rank, E-7
    #[serde(rename = "G4")]
    GunnerySergeant,
    /// Lance Corporal (L1) - Marine Corps enlisted rank, E-3
    #[serde(rename = "L1")]
    LanceCorporal,
    /// Lieutenant (L2) - Navy officer rank, O-3
    #[serde(rename = "L2")]
    Lieutenant,
    /// Lieutenant Colonel (L3) - Army/Air Force/Marine Corps officer rank, O-5
    #[serde(rename = "L3")]
    LieutenantColonel,
    /// Lieutenant Commander (L4) - Navy officer rank, O-4
    #[serde(rename = "L4")]
    LieutenantCommander,
    /// Lieutenant General (L5) - Army/Air Force/Marine Corps flag officer rank, O-9
    #[serde(rename = "L5")]
    LieutenantGeneral,
    /// Lieutenant Junior Grade (L6) - Navy officer rank, O-2
    #[serde(rename = "L6")]
    LieutenantJuniorGrade,
    /// Major (M1) - Army/Air Force/Marine Corps officer rank, O-4
    #[serde(rename = "M1")]
    Major,
    /// Major General (M2) - Army/Air Force/Marine Corps flag officer rank, O-8
    #[serde(rename = "M2")]
    MajorGeneral,
    /// Master Chief Petty Officer (M3) - Navy enlisted rank, E-9
    #[serde(rename = "M3")]
    MasterChiefPettyOfficer,
    /// Master Gunnery Sergeant Major (M4) - Marine Corps enlisted rank, E-9
    #[serde(rename = "M4")]
    MasterGunnerySergeantMajor,
    /// Master Sergeant (M5) - Air Force/Marine Corps enlisted rank, E-7/E-8
    #[serde(rename = "M5")]
    MasterSergeant,
    /// Master Sergeant Specialist 8 (M6) - Army enlisted rank, E-8
    #[serde(rename = "M6")]
    MasterSergeantSpecialist8,
    /// Petty Officer First Class (P1) - Navy enlisted rank, E-6
    #[serde(rename = "P1")]
    PettyOfficerFirstClass,
    /// Petty Officer Second Class (P2) - Navy enlisted rank, E-5
    #[serde(rename = "P2")]
    PettyOfficerSecondClass,
    /// Petty Officer Third Class (P3) - Navy enlisted rank, E-4
    #[serde(rename = "P3")]
    PettyOfficerThirdClass,
    /// Private (P4) - Army/Marine Corps enlisted rank, E-1/E-2
    #[serde(rename = "P4")]
    Private,
    /// Private First Class (P5) - Army/Marine Corps enlisted rank, E-2/E-3
    #[serde(rename = "P5")]
    PrivateFirstClass,
    /// Rear Admiral (R1) - Navy flag officer rank, O-8
    #[serde(rename = "R1")]
    RearAdmiral,
    /// Recruit (R2) - Army enlisted rank, E-1
    #[serde(rename = "R2")]
    Recruit,
    /// Seaman (S1) - Navy enlisted rank, E-3
    #[serde(rename = "S1")]
    Seaman,
    /// Seaman Apprentice (S2) - Navy enlisted rank, E-2
    #[serde(rename = "S2")]
    SeamanApprentice,
    /// Seaman Recruit (S3) - Navy enlisted rank, E-1
    #[serde(rename = "S3")]
    SeamanRecruit,
    /// Second Lieutenant (S4) - Army/Air Force/Marine Corps officer rank, O-1
    #[serde(rename = "S4")]
    SecondLieutenant,
    /// Senior Chief Petty Officer (S5) - Navy enlisted rank, E-8
    #[serde(rename = "S5")]
    SeniorChiefPettyOfficer,
    /// Senior Master Sergeant (S6) - Air Force enlisted rank, E-8
    #[serde(rename = "S6")]
    SeniorMasterSergeant,
    /// Sergeant (S7) - Army/Air Force/Marine Corps enlisted rank, E-4/E-5
    #[serde(rename = "S7")]
    Sergeant,
    /// Sergeant First Class Specialist 7 (S8) - Army enlisted rank, E-7
    #[serde(rename = "S8")]
    SergeantFirstClassSpecialist7,
    /// Sergeant Major Specialist 9 (S9) - Army/Marine Corps enlisted rank, E-9
    #[serde(rename = "S9")]
    SergeantMajorSpecialist9,
    /// Sergeant Specialist 5 (SA) - Army enlisted rank, E-5
    #[serde(rename = "SA")]
    SergeantSpecialist5,
    /// Staff Sergeant (SB) - Air Force/Marine Corps enlisted rank, E-5/E-6
    #[serde(rename = "SB")]
    StaffSergeant,
    /// Staff Sergeant Specialist 6 (SC) - Army enlisted rank, E-6
    #[serde(rename = "SC")]
    StaffSergeantSpecialist6,
    /// Technical Sergeant (T1) - Air Force enlisted rank, E-6
    #[serde(rename = "T1")]
    TechnicalSergeant,
    /// Vice Admiral (V1) - Navy flag officer rank, O-9
    #[serde(rename = "V1")]
    ViceAdmiral,
    /// Warrant Officer (W1) - Warrant officer rank, W-1/W-2/W-3
    #[serde(rename = "W1")]
    WarrantOfficer,
}

impl std::fmt::Display for MilitaryServiceRankCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Admiral => write!(f, "A1"),
            Self::Airman => write!(f, "A2"),
            Self::AirmanFirstClass => write!(f, "A3"),
            Self::BasicAirman => write!(f, "B1"),
            Self::BrigadierGeneral => write!(f, "B2"),
            Self::Captain => write!(f, "C1"),
            Self::ChiefMasterSergeant => write!(f, "C2"),
            Self::ChiefPettyOfficer => write!(f, "C3"),
            Self::ChiefWarrant => write!(f, "C4"),
            Self::Colonel => write!(f, "C5"),
            Self::Commander => write!(f, "C6"),
            Self::Commodore => write!(f, "C7"),
            Self::Corporal => write!(f, "C8"),
            Self::CorporalSpecialist4 => write!(f, "C9"),
            Self::Ensign => write!(f, "E1"),
            Self::FirstLieutenant => write!(f, "F1"),
            Self::FirstSergeant => write!(f, "F2"),
            Self::FirstSergeantMasterSergeant => write!(f, "F3"),
            Self::FleetAdmiral => write!(f, "F4"),
            Self::General => write!(f, "G1"),
            Self::GunnerySergeant => write!(f, "G4"),
            Self::LanceCorporal => write!(f, "L1"),
            Self::Lieutenant => write!(f, "L2"),
            Self::LieutenantColonel => write!(f, "L3"),
            Self::LieutenantCommander => write!(f, "L4"),
            Self::LieutenantGeneral => write!(f, "L5"),
            Self::LieutenantJuniorGrade => write!(f, "L6"),
            Self::Major => write!(f, "M1"),
            Self::MajorGeneral => write!(f, "M2"),
            Self::MasterChiefPettyOfficer => write!(f, "M3"),
            Self::MasterGunnerySergeantMajor => write!(f, "M4"),
            Self::MasterSergeant => write!(f, "M5"),
            Self::MasterSergeantSpecialist8 => write!(f, "M6"),
            Self::PettyOfficerFirstClass => write!(f, "P1"),
            Self::PettyOfficerSecondClass => write!(f, "P2"),
            Self::PettyOfficerThirdClass => write!(f, "P3"),
            Self::Private => write!(f, "P4"),
            Self::PrivateFirstClass => write!(f, "P5"),
            Self::RearAdmiral => write!(f, "R1"),
            Self::Recruit => write!(f, "R2"),
            Self::Seaman => write!(f, "S1"),
            Self::SeamanApprentice => write!(f, "S2"),
            Self::SeamanRecruit => write!(f, "S3"),
            Self::SecondLieutenant => write!(f, "S4"),
            Self::SeniorChiefPettyOfficer => write!(f, "S5"),
            Self::SeniorMasterSergeant => write!(f, "S6"),
            Self::Sergeant => write!(f, "S7"),
            Self::SergeantFirstClassSpecialist7 => write!(f, "S8"),
            Self::SergeantMajorSpecialist9 => write!(f, "S9"),
            Self::SergeantSpecialist5 => write!(f, "SA"),
            Self::StaffSergeant => write!(f, "SB"),
            Self::StaffSergeantSpecialist6 => write!(f, "SC"),
            Self::TechnicalSergeant => write!(f, "T1"),
            Self::ViceAdmiral => write!(f, "V1"),
            Self::WarrantOfficer => write!(f, "W1"),
        }
    }
}
