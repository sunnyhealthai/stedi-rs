use serde::{Deserialize, Serialize};

/// StateOrProvinceCode : The US state or Canadian province code. For example, `TN` for Tennessee or `NB` for New Brunswick.  Payers may sometimes return other non-compliant values.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum StateOrProvinceCode {
    /// Newfoundland and Labrador (Canadian province)
    #[serde(rename = "NL")]
    #[default]
    Nl,
    /// Prince Edward Island (Canadian province)
    #[serde(rename = "PE")]
    Pe,
    /// Nova Scotia (Canadian province)
    #[serde(rename = "NS")]
    Ns,
    /// New Brunswick (Canadian province)
    #[serde(rename = "NB")]
    Nb,
    /// Quebec (Canadian province)
    #[serde(rename = "QC")]
    Qc,
    /// Ontario (Canadian province)
    #[serde(rename = "ON")]
    On,
    /// Manitoba (Canadian province)
    #[serde(rename = "MB")]
    Mb,
    /// Saskatchewan (Canadian province)
    #[serde(rename = "SK")]
    Sk,
    /// Alberta (Canadian province)
    #[serde(rename = "AB")]
    Ab,
    /// British Columbia (Canadian province)
    #[serde(rename = "BC")]
    Bc,
    /// Yukon (Canadian territory)
    #[serde(rename = "YT")]
    Yt,
    /// Northwest Territories (Canadian territory)
    #[serde(rename = "NT")]
    Nt,
    /// Nunavut (Canadian territory)
    #[serde(rename = "NU")]
    Nu,
    /// District of Columbia (US federal district)
    #[serde(rename = "DC")]
    Dc,
    /// American Samoa (US territory)
    #[serde(rename = "AS")]
    As,
    /// Guam (US territory)
    #[serde(rename = "GU")]
    Gu,
    /// Northern Mariana Islands (US territory)
    #[serde(rename = "MP")]
    Mp,
    /// Puerto Rico (US territory)
    #[serde(rename = "PR")]
    Pr,
    /// U.S. Minor Outlying Islands (US territory)
    #[serde(rename = "UM")]
    Um,
    /// U.S. Virgin Islands (US territory)
    #[serde(rename = "VI")]
    Vi,
    /// Alaska (US state)
    #[serde(rename = "AK")]
    Ak,
    /// Alabama (US state)
    #[serde(rename = "AL")]
    Al,
    /// Arkansas (US state)
    #[serde(rename = "AR")]
    Ar,
    /// Arizona (US state)
    #[serde(rename = "AZ")]
    Az,
    /// California (US state)
    #[serde(rename = "CA")]
    Ca,
    /// Colorado (US state)
    #[serde(rename = "CO")]
    Co,
    /// Connecticut (US state)
    #[serde(rename = "CT")]
    Ct,
    /// Delaware (US state)
    #[serde(rename = "DE")]
    De,
    /// Florida (US state)
    #[serde(rename = "FL")]
    Fl,
    /// Georgia (US state)
    #[serde(rename = "GA")]
    Ga,
    /// Hawaii (US state)
    #[serde(rename = "HI")]
    Hi,
    /// Iowa (US state)
    #[serde(rename = "IA")]
    Ia,
    /// Idaho (US state)
    #[serde(rename = "ID")]
    Id,
    /// Illinois (US state)
    #[serde(rename = "IL")]
    Il,
    /// Indiana (US state)
    #[serde(rename = "IN")]
    In,
    /// Kansas (US state)
    #[serde(rename = "KS")]
    Ks,
    /// Kentucky (US state)
    #[serde(rename = "KY")]
    Ky,
    /// Louisiana (US state)
    #[serde(rename = "LA")]
    La,
    /// Massachusetts (US state)
    #[serde(rename = "MA")]
    Ma,
    /// Maryland (US state)
    #[serde(rename = "MD")]
    Md,
    /// Maine (US state)
    #[serde(rename = "ME")]
    Me,
    /// Michigan (US state)
    #[serde(rename = "MI")]
    Mi,
    /// Minnesota (US state)
    #[serde(rename = "MN")]
    Mn,
    /// Missouri (US state)
    #[serde(rename = "MO")]
    Mo,
    /// Mississippi (US state)
    #[serde(rename = "MS")]
    Ms,
    /// Montana (US state)
    #[serde(rename = "MT")]
    Mt,
    /// North Carolina (US state)
    #[serde(rename = "NC")]
    Nc,
    /// North Dakota (US state)
    #[serde(rename = "ND")]
    Nd,
    /// Nebraska (US state)
    #[serde(rename = "NE")]
    Ne,
    /// New Hampshire (US state)
    #[serde(rename = "NH")]
    Nh,
    /// New Jersey (US state)
    #[serde(rename = "NJ")]
    Nj,
    /// New Mexico (US state)
    #[serde(rename = "NM")]
    Nm,
    /// Nevada (US state)
    #[serde(rename = "NV")]
    Nv,
    /// New York (US state)
    #[serde(rename = "NY")]
    Ny,
    /// Ohio (US state)
    #[serde(rename = "OH")]
    Oh,
    /// Oklahoma (US state)
    #[serde(rename = "OK")]
    Ok,
    /// Oregon (US state)
    #[serde(rename = "OR")]
    Or,
    /// Pennsylvania (US state)
    #[serde(rename = "PA")]
    Pa,
    /// Rhode Island (US state)
    #[serde(rename = "RI")]
    Ri,
    /// South Carolina (US state)
    #[serde(rename = "SC")]
    Sc,
    /// South Dakota (US state)
    #[serde(rename = "SD")]
    Sd,
    /// Tennessee (US state)
    #[serde(rename = "TN")]
    Tn,
    /// Texas (US state)
    #[serde(rename = "TX")]
    Tx,
    /// Utah (US state)
    #[serde(rename = "UT")]
    Ut,
    /// Virginia (US state)
    #[serde(rename = "VA")]
    Va,
    /// Vermont (US state)
    #[serde(rename = "VT")]
    Vt,
    /// Washington (US state)
    #[serde(rename = "WA")]
    Wa,
    /// Wisconsin (US state)
    #[serde(rename = "WI")]
    Wi,
    /// West Virginia (US state)
    #[serde(rename = "WV")]
    Wv,
    /// Wyoming (US state)
    #[serde(rename = "WY")]
    Wy,
}

impl std::fmt::Display for StateOrProvinceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Nl => write!(f, "NL"),
            Self::Pe => write!(f, "PE"),
            Self::Ns => write!(f, "NS"),
            Self::Nb => write!(f, "NB"),
            Self::Qc => write!(f, "QC"),
            Self::On => write!(f, "ON"),
            Self::Mb => write!(f, "MB"),
            Self::Sk => write!(f, "SK"),
            Self::Ab => write!(f, "AB"),
            Self::Bc => write!(f, "BC"),
            Self::Yt => write!(f, "YT"),
            Self::Nt => write!(f, "NT"),
            Self::Nu => write!(f, "NU"),
            Self::Dc => write!(f, "DC"),
            Self::As => write!(f, "AS"),
            Self::Gu => write!(f, "GU"),
            Self::Mp => write!(f, "MP"),
            Self::Pr => write!(f, "PR"),
            Self::Um => write!(f, "UM"),
            Self::Vi => write!(f, "VI"),
            Self::Ak => write!(f, "AK"),
            Self::Al => write!(f, "AL"),
            Self::Ar => write!(f, "AR"),
            Self::Az => write!(f, "AZ"),
            Self::Ca => write!(f, "CA"),
            Self::Co => write!(f, "CO"),
            Self::Ct => write!(f, "CT"),
            Self::De => write!(f, "DE"),
            Self::Fl => write!(f, "FL"),
            Self::Ga => write!(f, "GA"),
            Self::Hi => write!(f, "HI"),
            Self::Ia => write!(f, "IA"),
            Self::Id => write!(f, "ID"),
            Self::Il => write!(f, "IL"),
            Self::In => write!(f, "IN"),
            Self::Ks => write!(f, "KS"),
            Self::Ky => write!(f, "KY"),
            Self::La => write!(f, "LA"),
            Self::Ma => write!(f, "MA"),
            Self::Md => write!(f, "MD"),
            Self::Me => write!(f, "ME"),
            Self::Mi => write!(f, "MI"),
            Self::Mn => write!(f, "MN"),
            Self::Mo => write!(f, "MO"),
            Self::Ms => write!(f, "MS"),
            Self::Mt => write!(f, "MT"),
            Self::Nc => write!(f, "NC"),
            Self::Nd => write!(f, "ND"),
            Self::Ne => write!(f, "NE"),
            Self::Nh => write!(f, "NH"),
            Self::Nj => write!(f, "NJ"),
            Self::Nm => write!(f, "NM"),
            Self::Nv => write!(f, "NV"),
            Self::Ny => write!(f, "NY"),
            Self::Oh => write!(f, "OH"),
            Self::Ok => write!(f, "OK"),
            Self::Or => write!(f, "OR"),
            Self::Pa => write!(f, "PA"),
            Self::Ri => write!(f, "RI"),
            Self::Sc => write!(f, "SC"),
            Self::Sd => write!(f, "SD"),
            Self::Tn => write!(f, "TN"),
            Self::Tx => write!(f, "TX"),
            Self::Ut => write!(f, "UT"),
            Self::Va => write!(f, "VA"),
            Self::Vt => write!(f, "VT"),
            Self::Wa => write!(f, "WA"),
            Self::Wi => write!(f, "WI"),
            Self::Wv => write!(f, "WV"),
            Self::Wy => write!(f, "WY"),
        }
    }
}
