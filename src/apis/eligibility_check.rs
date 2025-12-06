use super::{super::models, ResponseContent};
use super::{ContentType, Error, configuration};
use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};

/// Error types that can be returned by the eligibility check API.
///
/// These errors correspond to different HTTP status codes and contain detailed
/// error information to help diagnose issues with eligibility check requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EligibilityCheckError {
    /// Validation error (HTTP 400) - Request contains invalid or missing required fields
    Status400(models::ValidationException),

    /// Access denied (HTTP 403) - Authentication failed or insufficient permissions
    Status403(models::AccessDeniedExceptionResponseContent),

    /// Resource not found (HTTP 404) - Requested resource does not exist
    Status404(models::ResourceNotFoundExceptionResponseContent),

    /// Rate limit exceeded (HTTP 429) - Too many requests, retry with backoff
    Status429(models::ThrottlingExceptionResponseContent),

    /// Internal server error (HTTP 500) - Stedi service error
    Status500(models::InternalFailureExceptionResponseContent),

    /// Service unavailable (HTTP 503) - Stedi service temporarily unavailable
    Status503(models::ServiceUnavailableExceptionResponseContent),

    /// Gateway timeout (HTTP 504) - Request timed out
    Status504(models::GatewayTimeoutExceptionResponseContent),

    /// Unknown error response that doesn't match expected error types
    UnknownValue(serde_json::Value),
}

/// Submit a real-time 270/271 eligibility check in JSON format.
///
/// This function performs a real-time healthcare eligibility check to verify whether
/// a patient has coverage for specific medical benefits under their health insurance plan.
///
/// ## Parameters
///
/// - `configuration`: API client configuration containing authentication and endpoint details
/// - `eligibility_check_request_content`: The eligibility check request with patient, provider, and encounter information
///
/// ## Returns
///
/// Returns a `Result` containing either:
/// - **Success**: `EligibilityCheckResponseContent` with detailed benefit information
/// - **Error**: `Error<EligibilityCheckError>` with specific error details
///
/// ## Example
///
/// ```rust,no_run
/// use stedi_rs::apis::{configuration::Configuration, eligibility_check};
/// use stedi_rs::models::*;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Configuration::new();
///
/// let request = EligibilityCheckRequestContent {
///     control_number: "123456789".to_string(),
///     trading_partner_service_id: "AETNA".to_string(),
///     provider: Provider {
///         organization_name: Some("ACME Health Services".to_string()),
///         npi: Some("1234567890".to_string()),
///         ..Default::default()
///     },
///     subscriber: RequestSubscriber {
///         member_id: Some("1234567890".to_string()),
///         first_name: Some("Jane".to_string()),
///         last_name: Some("Doe".to_string()),
///         date_of_birth: Some("19800101".to_string()),
///         ..Default::default()
///     },
///     ..Default::default()
/// };
///
/// let response = eligibility_check::eligibility_check(&config, request).await?;
/// println!("Coverage status: {:?}", response.plan_status);
/// # Ok(())
/// # }
/// ```
///
/// ## Error Handling
///
/// Common error scenarios:
/// - **400 Validation Error**: Missing required fields or invalid data format
/// - **403 Access Denied**: Invalid API key or insufficient permissions  
/// - **429 Rate Limited**: Too many concurrent requests
/// - **503 Service Unavailable**: Temporary service outage
///
/// For payer connectivity issues, implement retry logic as described in the
/// [Stedi retry strategy documentation](https://www.stedi.com/docs/healthcare/eligibility-troubleshooting#retry-strategy).
///
/// ## Timeout
///
/// Requests are held open for up to 120 seconds to allow for payer response times.
/// Insurance payers may take up to 60 seconds to respond.
pub async fn eligibility_check(
    configuration: &configuration::Configuration,
    eligibility_check_request_content: models::EligibilityCheckRequestContent,
) -> Result<models::EligibilityCheckResponseContent, Error<EligibilityCheckError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_eligibility_check_request_content = eligibility_check_request_content;

    let uri_str = format!(
        "{}/change/medicalnetwork/eligibility/v3",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        req_builder = req_builder.header(
            reqwest::header::AUTHORIZATION,
            bearer_access_token.clone().to_string(),
        );
    }

    req_builder = req_builder.json(&p_eligibility_check_request_content);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::EligibilityCheckResponseContent`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::EligibilityCheckResponseContent`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EligibilityCheckError> = serde_json::from_str(&content).ok();
        Err(Error::Response(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
