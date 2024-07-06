use crate::Error;

/// Two-letter country codes defined in ISO 3166-1 alpha-2 .
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Country {
    #[cfg(feature = "AO")]
    /// Angola
    AO,
    #[cfg(feature = "AR")]
    /// Argentina
    AR,
    #[cfg(feature = "AM")]
    /// Armenia
    AM,
    #[cfg(feature = "AW")]
    /// Aruba
    AW,
    #[cfg(feature = "AU")]
    /// Australia
    AU,
    #[cfg(feature = "AT")]
    /// Austria
    AT,
    #[cfg(feature = "AZ")]
    /// Azerbaijan
    AZ,
    #[cfg(feature = "BD")]
    /// Bangladesh
    BD,
    #[cfg(feature = "BY")]
    /// Belarus
    BY,
    #[cfg(feature = "BE")]
    /// Belgium
    BE,
    #[cfg(feature = "BO")]
    /// Bolivia
    BO,
    #[cfg(feature = "BA")]
    /// Bosnia and Herzegovina
    BA,
    #[cfg(feature = "BW")]
    /// Botswana
    BW,
    #[cfg(feature = "BR")]
    /// Brazil
    BR,
    #[cfg(feature = "BG")]
    /// Bulgaria
    BG,
    #[cfg(feature = "BI")]
    /// Burundi
    BI,
    #[cfg(feature = "CA")]
    /// Canada
    CA,
    #[cfg(feature = "CL")]
    /// Chile
    CL,
    #[cfg(feature = "CN")]
    /// China
    CN,
    #[cfg(feature = "CO")]
    /// Colombia
    CO,
    #[cfg(feature = "HR")]
    /// Croatia
    HR,
    #[cfg(feature = "CU")]
    /// Cuba
    CU,
    #[cfg(feature = "CW")]
    /// Curaçao
    CW,
    #[cfg(feature = "CY")]
    /// Cyprus
    CY,
    #[cfg(feature = "CZ")]
    /// Czechia
    CZ,
    #[cfg(feature = "DK")]
    /// Denmark
    DK,
    #[cfg(feature = "DJ")]
    /// Djibouti
    DJ,
    #[cfg(feature = "DO")]
    /// Dominican Republic
    DO,
    #[cfg(feature = "EG")]
    /// Egypt
    EG,
    #[cfg(feature = "EE")]
    /// Estonia
    EE,
    #[cfg(feature = "ET")]
    /// Ethiopia
    ET,
    #[cfg(feature = "FI")]
    /// Finland
    FI,
    #[cfg(feature = "FR")]
    /// France
    FR,
    #[cfg(feature = "GE")]
    /// Georgia
    GE,
    #[cfg(feature = "DE")]
    /// Germany
    DE,
    #[cfg(feature = "DE")]
    /// Germany (Brandenburg)
    DE_BB,
    #[cfg(feature = "DE")]
    /// Germany (Berlin)
    DE_BE,
    #[cfg(feature = "DE")]
    /// Germany (Baden-Württemberg)
    DE_BW,
    #[cfg(feature = "DE")]
    /// Germany (Bavaria (Bayern))
    DE_BY,
    #[cfg(feature = "DE")]
    /// Germany (Bavaria (Bayern) with more protestants)
    DE_BYP,
    #[cfg(feature = "DE")]
    /// Germany (Bremen)
    DE_HB,
    #[cfg(feature = "DE")]
    /// Germany (Hesse (Hessen))
    DE_HE,
    #[cfg(feature = "DE")]
    /// Germany (Hamburg)
    DE_HH,
    #[cfg(feature = "DE")]
    /// Germany (Mecklenburg-Vorpommern)
    DE_MV,
    #[cfg(feature = "DE")]
    /// Germany (Lower Saxony (Niedersachsen))
    DE_NI,
    #[cfg(feature = "DE")]
    /// Germany (North Rhine-Westphalia (Nordrhein-Westfalen))
    DE_NW,
    #[cfg(feature = "DE")]
    /// Germany (Rhineland-Palatinate (Rheinland-Pfalz))
    DE_RP,
    #[cfg(feature = "DE")]
    /// Germany (Schleswig-Holstein)
    DE_SH,
    #[cfg(feature = "DE")]
    /// Germany (Saarland)
    DE_SL,
    #[cfg(feature = "DE")]
    /// Germany (Saxony (Sachsen))
    DE_SN,
    #[cfg(feature = "DE")]
    /// Germany (Saxony-Anhalt (Sachsen-Anhalt))
    DE_ST,
    #[cfg(feature = "DE")]
    /// Germany (Thuringia (Thüringen))
    DE_TH,
    #[cfg(feature = "GR")]
    /// Greece
    GR,
    #[cfg(feature = "HN")]
    /// Honduras
    HN,
    #[cfg(feature = "HK")]
    /// Hong Kong
    HK,
    #[cfg(feature = "HU")]
    /// Hungary
    HU,
    #[cfg(feature = "IS")]
    /// Iceland
    IS,
    #[cfg(feature = "IN")]
    /// India
    IN,
    #[cfg(feature = "ID")]
    /// Indonesia
    ID,
    #[cfg(feature = "IE")]
    /// Ireland
    IE,
    #[cfg(feature = "IM")]
    /// Isle of Man
    IM,
    #[cfg(feature = "IL")]
    /// Israel
    IL,
    #[cfg(feature = "IT")]
    /// Italy
    IT,
    #[cfg(feature = "JM")]
    /// Jamaica
    JM,
    #[cfg(feature = "JP")]
    /// Japan
    JP,
    #[cfg(feature = "KZ")]
    /// Kazakhstan
    KZ,
    #[cfg(feature = "KE")]
    /// Kenya
    KE,
    #[cfg(feature = "LV")]
    /// Latvia
    LV,
    #[cfg(feature = "LS")]
    /// Lesotho
    LS,
    #[cfg(feature = "LI")]
    /// Liechtenstein
    LI,
    #[cfg(feature = "LT")]
    /// Lithuania
    LT,
    #[cfg(feature = "LU")]
    /// Luxembourg
    LU,
    #[cfg(feature = "MG")]
    /// Madagascar
    MG,
    #[cfg(feature = "MY")]
    /// Malaysia
    MY,
    #[cfg(feature = "MW")]
    /// Malawi
    MW,
    #[cfg(feature = "MT")]
    /// Malta
    MT,
    #[cfg(feature = "MX")]
    /// Mexico
    MX,
    #[cfg(feature = "MD")]
    /// Moldova
    MD,
    #[cfg(feature = "MA")]
    /// Morocco
    MA,
    #[cfg(feature = "MZ")]
    /// Mozambique
    MZ,
    #[cfg(feature = "NL")]
    /// Netherlands
    NL,
    #[cfg(feature = "NA")]
    /// Namibia
    NA,
    #[cfg(feature = "NZ")]
    /// New Zealand
    NZ,
    #[cfg(feature = "NI")]
    /// Nicaragua
    NI,
    #[cfg(feature = "NG")]
    /// Nigeria
    NG,
    #[cfg(feature = "MK")]
    /// North Macedonia
    MK,
    #[cfg(feature = "NO")]
    /// Norway
    NO,
    #[cfg(feature = "PK")]
    /// Pakistan
    PK,
    #[cfg(feature = "PY")]
    /// Paraguay
    PY,
    #[cfg(feature = "PE")]
    /// Peru
    PE,
    #[cfg(feature = "PL")]
    /// Poland
    PL,
    #[cfg(feature = "PT")]
    /// Portugal
    PT,
    #[cfg(feature = "RO")]
    /// Romania
    RO,
    #[cfg(feature = "RU")]
    /// Russia
    RU,
    #[cfg(feature = "SA")]
    /// Saudi Arabia
    SA,
    #[cfg(feature = "RS")]
    /// Serbia
    RS,
    #[cfg(feature = "SG")]
    /// Singapore
    SG,
    #[cfg(feature = "SK")]
    /// Slovakia
    SK,
    #[cfg(feature = "SI")]
    /// Slovenia
    SI,
    #[cfg(feature = "ZA")]
    /// South Africa
    ZA,
    #[cfg(feature = "KR")]
    /// South Korea
    KR,
    #[cfg(feature = "ES")]
    /// Spain
    ES,
    #[cfg(feature = "SZ")]
    /// Swaziland
    SZ,
    #[cfg(feature = "SE")]
    /// Sweden
    SE,
    #[cfg(feature = "CH")]
    /// Switzerland
    CH,
    #[cfg(feature = "TW")]
    /// Taiwan
    TW,
    #[cfg(feature = "TR")]
    /// Turkey
    TR,
    #[cfg(feature = "TN")]
    /// Tunisia
    TN,
    #[cfg(feature = "UA")]
    /// Ukraine
    UA,
    #[cfg(feature = "AE")]
    /// United Arab Emirates
    AE,
    #[cfg(feature = "GB")]
    /// United Kingdom
    GB,
    #[cfg(feature = "US")]
    /// United States
    US,
    #[cfg(feature = "US")]
    /// United States (Alaska)
    US_AK,
    #[cfg(feature = "US")]
    /// United States (Alabama)
    US_AL,
    #[cfg(feature = "US")]
    /// United States (Arkansas)
    US_AR,
    #[cfg(feature = "US")]
    /// United States (American Samoa)
    US_AS,
    #[cfg(feature = "US")]
    /// United States (Arizona)
    US_AZ,
    #[cfg(feature = "US")]
    /// United States (California)
    US_CA,
    #[cfg(feature = "US")]
    /// United States (Colorado)
    US_CO,
    #[cfg(feature = "US")]
    /// United States (Connecticut)
    US_CT,
    #[cfg(feature = "US")]
    /// United States (District of Columbia)
    US_DC,
    #[cfg(feature = "US")]
    /// United States (Delaware)
    US_DE,
    #[cfg(feature = "US")]
    /// United States (Florida)
    US_FL,
    #[cfg(feature = "US")]
    /// United States (Georgia)
    US_GA,
    #[cfg(feature = "US")]
    /// United States (Guam)
    US_GU,
    #[cfg(feature = "US")]
    /// United States (Hawaii)
    US_HI,
    #[cfg(feature = "US")]
    /// United States (Iowa)
    US_IA,
    #[cfg(feature = "US")]
    /// United States (Idaho)
    US_ID,
    #[cfg(feature = "US")]
    /// United States (Illinois)
    US_IL,
    #[cfg(feature = "US")]
    /// United States (Indiana)
    US_IN,
    #[cfg(feature = "US")]
    /// United States (Kansas)
    US_KS,
    #[cfg(feature = "US")]
    /// United States (Kentucky)
    US_KY,
    #[cfg(feature = "US")]
    /// United States (Louisiana)
    US_LA,
    #[cfg(feature = "US")]
    /// United States (Massachusetts)
    US_MA,
    #[cfg(feature = "US")]
    /// United States (Maryland)
    US_MD,
    #[cfg(feature = "US")]
    /// United States (Maine)
    US_ME,
    #[cfg(feature = "US")]
    /// United States (Michigan)
    US_MI,
    #[cfg(feature = "US")]
    /// United States (Minnesota)
    US_MN,
    #[cfg(feature = "US")]
    /// United States (Missouri)
    US_MO,
    #[cfg(feature = "US")]
    /// United States (Northern Mariana Islands)
    US_MP,
    #[cfg(feature = "US")]
    /// United States (Mississippi)
    US_MS,
    #[cfg(feature = "US")]
    /// United States (Montana)
    US_MT,
    #[cfg(feature = "US")]
    /// United States (North Carolina)
    US_NC,
    #[cfg(feature = "US")]
    /// United States (North Dakota)
    US_ND,
    #[cfg(feature = "US")]
    /// United States (Nebraska)
    US_NE,
    #[cfg(feature = "US")]
    /// United States (New Hampshire)
    US_NH,
    #[cfg(feature = "US")]
    /// United States (New Jersey)
    US_NJ,
    #[cfg(feature = "US")]
    /// United States (New Mexico)
    US_NM,
    #[cfg(feature = "US")]
    /// United States (Nevada)
    US_NV,
    #[cfg(feature = "US")]
    /// United States (New York)
    US_NY,
    #[cfg(feature = "US")]
    /// United States (Ohio)
    US_OH,
    #[cfg(feature = "US")]
    /// United States (Oklahoma)
    US_OK,
    #[cfg(feature = "US")]
    /// United States (Oregon)
    US_OR,
    #[cfg(feature = "US")]
    /// United States (Pennsylvania)
    US_PA,
    #[cfg(feature = "US")]
    /// United States (Puerto Rico)
    US_PR,
    #[cfg(feature = "US")]
    /// United States (Rhode Island)
    US_RI,
    #[cfg(feature = "US")]
    /// United States (South Carolina)
    US_SC,
    #[cfg(feature = "US")]
    /// United States (South Dakota)
    US_SD,
    #[cfg(feature = "US")]
    /// United States (Tennessee)
    US_TN,
    #[cfg(feature = "US")]
    /// United States (Texas)
    US_TX,
    #[cfg(feature = "US")]
    /// United States (United States Minor Outlying Islands)
    US_UM,
    #[cfg(feature = "US")]
    /// United States (Utah)
    US_UT,
    #[cfg(feature = "US")]
    /// United States (Virginia)
    US_VA,
    #[cfg(feature = "US")]
    /// United States (Virgin Islands, U.S..)
    US_VI,
    #[cfg(feature = "US")]
    /// United States (Vermont)
    US_VT,
    #[cfg(feature = "US")]
    /// United States (Washington)
    US_WA,
    #[cfg(feature = "US")]
    /// United States (Wisconsin)
    US_WI,
    #[cfg(feature = "US")]
    /// United States (West Virginia)
    US_WV,
    #[cfg(feature = "US")]
    /// United States (Wyoming)
    US_WY,
    #[cfg(feature = "UY")]
    /// Uruguay
    UY,
    #[cfg(feature = "UZ")]
    /// Uzbekistan
    UZ,
    #[cfg(feature = "VE")]
    /// Venezuela
    VE,
    #[cfg(feature = "VN")]
    /// Vietnam
    VN,
    #[cfg(feature = "ZM")]
    /// Zambia
    ZM,
    #[cfg(feature = "ZW")]
    /// Zimbabwe
    ZW,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Country {
    #[allow(clippy::too_many_lines)]
    fn as_ref(&self) -> &str {
        match self {
            #[cfg(feature = "AO")]
            Country::AO => "AO",
            #[cfg(feature = "AR")]
            Country::AR => "AR",
            #[cfg(feature = "AM")]
            Country::AM => "AM",
            #[cfg(feature = "AW")]
            Country::AW => "AW",
            #[cfg(feature = "AU")]
            Country::AU => "AU",
            #[cfg(feature = "AT")]
            Country::AT => "AT",
            #[cfg(feature = "AZ")]
            Country::AZ => "AZ",
            #[cfg(feature = "BD")]
            Country::BD => "BD",
            #[cfg(feature = "BY")]
            Country::BY => "BY",
            #[cfg(feature = "BE")]
            Country::BE => "BE",
            #[cfg(feature = "BO")]
            Country::BO => "BO",
            #[cfg(feature = "BA")]
            Country::BA => "BA",
            #[cfg(feature = "BW")]
            Country::BW => "BW",
            #[cfg(feature = "BR")]
            Country::BR => "BR",
            #[cfg(feature = "BG")]
            Country::BG => "BG",
            #[cfg(feature = "BI")]
            Country::BI => "BI",
            #[cfg(feature = "CA")]
            Country::CA => "CA",
            #[cfg(feature = "CL")]
            Country::CL => "CL",
            #[cfg(feature = "CN")]
            Country::CN => "CN",
            #[cfg(feature = "CO")]
            Country::CO => "CO",
            #[cfg(feature = "HR")]
            Country::HR => "HR",
            #[cfg(feature = "CU")]
            Country::CU => "CU",
            #[cfg(feature = "CW")]
            Country::CW => "CW",
            #[cfg(feature = "CY")]
            Country::CY => "CY",
            #[cfg(feature = "CZ")]
            Country::CZ => "CZ",
            #[cfg(feature = "DK")]
            Country::DK => "DK",
            #[cfg(feature = "DJ")]
            Country::DJ => "DJ",
            #[cfg(feature = "DO")]
            Country::DO => "DO",
            #[cfg(feature = "EG")]
            Country::EG => "EG",
            #[cfg(feature = "EE")]
            Country::EE => "EE",
            #[cfg(feature = "ET")]
            Country::ET => "ET",
            #[cfg(feature = "FI")]
            Country::FI => "FI",
            #[cfg(feature = "FR")]
            Country::FR => "FR",
            #[cfg(feature = "GE")]
            Country::GE => "GE",
            #[cfg(feature = "DE")]
            Country::DE => "DE",
            #[cfg(feature = "DE")]
            Country::DE_BB => "DE_BB",
            #[cfg(feature = "DE")]
            Country::DE_BE => "DE_BE",
            #[cfg(feature = "DE")]
            Country::DE_BW => "DE_BW",
            #[cfg(feature = "DE")]
            Country::DE_BY => "DE_BY",
            #[cfg(feature = "DE")]
            Country::DE_BYP => "DE_BYP",
            #[cfg(feature = "DE")]
            Country::DE_HB => "DE_HB",
            #[cfg(feature = "DE")]
            Country::DE_HE => "DE_HE",
            #[cfg(feature = "DE")]
            Country::DE_HH => "DE_HH",
            #[cfg(feature = "DE")]
            Country::DE_MV => "DE_MV",
            #[cfg(feature = "DE")]
            Country::DE_NI => "DE_NI",
            #[cfg(feature = "DE")]
            Country::DE_NW => "DE_NW",
            #[cfg(feature = "DE")]
            Country::DE_RP => "DE_RP",
            #[cfg(feature = "DE")]
            Country::DE_SH => "DE_SH",
            #[cfg(feature = "DE")]
            Country::DE_SL => "DE_SL",
            #[cfg(feature = "DE")]
            Country::DE_SN => "DE_SN",
            #[cfg(feature = "DE")]
            Country::DE_ST => "DE_ST",
            #[cfg(feature = "DE")]
            Country::DE_TH => "DE_TH",
            #[cfg(feature = "GR")]
            Country::GR => "GR",
            #[cfg(feature = "HN")]
            Country::HN => "HN",
            #[cfg(feature = "HK")]
            Country::HK => "HK",
            #[cfg(feature = "HU")]
            Country::HU => "HU",
            #[cfg(feature = "IS")]
            Country::IS => "IS",
            #[cfg(feature = "IN")]
            Country::IN => "IN",
            #[cfg(feature = "ID")]
            Country::ID => "ID",
            #[cfg(feature = "IE")]
            Country::IE => "IE",
            #[cfg(feature = "IM")]
            Country::IM => "IM",
            #[cfg(feature = "IL")]
            Country::IL => "IL",
            #[cfg(feature = "IT")]
            Country::IT => "IT",
            #[cfg(feature = "JM")]
            Country::JM => "JM",
            #[cfg(feature = "JP")]
            Country::JP => "JP",
            #[cfg(feature = "KZ")]
            Country::KZ => "KZ",
            #[cfg(feature = "KE")]
            Country::KE => "KE",
            #[cfg(feature = "LV")]
            Country::LV => "LV",
            #[cfg(feature = "LS")]
            Country::LS => "LS",
            #[cfg(feature = "LI")]
            Country::LI => "LI",
            #[cfg(feature = "LT")]
            Country::LT => "LT",
            #[cfg(feature = "LU")]
            Country::LU => "LU",
            #[cfg(feature = "MG")]
            Country::MG => "MG",
            #[cfg(feature = "MY")]
            Country::MY => "MY",
            #[cfg(feature = "MW")]
            Country::MW => "MW",
            #[cfg(feature = "MT")]
            Country::MT => "MT",
            #[cfg(feature = "MX")]
            Country::MX => "MX",
            #[cfg(feature = "MD")]
            Country::MD => "MD",
            #[cfg(feature = "MA")]
            Country::MA => "MA",
            #[cfg(feature = "MZ")]
            Country::MZ => "MZ",
            #[cfg(feature = "NL")]
            Country::NL => "NL",
            #[cfg(feature = "NA")]
            Country::NA => "NA",
            #[cfg(feature = "NZ")]
            Country::NZ => "NZ",
            #[cfg(feature = "NI")]
            Country::NI => "NI",
            #[cfg(feature = "NG")]
            Country::NG => "NG",
            #[cfg(feature = "MK")]
            Country::MK => "MK",
            #[cfg(feature = "NO")]
            Country::NO => "NO",
            #[cfg(feature = "PK")]
            Country::PK => "PK",
            #[cfg(feature = "PY")]
            Country::PY => "PY",
            #[cfg(feature = "PE")]
            Country::PE => "PE",
            #[cfg(feature = "PL")]
            Country::PL => "PL",
            #[cfg(feature = "PT")]
            Country::PT => "PT",
            #[cfg(feature = "RO")]
            Country::RO => "RO",
            #[cfg(feature = "RU")]
            Country::RU => "RU",
            #[cfg(feature = "SA")]
            Country::SA => "SA",
            #[cfg(feature = "RS")]
            Country::RS => "RS",
            #[cfg(feature = "SG")]
            Country::SG => "SG",
            #[cfg(feature = "SK")]
            Country::SK => "SK",
            #[cfg(feature = "SI")]
            Country::SI => "SI",
            #[cfg(feature = "ZA")]
            Country::ZA => "ZA",
            #[cfg(feature = "KR")]
            Country::KR => "KR",
            #[cfg(feature = "ES")]
            Country::ES => "ES",
            #[cfg(feature = "SZ")]
            Country::SZ => "SZ",
            #[cfg(feature = "SE")]
            Country::SE => "SE",
            #[cfg(feature = "CH")]
            Country::CH => "CH",
            #[cfg(feature = "TW")]
            Country::TW => "TW",
            #[cfg(feature = "TR")]
            Country::TR => "TR",
            #[cfg(feature = "TN")]
            Country::TN => "TN",
            #[cfg(feature = "UA")]
            Country::UA => "UA",
            #[cfg(feature = "AE")]
            Country::AE => "AE",
            #[cfg(feature = "GB")]
            Country::GB => "GB",
            #[cfg(feature = "US")]
            Country::US => "US",
            #[cfg(feature = "US")]
            Country::US_AK => "US_AK",
            #[cfg(feature = "US")]
            Country::US_AL => "US_AL",
            #[cfg(feature = "US")]
            Country::US_AR => "US_AR",
            #[cfg(feature = "US")]
            Country::US_AS => "US_AS",
            #[cfg(feature = "US")]
            Country::US_AZ => "US_AZ",
            #[cfg(feature = "US")]
            Country::US_CA => "US_CA",
            #[cfg(feature = "US")]
            Country::US_CO => "US_CO",
            #[cfg(feature = "US")]
            Country::US_CT => "US_CT",
            #[cfg(feature = "US")]
            Country::US_DC => "US_DC",
            #[cfg(feature = "US")]
            Country::US_DE => "US_DE",
            #[cfg(feature = "US")]
            Country::US_FL => "US_FL",
            #[cfg(feature = "US")]
            Country::US_GA => "US_GA",
            #[cfg(feature = "US")]
            Country::US_GU => "US_GU",
            #[cfg(feature = "US")]
            Country::US_HI => "US_HI",
            #[cfg(feature = "US")]
            Country::US_IA => "US_IA",
            #[cfg(feature = "US")]
            Country::US_ID => "US_ID",
            #[cfg(feature = "US")]
            Country::US_IL => "US_IL",
            #[cfg(feature = "US")]
            Country::US_IN => "US_IN",
            #[cfg(feature = "US")]
            Country::US_KS => "US_KS",
            #[cfg(feature = "US")]
            Country::US_KY => "US_KY",
            #[cfg(feature = "US")]
            Country::US_LA => "US_LA",
            #[cfg(feature = "US")]
            Country::US_MA => "US_MA",
            #[cfg(feature = "US")]
            Country::US_MD => "US_MD",
            #[cfg(feature = "US")]
            Country::US_ME => "US_ME",
            #[cfg(feature = "US")]
            Country::US_MI => "US_MI",
            #[cfg(feature = "US")]
            Country::US_MN => "US_MN",
            #[cfg(feature = "US")]
            Country::US_MO => "US_MO",
            #[cfg(feature = "US")]
            Country::US_MP => "US_MP",
            #[cfg(feature = "US")]
            Country::US_MS => "US_MS",
            #[cfg(feature = "US")]
            Country::US_MT => "US_MT",
            #[cfg(feature = "US")]
            Country::US_NC => "US_NC",
            #[cfg(feature = "US")]
            Country::US_ND => "US_ND",
            #[cfg(feature = "US")]
            Country::US_NE => "US_NE",
            #[cfg(feature = "US")]
            Country::US_NH => "US_NH",
            #[cfg(feature = "US")]
            Country::US_NJ => "US_NJ",
            #[cfg(feature = "US")]
            Country::US_NM => "US_NM",
            #[cfg(feature = "US")]
            Country::US_NV => "US_NV",
            #[cfg(feature = "US")]
            Country::US_NY => "US_NY",
            #[cfg(feature = "US")]
            Country::US_OH => "US_OH",
            #[cfg(feature = "US")]
            Country::US_OK => "US_OK",
            #[cfg(feature = "US")]
            Country::US_OR => "US_OR",
            #[cfg(feature = "US")]
            Country::US_PA => "US_PA",
            #[cfg(feature = "US")]
            Country::US_PR => "US_PR",
            #[cfg(feature = "US")]
            Country::US_RI => "US_RI",
            #[cfg(feature = "US")]
            Country::US_SC => "US_SC",
            #[cfg(feature = "US")]
            Country::US_SD => "US_SD",
            #[cfg(feature = "US")]
            Country::US_TN => "US_TN",
            #[cfg(feature = "US")]
            Country::US_TX => "US_TX",
            #[cfg(feature = "US")]
            Country::US_UM => "US_UM",
            #[cfg(feature = "US")]
            Country::US_UT => "US_UT",
            #[cfg(feature = "US")]
            Country::US_VA => "US_VA",
            #[cfg(feature = "US")]
            Country::US_VI => "US_VI",
            #[cfg(feature = "US")]
            Country::US_VT => "US_VT",
            #[cfg(feature = "US")]
            Country::US_WA => "US_WA",
            #[cfg(feature = "US")]
            Country::US_WI => "US_WI",
            #[cfg(feature = "US")]
            Country::US_WV => "US_WV",
            #[cfg(feature = "US")]
            Country::US_WY => "US_WY",
            #[cfg(feature = "UY")]
            Country::UY => "UY",
            #[cfg(feature = "UZ")]
            Country::UZ => "UZ",
            #[cfg(feature = "VE")]
            Country::VE => "VE",
            #[cfg(feature = "VN")]
            Country::VN => "VN",
            #[cfg(feature = "ZM")]
            Country::ZM => "ZM",
            #[cfg(feature = "ZW")]
            Country::ZW => "ZW",
        }
    }
}

impl std::str::FromStr for Country {
    type Err = Error;

    #[allow(clippy::too_many_lines)]
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            #[cfg(feature = "AO")]
            "AO" => Country::AO,
            #[cfg(feature = "AR")]
            "AR" => Country::AR,
            #[cfg(feature = "AM")]
            "AM" => Country::AM,
            #[cfg(feature = "AW")]
            "AW" => Country::AW,
            #[cfg(feature = "AU")]
            "AU" => Country::AU,
            #[cfg(feature = "AT")]
            "AT" => Country::AT,
            #[cfg(feature = "AZ")]
            "AZ" => Country::AZ,
            #[cfg(feature = "BD")]
            "BD" => Country::BD,
            #[cfg(feature = "BY")]
            "BY" => Country::BY,
            #[cfg(feature = "BE")]
            "BE" => Country::BE,
            #[cfg(feature = "BO")]
            "BO" => Country::BO,
            #[cfg(feature = "BA")]
            "BA" => Country::BA,
            #[cfg(feature = "BW")]
            "BW" => Country::BW,
            #[cfg(feature = "BR")]
            "BR" => Country::BR,
            #[cfg(feature = "BG")]
            "BG" => Country::BG,
            #[cfg(feature = "BI")]
            "BI" => Country::BI,
            #[cfg(feature = "CA")]
            "CA" => Country::CA,
            #[cfg(feature = "CL")]
            "CL" => Country::CL,
            #[cfg(feature = "CN")]
            "CN" => Country::CN,
            #[cfg(feature = "CO")]
            "CO" => Country::CO,
            #[cfg(feature = "HR")]
            "HR" => Country::HR,
            #[cfg(feature = "CU")]
            "CU" => Country::CU,
            #[cfg(feature = "CW")]
            "CW" => Country::CW,
            #[cfg(feature = "CY")]
            "CY" => Country::CY,
            #[cfg(feature = "CZ")]
            "CZ" => Country::CZ,
            #[cfg(feature = "DK")]
            "DK" => Country::DK,
            #[cfg(feature = "DJ")]
            "DJ" => Country::DJ,
            #[cfg(feature = "DO")]
            "DO" => Country::DO,
            #[cfg(feature = "EG")]
            "EG" => Country::EG,
            #[cfg(feature = "EE")]
            "EE" => Country::EE,
            #[cfg(feature = "ET")]
            "ET" => Country::ET,
            #[cfg(feature = "FI")]
            "FI" => Country::FI,
            #[cfg(feature = "FR")]
            "FR" => Country::FR,
            #[cfg(feature = "GE")]
            "GE" => Country::GE,
            #[cfg(feature = "DE")]
            "DE" => Country::DE,
            #[cfg(feature = "DE")]
            "DE_BB" => Country::DE_BB,
            #[cfg(feature = "DE")]
            "DE_BE" => Country::DE_BE,
            #[cfg(feature = "DE")]
            "DE_BW" => Country::DE_BW,
            #[cfg(feature = "DE")]
            "DE_BY" => Country::DE_BY,
            #[cfg(feature = "DE")]
            "DE_BYP" => Country::DE_BYP,
            #[cfg(feature = "DE")]
            "DE_HB" => Country::DE_HB,
            #[cfg(feature = "DE")]
            "DE_HE" => Country::DE_HE,
            #[cfg(feature = "DE")]
            "DE_HH" => Country::DE_HH,
            #[cfg(feature = "DE")]
            "DE_MV" => Country::DE_MV,
            #[cfg(feature = "DE")]
            "DE_NI" => Country::DE_NI,
            #[cfg(feature = "DE")]
            "DE_NW" => Country::DE_NW,
            #[cfg(feature = "DE")]
            "DE_RP" => Country::DE_RP,
            #[cfg(feature = "DE")]
            "DE_SH" => Country::DE_SH,
            #[cfg(feature = "DE")]
            "DE_SL" => Country::DE_SL,
            #[cfg(feature = "DE")]
            "DE_SN" => Country::DE_SN,
            #[cfg(feature = "DE")]
            "DE_ST" => Country::DE_ST,
            #[cfg(feature = "DE")]
            "DE_TH" => Country::DE_TH,
            #[cfg(feature = "GR")]
            "GR" => Country::GR,
            #[cfg(feature = "HN")]
            "HN" => Country::HN,
            #[cfg(feature = "HK")]
            "HK" => Country::HK,
            #[cfg(feature = "HU")]
            "HU" => Country::HU,
            #[cfg(feature = "IS")]
            "IS" => Country::IS,
            #[cfg(feature = "IN")]
            "IN" => Country::IN,
            #[cfg(feature = "ID")]
            "ID" => Country::ID,
            #[cfg(feature = "IE")]
            "IE" => Country::IE,
            #[cfg(feature = "IM")]
            "IM" => Country::IM,
            #[cfg(feature = "IL")]
            "IL" => Country::IL,
            #[cfg(feature = "IT")]
            "IT" => Country::IT,
            #[cfg(feature = "JM")]
            "JM" => Country::JM,
            #[cfg(feature = "JP")]
            "JP" => Country::JP,
            #[cfg(feature = "KZ")]
            "KZ" => Country::KZ,
            #[cfg(feature = "KE")]
            "KE" => Country::KE,
            #[cfg(feature = "LV")]
            "LV" => Country::LV,
            #[cfg(feature = "LS")]
            "LS" => Country::LS,
            #[cfg(feature = "LI")]
            "LI" => Country::LI,
            #[cfg(feature = "LT")]
            "LT" => Country::LT,
            #[cfg(feature = "LU")]
            "LU" => Country::LU,
            #[cfg(feature = "MG")]
            "MG" => Country::MG,
            #[cfg(feature = "MY")]
            "MY" => Country::MY,
            #[cfg(feature = "MW")]
            "MW" => Country::MW,
            #[cfg(feature = "MT")]
            "MT" => Country::MT,
            #[cfg(feature = "MX")]
            "MX" => Country::MX,
            #[cfg(feature = "MD")]
            "MD" => Country::MD,
            #[cfg(feature = "MA")]
            "MA" => Country::MA,
            #[cfg(feature = "MZ")]
            "MZ" => Country::MZ,
            #[cfg(feature = "NL")]
            "NL" => Country::NL,
            #[cfg(feature = "NA")]
            "NA" => Country::NA,
            #[cfg(feature = "NZ")]
            "NZ" => Country::NZ,
            #[cfg(feature = "NI")]
            "NI" => Country::NI,
            #[cfg(feature = "NG")]
            "NG" => Country::NG,
            #[cfg(feature = "MK")]
            "MK" => Country::MK,
            #[cfg(feature = "NO")]
            "NO" => Country::NO,
            #[cfg(feature = "PK")]
            "PK" => Country::PK,
            #[cfg(feature = "PY")]
            "PY" => Country::PY,
            #[cfg(feature = "PE")]
            "PE" => Country::PE,
            #[cfg(feature = "PL")]
            "PL" => Country::PL,
            #[cfg(feature = "PT")]
            "PT" => Country::PT,
            #[cfg(feature = "RO")]
            "RO" => Country::RO,
            #[cfg(feature = "RU")]
            "RU" => Country::RU,
            #[cfg(feature = "SA")]
            "SA" => Country::SA,
            #[cfg(feature = "RS")]
            "RS" => Country::RS,
            #[cfg(feature = "SG")]
            "SG" => Country::SG,
            #[cfg(feature = "SK")]
            "SK" => Country::SK,
            #[cfg(feature = "SI")]
            "SI" => Country::SI,
            #[cfg(feature = "ZA")]
            "ZA" => Country::ZA,
            #[cfg(feature = "KR")]
            "KR" => Country::KR,
            #[cfg(feature = "ES")]
            "ES" => Country::ES,
            #[cfg(feature = "SZ")]
            "SZ" => Country::SZ,
            #[cfg(feature = "SE")]
            "SE" => Country::SE,
            #[cfg(feature = "CH")]
            "CH" => Country::CH,
            #[cfg(feature = "TW")]
            "TW" => Country::TW,
            #[cfg(feature = "TR")]
            "TR" => Country::TR,
            #[cfg(feature = "TN")]
            "TN" => Country::TN,
            #[cfg(feature = "UA")]
            "UA" => Country::UA,
            #[cfg(feature = "AE")]
            "AE" => Country::AE,
            #[cfg(feature = "GB")]
            "GB" => Country::GB,
            #[cfg(feature = "US")]
            "US" => Country::US,
            #[cfg(feature = "US")]
            "US_AK" => Country::US_AK,
            #[cfg(feature = "US")]
            "US_AL" => Country::US_AL,
            #[cfg(feature = "US")]
            "US_AR" => Country::US_AR,
            #[cfg(feature = "US")]
            "US_AS" => Country::US_AS,
            #[cfg(feature = "US")]
            "US_AZ" => Country::US_AZ,
            #[cfg(feature = "US")]
            "US_CA" => Country::US_CA,
            #[cfg(feature = "US")]
            "US_CO" => Country::US_CO,
            #[cfg(feature = "US")]
            "US_CT" => Country::US_CT,
            #[cfg(feature = "US")]
            "US_DC" => Country::US_DC,
            #[cfg(feature = "US")]
            "US_DE" => Country::US_DE,
            #[cfg(feature = "US")]
            "US_FL" => Country::US_FL,
            #[cfg(feature = "US")]
            "US_GA" => Country::US_GA,
            #[cfg(feature = "US")]
            "US_GU" => Country::US_GU,
            #[cfg(feature = "US")]
            "US_HI" => Country::US_HI,
            #[cfg(feature = "US")]
            "US_IA" => Country::US_IA,
            #[cfg(feature = "US")]
            "US_ID" => Country::US_ID,
            #[cfg(feature = "US")]
            "US_IL" => Country::US_IL,
            #[cfg(feature = "US")]
            "US_IN" => Country::US_IN,
            #[cfg(feature = "US")]
            "US_KS" => Country::US_KS,
            #[cfg(feature = "US")]
            "US_KY" => Country::US_KY,
            #[cfg(feature = "US")]
            "US_LA" => Country::US_LA,
            #[cfg(feature = "US")]
            "US_MA" => Country::US_MA,
            #[cfg(feature = "US")]
            "US_MD" => Country::US_MD,
            #[cfg(feature = "US")]
            "US_ME" => Country::US_ME,
            #[cfg(feature = "US")]
            "US_MI" => Country::US_MI,
            #[cfg(feature = "US")]
            "US_MN" => Country::US_MN,
            #[cfg(feature = "US")]
            "US_MO" => Country::US_MO,
            #[cfg(feature = "US")]
            "US_MP" => Country::US_MP,
            #[cfg(feature = "US")]
            "US_MS" => Country::US_MS,
            #[cfg(feature = "US")]
            "US_MT" => Country::US_MT,
            #[cfg(feature = "US")]
            "US_NC" => Country::US_NC,
            #[cfg(feature = "US")]
            "US_ND" => Country::US_ND,
            #[cfg(feature = "US")]
            "US_NE" => Country::US_NE,
            #[cfg(feature = "US")]
            "US_NH" => Country::US_NH,
            #[cfg(feature = "US")]
            "US_NJ" => Country::US_NJ,
            #[cfg(feature = "US")]
            "US_NM" => Country::US_NM,
            #[cfg(feature = "US")]
            "US_NV" => Country::US_NV,
            #[cfg(feature = "US")]
            "US_NY" => Country::US_NY,
            #[cfg(feature = "US")]
            "US_OH" => Country::US_OH,
            #[cfg(feature = "US")]
            "US_OK" => Country::US_OK,
            #[cfg(feature = "US")]
            "US_OR" => Country::US_OR,
            #[cfg(feature = "US")]
            "US_PA" => Country::US_PA,
            #[cfg(feature = "US")]
            "US_PR" => Country::US_PR,
            #[cfg(feature = "US")]
            "US_RI" => Country::US_RI,
            #[cfg(feature = "US")]
            "US_SC" => Country::US_SC,
            #[cfg(feature = "US")]
            "US_SD" => Country::US_SD,
            #[cfg(feature = "US")]
            "US_TN" => Country::US_TN,
            #[cfg(feature = "US")]
            "US_TX" => Country::US_TX,
            #[cfg(feature = "US")]
            "US_UM" => Country::US_UM,
            #[cfg(feature = "US")]
            "US_UT" => Country::US_UT,
            #[cfg(feature = "US")]
            "US_VA" => Country::US_VA,
            #[cfg(feature = "US")]
            "US_VI" => Country::US_VI,
            #[cfg(feature = "US")]
            "US_VT" => Country::US_VT,
            #[cfg(feature = "US")]
            "US_WA" => Country::US_WA,
            #[cfg(feature = "US")]
            "US_WI" => Country::US_WI,
            #[cfg(feature = "US")]
            "US_WV" => Country::US_WV,
            #[cfg(feature = "US")]
            "US_WY" => Country::US_WY,
            #[cfg(feature = "UY")]
            "UY" => Country::UY,
            #[cfg(feature = "UZ")]
            "UZ" => Country::UZ,
            #[cfg(feature = "VE")]
            "VE" => Country::VE,
            #[cfg(feature = "VN")]
            "VN" => Country::VN,
            #[cfg(feature = "ZM")]
            "ZM" => Country::ZM,
            #[cfg(feature = "ZW")]
            "ZW" => Country::ZW,
            _ => return Err(Error::CountryNotAvailable),
        })
    }
}

impl Country {
    #[must_use]
    pub fn country_from_subdivision(subdivision: Country) -> Option<Self> {
        Some(match subdivision {
            #[cfg(feature = "DE")]
            Country::DE_BB
            | Country::DE_BE
            | Country::DE_BW
            | Country::DE_BY
            | Country::DE_BYP
            | Country::DE_HB
            | Country::DE_HE
            | Country::DE_HH
            | Country::DE_MV
            | Country::DE_NI
            | Country::DE_NW
            | Country::DE_RP
            | Country::DE_SH
            | Country::DE_SL
            | Country::DE_SN
            | Country::DE_ST
            | Country::DE_TH => Country::DE,
            #[cfg(feature = "US")]
            Country::US_AK
            | Country::US_AL
            | Country::US_AR
            | Country::US_AS
            | Country::US_AZ
            | Country::US_CA
            | Country::US_CO
            | Country::US_CT
            | Country::US_DC
            | Country::US_DE
            | Country::US_FL
            | Country::US_GA
            | Country::US_GU
            | Country::US_HI
            | Country::US_IA
            | Country::US_ID
            | Country::US_IL
            | Country::US_IN
            | Country::US_KS
            | Country::US_KY
            | Country::US_LA
            | Country::US_MA
            | Country::US_MD
            | Country::US_ME
            | Country::US_MI
            | Country::US_MN
            | Country::US_MO
            | Country::US_MP
            | Country::US_MS
            | Country::US_MT
            | Country::US_NC
            | Country::US_ND
            | Country::US_NE
            | Country::US_NH
            | Country::US_NJ
            | Country::US_NM
            | Country::US_NV
            | Country::US_NY
            | Country::US_OH
            | Country::US_OK
            | Country::US_OR
            | Country::US_PA
            | Country::US_PR
            | Country::US_RI
            | Country::US_SC
            | Country::US_SD
            | Country::US_TN
            | Country::US_TX
            | Country::US_UM
            | Country::US_UT
            | Country::US_VA
            | Country::US_VI
            | Country::US_VT
            | Country::US_WA
            | Country::US_WI
            | Country::US_WV
            | Country::US_WY => Country::US,
            _ => return None,
        })
    }
}
