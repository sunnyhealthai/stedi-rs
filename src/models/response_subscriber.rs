use serde::{Deserialize, Serialize};

/// ResponseSubscriber : Information about the primary policyholder for the insurance plan listed in the original eligibility check request. The response will always include either the subscriber's name or member ID for identification, but most payers will also return the subscriber's date of birth and other identifying information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseSubscriber {
    /// AAA Request Validation segments that specify the reasons for rejection and any recommended follow-up actions when a payer rejects the eligibility check. These errors can be present at the subscriber level and are also reported in the top-level `errors` array in the eligibility check response. Common causes include missing or incorrect information for the subscriber, dependent, provider, or payer. Visit [Eligibility troubleshooting](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#payer-aaa-errors) for more information.
    #[serde(rename = "aaaErrors", skip_serializing_if = "Option::is_none")]
    pub aaa_errors: Option<Vec<super::EligibilityCheckSubscriberError>>,
    /// The subscriber's address information.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<super::Address>,
    /// The number assigned to each family member born with the same birth date, such as twins or triplets. Indicates the birth order when there are multiple births associated with the provided birth date.
    #[serde(
        rename = "birthSequenceNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub birth_sequence_number: Option<String>,
    /// The member's date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The military service date.
    #[serde(rename = "dateTimePeriod", skip_serializing_if = "Option::is_none")]
    pub date_time_period: Option<String>,
    /// The format of the military service date and time period. Can be `D8` - Date or `RD8` - Range of Dates.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "dateTimePeriodFormatQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_time_period_format_qualifier: Option<super::DateTimePeriodFormatQualifier>,
    /// Context that identifies the exact military unit. Used to report military service data.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The member's employment status code, used to report military service data. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#employment-status-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "employmentStatusCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub employment_status_code: Option<super::EmploymentStatusCode>,
    /// The military service end date.
    #[serde(rename = "endDateTimePeriod", skip_serializing_if = "Option::is_none")]
    pub end_date_time_period: Option<String>,
    /// The entity identifier that specifies the role of the subscriber, such as "Insured or Subscriber" or "Patient".
    #[serde(rename = "entityIdentifier", skip_serializing_if = "Option::is_none")]
    pub entity_identifier: Option<super::ResponseSubscriberEntityIdentifier>,
    /// The entity type for the member. It can technically be set to `Person` or `Non-Person Entity`. In practice, our customers only receive `Person`.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<super::EntityTypeQualifier>,
    /// The member's first name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Code indicating the patient's gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<super::GenderWithUnknown>,
    /// The member's government service affiliation code, used to report military service data. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#government-service-affiliation-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "governmentServiceAffiliationCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub government_service_affiliation_code: Option<super::GovernmentServiceAffiliationCode>,
    /// Group name
    #[serde(rename = "groupDescription", skip_serializing_if = "Option::is_none")]
    pub group_description: Option<String>,
    /// The group number associated with the insurance policy.
    #[serde(rename = "groupNumber", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    /// Diagnosis codes associated with the subscriber. These codes may be required by some payers for eligibility checks and are used to determine coverage for specific conditions or procedures.
    #[serde(
        rename = "healthCareDiagnosisCodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub health_care_diagnosis_codes: Option<Vec<super::HealthCareDiagnosisCode>>,
    /// The status of the member's information, used to report military service data. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#information-status-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "informationStatusCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub information_status_code: Option<super::InformationStatusCode>,
    /// Indicator that specifies whether the subscriber is the insured party or a dependent on the insurance policy.
    #[serde(rename = "insuredIndicator", skip_serializing_if = "Option::is_none")]
    pub insured_indicator: Option<super::SubscriberInsuredIndicator>,
    /// The member's last name.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The reason code for maintenance of the subscriber's eligibility information. This code indicates why the subscriber's information was updated or maintained in the payer's system.
    #[serde(
        rename = "maintenanceReasonCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_reason_code: Option<super::MaintenanceReasonCode>,
    /// The type code for maintenance of the subscriber's eligibility information. This code indicates the type of maintenance action performed on the subscriber's record.
    #[serde(
        rename = "maintenanceTypeCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_type_code: Option<super::MaintenanceTypeCode>,
    /// The member ID for the insurance policy.
    #[serde(rename = "memberId", skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The member's middle name or initial.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The member's military service rank code. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#military-service-rank-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "militaryServiceRankCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub military_service_rank_code: Option<super::MilitaryServiceRankCode>,
    /// Plan name
    #[serde(rename = "planDescription", skip_serializing_if = "Option::is_none")]
    pub plan_description: Option<String>,
    /// Plan network name
    #[serde(
        rename = "planNetworkDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_network_description: Option<String>,
    /// The network identification number associated with the insurance policy.
    #[serde(
        rename = "planNetworkIdNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_network_id_number: Option<String>,
    /// The plan number associated with the insurance policy.
    #[serde(rename = "planNumber", skip_serializing_if = "Option::is_none")]
    pub plan_number: Option<String>,
    /// The relationship of the subscriber to themselves (typically "Self") or, if this is a dependent record, the relationship of the dependent to the primary subscriber. Common values include "Self", "Spouse", "Child", or "Other Adult".
    #[serde(
        rename = "relationToSubscriber",
        skip_serializing_if = "Option::is_none"
    )]
    pub relation_to_subscriber: Option<super::SubscriberRelationship>,
    /// The coded relationship of the subscriber to themselves (typically "18" - Self) or, if this is a dependent record, the relationship code of the dependent to the primary subscriber. Common codes include "01" - Spouse, "19" - Child, "34" - Other Adult.
    #[serde(
        rename = "relationToSubscriberCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub relation_to_subscriber_code: Option<super::SubscriberRelationshipCode>,
    /// Provider information returned in the eligibility check response. This may include details about the provider who requested the eligibility check or the primary care provider associated with the subscriber's plan.
    #[serde(rename = "responseProvider", skip_serializing_if = "Option::is_none")]
    pub response_provider: Option<super::ResponseProvider>,
    /// The member's Social Security Number (SSN).
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The military service start date.
    #[serde(
        rename = "startDateTimePeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_time_period: Option<String>,
    /// The name suffix, such as Jr., Sr., or III.
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// The member's unique health identifier.
    #[serde(
        rename = "uniqueHealthIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_health_identifier: Option<String>,
}

impl ResponseSubscriber {
    /// Information about the primary policyholder for the insurance plan listed in the original eligibility check request. The response will always include either the subscriber's name or member ID for identification, but most payers will also return the subscriber's date of birth and other identifying information.
    pub fn new() -> ResponseSubscriber {
        ResponseSubscriber {
            aaa_errors: None,
            address: None,
            birth_sequence_number: None,
            date_of_birth: None,
            date_time_period: None,
            date_time_period_format_qualifier: None,
            description: None,
            employment_status_code: None,
            end_date_time_period: None,
            entity_identifier: None,
            entity_type: None,
            first_name: None,
            gender: None,
            government_service_affiliation_code: None,
            group_description: None,
            group_number: None,
            health_care_diagnosis_codes: None,
            information_status_code: None,
            insured_indicator: None,
            last_name: None,
            maintenance_reason_code: None,
            maintenance_type_code: None,
            member_id: None,
            middle_name: None,
            military_service_rank_code: None,
            plan_description: None,
            plan_network_description: None,
            plan_network_id_number: None,
            plan_number: None,
            relation_to_subscriber: None,
            relation_to_subscriber_code: None,
            response_provider: None,
            ssn: None,
            start_date_time_period: None,
            suffix: None,
            unique_health_identifier: None,
        }
    }
}
