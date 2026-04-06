use serde::{Deserialize, Serialize};

/// A code identifying a type of service. For example, `7` - Anesthesia. Visit [Service Type Codes](https://www.stedi.com/docs/healthcare/send-eligibility-checks#service-type-codes) for a complete list.    This list is specific to X12 version 005010, which is the mandated version for eligibility checks. It differs from the current [X12 Service Type Codes](https://x12.org/codes/service-type-codes) list, which applies to X12 versions later than 005010.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ResponseEligibilityServiceTypeCode {
    /// Medical Care
    #[serde(rename = "1")]
    #[default]
    MedicalCare,
    /// Surgical
    #[serde(rename = "2")]
    Surgical,
    /// Consultation
    #[serde(rename = "3")]
    Consultation,
    /// Diagnostic X-Ray
    #[serde(rename = "4")]
    DiagnosticXRay,
    /// Diagnostic Lab
    #[serde(rename = "5")]
    DiagnosticLab,
    /// Radiation Therapy
    #[serde(rename = "6")]
    RadiationTherapy,
    /// Anesthesia
    #[serde(rename = "7")]
    Anesthesia,
    /// Surgical Assistance
    #[serde(rename = "8")]
    SurgicalAssistance,
    /// Other Medical
    #[serde(rename = "9")]
    OtherMedical,
    /// Blood Charges
    #[serde(rename = "10")]
    BloodCharges,
    /// Used Durable Medical Equipment
    #[serde(rename = "11")]
    UsedDurableMedicalEquipment,
    /// Durable Medical Equipment Purchase
    #[serde(rename = "12")]
    DurableMedicalEquipmentPurchase,
    /// Ambulatory Service Center Facility
    #[serde(rename = "13")]
    AmbulatoryServiceCenterFacility,
    /// Renal Supplies in the Home
    #[serde(rename = "14")]
    RenalSuppliesInTheHome,
    /// Alternate Method Dialysis
    #[serde(rename = "15")]
    AlternateMethodDialysis,
    /// Chronic Renal Disease (CRD) Equipment
    #[serde(rename = "16")]
    ChronicRenalDiseaseEquipment,
    /// Pre-Admission Testing
    #[serde(rename = "17")]
    PreAdmissionTesting,
    /// Durable Medical Equipment Rental
    #[serde(rename = "18")]
    DurableMedicalEquipmentRental,
    /// Pneumonia Vaccine
    #[serde(rename = "19")]
    PneumoniaVaccine,
    /// Second Surgical Opinion
    #[serde(rename = "20")]
    SecondSurgicalOpinion,
    /// Third Surgical Opinion
    #[serde(rename = "21")]
    ThirdSurgicalOpinion,
    /// Social Work
    #[serde(rename = "22")]
    SocialWork,
    /// Diagnostic Dental
    #[serde(rename = "23")]
    DiagnosticDental,
    /// Periodontics
    #[serde(rename = "24")]
    Periodontics,
    /// Restorative
    #[serde(rename = "25")]
    Restorative,
    /// Endodontics
    #[serde(rename = "26")]
    Endodontics,
    /// Maxillofacial Prosthetics
    #[serde(rename = "27")]
    MaxillofacialProsthetics,
    /// Adjunctive Dental Services
    #[serde(rename = "28")]
    AdjunctiveDentalServices,
    /// Health Benefit Plan Coverage
    #[serde(rename = "30")]
    HealthBenefitPlanCoverage,
    /// Plan Waiting Period
    #[serde(rename = "32")]
    PlanWaitingPeriod,
    /// Chiropractic
    #[serde(rename = "33")]
    Chiropractic,
    /// Chiropractic Office Visits
    #[serde(rename = "34")]
    ChiropracticOfficeVisits,
    /// Dental Care
    #[serde(rename = "35")]
    DentalCare,
    /// Dental Crowns
    #[serde(rename = "36")]
    DentalCrowns,
    /// Dental Accident
    #[serde(rename = "37")]
    DentalAccident,
    /// Orthodontics
    #[serde(rename = "38")]
    Orthodontics,
    /// Prosthodontics
    #[serde(rename = "39")]
    Prosthodontics,
    /// Oral Surgery
    #[serde(rename = "40")]
    OralSurgery,
    /// Routine (Preventive) Dental
    #[serde(rename = "41")]
    RoutinePreventiveDental,
    /// Home Health Care
    #[serde(rename = "42")]
    HomeHealthCare,
    /// Home Health Prescriptions
    #[serde(rename = "43")]
    HomeHealthPrescriptions,
    /// Home Health Visits
    #[serde(rename = "44")]
    HomeHealthVisits,
    /// Hospice
    #[serde(rename = "45")]
    Hospice,
    /// Respite Care
    #[serde(rename = "46")]
    RespiteCare,
    /// Hospital
    #[serde(rename = "47")]
    Hospital,
    /// Hospital - Inpatient
    #[serde(rename = "48")]
    HospitalInpatient,
    /// Hospital - Room and Board
    #[serde(rename = "49")]
    HospitalRoomAndBoard,
    /// Hospital - Outpatient
    #[serde(rename = "50")]
    HospitalOutpatient,
    /// Hospital - Emergency Accident
    #[serde(rename = "51")]
    HospitalEmergencyAccident,
    /// Hospital - Emergency Medical
    #[serde(rename = "52")]
    HospitalEmergencyMedical,
    /// Hospital - Ambulatory Surgical
    #[serde(rename = "53")]
    HospitalAmbulatorySurgical,
    /// Long Term Care
    #[serde(rename = "54")]
    LongTermCare,
    /// Major Medical
    #[serde(rename = "55")]
    MajorMedical,
    /// Medically Related Transportation
    #[serde(rename = "56")]
    MedicallyRelatedTransportation,
    /// Air Transportation
    #[serde(rename = "57")]
    AirTransportation,
    /// Cabulance
    #[serde(rename = "58")]
    Cabulance,
    /// Licensed Ambulance
    #[serde(rename = "59")]
    LicensedAmbulance,
    /// General Benefits
    #[serde(rename = "60")]
    GeneralBenefits,
    /// In-vitro Fertilization
    #[serde(rename = "61")]
    InVitroFertilization,
    /// MRI/CAT Scan
    #[serde(rename = "62")]
    MriCatScan,
    /// Donor Procedures
    #[serde(rename = "63")]
    DonorProcedures,
    /// Acupuncture
    #[serde(rename = "64")]
    Acupuncture,
    /// Newborn Care
    #[serde(rename = "65")]
    NewbornCare,
    /// Pathology
    #[serde(rename = "66")]
    Pathology,
    /// Smoking Cessation
    #[serde(rename = "67")]
    SmokingCessation,
    /// Well Baby Care
    #[serde(rename = "68")]
    WellBabyCare,
    /// Maternity
    #[serde(rename = "69")]
    Maternity,
    /// Transplants
    #[serde(rename = "70")]
    Transplants,
    /// Audiology Exam
    #[serde(rename = "71")]
    AudiologyExam,
    /// Inhalation Therapy
    #[serde(rename = "72")]
    InhalationTherapy,
    /// Diagnostic Medical
    #[serde(rename = "73")]
    DiagnosticMedical,
    /// Private Duty Nursing
    #[serde(rename = "74")]
    PrivateDutyNursing,
    /// Prosthetic Device
    #[serde(rename = "75")]
    ProstheticDevice,
    /// Dialysis
    #[serde(rename = "76")]
    Dialysis,
    /// Otological Exam
    #[serde(rename = "77")]
    OtologicalExam,
    /// Chemotherapy
    #[serde(rename = "78")]
    Chemotherapy,
    /// Allergy Testing
    #[serde(rename = "79")]
    AllergyTesting,
    /// Immunizations
    #[serde(rename = "80")]
    Immunizations,
    /// Routine Physical
    #[serde(rename = "81")]
    RoutinePhysical,
    /// Family Planning
    #[serde(rename = "82")]
    FamilyPlanning,
    /// Infertility
    #[serde(rename = "83")]
    Infertility,
    /// Abortion
    #[serde(rename = "84")]
    Abortion,
    /// AIDS
    #[serde(rename = "85")]
    Aids,
    /// Emergency Services
    #[serde(rename = "86")]
    EmergencyServices,
    /// Cancer
    #[serde(rename = "87")]
    Cancer,
    /// Pharmacy
    #[serde(rename = "88")]
    Pharmacy,
    /// Free Standing Prescription Drug
    #[serde(rename = "89")]
    FreeStandingPrescriptionDrug,
    /// Mail Order Prescription Drug
    #[serde(rename = "90")]
    MailOrderPrescriptionDrug,
    /// Brand Name Prescription Drug
    #[serde(rename = "91")]
    BrandNamePrescriptionDrug,
    /// Generic Prescription Drug
    #[serde(rename = "92")]
    GenericPrescriptionDrug,
    /// Podiatry
    #[serde(rename = "93")]
    Podiatry,
    /// Podiatry - Office Visits
    #[serde(rename = "94")]
    PodiatryOfficeVisits,
    /// Podiatry - Nursing Home Visits
    #[serde(rename = "95")]
    PodiatryNursingHomeVisits,
    /// Professional (Physician)
    #[serde(rename = "96")]
    ProfessionalPhysician,
    /// Anesthesiologist
    #[serde(rename = "97")]
    Anesthesiologist,
    /// Professional (Physician) Visit - Office
    #[serde(rename = "98")]
    ProfessionalPhysicianVisitOffice,
    /// Professional (Physician) Visit - Inpatient
    #[serde(rename = "99")]
    ProfessionalPhysicianVisitInpatient,
    /// Professional (Physician) Visit - Outpatient
    #[serde(rename = "A0")]
    ProfessionalPhysicianVisitOutpatient,
    /// Professional (Physician) Visit - Nursing Home
    #[serde(rename = "A1")]
    ProfessionalPhysicianVisitNursingHome,
    /// Professional (Physician) Visit - Skilled Nursing Facility
    #[serde(rename = "A2")]
    ProfessionalPhysicianVisitSkilledNursingFacility,
    /// Professional (Physician) Visit - Home
    #[serde(rename = "A3")]
    ProfessionalPhysicianVisitHome,
    /// Psychiatric
    #[serde(rename = "A4")]
    Psychiatric,
    /// Psychiatric - Room and Board
    #[serde(rename = "A5")]
    PsychiatricRoomAndBoard,
    /// Psychotherapy
    #[serde(rename = "A6")]
    Psychotherapy,
    /// Psychiatric - Inpatient
    #[serde(rename = "A7")]
    PsychiatricInpatient,
    /// Psychiatric - Outpatient
    #[serde(rename = "A8")]
    PsychiatricOutpatient,
    /// Rehabilitation
    #[serde(rename = "A9")]
    Rehabilitation,
    /// Rehabilitation - Room and Board
    #[serde(rename = "AA")]
    RehabilitationRoomAndBoard,
    /// Rehabilitation - Inpatient
    #[serde(rename = "AB")]
    RehabilitationInpatient,
    /// Rehabilitation - Outpatient
    #[serde(rename = "AC")]
    RehabilitationOutpatient,
    /// Occupational Therapy
    #[serde(rename = "AD")]
    OccupationalTherapy,
    /// Physical Medicine
    #[serde(rename = "AE")]
    PhysicalMedicine,
    /// Speech Therapy
    #[serde(rename = "AF")]
    SpeechTherapy,
    /// Skilled Nursing Care
    #[serde(rename = "AG")]
    SkilledNursingCare,
    /// Skilled Nursing Care - Room and Board
    #[serde(rename = "AH")]
    SkilledNursingCareRoomAndBoard,
    /// Substance Abuse
    #[serde(rename = "AI")]
    SubstanceAbuse,
    /// Alcoholism
    #[serde(rename = "AJ")]
    Alcoholism,
    /// Drug Addiction
    #[serde(rename = "AK")]
    DrugAddiction,
    /// Vision (Optometry)
    #[serde(rename = "AL")]
    VisionOptometry,
    /// Frames
    #[serde(rename = "AM")]
    Frames,
    /// Routine Exam - Use for Routine Vision Exam only
    #[serde(rename = "AN")]
    RoutineVisionExam,
    /// Lenses
    #[serde(rename = "AO")]
    Lenses,
    /// Nonmedically Necessary Physical
    #[serde(rename = "AQ")]
    NonmedicallyNecessaryPhysical,
    /// Experimental Drug Therapy
    #[serde(rename = "AR")]
    ExperimentalDrugTherapy,
    /// Burn Care
    #[serde(rename = "B1")]
    BurnCare,
    /// Brand Name Prescription Drug - Formulary
    #[serde(rename = "B2")]
    BrandNamePrescriptionDrugFormulary,
    /// Brand Name Prescription Drug - Non-Formulary
    #[serde(rename = "B3")]
    BrandNamePrescriptionDrugNonFormulary,
    /// Independent Medical Evaluation
    #[serde(rename = "BA")]
    IndependentMedicalEvaluation,
    /// Partial Hospitalization (Psychiatric)
    #[serde(rename = "BB")]
    PartialHospitalizationPsychiatric,
    /// Day Care (Psychiatric)
    #[serde(rename = "BC")]
    DayCarePsychiatric,
    /// Cognitive Therapy
    #[serde(rename = "BD")]
    CognitiveTherapy,
    /// Massage Therapy
    #[serde(rename = "BE")]
    MassageTherapy,
    /// Pulmonary Rehabilitation
    #[serde(rename = "BF")]
    PulmonaryRehabilitation,
    /// Cardiac Rehabilitation
    #[serde(rename = "BG")]
    CardiacRehabilitation,
    /// Pediatric
    #[serde(rename = "BH")]
    Pediatric,
    /// Nursery
    #[serde(rename = "BI")]
    Nursery,
    /// Skin
    #[serde(rename = "BJ")]
    Skin,
    /// Orthopedic
    #[serde(rename = "BK")]
    Orthopedic,
    /// Cardiac
    #[serde(rename = "BL")]
    Cardiac,
    /// Lymphatic
    #[serde(rename = "BM")]
    Lymphatic,
    /// Gastrointestinal
    #[serde(rename = "BN")]
    Gastrointestinal,
    /// Endocrine
    #[serde(rename = "BP")]
    Endocrine,
    /// Neurology
    #[serde(rename = "BQ")]
    Neurology,
    /// Eye
    #[serde(rename = "BR")]
    Eye,
    /// Invasive Procedures
    #[serde(rename = "BS")]
    InvasiveProcedures,
    /// Gynecological
    #[serde(rename = "BT")]
    Gynecological,
    /// Obstetrical
    #[serde(rename = "BU")]
    Obstetrical,
    /// Obstetrical/Gynecological
    #[serde(rename = "BV")]
    ObstetricalGynecological,
    /// Mail Order Prescription Drug - Brand Name
    #[serde(rename = "BW")]
    MailOrderPrescriptionDrugBrandName,
    /// Mail Order Prescription Drug - Generic
    #[serde(rename = "BX")]
    MailOrderPrescriptionDrugGeneric,
    /// Physician Visit - Office: Sick
    #[serde(rename = "BY")]
    PhysicianVisitOfficeSick,
    /// Physician Visit - Office: Well
    #[serde(rename = "BZ")]
    PhysicianVisitOfficeWell,
    /// Coronary Care
    #[serde(rename = "C1")]
    CoronaryCare,
    /// Private Duty Nursing - Inpatient
    #[serde(rename = "CA")]
    PrivateDutyNursingInpatient,
    /// Private Duty Nursing - Home
    #[serde(rename = "CB")]
    PrivateDutyNursingHome,
    /// Surgical Benefits - Professional (Physician)
    #[serde(rename = "CC")]
    SurgicalBenefitsProfessionalPhysician,
    /// Surgical Benefits - Facility
    #[serde(rename = "CD")]
    SurgicalBenefitsFacility,
    /// Mental Health Provider - Inpatient
    #[serde(rename = "CE")]
    MentalHealthProviderInpatient,
    /// Mental Health Provider - Outpatient
    #[serde(rename = "CF")]
    MentalHealthProviderOutpatient,
    /// Mental Health Facility - Inpatient
    #[serde(rename = "CG")]
    MentalHealthFacilityInpatient,
    /// Mental Health Facility - Outpatient
    #[serde(rename = "CH")]
    MentalHealthFacilityOutpatient,
    /// Substance Abuse Facility - Inpatient
    #[serde(rename = "CI")]
    SubstanceAbuseFacilityInpatient,
    /// Substance Abuse Facility - Outpatient
    #[serde(rename = "CJ")]
    SubstanceAbuseFacilityOutpatient,
    /// Screening X-ray
    #[serde(rename = "CK")]
    ScreeningXRay,
    /// Screening Laboratory
    #[serde(rename = "CL")]
    ScreeningLaboratory,
    /// Mammogram, High Risk Patient
    #[serde(rename = "CM")]
    MammogramHighRiskPatient,
    /// Mammogram, Low Risk Patient
    #[serde(rename = "CN")]
    MammogramLowRiskPatient,
    /// Flu Vaccination
    #[serde(rename = "CO")]
    FluVaccination,
    /// Eyewear and Eyewear Accessories
    #[serde(rename = "CP")]
    EyewearAndEyewearAccessories,
    /// Case Management
    #[serde(rename = "CQ")]
    CaseManagement,
    /// Dermatology
    #[serde(rename = "DG")]
    Dermatology,
    /// Durable Medical Equipment
    #[serde(rename = "DM")]
    DurableMedicalEquipment,
    /// Diabetic Supplies
    #[serde(rename = "DS")]
    DiabeticSupplies,
    /// Generic Prescription Drug - Formulary
    #[serde(rename = "GF")]
    GenericPrescriptionDrugFormulary,
    /// Generic Prescription Drug - Non-Formulary
    #[serde(rename = "GN")]
    GenericPrescriptionDrugNonFormulary,
    /// Allergy
    #[serde(rename = "GY")]
    Allergy,
    /// Intensive Care
    #[serde(rename = "IC")]
    IntensiveCare,
    /// Mental Health
    #[serde(rename = "MH")]
    MentalHealth,
    /// Neonatal Intensive Care
    #[serde(rename = "NI")]
    NeonatalIntensiveCare,
    /// Oncology
    #[serde(rename = "ON")]
    Oncology,
    /// Physical Therapy
    #[serde(rename = "PT")]
    PhysicalTherapy,
    /// Pulmonary
    #[serde(rename = "PU")]
    Pulmonary,
    /// Renal
    #[serde(rename = "RN")]
    Renal,
    /// Residential Psychiatric Treatment
    #[serde(rename = "RT")]
    ResidentialPsychiatricTreatment,
    /// Transitional Care
    #[serde(rename = "TC")]
    TransitionalCare,
    /// Transitional Nursery Care
    #[serde(rename = "TN")]
    TransitionalNurseryCare,
    /// Urgent Care
    #[serde(rename = "UC")]
    UrgentCare,
    /// Fluoride Treatments
    #[serde(rename = "EI")]
    FluorideTreatments,
    /// Dental Prophylaxis
    #[serde(rename = "EF")]
    DentalProphylaxis,
    /// Space Maintenance
    #[serde(rename = "V13")]
    SpaceMaintenance,
    /// Harmful Habits Appliance
    #[serde(rename = "V29")]
    HarmfulHabitsAppliance,
    /// Composites
    #[serde(rename = "V38")]
    Composites,
    /// Bitewing X-Rays
    #[serde(rename = "V9")]
    BitewingXRays,
    /// Root Canal / Retreatment
    #[serde(rename = "V40")]
    RootCanalRetreatment,
    /// Oral Evaluation
    #[serde(rename = "EE")]
    OralEvaluation,
    /// Full Mouth / Panoramic X-Rays
    #[serde(rename = "V37")]
    FullMouthPanoramicXRays,
    /// Periapical X-Ray
    #[serde(rename = "V11")]
    PeriapicalXRay,
    /// Surgical Periodontics
    #[serde(rename = "V18")]
    SurgicalPeriodontics,
    /// Denture Adjust, Rebase, Reline, Repair
    #[serde(rename = "V23")]
    DentureAdjustRebaseRelineRepair,
    /// Inlay / Onlay
    #[serde(rename = "V31")]
    InlayOnlay,
    /// Buildups / Post and Core
    #[serde(rename = "V30")]
    BuildupsPostAndCore,
    /// Amalgam / Composite Restorations
    #[serde(rename = "V32")]
    AmalgamCompositeRestorations,
    /// Dentures
    #[serde(rename = "V33")]
    Dentures,
    /// Dentures - Repair
    #[serde(rename = "V34")]
    DenturesRepair,
    /// Dentures - Reline
    #[serde(rename = "V35")]
    DenturesReline,
    /// Stainless Steel, Resin, Acrylic Crowns
    #[serde(rename = "V39")]
    StainlessSteelResinAcrylicCrowns,
    /// Sealants
    #[serde(rename = "EH")]
    Sealants,
    /// Dental Implants
    #[serde(rename = "EJ")]
    DentalImplants,
    /// Temporomandibular Joint Dysfunction
    #[serde(rename = "EK")]
    TemporomandibularJointDysfunction,
    /// Specialty Pharmacy
    #[serde(rename = "EB")]
    SpecialtyPharmacy,
    /// Durable Medical Equipment (New)
    #[serde(rename = "EC")]
    DurableMedicalEquipmentNew,
    /// Diagnostic Imaging
    #[serde(rename = "E11")]
    DiagnosticImaging,
    /// Fertility Preservation
    #[serde(rename = "EG")]
    FertilityPreservation,
    /// Applied Behavioral Analysis Therapy
    #[serde(rename = "E0")]
    AppliedBehavioralAnalysisTherapy,
    /// Non-Medical Equipment
    #[serde(rename = "E1")]
    NonMedicalEquipment,
    /// Psychiatric Emergency
    #[serde(rename = "E2")]
    PsychiatricEmergency,
    /// Step Down Unit
    #[serde(rename = "E3")]
    StepDownUnit,
    /// Skilled Nursing Facility Head Level
    #[serde(rename = "E4")]
    SkilledNursingFacilityHeadLevel,
    /// Skilled Nursing Facility Ventilator Level
    #[serde(rename = "E5")]
    SkilledNursingFacilityVentilatorLevel,
    /// Level of Care 1
    #[serde(rename = "E6")]
    LevelOfCare1,
    /// Level of Care 2
    #[serde(rename = "E7")]
    LevelOfCare2,
    /// Level of Care 3
    #[serde(rename = "E8")]
    LevelOfCare3,
    /// Level of Care 4
    #[serde(rename = "E9")]
    LevelOfCare4,
    /// Radiographs
    #[serde(rename = "E10")]
    Radiographs,
    /// Fixed Prosthodontics
    #[serde(rename = "E14")]
    FixedProsthodontics,
    /// Removable Prosthodontics
    #[serde(rename = "E15")]
    RemovableProsthodontics,
    /// Intraoral Images Complete Series
    #[serde(rename = "E16")]
    IntraoralImagesCompleteSeries,
    /// Oral Evaluation (E17)
    #[serde(rename = "E17")]
    OralEvaluationE17,
    /// Dental Prophylaxis (E18)
    #[serde(rename = "E18")]
    DentalProphylaxisE18,
    /// Panoramic Images
    #[serde(rename = "E19")]
    PanoramicImages,
    /// Sealants (E20)
    #[serde(rename = "E20")]
    SealantsE20,
    /// Fluoride Treatments (E21)
    #[serde(rename = "E21")]
    FluorideTreatmentsE21,
    /// Dental Implants (E22)
    #[serde(rename = "E22")]
    DentalImplantsE22,
    /// Temporomandibular Joint Dysfunction (E23)
    #[serde(rename = "E23")]
    TemporomandibularJointDysfunctionE23,
    /// Long Term Care Pharmacy
    #[serde(rename = "E25")]
    LongTermCarePharmacy,
    /// Comprehensive Medication Therapy Management
    #[serde(rename = "E26")]
    ComprehensiveMedicationTherapyManagement,
    /// Targeted Medication Therapy Management
    #[serde(rename = "E27")]
    TargetedMedicationTherapyManagement,
    /// Dietary/Nutritional Services
    #[serde(rename = "E28")]
    DietaryNutritionalServices,
    /// Intensive Cardiac Rehabilitation
    #[serde(rename = "E33")]
    IntensiveCardiacRehabilitation,
    /// Convenience Care
    #[serde(rename = "E36")]
    ConvenienceCare,
    /// Telemedicine
    #[serde(rename = "E37")]
    Telemedicine,
    /// Pharmacist Services
    #[serde(rename = "E38")]
    PharmacistServices,
    /// Diabetic Education
    #[serde(rename = "E39")]
    DiabeticEducation,
    /// Early Intervention
    #[serde(rename = "E40")]
    EarlyIntervention,
    /// Medically Tailored Meals
    #[serde(rename = "EM")]
    MedicallyTailoredMeals,
    /// Serious Mental Health
    #[serde(rename = "SMH")]
    SeriousMentalHealth,
    /// Remote Patient Monitoring
    #[serde(rename = "UP")]
    RemotePatientMonitoring,
    /// Remote Therapeutic Monitoring
    #[serde(rename = "UR")]
    RemoteTherapeuticMonitoring,
    /// Special Supplemental Benefits
    #[serde(rename = "US")]
    SpecialSupplementalBenefits,
    /// Positron Emission Tomography (PET) Scan
    #[serde(rename = "PE")]
    PositronEmissionTomographyPetScan,
    /// IV Therapy
    #[serde(rename = "ED")]
    CatScan,
    /// Unknown / unrecognized service type code from payer response
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ResponseEligibilityServiceTypeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MedicalCare => write!(f, "1"),
            Self::Surgical => write!(f, "2"),
            Self::Consultation => write!(f, "3"),
            Self::DiagnosticXRay => write!(f, "4"),
            Self::DiagnosticLab => write!(f, "5"),
            Self::RadiationTherapy => write!(f, "6"),
            Self::Anesthesia => write!(f, "7"),
            Self::SurgicalAssistance => write!(f, "8"),
            Self::OtherMedical => write!(f, "9"),
            Self::BloodCharges => write!(f, "10"),
            Self::UsedDurableMedicalEquipment => write!(f, "11"),
            Self::DurableMedicalEquipmentPurchase => write!(f, "12"),
            Self::AmbulatoryServiceCenterFacility => write!(f, "13"),
            Self::RenalSuppliesInTheHome => write!(f, "14"),
            Self::AlternateMethodDialysis => write!(f, "15"),
            Self::ChronicRenalDiseaseEquipment => write!(f, "16"),
            Self::PreAdmissionTesting => write!(f, "17"),
            Self::DurableMedicalEquipmentRental => write!(f, "18"),
            Self::PneumoniaVaccine => write!(f, "19"),
            Self::SecondSurgicalOpinion => write!(f, "20"),
            Self::ThirdSurgicalOpinion => write!(f, "21"),
            Self::SocialWork => write!(f, "22"),
            Self::DiagnosticDental => write!(f, "23"),
            Self::Periodontics => write!(f, "24"),
            Self::Restorative => write!(f, "25"),
            Self::Endodontics => write!(f, "26"),
            Self::MaxillofacialProsthetics => write!(f, "27"),
            Self::AdjunctiveDentalServices => write!(f, "28"),
            Self::HealthBenefitPlanCoverage => write!(f, "30"),
            Self::PlanWaitingPeriod => write!(f, "32"),
            Self::Chiropractic => write!(f, "33"),
            Self::ChiropracticOfficeVisits => write!(f, "34"),
            Self::DentalCare => write!(f, "35"),
            Self::DentalCrowns => write!(f, "36"),
            Self::DentalAccident => write!(f, "37"),
            Self::Orthodontics => write!(f, "38"),
            Self::Prosthodontics => write!(f, "39"),
            Self::OralSurgery => write!(f, "40"),
            Self::RoutinePreventiveDental => write!(f, "41"),
            Self::HomeHealthCare => write!(f, "42"),
            Self::HomeHealthPrescriptions => write!(f, "43"),
            Self::HomeHealthVisits => write!(f, "44"),
            Self::Hospice => write!(f, "45"),
            Self::RespiteCare => write!(f, "46"),
            Self::Hospital => write!(f, "47"),
            Self::HospitalInpatient => write!(f, "48"),
            Self::HospitalRoomAndBoard => write!(f, "49"),
            Self::HospitalOutpatient => write!(f, "50"),
            Self::HospitalEmergencyAccident => write!(f, "51"),
            Self::HospitalEmergencyMedical => write!(f, "52"),
            Self::HospitalAmbulatorySurgical => write!(f, "53"),
            Self::LongTermCare => write!(f, "54"),
            Self::MajorMedical => write!(f, "55"),
            Self::MedicallyRelatedTransportation => write!(f, "56"),
            Self::AirTransportation => write!(f, "57"),
            Self::Cabulance => write!(f, "58"),
            Self::LicensedAmbulance => write!(f, "59"),
            Self::GeneralBenefits => write!(f, "60"),
            Self::InVitroFertilization => write!(f, "61"),
            Self::MriCatScan => write!(f, "62"),
            Self::DonorProcedures => write!(f, "63"),
            Self::Acupuncture => write!(f, "64"),
            Self::NewbornCare => write!(f, "65"),
            Self::Pathology => write!(f, "66"),
            Self::SmokingCessation => write!(f, "67"),
            Self::WellBabyCare => write!(f, "68"),
            Self::Maternity => write!(f, "69"),
            Self::Transplants => write!(f, "70"),
            Self::AudiologyExam => write!(f, "71"),
            Self::InhalationTherapy => write!(f, "72"),
            Self::DiagnosticMedical => write!(f, "73"),
            Self::PrivateDutyNursing => write!(f, "74"),
            Self::ProstheticDevice => write!(f, "75"),
            Self::Dialysis => write!(f, "76"),
            Self::OtologicalExam => write!(f, "77"),
            Self::Chemotherapy => write!(f, "78"),
            Self::AllergyTesting => write!(f, "79"),
            Self::Immunizations => write!(f, "80"),
            Self::RoutinePhysical => write!(f, "81"),
            Self::FamilyPlanning => write!(f, "82"),
            Self::Infertility => write!(f, "83"),
            Self::Abortion => write!(f, "84"),
            Self::Aids => write!(f, "85"),
            Self::EmergencyServices => write!(f, "86"),
            Self::Cancer => write!(f, "87"),
            Self::Pharmacy => write!(f, "88"),
            Self::FreeStandingPrescriptionDrug => write!(f, "89"),
            Self::MailOrderPrescriptionDrug => write!(f, "90"),
            Self::BrandNamePrescriptionDrug => write!(f, "91"),
            Self::GenericPrescriptionDrug => write!(f, "92"),
            Self::Podiatry => write!(f, "93"),
            Self::PodiatryOfficeVisits => write!(f, "94"),
            Self::PodiatryNursingHomeVisits => write!(f, "95"),
            Self::ProfessionalPhysician => write!(f, "96"),
            Self::Anesthesiologist => write!(f, "97"),
            Self::ProfessionalPhysicianVisitOffice => write!(f, "98"),
            Self::ProfessionalPhysicianVisitInpatient => write!(f, "99"),
            Self::ProfessionalPhysicianVisitOutpatient => write!(f, "A0"),
            Self::ProfessionalPhysicianVisitNursingHome => write!(f, "A1"),
            Self::ProfessionalPhysicianVisitSkilledNursingFacility => write!(f, "A2"),
            Self::ProfessionalPhysicianVisitHome => write!(f, "A3"),
            Self::Psychiatric => write!(f, "A4"),
            Self::PsychiatricRoomAndBoard => write!(f, "A5"),
            Self::Psychotherapy => write!(f, "A6"),
            Self::PsychiatricInpatient => write!(f, "A7"),
            Self::PsychiatricOutpatient => write!(f, "A8"),
            Self::Rehabilitation => write!(f, "A9"),
            Self::RehabilitationRoomAndBoard => write!(f, "AA"),
            Self::RehabilitationInpatient => write!(f, "AB"),
            Self::RehabilitationOutpatient => write!(f, "AC"),
            Self::OccupationalTherapy => write!(f, "AD"),
            Self::PhysicalMedicine => write!(f, "AE"),
            Self::SpeechTherapy => write!(f, "AF"),
            Self::SkilledNursingCare => write!(f, "AG"),
            Self::SkilledNursingCareRoomAndBoard => write!(f, "AH"),
            Self::SubstanceAbuse => write!(f, "AI"),
            Self::Alcoholism => write!(f, "AJ"),
            Self::DrugAddiction => write!(f, "AK"),
            Self::VisionOptometry => write!(f, "AL"),
            Self::Frames => write!(f, "AM"),
            Self::RoutineVisionExam => write!(f, "AN"),
            Self::Lenses => write!(f, "AO"),
            Self::NonmedicallyNecessaryPhysical => write!(f, "AQ"),
            Self::ExperimentalDrugTherapy => write!(f, "AR"),
            Self::BurnCare => write!(f, "B1"),
            Self::BrandNamePrescriptionDrugFormulary => write!(f, "B2"),
            Self::BrandNamePrescriptionDrugNonFormulary => write!(f, "B3"),
            Self::IndependentMedicalEvaluation => write!(f, "BA"),
            Self::PartialHospitalizationPsychiatric => write!(f, "BB"),
            Self::DayCarePsychiatric => write!(f, "BC"),
            Self::CognitiveTherapy => write!(f, "BD"),
            Self::MassageTherapy => write!(f, "BE"),
            Self::PulmonaryRehabilitation => write!(f, "BF"),
            Self::CardiacRehabilitation => write!(f, "BG"),
            Self::Pediatric => write!(f, "BH"),
            Self::Nursery => write!(f, "BI"),
            Self::Skin => write!(f, "BJ"),
            Self::Orthopedic => write!(f, "BK"),
            Self::Cardiac => write!(f, "BL"),
            Self::Lymphatic => write!(f, "BM"),
            Self::Gastrointestinal => write!(f, "BN"),
            Self::Endocrine => write!(f, "BP"),
            Self::Neurology => write!(f, "BQ"),
            Self::Eye => write!(f, "BR"),
            Self::InvasiveProcedures => write!(f, "BS"),
            Self::Gynecological => write!(f, "BT"),
            Self::Obstetrical => write!(f, "BU"),
            Self::ObstetricalGynecological => write!(f, "BV"),
            Self::MailOrderPrescriptionDrugBrandName => write!(f, "BW"),
            Self::MailOrderPrescriptionDrugGeneric => write!(f, "BX"),
            Self::PhysicianVisitOfficeSick => write!(f, "BY"),
            Self::PhysicianVisitOfficeWell => write!(f, "BZ"),
            Self::CoronaryCare => write!(f, "C1"),
            Self::PrivateDutyNursingInpatient => write!(f, "CA"),
            Self::PrivateDutyNursingHome => write!(f, "CB"),
            Self::SurgicalBenefitsProfessionalPhysician => write!(f, "CC"),
            Self::SurgicalBenefitsFacility => write!(f, "CD"),
            Self::MentalHealthProviderInpatient => write!(f, "CE"),
            Self::MentalHealthProviderOutpatient => write!(f, "CF"),
            Self::MentalHealthFacilityInpatient => write!(f, "CG"),
            Self::MentalHealthFacilityOutpatient => write!(f, "CH"),
            Self::SubstanceAbuseFacilityInpatient => write!(f, "CI"),
            Self::SubstanceAbuseFacilityOutpatient => write!(f, "CJ"),
            Self::ScreeningXRay => write!(f, "CK"),
            Self::ScreeningLaboratory => write!(f, "CL"),
            Self::MammogramHighRiskPatient => write!(f, "CM"),
            Self::MammogramLowRiskPatient => write!(f, "CN"),
            Self::FluVaccination => write!(f, "CO"),
            Self::EyewearAndEyewearAccessories => write!(f, "CP"),
            Self::CaseManagement => write!(f, "CQ"),
            Self::Dermatology => write!(f, "DG"),
            Self::DurableMedicalEquipment => write!(f, "DM"),
            Self::DiabeticSupplies => write!(f, "DS"),
            Self::GenericPrescriptionDrugFormulary => write!(f, "GF"),
            Self::GenericPrescriptionDrugNonFormulary => write!(f, "GN"),
            Self::Allergy => write!(f, "GY"),
            Self::IntensiveCare => write!(f, "IC"),
            Self::MentalHealth => write!(f, "MH"),
            Self::NeonatalIntensiveCare => write!(f, "NI"),
            Self::Oncology => write!(f, "ON"),
            Self::PhysicalTherapy => write!(f, "PT"),
            Self::Pulmonary => write!(f, "PU"),
            Self::Renal => write!(f, "RN"),
            Self::ResidentialPsychiatricTreatment => write!(f, "RT"),
            Self::TransitionalCare => write!(f, "TC"),
            Self::TransitionalNurseryCare => write!(f, "TN"),
            Self::UrgentCare => write!(f, "UC"),
            Self::FluorideTreatments => write!(f, "EI"),
            Self::DentalProphylaxis => write!(f, "EF"),
            Self::SpaceMaintenance => write!(f, "V13"),
            Self::HarmfulHabitsAppliance => write!(f, "V29"),
            Self::Composites => write!(f, "V38"),
            Self::BitewingXRays => write!(f, "V9"),
            Self::RootCanalRetreatment => write!(f, "V40"),
            Self::OralEvaluation => write!(f, "EE"),
            Self::FullMouthPanoramicXRays => write!(f, "V37"),
            Self::PeriapicalXRay => write!(f, "V11"),
            Self::SurgicalPeriodontics => write!(f, "V18"),
            Self::DentureAdjustRebaseRelineRepair => write!(f, "V23"),
            Self::InlayOnlay => write!(f, "V31"),
            Self::BuildupsPostAndCore => write!(f, "V30"),
            Self::AmalgamCompositeRestorations => write!(f, "V32"),
            Self::Dentures => write!(f, "V33"),
            Self::DenturesRepair => write!(f, "V34"),
            Self::DenturesReline => write!(f, "V35"),
            Self::StainlessSteelResinAcrylicCrowns => write!(f, "V39"),
            Self::Sealants => write!(f, "EH"),
            Self::DentalImplants => write!(f, "EJ"),
            Self::TemporomandibularJointDysfunction => write!(f, "EK"),
            Self::SpecialtyPharmacy => write!(f, "EB"),
            Self::DurableMedicalEquipmentNew => write!(f, "EC"),
            Self::DiagnosticImaging => write!(f, "E11"),
            Self::FertilityPreservation => write!(f, "EG"),
            Self::AppliedBehavioralAnalysisTherapy => write!(f, "E0"),
            Self::NonMedicalEquipment => write!(f, "E1"),
            Self::PsychiatricEmergency => write!(f, "E2"),
            Self::StepDownUnit => write!(f, "E3"),
            Self::SkilledNursingFacilityHeadLevel => write!(f, "E4"),
            Self::SkilledNursingFacilityVentilatorLevel => write!(f, "E5"),
            Self::LevelOfCare1 => write!(f, "E6"),
            Self::LevelOfCare2 => write!(f, "E7"),
            Self::LevelOfCare3 => write!(f, "E8"),
            Self::LevelOfCare4 => write!(f, "E9"),
            Self::Radiographs => write!(f, "E10"),
            Self::FixedProsthodontics => write!(f, "E14"),
            Self::RemovableProsthodontics => write!(f, "E15"),
            Self::IntraoralImagesCompleteSeries => write!(f, "E16"),
            Self::OralEvaluationE17 => write!(f, "E17"),
            Self::DentalProphylaxisE18 => write!(f, "E18"),
            Self::PanoramicImages => write!(f, "E19"),
            Self::SealantsE20 => write!(f, "E20"),
            Self::FluorideTreatmentsE21 => write!(f, "E21"),
            Self::DentalImplantsE22 => write!(f, "E22"),
            Self::TemporomandibularJointDysfunctionE23 => write!(f, "E23"),
            Self::LongTermCarePharmacy => write!(f, "E25"),
            Self::ComprehensiveMedicationTherapyManagement => write!(f, "E26"),
            Self::TargetedMedicationTherapyManagement => write!(f, "E27"),
            Self::DietaryNutritionalServices => write!(f, "E28"),
            Self::IntensiveCardiacRehabilitation => write!(f, "E33"),
            Self::ConvenienceCare => write!(f, "E36"),
            Self::Telemedicine => write!(f, "E37"),
            Self::PharmacistServices => write!(f, "E38"),
            Self::DiabeticEducation => write!(f, "E39"),
            Self::EarlyIntervention => write!(f, "E40"),
            Self::MedicallyTailoredMeals => write!(f, "EM"),
            Self::SeriousMentalHealth => write!(f, "SMH"),
            Self::RemotePatientMonitoring => write!(f, "UP"),
            Self::RemoteTherapeuticMonitoring => write!(f, "UR"),
            Self::SpecialSupplementalBenefits => write!(f, "US"),
            Self::PositronEmissionTomographyPetScan => write!(f, "PE"),
            Self::CatScan => write!(f, "ED"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
