use crate::Error;

/// Two-letter country codes defined in ISO 3166-1 alpha-2 .
#[allow(dead_code)]
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
