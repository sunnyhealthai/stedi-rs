use serde::{Deserialize, Serialize};

/// InfallibleStateOrProvinceCode : The US state or Canadian province code with unknown option. For example, `TN` for Tennessee or `NB` for New Brunswick.  Payers may sometimes return other non-compliant values.
/// The US state or Canadian province code with unknown option. For example, `TN` for Tennessee or `NB` for New Brunswick.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum InfallibleStateOrProvinceCode {
    /// Newfoundland and Labrador
    #[serde(rename = "NL")]
    #[default]
    Newfoundland,
    /// Prince Edward Island
    #[serde(rename = "PE")]
    PrinceEdwardIsland,
    /// Nova Scotia
    #[serde(rename = "NS")]
    NovaScotia,
    /// New Brunswick
    #[serde(rename = "NB")]
    NewBrunswick,
    /// Quebec
    #[serde(rename = "QC")]
    Quebec,
    /// Ontario
    #[serde(rename = "ON")]
    Ontario,
    /// Manitoba
    #[serde(rename = "MB")]
    Manitoba,
    /// Saskatchewan
    #[serde(rename = "SK")]
    Saskatchewan,
    /// Alberta
    #[serde(rename = "AB")]
    Alberta,
    /// British Columbia
    #[serde(rename = "BC")]
    BritishColumbia,
    /// Yukon
    #[serde(rename = "YT")]
    Yukon,
    /// Northwest Territories
    #[serde(rename = "NT")]
    NorthwestTerritories,
    /// Nunavut
    #[serde(rename = "NU")]
    Nunavut,
    /// Washington D.C.
    #[serde(rename = "DC")]
    WashingtonDc,
    /// American Samoa
    #[serde(rename = "AS")]
    AmericanSamoa,
    /// Guam
    #[serde(rename = "GU")]
    Guam,
    /// Northern Mariana Islands
    #[serde(rename = "MP")]
    NorthernMarianaIslands,
    /// Puerto Rico
    #[serde(rename = "PR")]
    PuertoRico,
    /// United States Minor Outlying Islands
    #[serde(rename = "UM")]
    UnitedStatesMinorOutlyingIslands,
    /// Virgin Islands
    #[serde(rename = "VI")]
    VirginIslands,
    /// Alaska
    #[serde(rename = "AK")]
    Alaska,
    /// Alabama
    #[serde(rename = "AL")]
    Alabama,
    /// Arkansas
    #[serde(rename = "AR")]
    Arkansas,
    /// Arizona
    #[serde(rename = "AZ")]
    Arizona,
    /// California
    #[serde(rename = "CA")]
    California,
    /// Colorado
    #[serde(rename = "CO")]
    Colorado,
    /// Connecticut
    #[serde(rename = "CT")]
    Connecticut,
    /// Delaware
    #[serde(rename = "DE")]
    Delaware,
    /// Florida
    #[serde(rename = "FL")]
    Florida,
    /// Georgia
    #[serde(rename = "GA")]
    Georgia,
    /// Hawaii
    #[serde(rename = "HI")]
    Hawaii,
    /// Iowa
    #[serde(rename = "IA")]
    Iowa,
    /// Idaho
    #[serde(rename = "ID")]
    Idaho,
    /// Illinois
    #[serde(rename = "IL")]
    Illinois,
    /// Indiana
    #[serde(rename = "IN")]
    Indiana,
    /// Kansas
    #[serde(rename = "KS")]
    Kansas,
    /// Kentucky
    #[serde(rename = "KY")]
    Kentucky,
    /// Louisiana
    #[serde(rename = "LA")]
    Louisiana,
    /// Massachusetts
    #[serde(rename = "MA")]
    Massachusetts,
    /// Maryland
    #[serde(rename = "MD")]
    Maryland,
    /// Maine
    #[serde(rename = "ME")]
    Maine,
    /// Michigan
    #[serde(rename = "MI")]
    Michigan,
    /// Minnesota
    #[serde(rename = "MN")]
    Minnesota,
    /// Missouri
    #[serde(rename = "MO")]
    Missouri,
    /// Mississippi
    #[serde(rename = "MS")]
    Mississippi,
    /// Montana
    #[serde(rename = "MT")]
    Montana,
    /// North Carolina
    #[serde(rename = "NC")]
    NorthCarolina,
    /// North Dakota
    #[serde(rename = "ND")]
    NorthDakota,
    /// Nebraska
    #[serde(rename = "NE")]
    Nebraska,
    /// New Hampshire
    #[serde(rename = "NH")]
    NewHampshire,
    /// New Jersey
    #[serde(rename = "NJ")]
    NewJersey,
    /// New Mexico
    #[serde(rename = "NM")]
    NewMexico,
    /// Nevada
    #[serde(rename = "NV")]
    Nevada,
    /// New York
    #[serde(rename = "NY")]
    NewYork,
    /// Ohio
    #[serde(rename = "OH")]
    Ohio,
    /// Oklahoma
    #[serde(rename = "OK")]
    Oklahoma,
    /// Oregon
    #[serde(rename = "OR")]
    Oregon,
    /// Pennsylvania
    #[serde(rename = "PA")]
    Pennsylvania,
    /// Rhode Island
    #[serde(rename = "RI")]
    RhodeIsland,
    /// South Carolina
    #[serde(rename = "SC")]
    SouthCarolina,
    /// South Dakota
    #[serde(rename = "SD")]
    SouthDakota,
    /// Tennessee
    #[serde(rename = "TN")]
    Tennessee,
    /// Texas
    #[serde(rename = "TX")]
    Texas,
    /// Utah
    #[serde(rename = "UT")]
    Utah,
    /// Virginia
    #[serde(rename = "VA")]
    Virginia,
    /// Vermont
    #[serde(rename = "VT")]
    Vermont,
    /// Washington
    #[serde(rename = "WA")]
    Washington,
    /// Wisconsin
    #[serde(rename = "WI")]
    Wisconsin,
    /// West Virginia
    #[serde(rename = "WV")]
    WestVirginia,
    /// Wyoming
    #[serde(rename = "WY")]
    Wyoming,
}

impl std::fmt::Display for InfallibleStateOrProvinceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Newfoundland => write!(f, "NL"),
            Self::PrinceEdwardIsland => write!(f, "PE"),
            Self::NovaScotia => write!(f, "NS"),
            Self::NewBrunswick => write!(f, "NB"),
            Self::Quebec => write!(f, "QC"),
            Self::Ontario => write!(f, "ON"),
            Self::Manitoba => write!(f, "MB"),
            Self::Saskatchewan => write!(f, "SK"),
            Self::Alberta => write!(f, "AB"),
            Self::BritishColumbia => write!(f, "BC"),
            Self::Yukon => write!(f, "YT"),
            Self::NorthwestTerritories => write!(f, "NT"),
            Self::Nunavut => write!(f, "NU"),
            Self::WashingtonDc => write!(f, "DC"),
            Self::AmericanSamoa => write!(f, "AS"),
            Self::Guam => write!(f, "GU"),
            Self::NorthernMarianaIslands => write!(f, "MP"),
            Self::PuertoRico => write!(f, "PR"),
            Self::UnitedStatesMinorOutlyingIslands => write!(f, "UM"),
            Self::VirginIslands => write!(f, "VI"),
            Self::Alaska => write!(f, "AK"),
            Self::Alabama => write!(f, "AL"),
            Self::Arkansas => write!(f, "AR"),
            Self::Arizona => write!(f, "AZ"),
            Self::California => write!(f, "CA"),
            Self::Colorado => write!(f, "CO"),
            Self::Connecticut => write!(f, "CT"),
            Self::Delaware => write!(f, "DE"),
            Self::Florida => write!(f, "FL"),
            Self::Georgia => write!(f, "GA"),
            Self::Hawaii => write!(f, "HI"),
            Self::Iowa => write!(f, "IA"),
            Self::Idaho => write!(f, "ID"),
            Self::Illinois => write!(f, "IL"),
            Self::Indiana => write!(f, "IN"),
            Self::Kansas => write!(f, "KS"),
            Self::Kentucky => write!(f, "KY"),
            Self::Louisiana => write!(f, "LA"),
            Self::Massachusetts => write!(f, "MA"),
            Self::Maryland => write!(f, "MD"),
            Self::Maine => write!(f, "ME"),
            Self::Michigan => write!(f, "MI"),
            Self::Minnesota => write!(f, "MN"),
            Self::Missouri => write!(f, "MO"),
            Self::Mississippi => write!(f, "MS"),
            Self::Montana => write!(f, "MT"),
            Self::NorthCarolina => write!(f, "NC"),
            Self::NorthDakota => write!(f, "ND"),
            Self::Nebraska => write!(f, "NE"),
            Self::NewHampshire => write!(f, "NH"),
            Self::NewJersey => write!(f, "NJ"),
            Self::NewMexico => write!(f, "NM"),
            Self::Nevada => write!(f, "NV"),
            Self::NewYork => write!(f, "NY"),
            Self::Ohio => write!(f, "OH"),
            Self::Oklahoma => write!(f, "OK"),
            Self::Oregon => write!(f, "OR"),
            Self::Pennsylvania => write!(f, "PA"),
            Self::RhodeIsland => write!(f, "RI"),
            Self::SouthCarolina => write!(f, "SC"),
            Self::SouthDakota => write!(f, "SD"),
            Self::Tennessee => write!(f, "TN"),
            Self::Texas => write!(f, "TX"),
            Self::Utah => write!(f, "UT"),
            Self::Virginia => write!(f, "VA"),
            Self::Vermont => write!(f, "VT"),
            Self::Washington => write!(f, "WA"),
            Self::Wisconsin => write!(f, "WI"),
            Self::WestVirginia => write!(f, "WV"),
            Self::Wyoming => write!(f, "WY"),
        }
    }
}
