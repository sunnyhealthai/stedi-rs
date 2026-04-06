use serde::{Deserialize, Serialize};

/// Human-readable name for healthcare service type codes returned in eligibility responses.
///
/// This enum provides descriptive names for service type codes that identify specific types
/// of healthcare services or benefits. Service types are used in eligibility responses to
/// indicate which services are covered, what benefits are available, and how services are
/// categorized for benefit determination.
///
/// The corresponding code version (`ResponseEligibilityServiceTypeCode`) contains standardized
/// codes for programmatic use, while this enum provides human-readable names for display and
/// understanding.
///
/// **Important Note**: The word "physician" in service type codes refers to any healthcare
/// provider, including physician assistants, nurse practitioners, and other types of healthcare
/// professionals, not just medical doctors.
///
/// ## Service Type Categories
///
/// Service types can be grouped into several major categories:
///
/// ### General Medical Services
/// - **Medical Care**: General medical care services
/// - **Surgical**: Surgical procedures and services
/// - **Consultation**: Medical consultations
/// - **Emergency Services**: Emergency medical services
/// - **Urgent Care**: Urgent care services
///
/// ### Diagnostic Services
/// - **Diagnostic X-Ray**: X-ray diagnostic services
/// - **Diagnostic Lab**: Laboratory diagnostic services
/// - **Diagnostic Medical**: General diagnostic medical services
/// - **Diagnostic Dental**: Dental diagnostic services
/// - **MRI/CAT Scan**: Advanced imaging services
/// - **Screening X-ray**: Screening X-ray services
/// - **Screening laboratory**: Screening laboratory services
///
/// ### Hospital Services
/// - **Hospital**: General hospital services
/// - **Hospital - Inpatient**: Inpatient hospital services
/// - **Hospital - Outpatient**: Outpatient hospital services
/// - **Hospital - Emergency**: Emergency hospital services
/// - **Hospital - Room and Board**: Hospital room and board charges
/// - **Hospital - Ambulatory Surgical**: Ambulatory surgical services
///
/// ### Professional/Physician Services
/// - **Professional (Physician)**: General physician services
/// - **Professional (Physician) Visit - Office**: Office visits
/// - **Professional (Physician) Visit - Inpatient**: Inpatient visits
/// - **Professional (Physician) Visit - Outpatient**: Outpatient visits
/// - **Professional (Physician) Visit - Home**: Home visits
/// - **Professional (Physician) Visit - Nursing Home**: Nursing home visits
///
/// ### Dental Services
/// - **Dental Care**: General dental care
/// - **Diagnostic Dental**: Dental diagnostic services
/// - **Routine (Preventive) Dental**: Preventive dental care
/// - **Restorative**: Restorative dental procedures
/// - **Orthodontics**: Orthodontic services
/// - **Oral Surgery**: Oral surgery procedures
/// - **Periodontics**: Periodontal services
/// - **Endodontics**: Endodontic services (root canals)
/// - **Prosthodontics**: Prosthodontic services
///
/// ### Pharmacy Services
/// - **Pharmacy**: General pharmacy services
/// - **Brand Name Prescription Drug**: Brand name medications
/// - **Generic Prescription Drug**: Generic medications
/// - **Mail Order Prescription Drug**: Mail order pharmacy
/// - **Free Standing Prescription Drug**: Free-standing pharmacy
/// - **Formulary/Non-Formulary**: Formulary status variants
///
/// ### Mental Health Services
/// - **Psychiatric**: General psychiatric services
/// - **Psychiatric - Inpatient**: Inpatient psychiatric care
/// - **Psychiatric - Outpatient**: Outpatient psychiatric care
/// - **Psychotherapy**: Psychotherapy services
/// - **Mental Health**: General mental health services
/// - **Substance Abuse**: Substance abuse treatment
/// - **Alcoholism**: Alcoholism treatment
/// - **Drug Addiction**: Drug addiction treatment
///
/// ### Rehabilitation Services
/// - **Rehabilitation**: General rehabilitation services
/// - **Rehabilitation - Inpatient**: Inpatient rehabilitation
/// - **Rehabilitation - Outpatient**: Outpatient rehabilitation
/// - **Physical Therapy**: Physical therapy services
/// - **Occupational Therapy**: Occupational therapy services
/// - **Speech Therapy**: Speech therapy services
/// - **Cardiac Rehabilitation**: Cardiac rehabilitation
/// - **Pulmonary Rehabilitation**: Pulmonary rehabilitation
///
/// ### Specialized Care Services
/// - **Home Health Care**: Home health care services
/// - **Hospice**: Hospice care services
/// - **Skilled Nursing Care**: Skilled nursing facility care
/// - **Long Term Care**: Long-term care services
/// - **Respite Care**: Respite care services
/// - **Private Duty Nursing**: Private duty nursing services
///
/// ### Durable Medical Equipment
/// - **Durable Medical Equipment**: General DME
/// - **Durable Medical Equipment Purchase**: DME purchases
/// - **Durable Medical Equipment Rental**: DME rentals
/// - **Used Durable Medical Equipment**: Used DME
/// - **Prosthetic Device**: Prosthetic devices
/// - **Diabetic Supplies**: Diabetic supplies and equipment
///
/// ### Specialty Services
/// - **Chiropractic**: Chiropractic services
/// - **Podiatry**: Podiatry services
/// - **Vision (Optometry)**: Vision and optometry services
/// - **Audiology Exam**: Audiology services
/// - **Acupuncture**: Acupuncture services
/// - **Massage Therapy**: Massage therapy services
///
/// ### Preventive Services
/// - **Routine Physical**: Routine physical examinations
/// - **Well Baby Care**: Well-baby care services
/// - **Immunizations**: Immunization services
/// - **Flu Vaccination**: Flu vaccination services
/// - **Mammogram**: Mammography screening services
/// - **Smoking Cessation**: Smoking cessation programs
///
/// ### Maternity and Reproductive Services
/// - **Maternity**: Maternity care services
/// - **Newborn Care**: Newborn care services
/// - **Family Planning**: Family planning services
/// - **Infertility**: Infertility treatment services
/// - **In-vitro Fertilization**: IVF services
///
/// ### Transportation Services
/// - **Medically Related Transportation**: Medical transportation
/// - **Air Transportation**: Air ambulance services
/// - **Licensed Ambulance**: Licensed ambulance services
/// - **Cabulance**: Taxi/ambulance hybrid services
///
/// ### Other Specialized Services
/// - **Dialysis**: Dialysis services
/// - **Chemotherapy**: Chemotherapy services
/// - **Radiation Therapy**: Radiation therapy services
/// - **Transplants**: Organ transplant services
/// - **Cancer**: Cancer treatment services
/// - **Oncology**: Oncology services
/// - **Burn Care**: Burn care services
///
/// ## Usage Context
///
/// Service types are used in eligibility responses to:
///
/// - **Identify covered services**: Specify which healthcare services are covered under the plan
/// - **Benefit determination**: Understand what benefits are available for specific service types
/// - **Authorization requirements**: Determine if services require prior authorization
/// - **Network restrictions**: Identify in-network vs out-of-network service types
/// - **Cost sharing**: Understand copays, coinsurance, and deductibles by service type
/// - **Limitations**: Identify service-specific limitations or restrictions
///
/// Service types are typically returned in the `serviceTypeCodes` array in benefit information
/// to indicate which services are covered or have specific benefit structures.
///
/// ## Service Type Codes
///
/// Visit [Service Type Codes](https://www.stedi.com/docs/healthcare/send-eligibility-checks#service-type-codes)
/// for a complete list of codes and their names. The service type codes follow standardized
/// healthcare service classifications used across the industry.
///
/// ## Examples
///
/// Common service types in eligibility responses include:
/// - `MedicalCare` - General medical care coverage
/// - `Hospital` - Hospital services coverage
/// - `Pharmacy` - Prescription drug coverage
/// - `DentalCare` - Dental services coverage
/// - `VisionLeftParenthesisOptometryRightParenthesis` - Vision care coverage
/// - `MentalHealth` - Mental health services coverage
///
/// ## X12 HIPAA
///
/// Maps to service type code elements in X12 271 transactions, including service type
/// indicators in benefit information segments (EB loops) that specify which services are
/// covered under the plan.
///
/// Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum ResponseEligibilityServiceType {
    /// Medical Care - General medical care services.
    #[serde(rename = "Medical Care")]
    #[default]
    MedicalCare,
    /// Surgical - Surgical procedures and services.
    #[serde(rename = "Surgical")]
    Surgical,
    /// Consultation - Medical consultation services.
    #[serde(rename = "Consultation")]
    Consultation,
    /// Diagnostic X-Ray - X-ray diagnostic imaging services.
    #[serde(rename = "Diagnostic X-Ray")]
    DiagnosticXRay,
    /// Diagnostic Lab - Laboratory diagnostic testing services.
    #[serde(rename = "Diagnostic Lab")]
    DiagnosticLab,
    /// Radiation Therapy - Radiation therapy treatment services.
    #[serde(rename = "Radiation Therapy")]
    RadiationTherapy,
    /// Anesthesia - Anesthesia services for surgical procedures.
    #[serde(rename = "Anesthesia")]
    Anesthesia,
    /// Surgical Assistance - Surgical assistance services.
    #[serde(rename = "Surgical Assistance")]
    SurgicalAssistance,
    /// Other Medical - Other medical services not specifically categorized.
    #[serde(rename = "Other Medical")]
    OtherMedical,
    /// Blood Charges - Blood and blood product charges.
    #[serde(rename = "Blood Charges")]
    BloodCharges,
    /// Used Durable Medical Equipment - Previously used durable medical equipment.
    #[serde(rename = "Used Durable Medical Equipment")]
    UsedDurableMedicalEquipment,
    /// Durable Medical Equipment Purchase - Purchase of durable medical equipment.
    #[serde(rename = "Durable Medical Equipment Purchase")]
    DurableMedicalEquipmentPurchase,
    /// Ambulatory Service Center Facility - Ambulatory surgical center facility services.
    #[serde(rename = "Ambulatory Service Center Facility")]
    AmbulatoryServiceCenterFacility,
    /// Renal Supplies in the Home - Renal dialysis supplies for home use.
    #[serde(rename = "Renal Supplies in the Home")]
    RenalSuppliesInTheHome,
    /// Alternate Method Dialysis - Alternative dialysis methods.
    #[serde(rename = "Alternate Method Dialysis")]
    AlternateMethodDialysis,
    /// Chronic Renal Disease (CRD) Equipment - Equipment for chronic renal disease treatment.
    #[serde(rename = "Chronic Renal Disease (CRD) Equipment")]
    ChronicRenalDiseaseLeftParenthesisCrdRightParenthesisEquipment,
    /// Pre-Admission Testing - Pre-admission testing services.
    #[serde(rename = "Pre-Admission Testing")]
    PreAdmissionTesting,
    /// Durable Medical Equipment Rental - Rental of durable medical equipment.
    #[serde(rename = "Durable Medical Equipment Rental")]
    DurableMedicalEquipmentRental,
    /// Pneumonia Vaccine - Pneumonia vaccination services.
    #[serde(rename = "Pneumonia Vaccine")]
    PneumoniaVaccine,
    /// Second Surgical Opinion - Second opinion for surgical procedures.
    #[serde(rename = "Second Surgical Opinion")]
    SecondSurgicalOpinion,
    /// Third Surgical Opinion - Third opinion for surgical procedures.
    #[serde(rename = "Third Surgical Opinion")]
    ThirdSurgicalOpinion,
    /// Social Work - Social work services.
    #[serde(rename = "Social Work")]
    SocialWork,
    /// Diagnostic Dental - Dental diagnostic services and examinations.
    #[serde(rename = "Diagnostic Dental")]
    DiagnosticDental,
    /// Periodontics - Periodontal (gum) treatment services.
    #[serde(rename = "Periodontics")]
    Periodontics,
    /// Restorative - Restorative dental procedures (fillings, crowns, etc.).
    #[serde(rename = "Restorative")]
    Restorative,
    /// Endodontics - Endodontic services (root canal treatments).
    #[serde(rename = "Endodontics")]
    Endodontics,
    /// Maxillofacial Prosthetics - Maxillofacial prosthetic devices and services.
    #[serde(rename = "Maxillofacial Prosthetics")]
    MaxillofacialProsthetics,
    /// Adjunctive Dental Services - Additional supportive dental services.
    #[serde(rename = "Adjunctive Dental Services")]
    AdjunctiveDentalServices,
    /// Health Benefit Plan Coverage - General health benefit plan coverage information.
    #[serde(rename = "Health Benefit Plan Coverage")]
    HealthBenefitPlanCoverage,
    /// Plan Waiting Period - Plan waiting period information.
    #[serde(rename = "Plan Waiting Period")]
    PlanWaitingPeriod,
    /// Chiropractic - Chiropractic services.
    #[serde(rename = "Chiropractic")]
    Chiropractic,
    /// Chiropractic Office Visits - Chiropractic office visit services.
    #[serde(rename = "Chiropractic Office Visits")]
    ChiropracticOfficeVisits,
    /// Dental Care - General dental care services.
    #[serde(rename = "Dental Care")]
    DentalCare,
    /// Dental Crowns - Dental crown procedures.
    #[serde(rename = "Dental Crowns")]
    DentalCrowns,
    /// Dental Accident - Dental services for accident-related injuries.
    #[serde(rename = "Dental Accident")]
    DentalAccident,
    /// Orthodontics - Orthodontic services (braces, aligners, etc.).
    #[serde(rename = "Orthodontics")]
    Orthodontics,
    /// Prosthodontics - Prosthodontic services (dentures, bridges, etc.).
    #[serde(rename = "Prosthodontics")]
    Prosthodontics,
    /// Oral Surgery - Oral and maxillofacial surgery procedures.
    #[serde(rename = "Oral Surgery")]
    OralSurgery,
    /// Routine (Preventive) Dental - Routine preventive dental care (cleanings, exams).
    #[serde(rename = "Routine (Preventive) Dental")]
    RoutineLeftParenthesisPreventiveRightParenthesisDental,
    /// Home Health Care - Home health care services.
    #[serde(rename = "Home Health Care")]
    HomeHealthCare,
    /// Home Health Prescriptions - Prescription medications for home health care.
    #[serde(rename = "Home Health Prescriptions")]
    HomeHealthPrescriptions,
    /// Home Health Visits - Home health care provider visits.
    #[serde(rename = "Home Health Visits")]
    HomeHealthVisits,
    /// Hospice - Hospice care services for terminally ill patients.
    #[serde(rename = "Hospice")]
    Hospice,
    /// Respite Care - Respite care services for caregivers.
    #[serde(rename = "Respite Care")]
    RespiteCare,
    /// Hospital - General hospital services.
    #[serde(rename = "Hospital")]
    Hospital,
    /// Hospital - Inpatient - Inpatient hospital services.
    #[serde(rename = "Hospital - Inpatient")]
    HospitalInpatient,
    /// Hospital - Room and Board - Hospital room and board charges.
    #[serde(rename = "Hospital - Room and Board")]
    HospitalRoomAndBoard,
    /// Hospital - Outpatient - Outpatient hospital services.
    #[serde(rename = "Hospital - Outpatient")]
    HospitalOutpatient,
    /// Hospital - Emergency Accident - Emergency hospital services for accidents.
    #[serde(rename = "Hospital - Emergency Accident")]
    HospitalEmergencyAccident,
    /// Hospital - Emergency Medical - Emergency hospital medical services.
    #[serde(rename = "Hospital - Emergency Medical")]
    HospitalEmergencyMedical,
    /// Hospital - Ambulatory Surgical - Ambulatory surgical services at hospitals.
    #[serde(rename = "Hospital - Ambulatory Surgical")]
    HospitalAmbulatorySurgical,
    /// Long Term Care - Long-term care facility services.
    #[serde(rename = "Long Term Care")]
    LongTermCare,
    /// Major Medical - Major medical coverage and services.
    #[serde(rename = "Major Medical")]
    MajorMedical,
    /// Medically Related Transportation - Medical transportation services.
    #[serde(rename = "Medically Related Transportation")]
    MedicallyRelatedTransportation,
    /// Air Transportation - Air ambulance and air medical transportation services.
    #[serde(rename = "Air Transportation")]
    AirTransportation,
    /// Cabulance - Taxi/ambulance hybrid transportation services.
    #[serde(rename = "Cabulance")]
    Cabulance,
    /// Licensed Ambulance - Licensed ambulance transportation services.
    #[serde(rename = "Licensed Ambulance")]
    LicensedAmbulance,
    /// General Benefits - General benefit plan coverage.
    #[serde(rename = "General Benefits")]
    GeneralBenefits,
    /// In-vitro Fertilization - In-vitro fertilization (IVF) services.
    #[serde(rename = "In-vitro Fertilization")]
    InVitroFertilization,
    /// MRI/CAT Scan - Magnetic resonance imaging and computed tomography scan services.
    #[serde(rename = "MRI/CAT Scan")]
    MriSlashCatScan,
    /// Donor Procedures - Organ and tissue donor procedures.
    #[serde(rename = "Donor Procedures")]
    DonorProcedures,
    /// Acupuncture - Acupuncture services.
    #[serde(rename = "Acupuncture")]
    Acupuncture,
    /// Newborn Care - Newborn care services.
    #[serde(rename = "Newborn Care")]
    NewbornCare,
    /// Pathology - Pathology laboratory services.
    #[serde(rename = "Pathology")]
    Pathology,
    /// Smoking Cessation - Smoking cessation programs and services.
    #[serde(rename = "Smoking Cessation")]
    SmokingCessation,
    /// Well Baby Care - Well-baby care and preventive pediatric services.
    #[serde(rename = "Well Baby Care")]
    WellBabyCare,
    /// Maternity - Maternity care and pregnancy-related services.
    #[serde(rename = "Maternity")]
    Maternity,
    /// Transplants - Organ and tissue transplant services.
    #[serde(rename = "Transplants")]
    Transplants,
    /// Audiology Exam - Audiology examination and hearing services.
    #[serde(rename = "Audiology Exam")]
    AudiologyExam,
    /// Inhalation Therapy - Inhalation therapy and respiratory services.
    #[serde(rename = "Inhalation Therapy")]
    InhalationTherapy,
    /// Diagnostic Medical - General diagnostic medical services.
    #[serde(rename = "Diagnostic Medical")]
    DiagnosticMedical,
    /// Private Duty Nursing - Private duty nursing services.
    #[serde(rename = "Private Duty Nursing")]
    PrivateDutyNursing,
    /// Prosthetic Device - Prosthetic devices and services.
    #[serde(rename = "Prosthetic Device")]
    ProstheticDevice,
    /// Dialysis - Dialysis services for kidney disease.
    #[serde(rename = "Dialysis")]
    Dialysis,
    /// Otological Exam - Otological (ear) examination services.
    #[serde(rename = "Otological Exam")]
    OtologicalExam,
    /// Chemotherapy - Chemotherapy cancer treatment services.
    #[serde(rename = "Chemotherapy")]
    Chemotherapy,
    /// Allergy Testing - Allergy testing and diagnostic services.
    #[serde(rename = "Allergy Testing")]
    AllergyTesting,
    /// Immunizations - Immunization and vaccination services.
    #[serde(rename = "Immunizations")]
    Immunizations,
    /// Routine Physical - Routine physical examination services.
    #[serde(rename = "Routine Physical")]
    RoutinePhysical,
    /// Family Planning - Family planning and reproductive health services.
    #[serde(rename = "Family Planning")]
    FamilyPlanning,
    /// Infertility - Infertility treatment services.
    #[serde(rename = "Infertility")]
    Infertility,
    /// Abortion - Abortion services.
    #[serde(rename = "Abortion")]
    Abortion,
    /// AIDS - AIDS and HIV treatment services.
    #[serde(rename = "AIDS")]
    Aids,
    /// Emergency Services - Emergency medical services.
    #[serde(rename = "Emergency Services")]
    EmergencyServices,
    /// Cancer - Cancer treatment services.
    #[serde(rename = "Cancer")]
    Cancer,
    /// Pharmacy - General pharmacy services and prescription drug coverage.
    #[serde(rename = "Pharmacy")]
    Pharmacy,
    /// Free Standing Prescription Drug - Free-standing pharmacy prescription services.
    #[serde(rename = "Free Standing Prescription Drug")]
    FreeStandingPrescriptionDrug,
    /// Mail Order Prescription Drug - Mail order pharmacy prescription services.
    #[serde(rename = "Mail Order Prescription Drug")]
    MailOrderPrescriptionDrug,
    /// Brand Name Prescription Drug - Brand name prescription medications.
    #[serde(rename = "Brand Name Prescription Drug")]
    BrandNamePrescriptionDrug,
    /// Generic Prescription Drug - Generic prescription medications.
    #[serde(rename = "Generic Prescription Drug")]
    GenericPrescriptionDrug,
    /// Podiatry - Podiatry (foot care) services.
    #[serde(rename = "Podiatry")]
    Podiatry,
    /// Podiatry - Office Visits - Podiatry office visit services.
    #[serde(rename = "Podiatry - Office Visits")]
    PodiatryOfficeVisits,
    /// Podiatry - Nursing Home Visits - Podiatry services at nursing homes.
    #[serde(rename = "Podiatry - Nursing Home Visits")]
    PodiatryNursingHomeVisits,
    /// Professional (Physician) - General professional/physician services (includes all healthcare providers).
    #[serde(rename = "Professional (Physician)")]
    ProfessionalLeftParenthesisPhysicianRightParenthesis,
    /// Anesthesiologist - Anesthesiologist services.
    #[serde(rename = "Anesthesiologist")]
    Anesthesiologist,
    /// Professional (Physician) Visit - Office - Office visit services with healthcare providers.
    #[serde(rename = "Professional (Physician) Visit - Office")]
    ProfessionalLeftParenthesisPhysicianRightParenthesisVisitOffice,
    /// Professional (Physician) Visit - Inpatient - Inpatient visit services with healthcare providers.
    #[serde(rename = "Professional (Physician) Visit - Inpatient")]
    ProfessionalLeftParenthesisPhysicianRightParenthesisVisitInpatient,
    /// Professional (Physician) Visit - Outpatient - Outpatient visit services with healthcare providers.
    #[serde(rename = "Professional (Physician) Visit - Outpatient")]
    ProfessionalLeftParenthesisPhysicianRightParenthesisVisitOutpatient,
    /// Professional (Physician) Visit - Nursing Home - Nursing home visit services with healthcare providers.
    #[serde(rename = "Professional (Physician) Visit - Nursing Home")]
    ProfessionalLeftParenthesisPhysicianRightParenthesisVisitNursingHome,
    /// Professional (Physician) Visit - Skilled Nursing Facility - Skilled nursing facility visit services.
    #[serde(rename = "Professional (Physician) Visit - Skilled Nursing Facility")]
    ProfessionalLeftParenthesisPhysicianRightParenthesisVisitSkilledNursingFacility,
    /// Professional (Physician) Visit - Home - Home visit services with healthcare providers.
    #[serde(rename = "Professional (Physician) Visit - Home")]
    ProfessionalLeftParenthesisPhysicianRightParenthesisVisitHome,
    /// Psychiatric - General psychiatric services.
    #[serde(rename = "Psychiatric")]
    Psychiatric,
    /// Psychiatric - Room and Board - Psychiatric facility room and board charges.
    #[serde(rename = "Psychiatric - Room and Board")]
    PsychiatricRoomAndBoard,
    /// Psychotherapy - Psychotherapy and counseling services.
    #[serde(rename = "Psychotherapy")]
    Psychotherapy,
    /// Psychiatric - Inpatient - Inpatient psychiatric care services.
    #[serde(rename = "Psychiatric - Inpatient")]
    PsychiatricInpatient,
    /// Psychiatric - Outpatient - Outpatient psychiatric care services.
    #[serde(rename = "Psychiatric - Outpatient")]
    PsychiatricOutpatient,
    /// Rehabilitation - General rehabilitation services.
    #[serde(rename = "Rehabilitation")]
    Rehabilitation,
    /// Rehabilitation - Room and Board - Rehabilitation facility room and board charges.
    #[serde(rename = "Rehabilitation - Room and Board")]
    RehabilitationRoomAndBoard,
    /// Rehabilitation - Inpatient - Inpatient rehabilitation services.
    #[serde(rename = "Rehabilitation - Inpatient")]
    RehabilitationInpatient,
    /// Rehabilitation - Outpatient - Outpatient rehabilitation services.
    #[serde(rename = "Rehabilitation - Outpatient")]
    RehabilitationOutpatient,
    /// Occupational Therapy - Occupational therapy services.
    #[serde(rename = "Occupational Therapy")]
    OccupationalTherapy,
    /// Physical Medicine - Physical medicine and rehabilitation services.
    #[serde(rename = "Physical Medicine")]
    PhysicalMedicine,
    /// Speech Therapy - Speech therapy and language services.
    #[serde(rename = "Speech Therapy")]
    SpeechTherapy,
    /// Skilled Nursing Care - Skilled nursing facility care services.
    #[serde(rename = "Skilled Nursing Care")]
    SkilledNursingCare,
    /// Skilled Nursing Care - Room and Board - Skilled nursing facility room and board charges.
    #[serde(rename = "Skilled Nursing Care - Room and Board")]
    SkilledNursingCareRoomAndBoard,
    /// Substance Abuse - Substance abuse treatment services.
    #[serde(rename = "Substance Abuse")]
    SubstanceAbuse,
    /// Alcoholism - Alcoholism treatment services.
    #[serde(rename = "Alcoholism")]
    Alcoholism,
    /// Drug Addiction - Drug addiction treatment services.
    #[serde(rename = "Drug Addiction")]
    DrugAddiction,
    /// Vision (Optometry) - Vision and optometry services.
    #[serde(rename = "Vision (Optometry)")]
    VisionLeftParenthesisOptometryRightParenthesis,
    /// Frames - Eyeglass frames.
    #[serde(rename = "Frames")]
    Frames,
    /// Routine Exam - Routine examination services.
    #[serde(rename = "Routine Exam")]
    RoutineExam,
    /// Lenses - Eyeglass and contact lenses.
    #[serde(rename = "Lenses")]
    Lenses,
    /// Nonmedically Necessary Physical - Physical examinations not medically necessary.
    #[serde(rename = "Nonmedically Necessary Physical")]
    NonmedicallyNecessaryPhysical,
    /// Experimental Drug Therapy - Experimental drug therapy services.
    #[serde(rename = "Experimental Drug Therapy")]
    ExperimentalDrugTherapy,
    /// Burn Care - Burn care and treatment services.
    #[serde(rename = "Burn Care")]
    BurnCare,
    /// Brand Name Prescription Drug - Formulary - Brand name prescription drugs on formulary.
    #[serde(rename = "Brand Name Prescription Drug - Formulary")]
    BrandNamePrescriptionDrugFormulary,
    /// Brand Name Prescription Drug - Non-Formulary - Brand name prescription drugs not on formulary.
    #[serde(rename = "Brand Name Prescription Drug - Non-Formulary")]
    BrandNamePrescriptionDrugNonFormulary,
    /// Independent Medical Evaluation - Independent medical evaluation services.
    #[serde(rename = "Independent Medical Evaluation")]
    IndependentMedicalEvaluation,
    /// Partial Hospitalization (Psychiatric) - Partial hospitalization psychiatric services.
    #[serde(rename = "Partial Hospitalization (Psychiatric)")]
    PartialHospitalizationLeftParenthesisPsychiatricRightParenthesis,
    /// Day Care (Psychiatric) - Day care psychiatric services.
    #[serde(rename = "Day Care (Psychiatric)")]
    DayCareLeftParenthesisPsychiatricRightParenthesis,
    /// Cognitive Therapy - Cognitive therapy services.
    #[serde(rename = "Cognitive Therapy")]
    CognitiveTherapy,
    /// Massage Therapy - Massage therapy services.
    #[serde(rename = "Massage Therapy")]
    MassageTherapy,
    /// Pulmonary Rehabilitation - Pulmonary (lung) rehabilitation services.
    #[serde(rename = "Pulmonary Rehabilitation")]
    PulmonaryRehabilitation,
    /// Cardiac Rehabilitation - Cardiac (heart) rehabilitation services.
    #[serde(rename = "Cardiac Rehabilitation")]
    CardiacRehabilitation,
    /// Pediatric - Pediatric (children's) medical services.
    #[serde(rename = "Pediatric")]
    Pediatric,
    /// Nursery - Nursery and newborn care services.
    #[serde(rename = "Nursery")]
    Nursery,
    /// Skin - Dermatology and skin care services.
    #[serde(rename = "Skin")]
    Skin,
    /// Orthopedic - Orthopedic (bone and joint) services.
    #[serde(rename = "Orthopedic")]
    Orthopedic,
    /// Cardiac - Cardiac (heart) services.
    #[serde(rename = "Cardiac")]
    Cardiac,
    /// Lymphatic - Lymphatic system services.
    #[serde(rename = "Lymphatic")]
    Lymphatic,
    /// Gastrointestinal - Gastrointestinal (digestive system) services.
    #[serde(rename = "Gastrointestinal")]
    Gastrointestinal,
    /// Endocrine - Endocrine (hormone) system services.
    #[serde(rename = "Endocrine")]
    Endocrine,
    /// Neurology - Neurology (nervous system) services.
    #[serde(rename = "Neurology")]
    Neurology,
    /// Eye - Eye care and ophthalmology services.
    #[serde(rename = "Eye")]
    Eye,
    /// Invasive Procedures - Invasive medical procedures.
    #[serde(rename = "Invasive Procedures")]
    InvasiveProcedures,
    /// Gynecological - Gynecological (women's reproductive health) services.
    #[serde(rename = "Gynecological")]
    Gynecological,
    /// Obstetrical - Obstetrical (pregnancy and childbirth) services.
    #[serde(rename = "Obstetrical")]
    Obstetrical,
    /// Obstetrical/Gynecological - Combined obstetrical and gynecological services.
    #[serde(rename = "Obstetrical/Gynecological")]
    ObstetricalSlashGynecological,
    /// Mail Order Prescription Drug - Formulary - Mail order prescription drugs on formulary.
    #[serde(rename = "Mail Order Prescription Drug - Formulary")]
    MailOrderPrescriptionDrugFormulary,
    /// Mail Order Prescription Drug - Non-Formulary - Mail order prescription drugs not on formulary.
    #[serde(rename = "Mail Order Prescription Drug - Non-Formulary")]
    MailOrderPrescriptionDrugNonFormulary,
    /// Physician Visit - Office: Sick - Office visits for sick/illness care.
    #[serde(rename = "Physician Visit - Office: Sick")]
    PhysicianVisitOfficeColonSick,
    /// Physician Visit - Office: Well - Office visits for well/preventive care.
    #[serde(rename = "Physician Visit - Office: Well")]
    PhysicianVisitOfficeColonWell,
    /// Coronary Care - Coronary (heart) intensive care services.
    #[serde(rename = "Coronary Care")]
    CoronaryCare,
    /// Private Duty Nursing - Inpatient - Private duty nursing in inpatient settings.
    #[serde(rename = "Private Duty Nursing - Inpatient")]
    PrivateDutyNursingInpatient,
    /// Private Duty Nursing - Home - Private duty nursing in home settings.
    #[serde(rename = "Private Duty Nursing - Home")]
    PrivateDutyNursingHome,
    /// Surgical Benefits - Professional (Physician) - Professional/physician surgical benefits.
    #[serde(rename = "Surgical Benefits - Professional (Physician)")]
    SurgicalBenefitsProfessionalLeftParenthesisPhysicianRightParenthesis,
    /// Surgical Benefits - Facility - Facility surgical benefits.
    #[serde(rename = "Surgical Benefits - Facility")]
    SurgicalBenefitsFacility,
    /// Mental Health Provider - Inpatient - Inpatient mental health provider services.
    #[serde(rename = "Mental Health Provider - Inpatient")]
    MentalHealthProviderInpatient,
    /// Mental Health Provider - Outpatient - Outpatient mental health provider services.
    #[serde(rename = "Mental Health Provider - Outpatient")]
    MentalHealthProviderOutpatient,
    /// Mental Health Facility - Inpatient - Inpatient mental health facility services.
    #[serde(rename = "Mental Health Facility - Inpatient")]
    MentalHealthFacilityInpatient,
    /// Mental Health Facility - Outpatient - Outpatient mental health facility services.
    #[serde(rename = "Mental Health Facility - Outpatient")]
    MentalHealthFacilityOutpatient,
    /// Substance Abuse Facility - Inpatient - Inpatient substance abuse facility services.
    #[serde(rename = "Substance Abuse Facility - Inpatient")]
    SubstanceAbuseFacilityInpatient,
    /// Substance Abuse Facility - Outpatient - Outpatient substance abuse facility services.
    #[serde(rename = "Substance Abuse Facility - Outpatient")]
    SubstanceAbuseFacilityOutpatient,
    /// Screening X-ray - Screening X-ray services.
    #[serde(rename = "Screening X-ray")]
    ScreeningXRay,
    /// Screening laboratory - Screening laboratory services.
    #[serde(rename = "Screening laboratory")]
    ScreeningLaboratory,
    /// Mammogram, High Risk Patient - Mammography for high-risk patients.
    #[serde(rename = "Mammogram, High Risk Patient")]
    MammogramCommaHighRiskPatient,
    /// Mammogram, Low Risk Patient - Mammography for low-risk patients.
    #[serde(rename = "Mammogram, Low Risk Patient")]
    MammogramCommaLowRiskPatient,
    /// Flu Vaccination - Influenza vaccination services.
    #[serde(rename = "Flu Vaccination")]
    FluVaccination,
    /// Eyewear and Eyewear Accessories - Eyewear and accessory items.
    #[serde(rename = "Eyewear and Eyewear Accessories")]
    EyewearAndEyewearAccessories,
    /// Case Management - Case management services.
    #[serde(rename = "Case Management")]
    CaseManagement,
    /// Dermatology - Dermatology (skin) services.
    #[serde(rename = "Dermatology")]
    Dermatology,
    /// Durable Medical Equipment - General durable medical equipment services.
    #[serde(rename = "Durable Medical Equipment")]
    DurableMedicalEquipment,
    /// Diabetic Supplies - Diabetic supplies and equipment.
    #[serde(rename = "Diabetic Supplies")]
    DiabeticSupplies,
    /// Generic Prescription Drug - Formulary - Generic prescription drugs on formulary.
    #[serde(rename = "Generic Prescription Drug - Formulary")]
    GenericPrescriptionDrugFormulary,
    /// Generic Prescription Drug - Non-Formulary - Generic prescription drugs not on formulary.
    #[serde(rename = "Generic Prescription Drug - Non-Formulary")]
    GenericPrescriptionDrugNonFormulary,
    /// Allergy - Allergy services and treatment.
    #[serde(rename = "Allergy")]
    Allergy,
    /// Intensive Care - Intensive care unit services.
    #[serde(rename = "Intensive Care")]
    IntensiveCare,
    /// Mental Health - General mental health services.
    #[serde(rename = "Mental Health")]
    MentalHealth,
    /// Neonatol Intensive Care - Neonatal intensive care services.
    #[serde(rename = "Neonatol Intensive Care")]
    NeonatolIntensiveCare,
    /// Oncology - Oncology (cancer) services.
    #[serde(rename = "Oncology")]
    Oncology,
    /// Physical Therapy - Physical therapy services.
    #[serde(rename = "Physical Therapy")]
    PhysicalTherapy,
    /// Pulmonary - Pulmonary (lung) services.
    #[serde(rename = "Pulmonary")]
    Pulmonary,
    /// Renal - Renal (kidney) services.
    #[serde(rename = "Renal")]
    Renal,
    /// Residential Psychiatric Treatment - Residential psychiatric treatment services.
    #[serde(rename = "Residential Psychiatric Treatment")]
    ResidentialPsychiatricTreatment,
    /// Transitional Care - Transitional care services.
    #[serde(rename = "Transitional Care")]
    TransitionalCare,
    /// Transitional Nursery Care - Transitional nursery care services.
    #[serde(rename = "Transitional Nursery Care")]
    TransitionalNurseryCare,
    /// Urgent Care - Urgent care facility services.
    #[serde(rename = "Urgent Care")]
    UrgentCare,
    /// Fluoride Treatments - Dental fluoride treatment services.
    #[serde(rename = "Fluoride Treatments")]
    FluorideTreatments,
    /// Dental Prophylaxis - Dental prophylaxis (cleaning) services.
    #[serde(rename = "Dental Prophylaxis")]
    DentalProphylaxis,
    /// Space Maintenance - Dental space maintenance appliances.
    #[serde(rename = "Space Maintenance")]
    SpaceMaintenance,
    /// Harmful Habits Appliance - Appliances for harmful oral habits.
    #[serde(rename = "Harmful Habits Appliance")]
    HarmfulHabitsAppliance,
    /// Composites - Composite dental restorations.
    #[serde(rename = "Composites")]
    Composites,
    /// Bitewing X-Rays - Bitewing dental X-ray services.
    #[serde(rename = "Bitewing X-Rays")]
    BitewingXRays,
    /// Root Canal / Retreatment - Root canal and retreatment services.
    #[serde(rename = "Root Canal / Retreatment")]
    RootCanalSlashRetreatment,
    /// Oral Evaluation - Oral evaluation and examination services.
    #[serde(rename = "Oral Evaluation")]
    OralEvaluation,
    /// Full Mouth / Panoramic X-Rays - Full mouth and panoramic X-ray services.
    #[serde(rename = "Full Mouth / Panoramic X-Rays")]
    FullMouthSlashPanoramicXRays,
    /// Periapical X-Ray - Periapical dental X-ray services.
    #[serde(rename = "Periapical X-Ray")]
    PeriapicalXRay,
    /// Surgical Periodontics - Surgical periodontal procedures.
    #[serde(rename = "Surgical Periodontics")]
    SurgicalPeriodontics,
    /// Denture Adjust, Rebase, Reline, Repair - Denture adjustment, rebasing, relining, and repair services.
    #[serde(rename = "Denture Adjust, Rebase, Reline, Repair")]
    DentureAdjustCommaRebaseCommaRelineCommaRepair,
    /// Inlay / Onlay - Dental inlay and onlay restorations.
    #[serde(rename = "Inlay / Onlay")]
    InlaySlashOnlay,
    /// Buildups / Post and Core - Dental buildups, posts, and cores.
    #[serde(rename = "Buildups / Post and Core")]
    BuildupsSlashPostAndCore,
    /// Amalgam / Composite Restorations - Amalgam and composite dental restorations.
    #[serde(rename = "Amalgam / Composite Restorations")]
    AmalgamSlashCompositeRestorations,
    /// Dentures - Denture services.
    #[serde(rename = "Dentures")]
    Dentures,
    /// Dentures - Repair - Denture repair services.
    #[serde(rename = "Dentures - Repair")]
    DenturesRepair,
    /// Dentures - Reline - Denture relining services.
    #[serde(rename = "Dentures - Reline")]
    DenturesReline,
    /// Stainless Steel, Resin, Acrylic Crowns - Stainless steel, resin, and acrylic dental crowns.
    #[serde(rename = "Stainless Steel, Resin, Acrylic Crowns")]
    StainlessSteelCommaResinCommaAcrylicCrowns,
    /// Sealants - Dental sealant services.
    #[serde(rename = "Sealants")]
    Sealants,
    /// Dental Implants - Dental implant services.
    #[serde(rename = "Dental Implants")]
    DentalImplants,
    /// Temporomandibular Joint Dysfunction - TMJ (temporomandibular joint) dysfunction treatment services.
    #[serde(rename = "Temporomandibular Joint Dysfunction")]
    TemporomandibularJointDysfunction,
    #[serde(rename = "Specialty Pharmacy")]
    SpecialtyPharmacy,
    #[serde(rename = "Durable Medical Equipment (New)")]
    DurableMedicalEquipmentNew,
    #[serde(rename = "Diagnostic Imaging")]
    DiagnosticImaging,
    #[serde(rename = "Fertility Preservation")]
    FertilityPreservation,
    #[serde(rename = "Applied Behavioral Analysis Therapy")]
    AppliedBehavioralAnalysisTherapy,
    #[serde(rename = "Non-Medical Equipment")]
    NonMedicalEquipment,
    #[serde(rename = "Psychiatric Emergency")]
    PsychiatricEmergency,
    #[serde(rename = "Step Down Unit")]
    StepDownUnit,
    #[serde(rename = "Skilled Nursing Facility Head Level")]
    SkilledNursingFacilityHeadLevel,
    #[serde(rename = "Skilled Nursing Facility Ventilator Level")]
    SkilledNursingFacilityVentilatorLevel,
    #[serde(rename = "Level of Care 1")]
    LevelOfCare1,
    #[serde(rename = "Level of Care 2")]
    LevelOfCare2,
    #[serde(rename = "Level of Care 3")]
    LevelOfCare3,
    #[serde(rename = "Level of Care 4")]
    LevelOfCare4,
    #[serde(rename = "Radiographs")]
    Radiographs,
    #[serde(rename = "Fixed Prosthodontics")]
    FixedProsthodontics,
    #[serde(rename = "Removable Prosthodontics")]
    RemovableProsthodontics,
    #[serde(rename = "Intraoral Images - Complete Series")]
    IntraoralImagesCompleteSeries,
    #[serde(rename = "Oral Evaluation (E17)")]
    OralEvaluationE17,
    #[serde(rename = "Dental Prophylaxis (E18)")]
    DentalProphylaxisE18,
    #[serde(rename = "Panoramic Images")]
    PanoramicImages,
    #[serde(rename = "Sealants (E20)")]
    SealantsE20,
    #[serde(rename = "Fluoride Treatments (E21)")]
    FluorideTreatmentsE21,
    #[serde(rename = "Dental Implants (E22)")]
    DentalImplantsE22,
    #[serde(rename = "Temporomandibular Joint Dysfunction (E23)")]
    TemporomandibularJointDysfunctionE23,
    #[serde(rename = "Long Term Care Pharmacy")]
    LongTermCarePharmacy,
    #[serde(rename = "Comprehensive Medication Therapy Management")]
    ComprehensiveMedicationTherapyManagement,
    #[serde(rename = "Targeted Medication Therapy Management")]
    TargetedMedicationTherapyManagement,
    #[serde(rename = "Dietary/Nutritional Services")]
    DietaryNutritionalServices,
    #[serde(rename = "Intensive Cardiac Rehabilitation")]
    IntensiveCardiacRehabilitation,
    #[serde(rename = "Convenience Care")]
    ConvenienceCare,
    #[serde(rename = "Telemedicine")]
    Telemedicine,
    #[serde(rename = "Pharmacist Services")]
    PharmacistServices,
    #[serde(rename = "Diabetic Education")]
    DiabeticEducation,
    #[serde(rename = "Early Intervention")]
    EarlyIntervention,
    #[serde(rename = "Medically Tailored Meals")]
    MedicallyTailoredMeals,
    #[serde(rename = "Serious Mental Health")]
    SeriousMentalHealth,
    #[serde(rename = "Remote Patient Monitoring")]
    RemotePatientMonitoring,
    #[serde(rename = "Remote Therapeutic Monitoring")]
    RemoteTherapeuticMonitoring,
    #[serde(rename = "Special Supplemental Benefits")]
    SpecialSupplementalBenefits,
    #[serde(rename = "Positron Emission Tomography (PET) Scan")]
    PositronEmissionTomographyPetScan,
    #[serde(rename = "CAT Scan")]
    CatScan,
    #[serde(rename = "IV Therapy")]
    IvTherapy,
    /// Unknown / unrecognized service type name from payer response
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ResponseEligibilityServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MedicalCare => write!(f, "Medical Care"),
            Self::Surgical => write!(f, "Surgical"),
            Self::Consultation => write!(f, "Consultation"),
            Self::DiagnosticXRay => write!(f, "Diagnostic X-Ray"),
            Self::DiagnosticLab => write!(f, "Diagnostic Lab"),
            Self::RadiationTherapy => write!(f, "Radiation Therapy"),
            Self::Anesthesia => write!(f, "Anesthesia"),
            Self::SurgicalAssistance => write!(f, "Surgical Assistance"),
            Self::OtherMedical => write!(f, "Other Medical"),
            Self::BloodCharges => write!(f, "Blood Charges"),
            Self::UsedDurableMedicalEquipment => write!(f, "Used Durable Medical Equipment"),
            Self::DurableMedicalEquipmentPurchase => write!(f, "Durable Medical Equipment Purchase"),
            Self::AmbulatoryServiceCenterFacility => write!(f, "Ambulatory Service Center Facility"),
            Self::RenalSuppliesInTheHome => write!(f, "Renal Supplies in the Home"),
            Self::AlternateMethodDialysis => write!(f, "Alternate Method Dialysis"),
            Self::ChronicRenalDiseaseLeftParenthesisCrdRightParenthesisEquipment => write!(f, "Chronic Renal Disease (CRD) Equipment"),
            Self::PreAdmissionTesting => write!(f, "Pre-Admission Testing"),
            Self::DurableMedicalEquipmentRental => write!(f, "Durable Medical Equipment Rental"),
            Self::PneumoniaVaccine => write!(f, "Pneumonia Vaccine"),
            Self::SecondSurgicalOpinion => write!(f, "Second Surgical Opinion"),
            Self::ThirdSurgicalOpinion => write!(f, "Third Surgical Opinion"),
            Self::SocialWork => write!(f, "Social Work"),
            Self::DiagnosticDental => write!(f, "Diagnostic Dental"),
            Self::Periodontics => write!(f, "Periodontics"),
            Self::Restorative => write!(f, "Restorative"),
            Self::Endodontics => write!(f, "Endodontics"),
            Self::MaxillofacialProsthetics => write!(f, "Maxillofacial Prosthetics"),
            Self::AdjunctiveDentalServices => write!(f, "Adjunctive Dental Services"),
            Self::HealthBenefitPlanCoverage => write!(f, "Health Benefit Plan Coverage"),
            Self::PlanWaitingPeriod => write!(f, "Plan Waiting Period"),
            Self::Chiropractic => write!(f, "Chiropractic"),
            Self::ChiropracticOfficeVisits => write!(f, "Chiropractic Office Visits"),
            Self::DentalCare => write!(f, "Dental Care"),
            Self::DentalCrowns => write!(f, "Dental Crowns"),
            Self::DentalAccident => write!(f, "Dental Accident"),
            Self::Orthodontics => write!(f, "Orthodontics"),
            Self::Prosthodontics => write!(f, "Prosthodontics"),
            Self::OralSurgery => write!(f, "Oral Surgery"),
            Self::RoutineLeftParenthesisPreventiveRightParenthesisDental => write!(f, "Routine (Preventive) Dental"),
            Self::HomeHealthCare => write!(f, "Home Health Care"),
            Self::HomeHealthPrescriptions => write!(f, "Home Health Prescriptions"),
            Self::HomeHealthVisits => write!(f, "Home Health Visits"),
            Self::Hospice => write!(f, "Hospice"),
            Self::RespiteCare => write!(f, "Respite Care"),
            Self::Hospital => write!(f, "Hospital"),
            Self::HospitalInpatient => write!(f, "Hospital - Inpatient"),
            Self::HospitalRoomAndBoard => write!(f, "Hospital - Room and Board"),
            Self::HospitalOutpatient => write!(f, "Hospital - Outpatient"),
            Self::HospitalEmergencyAccident => write!(f, "Hospital - Emergency Accident"),
            Self::HospitalEmergencyMedical => write!(f, "Hospital - Emergency Medical"),
            Self::HospitalAmbulatorySurgical => write!(f, "Hospital - Ambulatory Surgical"),
            Self::LongTermCare => write!(f, "Long Term Care"),
            Self::MajorMedical => write!(f, "Major Medical"),
            Self::MedicallyRelatedTransportation => write!(f, "Medically Related Transportation"),
            Self::AirTransportation => write!(f, "Air Transportation"),
            Self::Cabulance => write!(f, "Cabulance"),
            Self::LicensedAmbulance => write!(f, "Licensed Ambulance"),
            Self::GeneralBenefits => write!(f, "General Benefits"),
            Self::InVitroFertilization => write!(f, "In-vitro Fertilization"),
            Self::MriSlashCatScan => write!(f, "MRI/CAT Scan"),
            Self::DonorProcedures => write!(f, "Donor Procedures"),
            Self::Acupuncture => write!(f, "Acupuncture"),
            Self::NewbornCare => write!(f, "Newborn Care"),
            Self::Pathology => write!(f, "Pathology"),
            Self::SmokingCessation => write!(f, "Smoking Cessation"),
            Self::WellBabyCare => write!(f, "Well Baby Care"),
            Self::Maternity => write!(f, "Maternity"),
            Self::Transplants => write!(f, "Transplants"),
            Self::AudiologyExam => write!(f, "Audiology Exam"),
            Self::InhalationTherapy => write!(f, "Inhalation Therapy"),
            Self::DiagnosticMedical => write!(f, "Diagnostic Medical"),
            Self::PrivateDutyNursing => write!(f, "Private Duty Nursing"),
            Self::ProstheticDevice => write!(f, "Prosthetic Device"),
            Self::Dialysis => write!(f, "Dialysis"),
            Self::OtologicalExam => write!(f, "Otological Exam"),
            Self::Chemotherapy => write!(f, "Chemotherapy"),
            Self::AllergyTesting => write!(f, "Allergy Testing"),
            Self::Immunizations => write!(f, "Immunizations"),
            Self::RoutinePhysical => write!(f, "Routine Physical"),
            Self::FamilyPlanning => write!(f, "Family Planning"),
            Self::Infertility => write!(f, "Infertility"),
            Self::Abortion => write!(f, "Abortion"),
            Self::Aids => write!(f, "AIDS"),
            Self::EmergencyServices => write!(f, "Emergency Services"),
            Self::Cancer => write!(f, "Cancer"),
            Self::Pharmacy => write!(f, "Pharmacy"),
            Self::FreeStandingPrescriptionDrug => write!(f, "Free Standing Prescription Drug"),
            Self::MailOrderPrescriptionDrug => write!(f, "Mail Order Prescription Drug"),
            Self::BrandNamePrescriptionDrug => write!(f, "Brand Name Prescription Drug"),
            Self::GenericPrescriptionDrug => write!(f, "Generic Prescription Drug"),
            Self::Podiatry => write!(f, "Podiatry"),
            Self::PodiatryOfficeVisits => write!(f, "Podiatry - Office Visits"),
            Self::PodiatryNursingHomeVisits => write!(f, "Podiatry - Nursing Home Visits"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesis => write!(f, "Professional (Physician)"),
            Self::Anesthesiologist => write!(f, "Anesthesiologist"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesisVisitOffice => write!(f, "Professional (Physician) Visit - Office"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesisVisitInpatient => write!(f, "Professional (Physician) Visit - Inpatient"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesisVisitOutpatient => write!(f, "Professional (Physician) Visit - Outpatient"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesisVisitNursingHome => write!(f, "Professional (Physician) Visit - Nursing Home"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesisVisitSkilledNursingFacility => write!(f, "Professional (Physician) Visit - Skilled Nursing Facility"),
            Self::ProfessionalLeftParenthesisPhysicianRightParenthesisVisitHome => write!(f, "Professional (Physician) Visit - Home"),
            Self::Psychiatric => write!(f, "Psychiatric"),
            Self::PsychiatricRoomAndBoard => write!(f, "Psychiatric - Room and Board"),
            Self::Psychotherapy => write!(f, "Psychotherapy"),
            Self::PsychiatricInpatient => write!(f, "Psychiatric - Inpatient"),
            Self::PsychiatricOutpatient => write!(f, "Psychiatric - Outpatient"),
            Self::Rehabilitation => write!(f, "Rehabilitation"),
            Self::RehabilitationRoomAndBoard => write!(f, "Rehabilitation - Room and Board"),
            Self::RehabilitationInpatient => write!(f, "Rehabilitation - Inpatient"),
            Self::RehabilitationOutpatient => write!(f, "Rehabilitation - Outpatient"),
            Self::OccupationalTherapy => write!(f, "Occupational Therapy"),
            Self::PhysicalMedicine => write!(f, "Physical Medicine"),
            Self::SpeechTherapy => write!(f, "Speech Therapy"),
            Self::SkilledNursingCare => write!(f, "Skilled Nursing Care"),
            Self::SkilledNursingCareRoomAndBoard => write!(f, "Skilled Nursing Care - Room and Board"),
            Self::SubstanceAbuse => write!(f, "Substance Abuse"),
            Self::Alcoholism => write!(f, "Alcoholism"),
            Self::DrugAddiction => write!(f, "Drug Addiction"),
            Self::VisionLeftParenthesisOptometryRightParenthesis => write!(f, "Vision (Optometry)"),
            Self::Frames => write!(f, "Frames"),
            Self::RoutineExam => write!(f, "Routine Exam"),
            Self::Lenses => write!(f, "Lenses"),
            Self::NonmedicallyNecessaryPhysical => write!(f, "Nonmedically Necessary Physical"),
            Self::ExperimentalDrugTherapy => write!(f, "Experimental Drug Therapy"),
            Self::BurnCare => write!(f, "Burn Care"),
            Self::BrandNamePrescriptionDrugFormulary => write!(f, "Brand Name Prescription Drug - Formulary"),
            Self::BrandNamePrescriptionDrugNonFormulary => write!(f, "Brand Name Prescription Drug - Non-Formulary"),
            Self::IndependentMedicalEvaluation => write!(f, "Independent Medical Evaluation"),
            Self::PartialHospitalizationLeftParenthesisPsychiatricRightParenthesis => write!(f, "Partial Hospitalization (Psychiatric)"),
            Self::DayCareLeftParenthesisPsychiatricRightParenthesis => write!(f, "Day Care (Psychiatric)"),
            Self::CognitiveTherapy => write!(f, "Cognitive Therapy"),
            Self::MassageTherapy => write!(f, "Massage Therapy"),
            Self::PulmonaryRehabilitation => write!(f, "Pulmonary Rehabilitation"),
            Self::CardiacRehabilitation => write!(f, "Cardiac Rehabilitation"),
            Self::Pediatric => write!(f, "Pediatric"),
            Self::Nursery => write!(f, "Nursery"),
            Self::Skin => write!(f, "Skin"),
            Self::Orthopedic => write!(f, "Orthopedic"),
            Self::Cardiac => write!(f, "Cardiac"),
            Self::Lymphatic => write!(f, "Lymphatic"),
            Self::Gastrointestinal => write!(f, "Gastrointestinal"),
            Self::Endocrine => write!(f, "Endocrine"),
            Self::Neurology => write!(f, "Neurology"),
            Self::Eye => write!(f, "Eye"),
            Self::InvasiveProcedures => write!(f, "Invasive Procedures"),
            Self::Gynecological => write!(f, "Gynecological"),
            Self::Obstetrical => write!(f, "Obstetrical"),
            Self::ObstetricalSlashGynecological => write!(f, "Obstetrical/Gynecological"),
            Self::MailOrderPrescriptionDrugFormulary => write!(f, "Mail Order Prescription Drug - Formulary"),
            Self::MailOrderPrescriptionDrugNonFormulary => write!(f, "Mail Order Prescription Drug - Non-Formulary"),
            Self::PhysicianVisitOfficeColonSick => write!(f, "Physician Visit - Office: Sick"),
            Self::PhysicianVisitOfficeColonWell => write!(f, "Physician Visit - Office: Well"),
            Self::CoronaryCare => write!(f, "Coronary Care"),
            Self::PrivateDutyNursingInpatient => write!(f, "Private Duty Nursing - Inpatient"),
            Self::PrivateDutyNursingHome => write!(f, "Private Duty Nursing - Home"),
            Self::SurgicalBenefitsProfessionalLeftParenthesisPhysicianRightParenthesis => write!(f, "Surgical Benefits - Professional (Physician)"),
            Self::SurgicalBenefitsFacility => write!(f, "Surgical Benefits - Facility"),
            Self::MentalHealthProviderInpatient => write!(f, "Mental Health Provider - Inpatient"),
            Self::MentalHealthProviderOutpatient => write!(f, "Mental Health Provider - Outpatient"),
            Self::MentalHealthFacilityInpatient => write!(f, "Mental Health Facility - Inpatient"),
            Self::MentalHealthFacilityOutpatient => write!(f, "Mental Health Facility - Outpatient"),
            Self::SubstanceAbuseFacilityInpatient => write!(f, "Substance Abuse Facility - Inpatient"),
            Self::SubstanceAbuseFacilityOutpatient => write!(f, "Substance Abuse Facility - Outpatient"),
            Self::ScreeningXRay => write!(f, "Screening X-ray"),
            Self::ScreeningLaboratory => write!(f, "Screening laboratory"),
            Self::MammogramCommaHighRiskPatient => write!(f, "Mammogram, High Risk Patient"),
            Self::MammogramCommaLowRiskPatient => write!(f, "Mammogram, Low Risk Patient"),
            Self::FluVaccination => write!(f, "Flu Vaccination"),
            Self::EyewearAndEyewearAccessories => write!(f, "Eyewear and Eyewear Accessories"),
            Self::CaseManagement => write!(f, "Case Management"),
            Self::Dermatology => write!(f, "Dermatology"),
            Self::DurableMedicalEquipment => write!(f, "Durable Medical Equipment"),
            Self::DiabeticSupplies => write!(f, "Diabetic Supplies"),
            Self::GenericPrescriptionDrugFormulary => write!(f, "Generic Prescription Drug - Formulary"),
            Self::GenericPrescriptionDrugNonFormulary => write!(f, "Generic Prescription Drug - Non-Formulary"),
            Self::Allergy => write!(f, "Allergy"),
            Self::IntensiveCare => write!(f, "Intensive Care"),
            Self::MentalHealth => write!(f, "Mental Health"),
            Self::NeonatolIntensiveCare => write!(f, "Neonatol Intensive Care"),
            Self::Oncology => write!(f, "Oncology"),
            Self::PhysicalTherapy => write!(f, "Physical Therapy"),
            Self::Pulmonary => write!(f, "Pulmonary"),
            Self::Renal => write!(f, "Renal"),
            Self::ResidentialPsychiatricTreatment => write!(f, "Residential Psychiatric Treatment"),
            Self::TransitionalCare => write!(f, "Transitional Care"),
            Self::TransitionalNurseryCare => write!(f, "Transitional Nursery Care"),
            Self::UrgentCare => write!(f, "Urgent Care"),
            Self::FluorideTreatments => write!(f, "Fluoride Treatments"),
            Self::DentalProphylaxis => write!(f, "Dental Prophylaxis"),
            Self::SpaceMaintenance => write!(f, "Space Maintenance"),
            Self::HarmfulHabitsAppliance => write!(f, "Harmful Habits Appliance"),
            Self::Composites => write!(f, "Composites"),
            Self::BitewingXRays => write!(f, "Bitewing X-Rays"),
            Self::RootCanalSlashRetreatment => write!(f, "Root Canal / Retreatment"),
            Self::OralEvaluation => write!(f, "Oral Evaluation"),
            Self::FullMouthSlashPanoramicXRays => write!(f, "Full Mouth / Panoramic X-Rays"),
            Self::PeriapicalXRay => write!(f, "Periapical X-Ray"),
            Self::SurgicalPeriodontics => write!(f, "Surgical Periodontics"),
            Self::DentureAdjustCommaRebaseCommaRelineCommaRepair => write!(f, "Denture Adjust, Rebase, Reline, Repair"),
            Self::InlaySlashOnlay => write!(f, "Inlay / Onlay"),
            Self::BuildupsSlashPostAndCore => write!(f, "Buildups / Post and Core"),
            Self::AmalgamSlashCompositeRestorations => write!(f, "Amalgam / Composite Restorations"),
            Self::Dentures => write!(f, "Dentures"),
            Self::DenturesRepair => write!(f, "Dentures - Repair"),
            Self::DenturesReline => write!(f, "Dentures - Reline"),
            Self::StainlessSteelCommaResinCommaAcrylicCrowns => write!(f, "Stainless Steel, Resin, Acrylic Crowns"),
            Self::Sealants => write!(f, "Sealants"),
            Self::DentalImplants => write!(f, "Dental Implants"),
            Self::TemporomandibularJointDysfunction => write!(f, "Temporomandibular Joint Dysfunction"),
            Self::SpecialtyPharmacy => write!(f, "Specialty Pharmacy"),
            Self::DurableMedicalEquipmentNew => write!(f, "Durable Medical Equipment (New)"),
            Self::DiagnosticImaging => write!(f, "Diagnostic Imaging"),
            Self::FertilityPreservation => write!(f, "Fertility Preservation"),
            Self::AppliedBehavioralAnalysisTherapy => write!(f, "Applied Behavioral Analysis Therapy"),
            Self::NonMedicalEquipment => write!(f, "Non-Medical Equipment"),
            Self::PsychiatricEmergency => write!(f, "Psychiatric Emergency"),
            Self::StepDownUnit => write!(f, "Step Down Unit"),
            Self::SkilledNursingFacilityHeadLevel => write!(f, "Skilled Nursing Facility Head Level"),
            Self::SkilledNursingFacilityVentilatorLevel => write!(f, "Skilled Nursing Facility Ventilator Level"),
            Self::LevelOfCare1 => write!(f, "Level of Care 1"),
            Self::LevelOfCare2 => write!(f, "Level of Care 2"),
            Self::LevelOfCare3 => write!(f, "Level of Care 3"),
            Self::LevelOfCare4 => write!(f, "Level of Care 4"),
            Self::Radiographs => write!(f, "Radiographs"),
            Self::FixedProsthodontics => write!(f, "Fixed Prosthodontics"),
            Self::RemovableProsthodontics => write!(f, "Removable Prosthodontics"),
            Self::IntraoralImagesCompleteSeries => write!(f, "Intraoral Images - Complete Series"),
            Self::OralEvaluationE17 => write!(f, "Oral Evaluation (E17)"),
            Self::DentalProphylaxisE18 => write!(f, "Dental Prophylaxis (E18)"),
            Self::PanoramicImages => write!(f, "Panoramic Images"),
            Self::SealantsE20 => write!(f, "Sealants (E20)"),
            Self::FluorideTreatmentsE21 => write!(f, "Fluoride Treatments (E21)"),
            Self::DentalImplantsE22 => write!(f, "Dental Implants (E22)"),
            Self::TemporomandibularJointDysfunctionE23 => write!(f, "Temporomandibular Joint Dysfunction (E23)"),
            Self::LongTermCarePharmacy => write!(f, "Long Term Care Pharmacy"),
            Self::ComprehensiveMedicationTherapyManagement => write!(f, "Comprehensive Medication Therapy Management"),
            Self::TargetedMedicationTherapyManagement => write!(f, "Targeted Medication Therapy Management"),
            Self::DietaryNutritionalServices => write!(f, "Dietary/Nutritional Services"),
            Self::IntensiveCardiacRehabilitation => write!(f, "Intensive Cardiac Rehabilitation"),
            Self::ConvenienceCare => write!(f, "Convenience Care"),
            Self::Telemedicine => write!(f, "Telemedicine"),
            Self::PharmacistServices => write!(f, "Pharmacist Services"),
            Self::DiabeticEducation => write!(f, "Diabetic Education"),
            Self::EarlyIntervention => write!(f, "Early Intervention"),
            Self::MedicallyTailoredMeals => write!(f, "Medically Tailored Meals"),
            Self::SeriousMentalHealth => write!(f, "Serious Mental Health"),
            Self::RemotePatientMonitoring => write!(f, "Remote Patient Monitoring"),
            Self::RemoteTherapeuticMonitoring => write!(f, "Remote Therapeutic Monitoring"),
            Self::SpecialSupplementalBenefits => write!(f, "Special Supplemental Benefits"),
            Self::PositronEmissionTomographyPetScan => write!(f, "Positron Emission Tomography (PET) Scan"),
            Self::CatScan => write!(f, "CAT Scan"),
            Self::IvTherapy => write!(f, "IV Therapy"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
