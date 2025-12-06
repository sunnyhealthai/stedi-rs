use serde::{Deserialize, Serialize};

/// BenefitsServiceDelivery : The delivery or usage pattern for the benefits.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BenefitsServiceDelivery {
    /// Code specifying the delivery or calendar pattern for benefits
    #[serde(
        rename = "deliveryOrCalendarPatternCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_or_calendar_pattern_code: Option<super::DeliveryOrCalendarPatternQualifier>,
    /// Qualifier describing the delivery or calendar pattern
    #[serde(
        rename = "deliveryOrCalendarPatternQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_or_calendar_pattern_qualifier: Option<super::DeliveryOrCalendarPatternQualifier>,
    /// Code for the delivery or calendar pattern qualifier
    #[serde(
        rename = "deliveryOrCalendarPatternQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_or_calendar_pattern_qualifier_code:
        Option<super::DeliveryOrCalendarPatternQualifierCode>,
    /// Code specifying timing for deliveries
    #[serde(
        rename = "deliveryPatternTimeCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_pattern_time_code: Option<super::DeliveryPatternTimeQualifier>,
    /// Qualifier describing delivery timing patterns
    #[serde(
        rename = "deliveryPatternTimeQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_pattern_time_qualifier: Option<super::DeliveryPatternTimeQualifier>,
    /// Code specifying the time for routine shipments or deliveries. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#delivery-pattern-time-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "deliveryPatternTimeQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_pattern_time_qualifier_code: Option<super::DeliveryPatternTimeQualifierCode>,
    /// The number of periods in the time period. For example, `12` when the `timePeriodQualifier` is `Hour`.
    #[serde(rename = "numOfPeriods", skip_serializing_if = "Option::is_none")]
    pub num_of_periods: Option<String>,
    /// The quantity of the benefit. For example, `10` when the `quantityQualifier` is `Visits`.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// The name of the `quantityQualifierCode`. For example, `Days`.  Payers may sometimes return other non-compliant values.
    #[serde(rename = "quantityQualifier", skip_serializing_if = "Option::is_none")]
    pub quantity_qualifier: Option<super::BenefitsServiceDeliveryQuantityQualifier>,
    /// Code specifying the type of quantity for the benefit. Can be `DY` - Days, `FL` - Units, `HS` - Hours, `MN` - Month, and `VS` - Visits.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "quantityQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_qualifier_code: Option<super::BenefitsServiceDeliveryQuantityQualifierCode>,
    /// Specifies the sampling frequency, based on the unit of measure. For example `every 2 months` or `once per calendar year`.
    #[serde(
        rename = "sampleSelectionModulus",
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_selection_modulus: Option<String>,
    /// The name of the `timePeriodQualifierCode`. For example, `Calendar Year`.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "timePeriodQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_period_qualifier: Option<super::TimePeriodQualifier>,
    /// Code specifying the time period for the benefit information. Visit [Eligibility code lists](https://www.stedi.com/docs/healthcare/eligibility-code-lists#time-qualifier-codes) for a complete list.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "timePeriodQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_period_qualifier_code: Option<super::TimePeriodQualifierCode>,
    /// Code specifying the unit of measurement
    #[serde(
        rename = "unitForMeasurementCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_for_measurement_code: Option<super::UnitForMeasurement>,
    /// The name of the `unitForMeasurementQualifierCode`. For example, `Days`.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "unitForMeasurementQualifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_for_measurement_qualifier: Option<super::UnitForMeasurement>,
    /// Code specifying the unit of measurement. For example, `DA` - Days, `MO` - Months, `VS` - Visits, `WK` - Week, and `YR` - Years.  Payers may sometimes return other non-compliant values.
    #[serde(
        rename = "unitForMeasurementQualifierCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_for_measurement_qualifier_code: Option<super::UnitForMeasurementCode>,
}

impl BenefitsServiceDelivery {
    /// The delivery or usage pattern for the benefits.
    pub fn new() -> BenefitsServiceDelivery {
        BenefitsServiceDelivery {
            delivery_or_calendar_pattern_code: None,
            delivery_or_calendar_pattern_qualifier: None,
            delivery_or_calendar_pattern_qualifier_code: None,
            delivery_pattern_time_code: None,
            delivery_pattern_time_qualifier: None,
            delivery_pattern_time_qualifier_code: None,
            num_of_periods: None,
            quantity: None,
            quantity_qualifier: None,
            quantity_qualifier_code: None,
            sample_selection_modulus: None,
            time_period_qualifier: None,
            time_period_qualifier_code: None,
            unit_for_measurement_code: None,
            unit_for_measurement_qualifier: None,
            unit_for_measurement_qualifier_code: None,
        }
    }
}
