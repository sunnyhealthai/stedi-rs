//! # Stedi Healthcare Eligibility API SDK
//!
//! **Note**: This is a community-maintained crate and is not officially supported by Stedi.
//!
//! This crate provides a Rust SDK for [Stedi's Healthcare Eligibility API](https://www.stedi.com/docs/healthcare),
//! enabling healthcare providers to perform real-time and batch eligibility checks.
//!
//! ## Overview
//!
//! Healthcare eligibility checks verify whether a patient has coverage for specific medical benefits
//! under their health insurance plan. This process allows patients and healthcare providers to
//! determine a patient's financial responsibilities for medical services.
//!
//! ## Benefits Information
//!
//! The API returns detailed benefits information including:
//!
//! - **Coverage Status**: Active, inactive, or pending coverage
//! - **Cost Sharing**: Co-payments, coinsurance, and deductibles
//! - **Benefit Limits**: Visit limits, dollar amounts, and time periods
//! - **Network Status**: In-network vs out-of-network provider status
//! - **Authorization Requirements**: Prior authorization and referral needs
//!
//! ## Features
//!
//! - **`benefits-calculator`** (optional): Enables utilities for calculating patient out-of-pocket costs
//!   from eligibility response data. Includes functions for parsing benefits and calculating
//!   deductibles, copays, coinsurance, and out-of-pocket maximums.
//!
//! ## Documentation
//!
//! For comprehensive documentation, visit:
//! - [Stedi Healthcare Documentation](https://www.stedi.com/docs/healthcare)
//! - [API Reference](https://www.stedi.com/docs/api-reference/healthcare)
//! - [Eligibility Check Guide](https://www.stedi.com/docs/healthcare/send-eligibility-checks)

/// API client functionality and error handling
pub mod apis;
/// Data models for healthcare eligibility API requests and responses
pub mod models;
/// Utilities for processing eligibility responses and calculating costs
pub mod utilities;
