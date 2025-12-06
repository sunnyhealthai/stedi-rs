use serde::{Deserialize, Serialize};

/// Healthcare plan status information indicating the current state of a health insurance plan.
///
/// This struct provides status information about a healthcare insurance plan, including
/// whether the plan is active, inactive, terminated, or in another state. Plan status
/// information helps determine eligibility and coverage availability for subscribers and
/// dependents.
///
/// ## Plan Status Fields
///
/// The plan status includes several components:
///
/// - **Status Code** (`statusCode`): Standardized code indicating the plan status
/// - **Status** (`status`): Human-readable description of the plan status
/// - **Plan Details** (`planDetails`): Additional details or notes about the plan status
/// - **Service Type Codes** (`serviceTypeCodes`): Service types associated with this
///   plan status, indicating which services are affected by the status
///
/// ## Common Plan Statuses
///
/// Plan statuses typically indicate:
///
/// - **Active**: Plan is currently active and providing coverage
/// - **Inactive**: Plan is not currently active but may be reactivated
/// - **Terminated**: Plan has been terminated and no longer provides coverage
/// - **Pending**: Plan status is pending activation or approval
/// - **Suspended**: Plan coverage is temporarily suspended
/// - **Cancelled**: Plan has been cancelled
///
/// ## Service Type Codes
///
/// The `serviceTypeCodes` field links the plan status to specific healthcare service
/// types. This allows payers to indicate that certain services may be affected by the
/// plan status while others remain available. For example, a plan might be active for
/// medical services but inactive for dental services.
///
/// ## Usage Context
///
/// Plan status information is used to:
///
/// - **Determine eligibility**: Verify whether a plan is active and providing coverage
/// - **Service availability**: Understand which services are available under the
///   current plan status
/// - **Claims processing**: Determine if claims can be submitted for the plan
/// - **Coordination of benefits**: Understand plan status when coordinating multiple
///   insurance policies
/// - **Member communication**: Inform members about their plan status and any
///   limitations or changes
///
/// ## X12 HIPAA
///
/// Maps to plan status information in X12 271 transactions, including:
/// - Status codes in plan information segments
/// - Service type associations with plan status
/// - Plan detail information segments
///
/// ## Examples
///
/// An active plan status might include:
/// - `statusCode`: "A" (Active)
/// - `status`: "Active"
/// - `serviceTypeCodes`: All service types the plan covers
///
/// A terminated plan might include:
/// - `statusCode`: "T" (Terminated)
/// - `status`: "Terminated"
/// - `planDetails`: "Plan terminated effective 2024-01-01"
/// - `serviceTypeCodes`: Empty or no service types
///
/// ## Stedi Notes
///
/// Plan status information is provided by payers in eligibility responses. The specific
/// status codes and descriptions vary by payer, but typically follow standard patterns
/// for active, inactive, and terminated states.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanStatus {
    /// Additional details or notes about the plan status.
    ///
    /// Provides supplementary information about the plan status, such as effective dates,
    /// termination reasons, or other relevant details. This field may contain free-form
    /// text describing the plan status in more detail than the standard status code.
    #[serde(rename = "planDetails", skip_serializing_if = "Option::is_none")]
    pub plan_details: Option<String>,
    /// An array of [Service Type Codes](https://www.stedi.com/docs/healthcare/send-eligibility-checks#service-type-codes) related to the benefit type.
    ///
    /// Specifies which healthcare service types are associated with this plan status.
    /// This allows payers to indicate that certain services may be affected by the plan
    /// status while others remain available. For example, a plan might be active for
    /// medical services (`30` - Plan coverage and general benefits) but inactive for
    /// dental services (`35` - Dental).
    ///
    /// When present, these service type codes indicate which services are subject to
    /// the plan status. When absent, the plan status applies to all services.
    #[serde(rename = "serviceTypeCodes", skip_serializing_if = "Option::is_none")]
    pub service_type_codes: Option<Vec<super::ResponseEligibilityServiceTypeCode>>,
    /// Human-readable description of the plan status.
    ///
    /// Provides a text description of the plan status, such as "Active", "Terminated",
    /// "Inactive", or other status descriptions. This field complements the `statusCode`
    /// field by providing a human-readable version of the status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Standardized code indicating the plan status.
    ///
    /// Contains a standardized code representing the plan status. Common codes include:
    /// - Active status codes (e.g., "A", "1")
    /// - Inactive status codes (e.g., "I", "0")
    /// - Terminated status codes (e.g., "T", "2")
    ///
    /// The specific codes vary by payer, but typically follow standard patterns for
    /// representing plan states. Use this code for programmatic status checking, while
    /// the `status` field provides human-readable descriptions.
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

impl PlanStatus {
    /// Creates a new `PlanStatus` instance with all fields initialized to `None`.
    ///
    /// This constructor initializes an empty plan status object. In practice, plan status
    /// information is populated by payers in eligibility responses, so this constructor is
    /// primarily useful for testing or when manually constructing plan status objects.
    ///
    /// ## Usage Example
    ///
    /// ```rust
    /// use stedi_rs::models::{PlanStatus, ResponseEligibilityServiceTypeCode};
    ///
    /// // Create a new plan status instance
    /// let mut plan_status = PlanStatus::new();
    ///
    /// // Set plan status fields
    /// plan_status.status_code = Some("A".to_string());
    /// plan_status.status = Some("Active".to_string());
    /// plan_status.service_type_codes = Some(vec![
    ///     ResponseEligibilityServiceTypeCode::HealthBenefitPlanCoverage,
    /// ]);
    /// ```
    ///
    /// ## Important Notes
    ///
    /// - **Response-only structure**: This struct is typically populated by payers in
    ///   eligibility responses, not constructed by API consumers
    /// - **Status code vs status**: The `statusCode` field contains standardized codes
    ///   for programmatic use, while `status` provides human-readable descriptions
    /// - **Service type codes**: When present, these codes indicate which services are
    ///   affected by the plan status
    /// - **Plan details**: Additional context about the plan status may be provided in
    ///   the `planDetails` field
    ///
    /// # Returns
    ///
    /// A new `PlanStatus` instance with all optional fields set to `None`.
    pub fn new() -> PlanStatus {
        PlanStatus {
            plan_details: None,
            service_type_codes: None,
            status: None,
            status_code: None,
        }
    }
}
