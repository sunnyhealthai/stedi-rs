use stedi_rs::models::EligibilityCheckResponseContent;

#[test]
fn test_guardian_dental_response_deserialization() {
    // This is the actual response structure from Guardian dental insurance (EOFHN)

    let guardian_response = r#"{
  "planDateInformation": {
    "eligibilityBegin": "20251001",
    "eligibilityEnd": "20501231"
  },
  "tradingPartnerServiceId": "EOFHN",
  "subscriber": {
    "lastName": "DOE",
    "groupNumber": "00000001",
    "firstName": "JOHN",
    "memberId": "123456789",
    "entityIdentifier": "Insured or Subscriber",
    "gender": "M",
    "dateOfBirth": "19900101",
    "entityType": "Person",
    "address": {
      "postalCode": "12345",
      "address1": "123 MAIN ST",
      "city": "ANYCITY",
      "state": "CA"
    },
    "groupDescription": "EXAMPLE COMPANY LLC"
  },
  "benefitsInformation": [
    {
      "additionalInformation": [
        {
          "description": "Your benefit period is from January 1 to December 31"
        }
      ],
      "code": "F",
      "name": "Limitations"
    },
    {
      "code": "1",
      "name": "Active Coverage",
      "planCoverage": "GUARDIAN",
      "serviceTypeCodes": ["35"],
      "serviceTypes": ["Dental Care"]
    },
    {
      "name": "Co-Insurance",
      "planCoverage": "DG GOLD, DG SILVER",
      "benefitPercent": "0",
      "code": "A",
      "additionalInformation": [
        {
          "description": "Deductible Waived"
        }
      ],
      "serviceTypeCodes": ["EI"],
      "inPlanNetworkIndicatorCode": "U",
      "inPlanNetworkIndicator": "Unknown",
      "benefitsDateInformation": {
        "benefitBegin": "20251001"
      },
      "serviceTypes": ["Fluoride Treatments"]
    },
    {
      "benefitPercent": "0",
      "code": "A",
      "name": "Co-Insurance",
      "serviceTypes": ["Dental Prophylaxis"],
      "serviceTypeCodes": ["EF"],
      "planCoverage": "DG GOLD, DG SILVER",
      "inPlanNetworkIndicator": "Unknown",
      "inPlanNetworkIndicatorCode": "U",
      "benefitsDateInformation": {
        "benefitBegin": "20251001"
      },
      "additionalInformation": [
        {
          "description": "Deductible Waived"
        }
      ]
    },
    {
      "code": "A",
      "planCoverage": "DG GOLD, DG SILVER",
      "benefitPercent": "0.1",
      "inPlanNetworkIndicatorCode": "U",
      "benefitsDateInformation": {
        "benefitBegin": "20251001"
      },
      "serviceTypes": ["Space Maintenance"],
      "inPlanNetworkIndicator": "Unknown",
      "additionalInformation": [
        {
          "description": "Deductible Applies"
        }
      ],
      "name": "Co-Insurance",
      "serviceTypeCodes": ["V13"]
    }
  ]
}"#;

    // Attempt to deserialize the response
    let result: Result<EligibilityCheckResponseContent, serde_json::Error> =
        serde_json::from_str(guardian_response);

    match &result {
        Ok(response) => {
            println!("✓ Successfully deserialized Guardian response!");
            println!(
                "  Trading Partner: {}",
                response.trading_partner_service_id.as_ref().unwrap()
            );
            println!(
                "  Benefits count: {}",
                response
                    .benefits_information
                    .as_ref()
                    .map(|b| b.len())
                    .unwrap_or(0)
            );

            // Verify specific service types are parsed
            if let Some(benefits) = &response.benefits_information {
                let service_types: Vec<String> = benefits
                    .iter()
                    .filter_map(|b| b.service_types.as_ref())
                    .flatten()
                    .map(|st| format!("{:?}", st))
                    .collect();
                println!("  Service types found: {}", service_types.join(", "));
            }
        }
        Err(e) => {
            eprintln!("✗ Deserialization failed!");
            eprintln!("  Error: {}", e);
            eprintln!("  Error location: line {}, column {}", e.line(), e.column());
        }
    }

    // Assert that deserialization succeeded
    assert!(
        result.is_ok(),
        "Failed to deserialize Guardian response: {:?}",
        result.err()
    );

    let response = result.unwrap();

    // Verify key fields
    assert_eq!(
        response.trading_partner_service_id,
        Some("EOFHN".to_string())
    );
    assert!(response.benefits_information.is_some());
    assert!(response.subscriber.is_some());
}
