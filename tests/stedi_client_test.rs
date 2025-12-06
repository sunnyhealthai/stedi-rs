use std::env;
use stedi_rs::apis::{configuration::Configuration, eligibility_check};
use stedi_rs::models::*;

#[tokio::test]
async fn test_stedi_uhc_eligibility_check() {
    // This test uses anonymized test data to verify the Stedi client can handle UnitedHealthcare responses

    let api_key =
        env::var("TEST_STEDI_API_KEY").expect("TEST_STEDI_API_KEY must be set (run with doppler)");

    // Create Stedi client configuration with proper auth header format
    let auth_header = format!("Key {}", api_key);
    let config = Configuration {
        base_path: "https://healthcare.us.stedi.com".to_string(),
        bearer_access_token: Some(auth_header),
        client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120)) // 120 seconds as per Stedi docs
            .build()
            .expect("Failed to create HTTP client"),
        ..Default::default()
    };

    println!("=== Testing UnitedHealthcare Eligibility Check ===");
    println!("Payer: 87726 (UnitedHealthcare)");
    println!("Member ID: UHC123456");
    println!();

    // Create eligibility check request with anonymized test data (UHC mock request)
    let request = EligibilityCheckRequestContent {
        control_number: "test-uhc-stedi-rs".to_string(),
        trading_partner_service_id: "87726".to_string(),
        provider: Provider {
            organization_name: Some("Provider Name".to_string()),
            npi: Some("1999999984".to_string()),
            ..Default::default()
        },
        subscriber: RequestSubscriber {
            member_id: Some("UHC123456".to_string()),
            first_name: Some("Jane".to_string()),
            last_name: Some("Doe".to_string()),
            date_of_birth: Some("19710101".to_string()),
            ..Default::default()
        },
        encounter: Some(Encounter {
            service_type_codes: Some(vec![
                RequestEligibilityServiceTypeCode::HealthBenefitPlanCoverage,
            ]),
            ..Default::default()
        }),
        ..Default::default()
    };

    println!("Making API call to Stedi...");
    let start = std::time::Instant::now();

    let result = eligibility_check::eligibility_check(&config, request).await;

    let elapsed = start.elapsed();
    println!("Request completed in {:.2} seconds", elapsed.as_secs_f64());
    println!();

    match result {
        Ok(response) => {
            println!("✓ SUCCESS - Eligibility check completed!");
            println!();

            // Print basic info
            if let Some(trading_partner) = &response.trading_partner_service_id {
                println!("Trading Partner ID: {}", trading_partner);
            }

            if let Some(plan_status) = &response.plan_status {
                println!("Plan Status: {:?}", plan_status);
            }

            // Print subscriber info
            if let Some(subscriber) = &response.subscriber {
                println!(
                    "Subscriber: {} {}",
                    subscriber.first_name.as_deref().unwrap_or(""),
                    subscriber.last_name.as_deref().unwrap_or("")
                );
                if let Some(group_num) = &subscriber.group_number {
                    println!("Group Number: {}", group_num);
                }
            }

            println!();

            // Print benefits information
            if let Some(benefits) = &response.benefits_information {
                println!("Benefits Information ({} entries):", benefits.len());
                println!();

                // Find and print plan coverage
                let plan_names: Vec<&String> = benefits
                    .iter()
                    .filter_map(|b| b.plan_coverage.as_ref())
                    .collect();

                if !plan_names.is_empty() {
                    println!("Plan Coverage: {}", plan_names[0]);
                }

                // Count unique service types
                let mut service_type_codes: Vec<String> = benefits
                    .iter()
                    .filter_map(|b| b.service_type_codes.as_ref())
                    .flatten()
                    .map(|code| format!("{:?}", code))
                    .collect();
                service_type_codes.sort();
                service_type_codes.dedup();

                let mut service_types: Vec<String> = benefits
                    .iter()
                    .filter_map(|b| b.service_types.as_ref())
                    .flatten()
                    .map(|st| format!("{:?}", st))
                    .collect();
                service_types.sort();
                service_types.dedup();

                println!(
                    "Unique Service Type Codes ({}): {}",
                    service_type_codes.len(),
                    service_type_codes.join(", ")
                );
                println!();
                println!(
                    "Unique Service Types ({}): {}",
                    service_types.len(),
                    service_types.join(", ")
                );
                println!();

                // Print first 5 benefits as examples
                println!("Sample Benefits:");
                for (i, benefit) in benefits.iter().take(5).enumerate() {
                    println!(
                        "  {}. {} - {:?}",
                        i + 1,
                        benefit
                            .name
                            .as_ref()
                            .map(|n| format!("{:?}", n))
                            .unwrap_or_else(|| "N/A".to_string()),
                        benefit
                            .code
                            .as_ref()
                            .map(|c| format!("{:?}", c))
                            .unwrap_or_else(|| "N/A".to_string())
                    );
                    if let Some(service_types) =
                        benefit.service_types.as_ref().filter(|st| !st.is_empty())
                    {
                        println!("     Service Types: {:?}", service_types);
                    }
                }

                if benefits.len() > 5 {
                    println!("  ... and {} more", benefits.len() - 5);
                }
            } else {
                println!("⚠ No benefits information in response");
            }

            println!();

            // Serialize and print the full response JSON
            match serde_json::to_string_pretty(&response) {
                Ok(json) => {
                    println!("=== Full Response JSON ===");
                    println!("{}", json);
                    println!();
                }
                Err(e) => {
                    eprintln!("Warning: Failed to serialize response to JSON: {}", e);
                }
            }

            println!("=== Test Passed ===");

            // Verify key assertions
            assert!(
                response.benefits_information.is_some(),
                "Benefits information should be present"
            );
            assert_eq!(
                response.trading_partner_service_id,
                Some("87726".to_string())
            );
        }
        Err(e) => {
            eprintln!("✗ FAILED - Eligibility check failed!");
            eprintln!();
            eprintln!("Error details: {:?}", e);
            eprintln!();
            eprintln!("This could be due to:");
            eprintln!("  1. Invalid API key");
            eprintln!("  2. Network timeout (check if timeout > 60s)");
            eprintln!("  3. Invalid payer ID or member information");
            eprintln!("  4. Payer connectivity issues");
            eprintln!();

            panic!("Eligibility check failed: {:?}", e);
        }
    }
}
