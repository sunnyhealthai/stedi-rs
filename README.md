# stedi-rs &emsp; [![Latest Version]][crates.io] [![Documentation]][docs.rs] [![License]][mit]

[Latest Version]: https://img.shields.io/crates/v/stedi-rs.svg
[crates.io]: https://crates.io/crates/stedi-rs
[Documentation]: https://docs.rs/stedi-rs/badge.svg
[docs.rs]: https://docs.rs/stedi-rs
[License]: https://img.shields.io/badge/license-MIT-blue.svg
[mit]: https://opensource.org/licenses/MIT

**Community-maintained Rust SDK for Stedi's Healthcare Eligibility API.**

> **Note**: This is a community-maintained crate and is not officially supported by Stedi.

---

You may be looking for:

- [stedi-rs documentation](https://docs.rs/stedi-rs)
- [Stedi Healthcare Documentation](https://www.stedi.com/docs/healthcare)
- [API Reference](https://www.stedi.com/docs/api-reference/healthcare)
- [Eligibility Check Guide](https://www.stedi.com/docs/healthcare/send-eligibility-checks)

## stedi-rs in action

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
stedi-rs = "0.0.1"
tokio = { version = "1.48", features = ["full"] }
```

</details>

<p></p>

```rust
use stedi_rs::apis::{configuration::Configuration, eligibility_check};
use stedi_rs::models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("STEDI_API_KEY")?;
    let config = Configuration::with_api_key(api_key);

    let request = EligibilityCheckRequestContent {
        control_number: "123456789".to_string(),
        trading_partner_service_id: "87726".to_string(),
        provider: Provider {
            organization_name: Some("ACME Health Services".to_string()),
            npi: Some("1234567890".to_string()),
            ..Default::default()
        },
        subscriber: RequestSubscriber {
            member_id: Some("1234567890".to_string()),
            first_name: Some("Jane".to_string()),
            last_name: Some("Doe".to_string()),
            date_of_birth: Some("19800101".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    let response = eligibility_check::eligibility_check(&config, request).await?;

    if let Some(plan_status) = &response.plan_status {
        println!("Plan Status: {:?}", plan_status);
    }

    Ok(())
}
```

<br>

#### License

<sup>
Licensed under the <a href="LICENSE">MIT license</a>.
</sup>
