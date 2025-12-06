use serde::{Deserialize, Serialize};

/// IndustryCode : The type of facility where the service was provided. You can set this to one of the [place of service codes](https://www.cms.gov/medicare/coding-billing/place-of-service-codes/code-sets).
/// The type of facility where the service was provided. You can set this to one of the [place of service codes](https://www.cms.gov/medicare/coding-billing/place-of-service-codes/code-sets).
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum IndustryCode {
    /// Pharmacy
    #[serde(rename = "01")]
    #[default]
    Pharmacy,
    /// Unassigned
    #[serde(rename = "02")]
    Unassigned,
    /// School
    #[serde(rename = "03")]
    School,
    /// Homeless Shelter
    #[serde(rename = "04")]
    HomelessShelter,
    /// Indian Health Service Free-standing Facility
    #[serde(rename = "05")]
    IndianHealthServiceFreeStandingFacility,
    /// Indian Health Service Provider-based Facility
    #[serde(rename = "06")]
    IndianHealthServiceProviderBasedFacility,
    /// Tribal 638 Free-standing Facility
    #[serde(rename = "07")]
    Tribal638FreeStandingFacility,
    /// Tribal 638 Provider-based Facility
    #[serde(rename = "08")]
    Tribal638ProviderBasedFacility,
    /// Prison/Correctional Facility
    #[serde(rename = "09")]
    PrisonCorrectionalFacility,
    /// Unassigned
    #[serde(rename = "10")]
    Unassigned10,
    /// Office
    #[serde(rename = "11")]
    Office,
    /// Home
    #[serde(rename = "12")]
    Home,
    /// Assisted Living Facility
    #[serde(rename = "13")]
    AssistedLivingFacility,
    /// Group Home
    #[serde(rename = "14")]
    GroupHome,
    /// Mobile Unit
    #[serde(rename = "15")]
    MobileUnit,
    /// Temporary Lodging
    #[serde(rename = "16")]
    TemporaryLodging,
    /// Walk-in Retail Health Clinic
    #[serde(rename = "17")]
    WalkInRetailHealthClinic,
    /// Place of Employment - Worksite
    #[serde(rename = "18")]
    PlaceOfEmploymentWorksite,
    /// Off Campus - Outpatient Hospital
    #[serde(rename = "19")]
    OffCampusOutpatientHospital,
    /// Urgent Care Facility
    #[serde(rename = "20")]
    UrgentCareFacility,
    /// Inpatient Hospital
    #[serde(rename = "21")]
    InpatientHospital,
    /// On Campus - Outpatient Hospital
    #[serde(rename = "22")]
    OnCampusOutpatientHospital,
    /// Emergency Room - Hospital
    #[serde(rename = "23")]
    EmergencyRoomHospital,
    /// Ambulatory Surgical Center
    #[serde(rename = "24")]
    AmbulatorySurgicalCenter,
    /// Birthing Center
    #[serde(rename = "25")]
    BirthingCenter,
    /// Military Treatment Facility
    #[serde(rename = "26")]
    MilitaryTreatmentFacility,
    /// Skilled Nursing Facility
    #[serde(rename = "31")]
    SkilledNursingFacility,
    /// Nursing Facility
    #[serde(rename = "32")]
    NursingFacility,
    /// Custodial Care Facility
    #[serde(rename = "33")]
    CustodialCareFacility,
    /// Hospice
    #[serde(rename = "34")]
    Hospice,
    /// Ambulance - Land
    #[serde(rename = "41")]
    AmbulanceLand,
    /// Ambulance - Air or Water
    #[serde(rename = "42")]
    AmbulanceAirOrWater,
    /// Independent Clinic
    #[serde(rename = "49")]
    IndependentClinic,
    /// Federally Qualified Health Center
    #[serde(rename = "50")]
    FederallyQualifiedHealthCenter,
    /// Inpatient Psychiatric Facility
    #[serde(rename = "51")]
    InpatientPsychiatricFacility,
    /// Psychiatric Facility - Partial Hospitalization
    #[serde(rename = "52")]
    PsychiatricFacilityPartialHospitalization,
    /// Community Mental Health Center
    #[serde(rename = "53")]
    CommunityMentalHealthCenter,
    /// Intermediate Care Facility/Individuals with Intellectual Disabilities
    #[serde(rename = "54")]
    IntermediateCareFacilityIntellectualDisabilities,
    /// Residential Substance Abuse Treatment Facility
    #[serde(rename = "55")]
    ResidentialSubstanceAbuseTreatmentFacility,
    /// Psychiatric Residential Treatment Facility
    #[serde(rename = "56")]
    PsychiatricResidentialTreatmentFacility,
    /// Non-residential Substance Abuse Treatment Facility
    #[serde(rename = "57")]
    NonResidentialSubstanceAbuseTreatmentFacility,
    /// Non-residential Opioid Treatment Facility
    #[serde(rename = "58")]
    NonResidentialOpioidTreatmentFacility,
    /// Mass Immunization Center
    #[serde(rename = "60")]
    MassImmunizationCenter,
    /// Comprehensive Inpatient Rehabilitation Facility
    #[serde(rename = "61")]
    ComprehensiveInpatientRehabilitationFacility,
    /// Comprehensive Outpatient Rehabilitation Facility
    #[serde(rename = "62")]
    ComprehensiveOutpatientRehabilitationFacility,
    /// End-Stage Renal Disease Treatment Facility
    #[serde(rename = "65")]
    EndStageRenalDiseaseTreatmentFacility,
    /// State or Local Public Health Clinic
    #[serde(rename = "71")]
    StateOrLocalPublicHealthClinic,
    /// Rural Health Clinic
    #[serde(rename = "72")]
    RuralHealthClinic,
    /// Independent Laboratory
    #[serde(rename = "81")]
    IndependentLaboratory,
    /// Other Place of Service
    #[serde(rename = "99")]
    OtherPlaceOfService,
}

impl std::fmt::Display for IndustryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pharmacy => write!(f, "01"),
            Self::Unassigned => write!(f, "02"),
            Self::School => write!(f, "03"),
            Self::HomelessShelter => write!(f, "04"),
            Self::IndianHealthServiceFreeStandingFacility => write!(f, "05"),
            Self::IndianHealthServiceProviderBasedFacility => write!(f, "06"),
            Self::Tribal638FreeStandingFacility => write!(f, "07"),
            Self::Tribal638ProviderBasedFacility => write!(f, "08"),
            Self::PrisonCorrectionalFacility => write!(f, "09"),
            Self::Unassigned10 => write!(f, "10"),
            Self::Office => write!(f, "11"),
            Self::Home => write!(f, "12"),
            Self::AssistedLivingFacility => write!(f, "13"),
            Self::GroupHome => write!(f, "14"),
            Self::MobileUnit => write!(f, "15"),
            Self::TemporaryLodging => write!(f, "16"),
            Self::WalkInRetailHealthClinic => write!(f, "17"),
            Self::PlaceOfEmploymentWorksite => write!(f, "18"),
            Self::OffCampusOutpatientHospital => write!(f, "19"),
            Self::UrgentCareFacility => write!(f, "20"),
            Self::InpatientHospital => write!(f, "21"),
            Self::OnCampusOutpatientHospital => write!(f, "22"),
            Self::EmergencyRoomHospital => write!(f, "23"),
            Self::AmbulatorySurgicalCenter => write!(f, "24"),
            Self::BirthingCenter => write!(f, "25"),
            Self::MilitaryTreatmentFacility => write!(f, "26"),
            Self::SkilledNursingFacility => write!(f, "31"),
            Self::NursingFacility => write!(f, "32"),
            Self::CustodialCareFacility => write!(f, "33"),
            Self::Hospice => write!(f, "34"),
            Self::AmbulanceLand => write!(f, "41"),
            Self::AmbulanceAirOrWater => write!(f, "42"),
            Self::IndependentClinic => write!(f, "49"),
            Self::FederallyQualifiedHealthCenter => write!(f, "50"),
            Self::InpatientPsychiatricFacility => write!(f, "51"),
            Self::PsychiatricFacilityPartialHospitalization => write!(f, "52"),
            Self::CommunityMentalHealthCenter => write!(f, "53"),
            Self::IntermediateCareFacilityIntellectualDisabilities => write!(f, "54"),
            Self::ResidentialSubstanceAbuseTreatmentFacility => write!(f, "55"),
            Self::PsychiatricResidentialTreatmentFacility => write!(f, "56"),
            Self::NonResidentialSubstanceAbuseTreatmentFacility => write!(f, "57"),
            Self::NonResidentialOpioidTreatmentFacility => write!(f, "58"),
            Self::MassImmunizationCenter => write!(f, "60"),
            Self::ComprehensiveInpatientRehabilitationFacility => write!(f, "61"),
            Self::ComprehensiveOutpatientRehabilitationFacility => write!(f, "62"),
            Self::EndStageRenalDiseaseTreatmentFacility => write!(f, "65"),
            Self::StateOrLocalPublicHealthClinic => write!(f, "71"),
            Self::RuralHealthClinic => write!(f, "72"),
            Self::IndependentLaboratory => write!(f, "81"),
            Self::OtherPlaceOfService => write!(f, "99"),
        }
    }
}
