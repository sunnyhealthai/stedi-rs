//! Healthcare eligibility data models and structures.
//!
//! This module contains all the data structures, enums, and types used for healthcare
//! eligibility transactions. These models represent the request and response payloads
//! for the Stedi Healthcare Eligibility API, which implements the X12 HIPAA 270/271
//! transaction standards.
//!
//! ## Module Organization
//!
//! The models are organized into several categories:
//!
//! ### Core Transaction Structures
//! - **Request Models**: `EligibilityCheckRequestContent`, `RequestSubscriber`, `RequestDependent`
//! - **Response Models**: `EligibilityCheckResponseContent`, `ResponseSubscriber`, `ResponseDependent`
//! - **Metadata**: `EligibilityMetaDataJson` for transaction tracking
//!
//! ### Entity Models
//! - **Providers**: `Provider`, `ResponseProvider`, `ProviderInformation`
//! - **Subscribers**: `RequestSubscriber`, `ResponseSubscriber`
//! - **Dependents**: `RequestDependent`, `ResponseDependent`
//! - **Payers**: `Payer` with identification and contact information
//!
//! ### Benefits Information
//! - **Benefits**: `BenefitsInformation`, `BenefitsServiceDelivery`, `BenefitsDateInformation`
//! - **Coverage**: `BenefitsInformationCode`, `BenefitsInformationCoverageLevelCode`
//! - **Related Entities**: `BenefitsRelatedEntity` for PBMs, carveouts, etc.
//!
//! ### Code Lists and Enumerations
//! - **Insurance Types**: `InsuranceTypeCode`, `InsuranceTypeName`
//! - **Service Types**: `RequestEligibilityServiceTypeCode`, `ResponseEligibilityServiceTypeCode`
//! - **Entity Types**: `EntityTypeQualifier`, `ProviderType`
//! - **Relationships**: `DependentRelationshipCode`, `SubscriberRelationshipCode`
//! - **Communication**: `CommunicationMode` for contact methods
//! - **Military**: `MilitaryServiceRankCode`, `GovernmentServiceAffiliationCode`
//!
//! ### Encounter and Procedure Information
//! - **Encounters**: `Encounter` for service context
//! - **Procedures**: `MedicalProcedure` with codes and modifiers
//! - **Diagnosis**: `HealthCareDiagnosisCode`, `DiagnosisTypeCode`
//!
//! ### Error Handling
//! - **Error Types**: `EligibilityCheckError`, `ValidationException`
//! - **Entity-Specific Errors**: `EligibilityCheckProviderError`, `EligibilityCheckSubscriberError`
//! - **HTTP Errors**: `AccessDeniedExceptionResponseContent`, `ResourceNotFoundExceptionResponseContent`
//!
//! ### Supporting Structures
//! - **Addresses**: `Address`, `RequestAddress`
//! - **Contact Information**: `ContactInformation`, `Contacts`
//! - **Dates**: `DtpDate`, `BenefitsDateInformation`, `PlanDateInformation`
//! - **Additional Information**: `AdditionalInformation`, `BenefitsAdditionalInformation`
//!
//! ## X12 HIPAA Mapping
//!
//! These models map to the X12 HIPAA 270/271 transaction standards:
//!
//! - **X12 270**: Healthcare Eligibility/Benefit Inquiry (request)
//! - **X12 271**: Healthcare Eligibility/Benefit Response (response)
//!
//! The structures correspond to segments, loops, and elements within these transactions,
//! providing a type-safe Rust representation of the X12 data structures.
//!
//! ## Usage Example
//!
//! ```rust
//! use stedi_rs::models::{
//!     EligibilityCheckRequestContent,
//!     RequestSubscriber,
//!     RequestProviderCode,
//! };
//!
//! // Create an eligibility check request
//! let request = EligibilityCheckRequestContent {
//!     control_number: "123456789".to_string(),
//!     trading_partner_service_id: "87726".to_string(),
//!     ..Default::default()
//! };
//! ```
//!
//! ## Stedi Notes
//!
//! These models are generated from Stedi's OpenAPI specification and are used by the
//! Stedi Healthcare Eligibility API. Some fields may contain non-standard values returned
//! by payers, as indicated in individual model documentation.

pub mod access_denied_exception_response_content;
/// Error response for access denied scenarios.
///
/// This module contains the structure for handling access denied exceptions
/// in the healthcare eligibility API. It represents HTTP 403 responses when
/// the requester lacks proper permissions.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards but follows Stedi's API error response patterns.
///
/// # Stedi Notes
/// Used internally by Stedi for access control violations.
pub use self::access_denied_exception_response_content::AccessDeniedExceptionResponseContent;

pub mod additional_identification_dependent;
/// Dependent additional identification information.
///
/// Contains structures for representing extra identification data for dependents
/// in healthcare eligibility requests. This supports supplementary ID fields beyond
/// the primary member ID.
///
/// # X12 HIPAA
/// Maps to segments and elements in the X12 270 transaction for dependent identification.
///
/// # Stedi Notes
/// Used to provide additional context about dependent identities in eligibility checks.
pub use self::additional_identification_dependent::AdditionalIdentificationDependent;

pub mod additional_identification_subscriber;
/// Subscriber additional identification information.
///
/// Contains structures for representing extra identification data for subscribers
/// in healthcare eligibility requests. This supports supplementary ID fields beyond
/// the primary member ID.
///
/// # X12 HIPAA
/// Maps to segments and elements in the X12 270 transaction for subscriber identification.
///
/// # Stedi Notes
/// Used to provide additional context about subscriber identities in eligibility checks.
pub use self::additional_identification_subscriber::AdditionalIdentificationSubscriber;

pub mod additional_information;
/// Generic additional information container.
///
/// Provides a flexible structure for including extra data elements in healthcare
/// eligibility transactions that don't fit into standard fields.
///
/// # X12 HIPAA
/// Corresponds to various informational segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used across multiple entity types to capture extended information.
pub use self::additional_information::AdditionalInformation;

pub mod address;
/// Standard address information structure.
///
/// Represents physical address details for healthcare entities in eligibility responses.
/// Includes fields for street address, city, state, and postal code.
///
/// # X12 HIPAA
/// Maps to N3/N4 segments in X12 271 responses for entity addresses.
///
/// # Stedi Notes
/// Normalized address format used in response entities.
pub use self::address::Address;

pub mod application_modes;
/// Application mode indicators.
///
/// Defines how applications or services are delivered under a healthcare plan,
/// such as in-network vs out-of-network coverage.
///
/// # X12 HIPAA
/// Related to benefit application patterns in X12 271 transactions.
///
/// # Stedi Notes
/// Referenced in benefits information to specify coverage application contexts.
pub use self::application_modes::ApplicationModes;

pub mod auth_or_cert_indicator;
/// Authorization or certification requirement indicators.
///
/// Specifies whether a healthcare service requires prior authorization or certification
/// before coverage can be determined or applied.
///
/// # X12 HIPAA
/// Maps to authorization/certification indicators in benefit information segments.
///
/// # Stedi Notes
/// Part of benefits service delivery specifications.
pub use self::auth_or_cert_indicator::AuthOrCertIndicator;

pub mod benefit_related_entity_identification;
/// Identification for benefit-related entities.
///
/// Contains identification details for entities associated with benefit information
/// such as plan administrators or third-party organizations.
///
/// # X12 HIPAA
/// Maps to entity identification segments in X12 271 benefit information.
///
/// # Stedi Notes
/// Used to link related entities to specific benefits.
pub use self::benefit_related_entity_identification::BenefitRelatedEntityIdentification;

pub mod benefits_additional_information;
/// Additional benefit information details.
///
/// Provides supplementary data about healthcare benefits that may not fit into
/// standard benefit information fields.
///
/// # X12 HIPAA
/// Corresponds to additional data segments in benefit information loops.
///
/// # Stedi Notes
/// Extends benefits information with custom data elements.
pub use self::benefits_additional_information::BenefitsAdditionalInformation;

pub mod benefits_date_information;
/// Date information for benefits.
///
/// Contains date-related fields for healthcare benefit coverage periods,
/// eligibility dates, and other temporal benefit information.
///
/// # X12 HIPAA
/// Maps to DTP segments in X12 271 benefit information.
///
/// # Stedi Notes
/// Used to specify effective dates, expiration dates, and other benefit timeframes.
pub use self::benefits_date_information::BenefitsDateInformation;

pub mod benefits_information;
/// Core benefits information structure.
///
/// Represents detailed healthcare coverage information including benefit types,
/// coverage levels, and service limitations. This is the primary container for
/// eligibility response benefit data.
///
/// # X12 HIPAA
/// Maps to EB loops in X12 271 transactions containing benefit details.
///
/// # Stedi Notes
/// Central to eligibility responses, containing all coverage and benefit data.
pub use self::benefits_information::BenefitsInformation;

pub mod benefits_information_code;
/// Benefit information type codes.
///
/// Enumerates the types of benefit information that can be returned in an
/// eligibility response, such as medical coverage, dental coverage, or co-pay
/// information.
///
/// # X12 HIPAA
/// Maps to EB01 elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::benefits_information_code::BenefitsInformationCode;

pub mod benefits_information_coverage_level_code;
/// Coverage level codes for benefits.
///
/// Specifies the coverage level associated with a benefit, such as individual,
/// family, or employee-only coverage.
///
/// # X12 HIPAA
/// Maps to EB06 elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::benefits_information_coverage_level_code::BenefitsInformationCoverageLevelCode;

pub mod benefits_information_coverage_level_name;
/// Human-readable coverage level names.
///
/// Provides descriptive names for coverage levels that correspond to the
/// coverage level codes.
///
/// # X12 HIPAA
/// Maps to EB06 elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::benefits_information_coverage_level_name::BenefitsInformationCoverageLevelName;

pub mod benefits_information_name;
/// Human-readable benefit information names.
///
/// Provides descriptive names for benefit information types that correspond to
/// the benefit information codes.
///
/// # X12 HIPAA
/// Maps to EB01 elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::benefits_information_name::BenefitsInformationName;

pub mod benefits_related_entity;
/// Entities related to benefits information.
///
/// Represents third-party entities associated with specific benefits such as
/// plan administrators or service providers.
///
/// # X12 HIPAA
/// Maps to related entity segments in X12 271 benefit information loops.
///
/// # Stedi Notes
/// Used to provide context about organizations involved in benefit delivery.
pub use self::benefits_related_entity::BenefitsRelatedEntity;

pub mod benefits_related_entity_relationship_code;
/// Relationship codes for benefit-related entities.
///
/// Defines the relationship between a benefit-related entity and the primary
/// entities in the eligibility transaction.
///
/// # X12 HIPAA
/// Maps to entity relationship codes in X12 271 benefit information.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::benefits_related_entity_relationship_code::BenefitsRelatedEntityRelationshipCode;

pub mod benefits_service_delivery;
/// Service delivery information for benefits.
///
/// Contains details about how healthcare services are delivered under a benefit,
/// including frequency limits, quantity limits, and authorization requirements.
///
/// # X12 HIPAA
/// Maps to benefit service delivery segments in X12 271 transactions.
///
/// # Stedi Notes
/// Provides structured information about service delivery patterns and limitations.
pub use self::benefits_service_delivery::BenefitsServiceDelivery;

pub mod benefits_service_delivery_quantity_qualifier;
/// Quantity qualifier for service delivery.
///
/// Specifies the type of quantity measurement used in service delivery
/// information, such as visits, days, or monetary amounts.
///
/// # X12 HIPAA
/// Maps to quantity qualifier elements in X12 271 benefit service delivery.
///
/// # Stedi Notes
/// Used with benefits service delivery to define measurement contexts.
pub use self::benefits_service_delivery_quantity_qualifier::BenefitsServiceDeliveryQuantityQualifier;

pub mod benefits_service_delivery_quantity_qualifier_code;
/// Codes for service delivery quantity qualifiers.
///
/// Enumerates standardized codes for quantity measurements in benefit service
/// delivery information.
///
/// # X12 HIPAA
/// Maps to quantity qualifier code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::benefits_service_delivery_quantity_qualifier_code::BenefitsServiceDeliveryQuantityQualifierCode;

pub mod code_list_qualifier_code;
/// Code list qualifier codes.
///
/// Specifies the source or type of code list used in healthcare transactions,
/// such as ICD-10, CPT, or HCPCS codes.
///
/// # X12 HIPAA
/// Maps to code list qualifier elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::code_list_qualifier_code::CodeListQualifierCode;

pub mod communication_mode;
/// Communication mode indicators.
///
/// Defines how communication with healthcare entities should be conducted,
/// such as email, phone, or fax.
///
/// # X12 HIPAA
/// Maps to communication number qualifier codes in X12 transactions.
///
/// # Stedi Notes
/// Used in contact information structures.
pub use self::communication_mode::CommunicationMode;

pub mod composite_medical_procedure_identifier;
/// Composite medical procedure identifiers.
///
/// Represents structured identifiers for medical procedures including code
/// qualifiers and procedure codes.
///
/// # X12 HIPAA
/// Maps to composite medical procedure identifier elements in X12 transactions.
///
/// # Stedi Notes
/// Used in encounter and medical procedure information.
pub use self::composite_medical_procedure_identifier::CompositeMedicalProcedureIdentifier;

pub mod contact_information;
/// Contact information for healthcare entities.
///
/// Contains communication details such as phone numbers, email addresses, and
/// contact methods for providers, subscribers, and other entities.
///
/// # X12 HIPAA
/// Maps to PER segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Provides contact methods for follow-up actions or entity communication.
pub use self::contact_information::ContactInformation;

pub mod contacts;
/// Container for multiple contact information entries.
///
/// Groups multiple contact information structures for a single entity.
///
/// # X12 HIPAA
/// Maps to multiple PER segments in X12 transactions.
///
/// # Stedi Notes
/// Used to provide various contact methods for healthcare entities.
pub use self::contacts::Contacts;

pub mod date_time_period_format_qualifier;
/// Date/time period format qualifiers.
///
/// Specifies the format of date or time period representations in healthcare
/// transactions, such as CCYYMMDD or MMDDCCYY.
///
/// # X12 HIPAA
/// Maps to date/time period format qualifier elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::date_time_period_format_qualifier::DateTimePeriodFormatQualifier;

pub mod delivery_or_calendar_pattern_qualifier;
/// Delivery or calendar pattern qualifiers.
///
/// Defines patterns for benefit service delivery such as frequency or calendar
/// based scheduling.
///
/// # X12 HIPAA
/// Maps to delivery pattern qualifier elements in X12 271 benefit information.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::delivery_or_calendar_pattern_qualifier::DeliveryOrCalendarPatternQualifier;

pub mod delivery_or_calendar_pattern_qualifier_code;
/// Codes for delivery or calendar pattern qualifiers.
///
/// Enumerates standardized codes for service delivery patterns.
///
/// # X12 HIPAA
/// Maps to delivery pattern qualifier code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::delivery_or_calendar_pattern_qualifier_code::DeliveryOrCalendarPatternQualifierCode;

pub mod delivery_pattern_time_qualifier;
/// Time qualifier for delivery patterns.
///
/// Specifies time-related qualifiers for benefit service delivery patterns.
///
/// # X12 HIPAA
/// Maps to time qualifier elements in X12 271 benefit delivery patterns.
///
/// # Stedi Notes
/// Used with delivery pattern specifications.
pub use self::delivery_pattern_time_qualifier::DeliveryPatternTimeQualifier;

pub mod delivery_pattern_time_qualifier_code;
/// Codes for delivery pattern time qualifiers.
///
/// Enumerates standardized codes for time qualifiers in service delivery.
///
/// # X12 HIPAA
/// Maps to time qualifier code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::delivery_pattern_time_qualifier_code::DeliveryPatternTimeQualifierCode;

pub mod dependent_insured_indicator;
/// Dependent insured status indicators.
///
/// Indicates whether a dependent is the insured party in a healthcare plan.
///
/// # X12 HIPAA
/// Maps to insured indicator elements for dependents in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::dependent_insured_indicator::DependentInsuredIndicator;

pub mod dependent_relationship;
/// Dependent relationship information.
///
/// Contains details about the relationship between a dependent and the
/// subscriber, including relationship codes and descriptions.
///
/// # X12 HIPAA
/// Maps to dependent relationship segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used to establish dependent-subscriber relationships in eligibility checks.
pub use self::dependent_relationship::DependentRelationship;

pub mod dependent_relationship_code;
/// Codes for dependent relationships.
///
/// Enumerates standardized codes for dependent relationships to subscribers.
///
/// # X12 HIPAA
/// Maps to relationship code elements for dependents in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::dependent_relationship_code::DependentRelationshipCode;

pub mod diagnosis_type_code;
/// Diagnosis type codes.
///
/// Specifies the type of diagnosis code being used, such as ICD-10 or ICD-9.
///
/// # X12 HIPAA
/// Maps to diagnosis type code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::diagnosis_type_code::DiagnosisTypeCode;

pub mod dtp_date;
/// Date information structure.
///
/// Represents standardized date information with format qualifiers and date
/// values for healthcare transactions.
///
/// # X12 HIPAA
/// Maps to DTP segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used throughout eligibility models for date-related fields.
pub use self::dtp_date::DtpDate;

pub mod eligibility_additional_information;
/// Additional eligibility information.
///
/// Provides supplementary data elements for eligibility requests and responses
/// that extend beyond standard fields.
///
/// # X12 HIPAA
/// Maps to additional information segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used to capture extended eligibility data.
pub use self::eligibility_additional_information::EligibilityAdditionalInformation;

pub mod eligibility_check_dependent_error;
/// Dependent-specific eligibility check errors.
///
/// Contains error information specific to dependent entities in eligibility
/// checks, including error codes and descriptions.
///
/// # X12 HIPAA
/// Maps to error segments in X12 271 dependent loops.
///
/// # Stedi Notes
/// Part of comprehensive error handling in eligibility responses.
pub use self::eligibility_check_dependent_error::EligibilityCheckDependentError;

pub mod eligibility_check_dependent_error_code;
/// Error codes for dependent eligibility issues.
///
/// Enumerates specific error codes that can occur with dependent entities
/// during eligibility checks.
///
/// # X12 HIPAA
/// Maps to dependent error code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::eligibility_check_dependent_error_code::EligibilityCheckDependentErrorCode;

pub mod eligibility_check_error;
/// General eligibility check error structure.
///
/// Represents top-level error information for healthcare eligibility checks,
/// including error codes and messages.
///
/// # X12 HIPAA
/// Maps to error handling segments in X12 271 transactions.
///
/// # Stedi Notes
/// Central error type for eligibility API responses.
pub use self::eligibility_check_error::EligibilityCheckError;

pub mod eligibility_check_error_code;
/// General eligibility error codes.
///
/// Enumerates standardized error codes for healthcare eligibility operations.
///
/// # X12 HIPAA
/// Maps to general error code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::eligibility_check_error_code::EligibilityCheckErrorCode;

pub mod eligibility_check_followup_action;
/// Follow-up actions for eligibility checks.
///
/// Specifies recommended actions for handling eligibility responses, such as
/// contacting the payer or requesting additional information.
///
/// # X12 HIPAA
/// Maps to follow-up action segments in X12 271 transactions.
///
/// # Stedi Notes
/// Used to guide next steps after eligibility verification.
pub use self::eligibility_check_followup_action::EligibilityCheckFollowupAction;

pub mod eligibility_check_payer_error;
/// Payer-specific eligibility check errors.
///
/// Contains error information specific to payer entities in eligibility checks.
///
/// # X12 HIPAA
/// Maps to payer error segments in X12 271 transactions.
///
/// # Stedi Notes
/// Part of comprehensive error handling for payer-related issues.
pub use self::eligibility_check_payer_error::EligibilityCheckPayerError;

pub mod eligibility_check_payer_error_code;
/// Error codes for payer eligibility issues.
///
/// Enumerates specific error codes that can occur with payer entities during
/// eligibility checks.
///
/// # X12 HIPAA
/// Maps to payer error code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::eligibility_check_payer_error_code::EligibilityCheckPayerErrorCode;

pub mod eligibility_check_provider_and_member_followup_action;
/// Provider and member follow-up actions.
///
/// Specifies follow-up actions that involve both providers and members in
/// eligibility responses.
///
/// # X12 HIPAA
/// Maps to provider/member follow-up action segments in X12 271 transactions.
///
/// # Stedi Notes
/// Used for coordinated follow-up between healthcare providers and members.
pub use self::eligibility_check_provider_and_member_followup_action::EligibilityCheckProviderAndMemberFollowupAction;

pub mod eligibility_check_provider_error;
/// Provider-specific eligibility check errors.
///
/// Contains error information specific to provider entities in eligibility
/// checks, including error codes and descriptions.
///
/// # X12 HIPAA
/// Maps to provider error segments in X12 271 transactions.
///
/// # Stedi Notes
/// Part of comprehensive error handling in eligibility responses.
pub use self::eligibility_check_provider_error::EligibilityCheckProviderError;

pub mod eligibility_check_provider_error_code;
/// Error codes for provider eligibility issues.
///
/// Enumerates specific error codes that can occur with provider entities
/// during eligibility checks.
///
/// # X12 HIPAA
/// Maps to provider error code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::eligibility_check_provider_error_code::EligibilityCheckProviderErrorCode;

pub mod eligibility_check_request_content;
/// Main eligibility check request structure.
///
/// Represents the complete request payload for healthcare eligibility checks,
/// including provider, subscriber, and dependent information.
///
/// # X12 HIPAA
/// Maps to the X12 270 transaction structure.
///
/// # Stedi Notes
/// Primary input structure for the eligibility API.
pub use self::eligibility_check_request_content::EligibilityCheckRequestContent;

pub mod eligibility_check_response_content;
/// Main eligibility check response structure.
///
/// Represents the complete response payload from healthcare payers for
/// eligibility checks, including benefit information and entity details.
///
/// # X12 HIPAA
/// Maps to the X12 271 transaction structure.
///
/// # Stedi Notes
/// Primary output structure from the eligibility API.
pub use self::eligibility_check_response_content::EligibilityCheckResponseContent;

pub mod eligibility_check_subscriber_error;
/// Subscriber-specific eligibility check errors.
///
/// Contains error information specific to subscriber entities in eligibility
/// checks, including error codes and descriptions.
///
/// # X12 HIPAA
/// Maps to subscriber error segments in X12 271 transactions.
///
/// # Stedi Notes
/// Part of comprehensive error handling in eligibility responses.
pub use self::eligibility_check_subscriber_error::EligibilityCheckSubscriberError;

pub mod eligibility_check_subscriber_error_code;
/// Error codes for subscriber eligibility issues.
///
/// Enumerates specific error codes that can occur with subscriber entities
/// during eligibility checks.
///
/// # X12 HIPAA
/// Maps to subscriber error code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::eligibility_check_subscriber_error_code::EligibilityCheckSubscriberErrorCode;

pub mod eligibility_meta_data_json;
/// Eligibility metadata structure.
///
/// Contains metadata about eligibility checks such as timestamps, trace
/// numbers, and transaction identifiers.
///
/// # X12 HIPAA
/// Maps to transaction header and trailer segments in X12 270/271.
///
/// # Stedi Notes
/// Used for tracking and auditing eligibility transactions.
pub use self::eligibility_meta_data_json::EligibilityMetaDataJson;

pub mod employment_status_code;
/// Employment status codes.
///
/// Defines employment status codes for subscribers, particularly relevant for
/// military and government healthcare programs.
///
/// # X12 HIPAA
/// Maps to employment status code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::employment_status_code::EmploymentStatusCode;

pub mod encounter;
/// Encounter information structure.
///
/// Represents healthcare encounter details including procedure codes,
/// diagnosis codes, and service dates for eligibility context.
///
/// # X12 HIPAA
/// Maps to encounter-related segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used to provide context for eligibility checks based on specific encounters.
pub use self::encounter::Encounter;

pub mod encounter_product_or_service_id_qualifier;
/// Product/service ID qualifiers for encounters.
///
/// Specifies the type of product or service identifier used in encounter
/// information.
///
/// # X12 HIPAA
/// Maps to product/service ID qualifier elements in X12 encounter segments.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::encounter_product_or_service_id_qualifier::EncounterProductOrServiceIdQualifier;

pub mod encounter_reference_identification_qualifier;
/// Reference identification qualifiers for encounters.
///
/// Defines the type of reference identification used in encounter data.
///
/// # X12 HIPAA
/// Maps to reference identification qualifier elements in X12 encounter segments.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::encounter_reference_identification_qualifier::EncounterReferenceIdentificationQualifier;

pub mod entity_type_qualifier;
/// Entity type qualifiers.
///
/// Specifies the type of entity in healthcare transactions such as provider,
/// subscriber, or dependent.
///
/// # X12 HIPAA
/// Maps to entity type qualifier elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::entity_type_qualifier::EntityTypeQualifier;

pub mod gateway_timeout_exception_response_content;
/// Gateway timeout error response.
///
/// Represents HTTP 504 error responses when the healthcare eligibility API
/// encounters gateway timeout issues.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards.
///
/// # Stedi Notes
/// Used internally for gateway timeout scenarios.
pub use self::gateway_timeout_exception_response_content::GatewayTimeoutExceptionResponseContent;

pub mod gender;
/// Gender codes.
///
/// Defines standardized gender codes for healthcare entities.
///
/// # X12 HIPAA
/// Maps to gender code elements in X12 270/271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::gender::Gender;

pub mod gender_with_unknown;
/// Extended gender codes including unknown.
///
/// Defines gender codes with an additional "unknown" option for cases where
/// gender information is not available.
///
/// # X12 HIPAA
/// Maps to gender code elements in X12 270/271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::gender_with_unknown::GenderWithUnknown;

pub mod government_service_affiliation_code;
/// Government service affiliation codes.
///
/// Specifies military or government service branch affiliations for subscribers.
///
/// # X12 HIPAA
/// Maps to service affiliation code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::government_service_affiliation_code::GovernmentServiceAffiliationCode;

pub mod health_care_diagnosis_code;
/// Healthcare diagnosis codes.
///
/// Represents diagnosis codes used in healthcare eligibility contexts.
///
/// # X12 HIPAA
/// Maps to diagnosis code elements in X12 270/271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::health_care_diagnosis_code::HealthCareDiagnosisCode;

pub mod health_care_information;
/// Healthcare information container.
///
/// Provides a structure for including healthcare-related information in
/// eligibility transactions.
///
/// # X12 HIPAA
/// Maps to healthcare information segments in X12 transactions.
///
/// # Stedi Notes
/// Used for general healthcare data elements.
pub use self::health_care_information::HealthCareInformation;

pub mod in_plan_network_indicator_code;
/// In-plan network indicator codes.
///
/// Specifies whether a healthcare service or provider is in-network or
/// out-of-network for a given plan.
///
/// # X12 HIPAA
/// Maps to in-plan network indicator elements in X12 271 benefit information.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::in_plan_network_indicator_code::InPlanNetworkIndicatorCode;

pub mod in_plan_network_indicator_name;
/// Human-readable in-plan network indicators.
///
/// Provides descriptive names for in-plan network statuses.
///
/// # X12 HIPAA
/// Maps to in-plan network indicator elements in X12 271 benefit information.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::in_plan_network_indicator_name::InPlanNetworkIndicatorName;

pub mod individual_relationship_code;
/// Individual relationship codes.
///
/// Defines relationship codes between individuals in healthcare contexts.
///
/// # X12 HIPAA
/// Maps to relationship code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::individual_relationship_code::IndividualRelationshipCode;

pub mod industry_code;
/// Industry classification codes.
///
/// Specifies industry classifications for healthcare organizations.
///
/// # X12 HIPAA
/// Maps to industry code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::industry_code::IndustryCode;

pub mod infallible_state_or_province_code;
/// State/province codes that cannot fail.
///
/// Defines standardized state or province codes with guaranteed valid values.
///
/// # X12 HIPAA
/// Maps to state/province code elements in X12 address segments.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::infallible_state_or_province_code::InfallibleStateOrProvinceCode;

pub mod information_receiver_name;
/// Information receiver name structure.
///
/// Represents the name of the entity receiving healthcare eligibility information.
///
/// # X12 HIPAA
/// Maps to information receiver name segments in X12 270 transactions.
///
/// # Stedi Notes
/// Used in request entities to identify information recipients.
pub use self::information_receiver_name::InformationReceiverName;

pub mod information_status_code;
/// Information status codes.
///
/// Defines the status of information provided in healthcare eligibility
/// transactions.
///
/// # X12 HIPAA
/// Maps to information status code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::information_status_code::InformationStatusCode;

pub mod injury_code_category;
/// Injury code categories.
///
/// Specifies categories for injury-related diagnosis codes.
///
/// # X12 HIPAA
/// Maps to injury code category elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::injury_code_category::InjuryCodeCategory;

pub mod insurance_type_code;
/// Insurance type codes.
///
/// Defines types of insurance coverage such as HMO, PPO, or Medicare.
///
/// # X12 HIPAA
/// Maps to insurance type code elements in X12 270/271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::insurance_type_code::InsuranceTypeCode;

pub mod insurance_type_name;
/// Human-readable insurance type names.
///
/// Provides descriptive names for insurance types.
///
/// # X12 HIPAA
/// Maps to insurance type code elements in X12 270/271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::insurance_type_name::InsuranceTypeName;

pub mod internal_failure_exception_response_content;
/// Internal failure error response.
///
/// Represents HTTP 500 error responses for internal system failures in the
/// healthcare eligibility API.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards.
///
/// # Stedi Notes
/// Used internally for system error scenarios.
pub use self::internal_failure_exception_response_content::InternalFailureExceptionResponseContent;

pub mod maintenance_reason_code;
/// Maintenance reason codes.
///
/// Specifies reasons for maintenance operations in healthcare eligibility data.
///
/// # X12 HIPAA
/// Maps to maintenance reason code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::maintenance_reason_code::MaintenanceReasonCode;

pub mod maintenance_type_code;
/// Maintenance type codes.
///
/// Defines types of maintenance operations performed on healthcare eligibility
/// data.
///
/// # X12 HIPAA
/// Maps to maintenance type code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::maintenance_type_code::MaintenanceTypeCode;

pub mod medical_procedure;
/// Medical procedure information.
///
/// Represents detailed medical procedure data including codes and identifiers
/// for healthcare services.
///
/// # X12 HIPAA
/// Maps to medical procedure segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used to specify procedures in eligibility checks.
pub use self::medical_procedure::MedicalProcedure;

pub mod medical_procedure_product_or_service_id_qualifier;
/// Product/service ID qualifiers for medical procedures.
///
/// Specifies the type of product or service identifier used in medical
/// procedure information.
///
/// # X12 HIPAA
/// Maps to product/service ID qualifier elements in X12 procedure segments.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::medical_procedure_product_or_service_id_qualifier::MedicalProcedureProductOrServiceIdQualifier;

pub mod military_service_rank_code;
/// Military service rank codes.
///
/// Defines standardized military rank codes for subscribers in government
/// healthcare programs.
///
/// # X12 HIPAA
/// Maps to military rank code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::military_service_rank_code::MilitaryServiceRankCode;

pub mod payer;
/// Payer entity information.
///
/// Represents the healthcare payer organization in eligibility transactions,
/// including identification and contact details.
///
/// # X12 HIPAA
/// Maps to payer entity segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used in eligibility responses to identify the responding payer.
pub use self::payer::Payer;

pub mod payer_entity_identifier;
/// Payer entity identifiers.
///
/// Defines standardized identifiers for payer entities in healthcare
/// transactions.
///
/// # X12 HIPAA
/// Maps to entity identifier code elements for payers in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::payer_entity_identifier::PayerEntityIdentifier;

pub mod plan_date_information;
/// Date information for healthcare plans.
///
/// Contains date-related fields specific to healthcare insurance plans such as
/// effective dates and renewal dates.
///
/// # X12 HIPAA
/// Maps to plan date segments in X12 271 transactions.
///
/// # Stedi Notes
/// Used in plan information structures.
pub use self::plan_date_information::PlanDateInformation;

pub mod plan_information;
/// Healthcare plan information.
///
/// Represents detailed information about healthcare insurance plans including
/// plan names, numbers, and coverage details.
///
/// # X12 HIPAA
/// Maps to healthcare plan information segments in X12 271 transactions.
///
/// # Stedi Notes
/// Contains plan-level details in eligibility responses.
pub use self::plan_information::PlanInformation;

pub mod plan_status;
/// Healthcare plan status information.
///
/// Defines the current status of healthcare insurance plans such as active,
/// inactive, or cancelled.
///
/// # X12 HIPAA
/// Maps to plan status elements in X12 271 transactions.
///
/// # Stedi Notes
/// Used to indicate plan eligibility status.
pub use self::plan_status::PlanStatus;

pub mod provider;
/// Provider entity information.
///
/// Represents healthcare provider details in eligibility responses including
/// NPI, name, and address information.
///
/// # X12 HIPAA
/// Maps to provider entity segments in X12 271 transactions.
///
/// # Stedi Notes
/// Response counterpart to RequestProvider.
pub use self::provider::Provider;

pub mod provider_information;
/// Additional provider information.
///
/// Contains supplementary information about healthcare providers in
/// eligibility transactions.
///
/// # X12 HIPAA
/// Maps to provider information segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Extends basic provider entity data.
pub use self::provider_information::ProviderInformation;

pub mod provider_type;
/// Provider type classifications.
///
/// Defines types of healthcare providers such as individual practitioner,
/// organization, or facility.
///
/// # X12 HIPAA
/// Maps to provider type elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::provider_type::ProviderType;

pub mod quantity_qualifier_code;
/// Quantity qualifier codes.
///
/// Specifies the type of quantity being measured in healthcare transactions.
///
/// # X12 HIPAA
/// Maps to quantity qualifier code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::quantity_qualifier_code::QuantityQualifierCode;

pub mod quantity_qualifier_name;
/// Human-readable quantity qualifiers.
///
/// Provides descriptive names for quantity qualifier codes.
///
/// # X12 HIPAA
/// Maps to quantity qualifier elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::quantity_qualifier_name::QuantityQualifierName;

pub mod related_entity_identifier_name;
/// Names for related entity identifiers.
///
/// Defines standardized names for identifiers used with related entities in
/// healthcare transactions.
///
/// # X12 HIPAA
/// Maps to related entity identifier name elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::related_entity_identifier_name::RelatedEntityIdentifierName;

pub mod request_address;
/// Address information for eligibility requests.
///
/// Represents physical address details for healthcare entities in eligibility
/// requests.
///
/// # X12 HIPAA
/// Maps to address segments in X12 270 requests.
///
/// # Stedi Notes
/// Request counterpart to Address.
pub use self::request_address::RequestAddress;

pub mod request_dependent;
/// Dependent entity for eligibility requests.
///
/// Represents dependent information in healthcare eligibility requests
/// including relationship to subscriber and identification details.
///
/// # X12 HIPAA
/// Maps to dependent entity loops in X12 270 transactions.
///
/// # Stedi Notes
/// Request counterpart to ResponseDependent.
pub use self::request_dependent::RequestDependent;

pub mod request_dependent_provider_code;
/// Provider codes for request dependents.
///
/// Defines provider type codes specific to dependent entities in eligibility
/// requests.
///
/// # X12 HIPAA
/// Maps to provider code elements for dependents in X12 270 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::request_dependent_provider_code::RequestDependentProviderCode;

pub mod request_dependent_reference_identification_qualifier;
/// Reference ID qualifiers for request dependents.
///
/// Specifies the type of reference identification used for dependent entities
/// in eligibility requests.
///
/// # X12 HIPAA
/// Maps to reference ID qualifier elements for dependents in X12 270 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::request_dependent_reference_identification_qualifier::RequestDependentReferenceIdentificationQualifier;

pub mod request_eligibility_service_type_code;
/// Service type codes for eligibility requests.
///
/// Defines standardized codes for healthcare service types being checked for
/// eligibility.
///
/// # X12 HIPAA
/// Maps to service type code elements in X12 270 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::request_eligibility_service_type_code::RequestEligibilityServiceTypeCode;

pub mod request_provider_code;
/// Provider codes for eligibility requests.
///
/// Defines provider type codes for healthcare providers in eligibility
/// requests.
///
/// # X12 HIPAA
/// Maps to provider code elements in X12 270 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::request_provider_code::RequestProviderCode;

pub mod request_subscriber;
/// Subscriber entity for eligibility requests.
///
/// Represents subscriber (primary insured) information in healthcare
/// eligibility requests including demographics and insurance details.
///
/// # X12 HIPAA
/// Maps to subscriber entity loops in X12 270 transactions.
///
/// # Stedi Notes
/// Request counterpart to ResponseSubscriber.
pub use self::request_subscriber::RequestSubscriber;

pub mod request_subscriber_provider_code;
/// Provider codes for request subscribers.
///
/// Defines provider type codes specific to subscriber entities in eligibility
/// requests.
///
/// # X12 HIPAA
/// Maps to provider code elements for subscribers in X12 270 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::request_subscriber_provider_code::RequestSubscriberProviderCode;

pub mod request_subscriber_reference_identification_qualifier;
/// Reference ID qualifiers for request subscribers.
///
/// Specifies the type of reference identification used for subscriber
/// entities in eligibility requests.
///
/// # X12 HIPAA
/// Maps to reference ID qualifier elements for subscribers in X12 270 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::request_subscriber_reference_identification_qualifier::RequestSubscriberReferenceIdentificationQualifier;

pub mod resource_not_found_exception_response_content;
/// Resource not found error response.
///
/// Represents HTTP 404 error responses when requested healthcare eligibility
/// resources cannot be found.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards.
///
/// # Stedi Notes
/// Used internally for missing resource scenarios.
pub use self::resource_not_found_exception_response_content::ResourceNotFoundExceptionResponseContent;

pub mod response_dependent;
/// Dependent entity for eligibility responses.
///
/// Represents dependent information in healthcare eligibility responses from
/// payers including benefit eligibility details.
///
/// # X12 HIPAA
/// Maps to dependent entity loops in X12 271 transactions.
///
/// # Stedi Notes
/// Response counterpart to RequestDependent.
pub use self::response_dependent::ResponseDependent;

pub mod response_dependent_entity_identifier;
/// Entity identifiers for response dependents.
///
/// Defines standardized identifiers for dependent entities in eligibility
/// responses.
///
/// # X12 HIPAA
/// Maps to entity identifier code elements for dependents in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::response_dependent_entity_identifier::ResponseDependentEntityIdentifier;

pub mod response_eligibility_service_type;
/// Service type information in eligibility responses.
///
/// Contains details about healthcare service types and their eligibility
/// status in payer responses.
///
/// # X12 HIPAA
/// Maps to service type segments in X12 271 transactions.
///
/// # Stedi Notes
/// Used to specify covered services in eligibility responses.
pub use self::response_eligibility_service_type::ResponseEligibilityServiceType;

pub mod response_eligibility_service_type_code;
/// Service type codes for eligibility responses.
///
/// Defines standardized codes for healthcare service types in eligibility
/// responses from payers.
///
/// # X12 HIPAA
/// Maps to service type code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::response_eligibility_service_type_code::ResponseEligibilityServiceTypeCode;

pub mod response_provider;
/// Provider entity for eligibility responses.
///
/// Represents healthcare provider information in eligibility responses from
/// payers.
///
/// # X12 HIPAA
/// Maps to provider entity segments in X12 271 transactions.
///
/// # Stedi Notes
/// Response counterpart to Provider.
pub use self::response_provider::ResponseProvider;

pub mod response_provider_code;
/// Provider codes for eligibility responses.
///
/// Defines provider type codes for healthcare providers in eligibility
/// responses.
///
/// # X12 HIPAA
/// Maps to provider code elements in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::response_provider_code::ResponseProviderCode;

pub mod response_provider_entity_identifier;
/// Entity identifiers for response providers.
///
/// Defines standardized identifiers for provider entities in eligibility
/// responses.
///
/// # X12 HIPAA
/// Maps to entity identifier code elements for providers in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::response_provider_entity_identifier::ResponseProviderEntityIdentifier;

pub mod response_subscriber;
/// Subscriber entity for eligibility responses.
///
/// Represents subscriber (primary insured) information in healthcare
/// eligibility responses from payers including benefit details.
///
/// # X12 HIPAA
/// Maps to subscriber entity loops in X12 271 transactions.
///
/// # Stedi Notes
/// Response counterpart to RequestSubscriber.
pub use self::response_subscriber::ResponseSubscriber;

pub mod response_subscriber_entity_identifier;
/// Entity identifiers for response subscribers.
///
/// Defines standardized identifiers for subscriber entities in eligibility
/// responses.
///
/// # X12 HIPAA
/// Maps to entity identifier code elements for subscribers in X12 271 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::response_subscriber_entity_identifier::ResponseSubscriberEntityIdentifier;

pub mod service_unavailable_exception_response_content;
/// Service unavailable error response.
///
/// Represents HTTP 503 error responses when the healthcare eligibility API
/// is temporarily unavailable.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards.
///
/// # Stedi Notes
/// Used internally for service unavailability scenarios.
pub use self::service_unavailable_exception_response_content::ServiceUnavailableExceptionResponseContent;

pub mod state_or_province_code;
/// State or province codes.
///
/// Defines standardized codes for US states and Canadian provinces.
///
/// # X12 HIPAA
/// Maps to state/province code elements in X12 address segments.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::state_or_province_code::StateOrProvinceCode;

pub mod subscriber_insured_indicator;
/// Subscriber insured status indicators.
///
/// Indicates whether a subscriber is the insured party in a healthcare plan.
///
/// # X12 HIPAA
/// Maps to insured indicator elements for subscribers in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::subscriber_insured_indicator::SubscriberInsuredIndicator;

pub mod subscriber_relationship;
/// Subscriber relationship information.
///
/// Contains details about the relationship between a subscriber and other
/// entities in healthcare transactions.
///
/// # X12 HIPAA
/// Maps to subscriber relationship segments in X12 transactions.
///
/// # Stedi Notes
/// Used to establish subscriber relationships in eligibility checks.
pub use self::subscriber_relationship::SubscriberRelationship;

pub mod subscriber_relationship_code;
/// Codes for subscriber relationships.
///
/// Defines standardized codes for subscriber relationships to other entities.
///
/// # X12 HIPAA
/// Maps to relationship code elements for subscribers in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::subscriber_relationship_code::SubscriberRelationshipCode;

pub mod subscriber_trace_number;
/// Subscriber trace numbers.
///
/// Represents trace numbers used to track subscriber-related transactions
/// through the healthcare eligibility system.
///
/// # X12 HIPAA
/// Maps to trace number segments in X12 270/271 transactions.
///
/// # Stedi Notes
/// Used for transaction tracking and correlation.
pub use self::subscriber_trace_number::SubscriberTraceNumber;

pub mod throttling_exception_response_content;
/// Throttling error response.
///
/// Represents HTTP 429 error responses when request rate limits are exceeded
/// in the healthcare eligibility API.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards.
///
/// # Stedi Notes
/// Used internally for rate limiting scenarios.
pub use self::throttling_exception_response_content::ThrottlingExceptionResponseContent;

pub mod time_period_qualifier;
/// Time period qualifiers.
///
/// Specifies the type of time period being referenced in healthcare
/// transactions such as eligibility periods or benefit periods.
///
/// # X12 HIPAA
/// Maps to time period qualifier elements in X12 transactions.
///
/// # Stedi Notes
/// Used with date information structures.
pub use self::time_period_qualifier::TimePeriodQualifier;

pub mod time_period_qualifier_code;
/// Codes for time period qualifiers.
///
/// Defines standardized codes for time period qualifiers in healthcare
/// transactions.
///
/// # X12 HIPAA
/// Maps to time period qualifier code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::time_period_qualifier_code::TimePeriodQualifierCode;

pub mod time_qualifier_code;
/// Time qualifier codes.
///
/// Specifies standardized codes for time-related qualifiers in healthcare
/// transactions.
///
/// # X12 HIPAA
/// Maps to time qualifier code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::time_qualifier_code::TimeQualifierCode;

pub mod time_qualifier_name;
/// Human-readable time qualifiers.
///
/// Provides descriptive names for time qualifier codes.
///
/// # X12 HIPAA
/// Maps to time qualifier elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::time_qualifier_name::TimeQualifierName;

pub mod unit_for_measurement;
/// Units of measurement information.
///
/// Represents standardized units of measurement used in healthcare
/// transactions such as monetary amounts or service quantities.
///
/// # X12 HIPAA
/// Maps to unit of measurement segments in X12 transactions.
///
/// # Stedi Notes
/// Used in benefits service delivery and quantity information.
pub use self::unit_for_measurement::UnitForMeasurement;

pub mod unit_for_measurement_code;
/// Codes for units of measurement.
///
/// Defines standardized codes for units of measurement in healthcare
/// transactions.
///
/// # X12 HIPAA
/// Maps to unit of measurement code elements in X12 transactions.
///
/// # Stedi Notes
/// See Stedi's code lists documentation for complete enumeration values.
pub use self::unit_for_measurement_code::UnitForMeasurementCode;

pub mod validation_exception;
/// Validation exception structure.
///
/// Represents validation errors that occur during healthcare eligibility
/// request processing.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards but follows Stedi's API error patterns.
///
/// # Stedi Notes
/// Used internally for request validation failures.
pub use self::validation_exception::ValidationException;

pub mod validation_exception_field;
/// Validation exception field details.
///
/// Contains information about specific fields that failed validation in
/// healthcare eligibility requests.
///
/// # X12 HIPAA
/// Not directly related to X12 HIPAA standards.
///
/// # Stedi Notes
/// Used to provide detailed validation error information.
pub use self::validation_exception_field::ValidationExceptionField;

pub mod warning;
/// Warning information structure.
///
/// Represents non-critical warning messages in healthcare eligibility
/// responses that don't prevent successful processing.
///
/// # X12 HIPAA
/// Maps to warning segments in X12 271 transactions.
///
/// # Stedi Notes
/// Used to communicate important but non-error information to API consumers.
pub use self::warning::Warning;
