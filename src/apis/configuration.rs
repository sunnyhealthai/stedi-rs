/// Configuration settings for the Stedi Healthcare API client.
///
/// This struct contains all the necessary configuration for making API requests to Stedi,
/// including authentication credentials, base URL, and HTTP client settings.
///
/// ## Authentication
///
/// Stedi uses API key authentication via the `bearer_access_token` field. The token should
/// be formatted as `"Key {your-api-key}"`.
///
/// ## Example
///
/// ```rust
/// use stedi_rs::apis::configuration::Configuration;
///
/// // Recommended: Use the helper method
/// let config = Configuration::with_api_key("your-api-key-here");
///
/// // Or manually set the bearer token
/// let mut config = Configuration::default();
/// config.bearer_access_token = Some(format!("Key {}", "your-api-key-here"));
/// ```
#[derive(Debug, Clone)]
pub struct Configuration {
    /// The base URL for the Stedi Healthcare API (defaults to <https://healthcare.us.stedi.com>)
    pub base_path: String,

    /// Optional User-Agent header value for HTTP requests
    pub user_agent: Option<String>,

    /// The HTTP client used for making requests
    pub client: reqwest::Client,

    /// Bearer token for authentication. Should be formatted as `"Key {api-key}"`.
    /// This is the primary authentication method for Stedi.
    pub bearer_access_token: Option<String>,

    /// Basic authentication credentials (username, optional password)
    #[doc(hidden)]
    pub basic_auth: Option<BasicAuth>,

    /// OAuth access token for authentication
    #[doc(hidden)]
    pub oauth_access_token: Option<String>,

    /// API key configuration for Stedi authentication
    #[doc(hidden)]
    pub api_key: Option<ApiKey>,
}

/// Basic authentication tuple containing username and optional password.
#[doc(hidden)]
pub type BasicAuth = (String, Option<String>);

/// API key configuration for Stedi authentication.
///
/// Contains the API key and optional prefix. For Stedi, the prefix is typically not needed.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ApiKey {
    /// Optional prefix for the API key (usually not needed for Stedi)
    pub prefix: Option<String>,

    /// The actual API key string obtained from your Stedi account
    pub key: String,
}

impl Configuration {
    /// Create a new Configuration with default settings.
    ///
    /// This creates a configuration pointing to the production Stedi Healthcare API
    /// at `https://healthcare.us.stedi.com` with a 120-second timeout. You'll need to
    /// set authentication credentials before making API calls.
    pub fn new() -> Configuration {
        Configuration::default()
    }

    /// Create a new Configuration with an API key.
    ///
    /// This is the recommended way to create a configuration. The API key will be
    /// automatically formatted as `"Key {api_key}"` and set as the bearer access token.
    ///
    /// The HTTP client is configured with a 120-second timeout, which is required
    /// as insurance payers may take up to 60 seconds to respond.
    ///
    /// # Example
    ///
    /// ```rust
    /// use stedi_rs::apis::configuration::Configuration;
    ///
    /// let config = Configuration::with_api_key("your-stedi-api-key");
    /// ```
    pub fn with_api_key(api_key: impl AsRef<str>) -> Configuration {
        Configuration {
            bearer_access_token: Some(format!("Key {}", api_key.as_ref())),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            ..Default::default()
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://healthcare.us.stedi.com".to_owned(),
            user_agent: Some("OpenAPI-Generator/2024-04-01/rust".to_owned()),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
