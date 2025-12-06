//! Healthcare prior authorization indicator types.
//!
//! This module defines the `AuthOrCertIndicator` enum which represents whether a healthcare benefit
//! requires prior authorization or certification before services can be rendered. This information
//! is typically returned in eligibility check responses from payers to indicate the authorization
//! requirements for specific benefits.

use serde::{Deserialize, Serialize};

/// Indicates whether a benefit requires prior authorization or certification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthOrCertIndicator {
    /// No prior authorization or certification is required for this benefit.
    #[serde(rename = "N")]
    NoAuthorizationRequired,
    /// The prior authorization or certification requirement is unknown or uncertain.
    #[serde(rename = "U")]
    UnknownAuthorizationRequirement,
    /// Prior authorization or certification is required for this benefit.
    #[serde(rename = "Y")]
    AuthorizationRequired,
}

impl std::fmt::Display for AuthOrCertIndicator {
    /// Formats the prior authorization indicator as its single-letter code representation.
    ///
    /// This implementation allows the enum to be easily converted to strings for display
    /// or serialization purposes, using the standard healthcare codes:
    /// - N for "No authorization required"
    /// - U for "Unknown authorization requirement"  
    /// - Y for "Authorization required"
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoAuthorizationRequired => write!(f, "N"),
            Self::UnknownAuthorizationRequirement => write!(f, "U"),
            Self::AuthorizationRequired => write!(f, "Y"),
        }
    }
}

impl Default for AuthOrCertIndicator {
    /// Returns the default prior authorization indicator, which is `NoAuthorizationRequired` (No authorization required).
    ///
    /// This default is chosen as the most common case where benefits do not require prior
    /// authorization, ensuring safe fallback behavior in eligibility processing workflows.
    fn default() -> AuthOrCertIndicator {
        Self::NoAuthorizationRequired
    }
}
