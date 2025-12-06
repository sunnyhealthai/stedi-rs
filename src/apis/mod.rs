use std::error;
use std::fmt;

/// Response content wrapper containing HTTP status, raw content, and parsed entity.
#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    /// HTTP status code from the response
    pub status: reqwest::StatusCode,
    /// Raw response content as a string
    pub content: String,
    /// Parsed response entity, if available
    pub entity: Option<T>,
}

/// Error types that can occur when making API requests.
#[derive(Debug)]
pub enum Error<T> {
    /// HTTP request error from reqwest
    Reqwest(reqwest::Error),
    /// JSON serialization/deserialization error
    Serde(serde_json::Error),
    /// I/O error
    Io(std::io::Error),
    /// HTTP response error with details
    Response(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::Response(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::Response(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// URL-encode a string for use in query parameters.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

/// Parse a JSON object into deep object format for form encoding.
///
/// Converts nested JSON objects into flat key-value pairs using bracket notation.
pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        params
    } else {
        unimplemented!("Only objects are supported with style=deepObject")
    }
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    /// JSON content type (application/json)
    Json,
    /// Plain text content type (text/plain)
    Text,
    /// Unsupported content type with the actual type string
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

/// Eligibility check API endpoints and functionality
pub mod eligibility_check;

/// Configuration for API clients
pub mod configuration;
