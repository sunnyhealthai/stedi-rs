//! Benefits information module for healthcare eligibility responses.
//!
//! This module contains the `BenefitsInformation` struct which represents detailed
//! insurance benefit data returned from eligibility checks. It includes information
//! about coverage status, patient financial responsibility (co-pays, deductibles,
//! coinsurance), benefit limits, and other coverage details.
//!
//! The benefits information helps healthcare providers understand what services
//! are covered for a patient, how much the patient will need to pay out of pocket,
//! and any restrictions or requirements that apply to the benefits.

use serde::{Deserialize, Serialize};

/// Benefits information returned in eligibility check responses.
///
/// This struct contains detailed information about a patient's insurance benefits,
/// including coverage status, cost-sharing requirements (co-pays, deductibles, coinsurance),
/// benefit limits, and other coverage details.
///
/// ## Key Benefit Types
///
/// The `code` field indicates the type of benefit information:
/// - **Active Coverage (1)**: Patient has active insurance coverage
/// - **Co-Insurance (A)**: Patient's percentage of costs (e.g., 20% coinsurance)
/// - **Co-Payment (B)**: Fixed amount patient pays (e.g., $25 copay)
/// - **Deductible (C)**: Amount patient must pay before insurance covers costs
/// - **Out of Pocket Stop Loss (G)**: Maximum amount patient pays in a period
///
/// ## Financial Responsibility
///
/// - `benefit_amount`: Monetary amounts (co-pays, deductibles, out-of-pocket maximums)
/// - `benefit_percent`: Percentage amounts (coinsurance rates)  
/// - `benefit_quantity`: Quantity limits (number of visits, days covered, etc.)
///
/// ## Coverage Levels
///
/// Benefits can apply to:
/// - **Individual (IND)**: Just the patient
/// - **Family (FAM)**: Patient's entire family
///
/// ## Network Status
///
/// The `in_plan_network_indicator` shows whether benefits apply to:
/// - **In-network providers**: Lower patient costs
/// - **Out-of-network providers**: Higher patient costs
///
/// See [Determine Patient Benefits](https://www.stedi.com/docs/healthcare/eligibility-active-coverage-benefits)
/// for detailed guidance on interpreting benefit information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BenefitsInformation {
    /// A free-form message containing additional information about the benefits in the response.
    #[serde(
        rename = "additionalInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_information: Option<Vec<super::AdditionalInformation>>,
    /// Code indicating whether the benefit is subject to prior authorization or certification. Can be `Y` - Yes, `N` - No, or `U` - Unknown.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "authOrCertIndicator",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_or_cert_indicator: Option<super::AuthOrCertIndicator>,
    /// The monetary benefit amount, such as a patient's co-pay or deductible. This value is expressed as a decimal, such as 100.00.     The payer will always send a value in this property when the `benefitsInformation.code` = `B` - Co-Payment, `C` - Deductible, `G` - Out of Pocket (Stop Loss), `J` - Cost Containment, or `Y` - Spend Down. For those codes, this value represents the patient's portion of responsibility.     The payer will **never** send this value when `benefitsInformation.code` = `A` - Co-Insurance. This property can contain zero when the patient has no responsibility.     Learn more about [patient costs](https://www.stedi.com/docs/healthcare/eligibility-patient-responsibility-benefits).
    #[serde(rename = "benefitAmount", skip_serializing_if = "Option::is_none")]
    pub benefit_amount: Option<String>,
    /// The percentage of the benefit, such as co-insurance. This property can contain zero when the patient has no responsibility.     The payer will always send a value in this property when `benefitsInformation.code` = `A` - Co-Insurance. For this code, this value represents the patient's portion of the responsibility. The percentage is expressed as a decimal, such as `0.80` represents 80%.     The payer will **never** send a value in this property when `benefitsInformation.code` = `B` - Co-Payment, `C` - Deductible, `G` - Out of Pocket (Stop Loss), `J` - Cost Containment, or `Y` - Spend Down.     Learn more about [patient costs](https://www.stedi.com/docs/healthcare/eligibility-patient-responsibility-benefits).
    #[serde(rename = "benefitPercent", skip_serializing_if = "Option::is_none")]
    pub benefit_percent: Option<String>,
    /// The quantity of the benefit, qualified by the type specified in `quantityQualifier`. For example, `10` when the `quantityQualifier` is `Visits`.
    #[serde(rename = "benefitQuantity", skip_serializing_if = "Option::is_none")]
    pub benefit_quantity: Option<String>,
    /// Identifying information specific to this type of benefit. Contains supplementary data about healthcare benefits that may not fit into standard benefit information fields, such as group numbers, plan descriptions, member IDs, and insurance policy numbers. For pharmacy benefits, this may include formulary numbers, coverage list IDs, and alternative list IDs. For Medicare beneficiaries, this may include the Health Insurance Claim Number (HICN) or Medicare Beneficiary Identifier (MBI). This information can also include plan network descriptions, prior authorization numbers, and referral numbers.
    #[serde(
        rename = "benefitsAdditionalInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub benefits_additional_information: Option<super::BenefitsAdditionalInformation>,
    /// Date information that determines the patient's eligibility for a specific type of benefits. When present, these dates override the dates in the `planDateInformation` object for this specific benefit type. Payers send benefit-specific dates when certain benefits within a plan have different activation rules or waiting periods than the overall plan coverage. Common scenarios include: benefits that activate based on employment duration or role, benefits with waiting periods (such as dental insurance requiring a 6-month wait for major services), Medicare plans with different effective dates for Parts A, B, and D, and plan changes during open enrollment. Key fields include `benefitBegin`, `benefitEnd`, `latestVisitOrConsultation` (for frequency-limited services like dental cleanings), and various eligibility dates. Visit [Benefit-specific eligibility dates](https://www.stedi.com/docs/healthcare/eligibility-active-coverage-benefits#benefit-specific-eligibility-dates) for more details.
    #[serde(
        rename = "benefitsDateInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub benefits_date_information: Option<super::BenefitsDateInformation>,
    /// Other entities associated with the eligibility or benefits. This could be a provider, an individual, an organization, or another payer. When present, this array typically contains information about the patient's primary care provider (PCP), another organization that handles a specific benefit type (such as telehealth mental health services), or another health plan for the patient (coordination of benefits scenarios). - This is where information for a crossover carrier such as Medicaid or Medicare is provided, if it's applicable to the patient and the payer supports it. - For Blue Cross Blue Shield (BCBS) payers, Stedi returns an entry containing information about the patient's home plan - the plan that actually verified the coverage. In this object, the `entityIdentifier` property is set to `Party Performing Verification`. [Learn more](https://www.stedi.com/docs/healthcare/eligibility-active-coverage-benefits#bcbs-home-plan)
    #[serde(
        rename = "benefitsRelatedEntities",
        skip_serializing_if = "Option::is_none"
    )]
    pub benefits_related_entities: Option<Vec<super::BenefitsRelatedEntity>>,
    /// Please use `benefitsInformation.benefitsRelatedEntities` instead.
    #[serde(
        rename = "benefitsRelatedEntity",
        skip_serializing_if = "Option::is_none"
    )]
    pub benefits_related_entity: Option<super::BenefitsRelatedEntity>,
    /// Indicates how often a service is allowed, such as "once every 6 months" or "twice per year". This field is commonly used for dental, vision, and Medicaid benefits, as well as some medical services like annual wellness visits or therapy sessions. Many payers don't populate this field and instead return this information as free text in `additionalInformation.description`. When present, it contains quantity limits (e.g., number of visits), time period qualifiers (e.g., "Calendar Year", "Month"), and delivery patterns. This information helps determine if a patient has already reached their allowed frequency for a service, which may affect coverage. Visit [Service history](https://www.stedi.com/docs/healthcare/eligibility-patient-responsibility-benefits#service-history) for more details.
    #[serde(
        rename = "benefitsServiceDelivery",
        skip_serializing_if = "Option::is_none"
    )]
    pub benefits_service_delivery: Option<Vec<super::BenefitsServiceDelivery>>,
    /// Benefit type code indicating the type of benefit information described in this object. Common codes include: `1` - Active Coverage, `A` - Co-Insurance, `B` - Co-Payment, `C` - Deductible, `D` - Benefit Description, `F` - Limitations, `G` - Out of Pocket (Stop Loss), `J` - Cost Containment, `U` - Contact Following Entity for Eligibility or Benefit Information, `V` - Cannot Process, and `Y` - Spend Down. The code determines which other fields in this object are populated (e.g., `benefitAmount` for co-payments vs. `benefitPercent` for co-insurance). See the struct-level documentation for a complete list of benefit type codes and their meanings.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<super::BenefitsInformationCode>,
    /// Identifies relevant medical procedures by their standard codes and modifiers. Used to specify which procedure codes (CPT, HCPCS, ADA dental codes, etc.) the benefit applies to. This allows payers to return benefit information specific to particular procedures rather than general service types. However, many payers do not support procedure code-specific eligibility checks and will return generic benefits for the service type code instead. When present, this field includes the procedure code, procedure modifiers, product/service ID qualifier (indicating the code set used), and optionally a range of procedure codes. This is commonly seen in dental eligibility responses where benefits vary by specific procedure codes.
    #[serde(
        rename = "compositeMedicalProcedureIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub composite_medical_procedure_identifier: Option<super::CompositeMedicalProcedureIdentifier>,
    /// The name of the coverage level indicating whether benefits apply to just the patient or the entire family. Common values include `Individual` (benefits apply only to the patient) and `Family` (benefits apply to the patient's entire family). This affects how benefit amounts, deductibles, and out-of-pocket maximums are calculated and applied. For example, a family deductible is typically higher than an individual deductible.
    #[serde(rename = "coverageLevel", skip_serializing_if = "Option::is_none")]
    pub coverage_level: Option<super::BenefitsInformationCoverageLevelName>,
    /// Code indicating the coverage level. Common values include `IND` - Individual (benefits apply only to the patient) and `FAM` - Family (benefits apply to the patient's entire family). This code determines how benefit amounts, deductibles, and out-of-pocket maximums are calculated. When not specified, benefits typically default to individual coverage. The coverage level affects cost-sharing calculations, as family plans often have higher deductibles and out-of-pocket maximums than individual plans.
    #[serde(rename = "coverageLevelCode", skip_serializing_if = "Option::is_none")]
    pub coverage_level_code: Option<super::BenefitsInformationCoverageLevelCode>,
    /// Please use `benefitsInformation.eligibilityAdditionalInformationList` instead.
    #[serde(
        rename = "eligibilityAdditionalInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub eligibility_additional_information: Option<super::EligibilityAdditionalInformation>,
    /// Used when there are multiple Nature of Injury Codes or a Facility Type Codes included in the response.
    #[serde(
        rename = "eligibilityAdditionalInformationList",
        skip_serializing_if = "Option::is_none"
    )]
    pub eligibility_additional_information_list:
        Option<Vec<super::EligibilityAdditionalInformation>>,
    /// The loop header identifier number in the `LS` segment of the original X12 EDI transaction.
    #[serde(
        rename = "headerLoopIdentifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub header_loop_identifier_code: Option<String>,
    /// The name indicating whether benefits apply to in-network or out-of-network providers. Common values include `Yes` (benefits apply to in-network providers), `No` (benefits apply to out-of-network providers), and `Not Applicable` (network status doesn't apply to this benefit type). In-network providers typically have lower patient costs (lower co-pays, deductibles, and coinsurance rates) compared to out-of-network providers. Note that this indicator doesn't tell you whether the provider requesting the eligibility check is in-network for the health plan - you must verify network status separately. Visit [Provider network status, authorizations, referrals](https://www.stedi.com/docs/healthcare/eligibility-network-status-authorization-referrals) for more details.
    #[serde(
        rename = "inPlanNetworkIndicator",
        skip_serializing_if = "Option::is_none"
    )]
    pub in_plan_network_indicator: Option<super::InPlanNetworkIndicatorName>,
    /// Code indicating whether benefits apply to in-network or out-of-network providers. Common values include `Y` - Yes (in-network), `N` - No (out-of-network), and `W` - Not Applicable. This code determines patient cost-sharing amounts, as in-network providers typically have lower co-pays, deductibles, and coinsurance rates. The presence of this code doesn't indicate whether the provider requesting the eligibility check is in-network - network verification must be done separately. Benefits may be returned separately for in-network and out-of-network scenarios, allowing you to compare costs.
    #[serde(
        rename = "inPlanNetworkIndicatorCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub in_plan_network_indicator_code: Option<super::InPlanNetworkIndicatorCode>,
    /// The name of the insurance type identifying the category of insurance policy. Common values include `Commercial`, `Medicare Part A`, `Medicare Part B`, `Medicaid`, `TRICARE`, and others. This field helps identify the type of health plan the patient has, which can affect benefit structures, coverage rules, and regulatory requirements. For example, Medicare Advantage plans may be identified by the presence of `Medicare Part A` and `Medicare Part B` insurance types. The corresponding `insuranceTypeCode` provides the standardized code for programmatic use.
    #[serde(rename = "insuranceType", skip_serializing_if = "Option::is_none")]
    pub insurance_type: Option<super::InsuranceTypeName>,
    /// Code identifying the type of insurance policy. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#insurance-type-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "insuranceTypeCode", skip_serializing_if = "Option::is_none")]
    pub insurance_type_code: Option<super::InsuranceTypeCode>,
    /// The human-readable name corresponding to the benefit type code in the `code` field. This provides a descriptive label for the type of benefit information, such as `Active Coverage`, `Co-Payment`, `Deductible`, `Co-Insurance`, `Out of Pocket (Stop Loss)`, `Limitations`, `Benefit Description`, `Inactive`, or `Contact Following Entity for Eligibility or Benefit Information`. This field makes it easier to understand what type of benefit information is contained in this object without needing to look up the code value. The name always corresponds to the `code` field value.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<super::BenefitsInformationName>,
    /// The specific product name or special program name for an insurance plan. For example `Gold 1-2-3`.  Payers are normally required to send the plan name when `benefitsInformation.code` is set to values `1` - `8` and the `benefitsInformation.serviceTypeCodes` contains `30` (Health Benefit Plan Coverage). However, behavior may vary by payer, so don't rely on this information being present in the response. Note that the plan name returned in this property may not exactly match the name the payer uses in official plan documents or marketing literature.  Visit [What's the plan name?](https://www.stedi.com/docs/healthcare/eligibility-active-coverage-benefits#what’s-the-plan-name%3F) in the benefits response documentation for more details.
    #[serde(rename = "planCoverage", skip_serializing_if = "Option::is_none")]
    pub plan_coverage: Option<String>,
    /// The name of the `quantityQualifierCode`.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "quantityQualifier", skip_serializing_if = "Option::is_none")]
    pub quantity_qualifier: Option<super::QuantityQualifierName>,
    /// Code indicating the type of quantity for the benefit. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#quantity-qualifier-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "quantityQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_qualifier_code: Option<super::QuantityQualifierCode>,
    /// An array of [Service Type Codes](https://www.stedi.com/docs/healthcare/send-eligibility-checks#service-type-codes) related to the benefit type.
    #[serde(rename = "serviceTypeCodes", skip_serializing_if = "Option::is_none")]
    pub service_type_codes: Option<Vec<super::ResponseEligibilityServiceTypeCode>>,
    /// The names of the service type codes listed in the `serviceTypeCodes` array.
    #[serde(rename = "serviceTypes", skip_serializing_if = "Option::is_none")]
    pub service_types: Option<Vec<super::ResponseEligibilityServiceType>>,
    /// The name of the `timeQualifierCode`.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "timeQualifier", skip_serializing_if = "Option::is_none")]
    pub time_qualifier: Option<super::TimeQualifierName>,
    /// Code indicating the time period for the benefit information. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#time-qualifier-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "timeQualifierCode", skip_serializing_if = "Option::is_none")]
    pub time_qualifier_code: Option<super::TimeQualifierCode>,
    /// The loop trailer identifier number in the `LE` segment of the original X12 EDI transaction.
    #[serde(
        rename = "trailerLoopIdentifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_loop_identifier_code: Option<String>,
}

impl BenefitsInformation {
    /// Creates a new instance of `BenefitsInformation` with all fields initialized to `None`.
    ///
    /// This constructor provides a clean starting point for building benefits information
    /// data structures, typically used when processing eligibility response data where
    /// specific benefit fields will be populated based on the received information.
    ///
    /// # Returns
    ///
    /// A new `BenefitsInformation` instance with all optional fields set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stedi_rs::models::BenefitsInformation;
    ///
    /// let benefits = BenefitsInformation::new();
    /// assert_eq!(benefits.benefit_amount, None);
    /// assert_eq!(benefits.benefit_percent, None);
    /// ```
    pub fn new() -> BenefitsInformation {
        BenefitsInformation {
            additional_information: None,
            auth_or_cert_indicator: None,
            benefit_amount: None,
            benefit_percent: None,
            benefit_quantity: None,
            benefits_additional_information: None,
            benefits_date_information: None,
            benefits_related_entities: None,
            benefits_related_entity: None,
            benefits_service_delivery: None,
            code: None,
            composite_medical_procedure_identifier: None,
            coverage_level: None,
            coverage_level_code: None,
            eligibility_additional_information: None,
            eligibility_additional_information_list: None,
            header_loop_identifier_code: None,
            in_plan_network_indicator: None,
            in_plan_network_indicator_code: None,
            insurance_type: None,
            insurance_type_code: None,
            name: None,
            plan_coverage: None,
            quantity_qualifier: None,
            quantity_qualifier_code: None,
            service_type_codes: None,
            service_types: None,
            time_qualifier: None,
            time_qualifier_code: None,
            trailer_loop_identifier_code: None,
        }
    }
}
