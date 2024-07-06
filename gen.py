import holidays
from dataclasses import dataclass
from jinja2 import Environment


@dataclass
class Country:
    code: str
    name: str
    subdivision_code: str | None = None
    subdivision_name: str | None = None

countries = [
    Country("AO", "Angola"),
    Country("AR", "Argentina"),
    Country("AM", "Armenia"),
    Country("AW", "Aruba"),
    Country("AU", "Australia"),
    Country("AT", "Austria"),
    Country("AZ", "Azerbaijan"),
    Country("BD", "Bangladesh"),
    Country("BY", "Belarus"),
    Country("BE", "Belgium"),
    Country("BO", "Bolivia"),
    Country("BA", "Bosnia and Herzegovina"),
    Country("BW", "Botswana"),
    Country("BR", "Brazil"),
    Country("BG", "Bulgaria"),
    Country("BI", "Burundi"),
    Country("CA", "Canada"),
    Country("CL", "Chile"),
    Country("CN", "China"),
    Country("CO", "Colombia"),
    Country("HR", "Croatia"),
    Country("CU", "Cuba"),
    Country("CW", "Curaçao"),
    Country("CY", "Cyprus"),
    Country("CZ", "Czechia"),
    Country("DK", "Denmark"),
    Country("DJ", "Djibouti"),
    Country("DO", "Dominican Republic"),
    Country("EG", "Egypt"),
    Country("EE", "Estonia"),
    Country("ET", "Ethiopia"),
    Country("FI", "Finland"),
    Country("FR", "France"),
    Country("GE", "Georgia"),
    Country("DE", "Germany"),
    Country("DE", "Germany", "BB", "Brandenburg"),
    Country("DE", "Germany", "BE", "Berlin"),
    Country("DE", "Germany", "BW", "Baden-Württemberg"),
    Country("DE", "Germany", "BY", "Bavaria (Bayern)"),
    Country("DE", "Germany", "BYP", "Bavaria (Bayern) with more protestants"),
    Country("DE", "Germany", "HB", "Bremen"),
    Country("DE", "Germany", "HE", "Hesse (Hessen)"),
    Country("DE", "Germany", "HH", "Hamburg"),
    Country("DE", "Germany", "MV", "Mecklenburg-Vorpommern"),
    Country("DE", "Germany", "NI", "Lower Saxony (Niedersachsen)"),
    Country("DE", "Germany", "NW", "North Rhine-Westphalia (Nordrhein-Westfalen)"),
    Country("DE", "Germany", "RP", "Rhineland-Palatinate (Rheinland-Pfalz)"),
    Country("DE", "Germany", "SH", "Schleswig-Holstein"),
    Country("DE", "Germany", "SL", "Saarland"),
    Country("DE", "Germany", "SN", "Saxony (Sachsen)"),
    Country("DE", "Germany", "ST", "Saxony-Anhalt (Sachsen-Anhalt)"),
    Country("DE", "Germany", "TH", "Thuringia (Thüringen)"),
    Country("GR", "Greece"),
    Country("HN", "Honduras"),
    Country("HK", "Hong Kong"),
    Country("HU", "Hungary"),
    Country("IS", "Iceland"),
    Country("IN", "India"),
    Country("ID", "Indonesia"),
    Country("IE", "Ireland"),
    Country("IM", "Isle of Man"),
    Country("IL", "Israel"),
    Country("IT", "Italy"),
    Country("JM", "Jamaica"),
    Country("JP", "Japan"),
    Country("KZ", "Kazakhstan"),
    Country("KE", "Kenya"),
    Country("LV", "Latvia"),
    Country("LS", "Lesotho"),
    Country("LI", "Liechtenstein"),
    Country("LT", "Lithuania"),
    Country("LU", "Luxembourg"),
    Country("MG", "Madagascar"),
    Country("MY", "Malaysia"),
    Country("MW", "Malawi"),
    Country("MT", "Malta"),
    Country("MX", "Mexico"),
    Country("MD", "Moldova"),
    Country("MA", "Morocco"),
    Country("MZ", "Mozambique"),
    Country("NL", "Netherlands"),
    Country("NA", "Namibia"),
    Country("NZ", "New Zealand"),
    Country("NI", "Nicaragua"),
    Country("NG", "Nigeria"),
    Country("MK", "North Macedonia"),
    Country("NO", "Norway"),
    Country("PK", "Pakistan"),
    Country("PY", "Paraguay"),
    Country("PE", "Peru"),
    Country("PL", "Poland"),
    Country("PT", "Portugal"),
    Country("RO", "Romania"),
    Country("RU", "Russia"),
    Country("SA", "Saudi Arabia"),
    Country("RS", "Serbia"),
    Country("SG", "Singapore"),
    Country("SK", "Slovakia"),
    Country("SI", "Slovenia"),
    Country("ZA", "South Africa"),
    Country("KR", "South Korea"),
    Country("ES", "Spain"),
    Country("SZ", "Swaziland"),
    Country("SE", "Sweden"),
    Country("CH", "Switzerland"),
    Country("TW", "Taiwan"),
    Country("TR", "Turkey"),
    Country("TN", "Tunisia"),
    Country("UA", "Ukraine"),
    Country("AE", "United Arab Emirates"),
    Country("GB", "United Kingdom"),
    Country("US", "United States"),
    Country("US", "United States", "AK", "Alaska"),
    Country("US", "United States", "AL", "Alabama"),
    Country("US", "United States", "AR", "Arkansas"),
    Country("US", "United States", "AS", "American Samoa"),
    Country("US", "United States", "AZ", "Arizona"),
    Country("US", "United States", "CA", "California"),
    Country("US", "United States", "CO", "Colorado"),
    Country("US", "United States", "CT", "Connecticut"),
    Country("US", "United States", "DC", "District of Columbia"),
    Country("US", "United States", "DE", "Delaware"),
    Country("US", "United States", "FL", "Florida"),
    Country("US", "United States", "GA", "Georgia"),
    Country("US", "United States", "GU", "Guam"),
    Country("US", "United States", "HI", "Hawaii"),
    Country("US", "United States", "IA", "Iowa"),
    Country("US", "United States", "ID", "Idaho"),
    Country("US", "United States", "IL", "Illinois"),
    Country("US", "United States", "IN", "Indiana"),
    Country("US", "United States", "KS", "Kansas"),
    Country("US", "United States", "KY", "Kentucky"),
    Country("US", "United States", "LA", "Louisiana"),
    Country("US", "United States", "MA", "Massachusetts"),
    Country("US", "United States", "MD", "Maryland"),
    Country("US", "United States", "ME", "Maine"),
    Country("US", "United States", "MI", "Michigan"),
    Country("US", "United States", "MN", "Minnesota"),
    Country("US", "United States", "MO", "Missouri"),
    Country("US", "United States", "MP", "Northern Mariana Islands"),
    Country("US", "United States", "MS", "Mississippi"),
    Country("US", "United States", "MT", "Montana"),
    Country("US", "United States", "NC", "North Carolina"),
    Country("US", "United States", "ND", "North Dakota"),
    Country("US", "United States", "NE", "Nebraska"),
    Country("US", "United States", "NH", "New Hampshire"),
    Country("US", "United States", "NJ", "New Jersey"),
    Country("US", "United States", "NM", "New Mexico"),
    Country("US", "United States", "NV", "Nevada"),
    Country("US", "United States", "NY", "New York"),
    Country("US", "United States", "OH", "Ohio"),
    Country("US", "United States", "OK", "Oklahoma"),
    Country("US", "United States", "OR", "Oregon"),
    Country("US", "United States", "PA", "Pennsylvania"),
    Country("US", "United States", "PR", "Puerto Rico"),
    Country("US", "United States", "RI", "Rhode Island"),
    Country("US", "United States", "SC", "South Carolina"),
    Country("US", "United States", "SD", "South Dakota"),
    Country("US", "United States", "TN", "Tennessee"),
    Country("US", "United States", "TX", "Texas"),
    Country("US", "United States", "UM", "United States Minor Outlying Islands"),
    Country("US", "United States", "UT", "Utah"),
    Country("US", "United States", "VA", "Virginia"),
    Country("US", "United States", "VI", "Virgin Islands, U.S.."),
    Country("US", "United States", "VT", "Vermont"),
    Country("US", "United States", "WA", "Washington"),
    Country("US", "United States", "WI", "Wisconsin"),
    Country("US", "United States", "WV", "West Virginia"),
    Country("US", "United States", "WY", "Wyoming"),
    Country("UY", "Uruguay"),
    Country("UZ", "Uzbekistan"),
    Country("VE", "Venezuela"),
    Country("VN", "Vietnam"),
    Country("ZM", "Zambia"),
    Country("ZW", "Zimbabwe"),
]

years = range(2000, 2031)

country = """
use crate::Error;

/// Two-letter country codes defined in ISO 3166-1 alpha-2 .
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Country {
{%- for country in countries %}
    #[cfg(feature = "{{country.code}}")]
    /// {{country|display_name}}
    {{country|enum_name}},
{%- endfor %}
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
{%- for country in countries %}
            #[cfg(feature = "{{country.code}}")]
            Country::{{country|enum_name}} => "{{country|enum_name}}",
{%- endfor %}
        }
    }
}

impl std::str::FromStr for Country {
    type Err = Error;

    #[allow(clippy::too_many_lines)]
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
{%- for country in countries %}
            #[cfg(feature = "{{country.code}}")]
            "{{country|enum_name}}" => Country::{{country|enum_name}},
{%- endfor %}
            _ => return Err(Error::CountryNotAvailable),
        })
    }
}

impl Country {
    #[must_use]
    pub fn country_from_subdivision(subdivision: Country) -> Option<Self> {
        Some(match subdivision {
{%- for country in countries %}
{%- if country.subdivision_code == None and subdivisionsPerCountry[country.code] %}
        #[cfg(feature = "{{country.code}}")]
{%- for subdiv in subdivisionsPerCountry[country.code] %}
        | Country::{{subdiv|enum_name}}
{%- endfor %}   
        => Country::{{country|enum_name}},
{%- endif %}
{%- endfor %}
            _ => return None,
        })
    }
}

"""

build = """
use std::collections::HashSet;

use crate::{
    data::{
        self,
        helper::{add_main_country_from_subdivisions, should_build},
    },
    prelude::*,
    HolidayMap, Result, Year,
};

/// Generate holiday map for the specified countries and years.
#[allow(clippy::too_many_lines)]
pub fn build(countries: Option<HashSet<Country>>, years: Option<&std::ops::Range<Year>>) -> Result<HolidayMap> {
    let countries = countries.map(|mut c| {
        add_main_country_from_subdivisions(&mut c);
        c
    });

    let countries = countries.as_ref();

    let mut map = HolidayMap::new();

{% for country in countries %}
    #[cfg(feature = "{{country.code}}")]
    if should_build(countries, Country::{{country|enum_name}}) {
        map.insert(Country::{{country|enum_name}}, data::{{country|mod_name|escape}}::build(years)?);
    }
{% endfor %}

    Ok(map)
}

"""

country_mod = """
pub(super) mod helper;

use crate::{prelude::*, HolidayPerYearMap, NaiveDateExt, Result, Year};
use helper::build_year;

use chrono::NaiveDate;
use std::collections::HashMap;

{% for country in countries %}
#[cfg(feature = "{{country.code}}")]
pub mod {{country|mod_name|escape}};
{% endfor %}
"""

build_country = """
//! {{country|display_name}}
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "{{country|display_name}}";
const COUNTY_CODE: Country = Country::{{country|enum_name}};

/// Generate holiday map for {{country|display_name}}.
#[allow(unused_mut, unused_variables, clippy::too_many_lines, clippy::missing_errors_doc)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();
{% if country.subdivision_code %}
    let mut national_holidays = de::build(years)?;
{%- endif %}

{%- for year in years %}
{% if holiday(years=year, subdiv=country.subdivision_code) %}

    build_year(
        years,
        {{year}},
        [
{%- for date, name in holiday(years=year, subdiv=country.subdivision_code).items() %}
{%- if country.subdivision_code == None or date not in holiday(years=year) %}
            (NaiveDate::from_ymd_res({{date|year}}, {{date|month}}, {{date|day}})?, "{{name}}"),
{%- endif %}
{%- endfor %}
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );
{%- endif %}
{%- endfor  %}

    Ok(map)
}
"""


def lower(code: str) -> str:
    return code.lower()

def escape(code: str) -> str:
    rust_keywords = ["as", "in", "do"]
    lower = code.lower()
    if lower in rust_keywords:
        return "r#" + lower
    else:
        return lower

def enum_name(country: Country) -> str:
    if country.subdivision_code:
        return f"{country.code}_{country.subdivision_code}"
    else:
        return country.code

def mod_name(country: Country) -> str:
    return enum_name(country).lower()

def display_name(country: Country) -> str:
    if country.subdivision_name:
        return f"{country.name} ({country.subdivision_name})"
    else:
        return country.name

def subdivisionsPerCountry(countries: list[Country]) -> dict[str, list[Country]]:
    subdivisionsPerCountry = {}
    for country in countries:
        if country.subdivision_code == None:
            subdivisionsPerCountry[country.code] = list(filter(lambda c: c.subdivision_code and c.code == country.code, countries))

    return subdivisionsPerCountry

if __name__ == "__main__":
    env = Environment()
    env.filters["year"] = lambda d: d.year
    env.filters["month"] = lambda d: d.month
    env.filters["day"] = lambda d: d.day
    env.filters["escape"] = escape
    env.filters["enum_name"] = enum_name
    env.filters["mod_name"] = mod_name
    env.filters["display_name"] = display_name

    with open("src/country.rs", "w") as f:
        rendered = env.from_string(country).render(
            countries=countries,
            subdivisionsPerCountry=subdivisionsPerCountry(countries))
        f.write(rendered)

    with open("src/build.rs", "w") as f:
        rendered = env.from_string(build).render(countries=countries)
        f.write(rendered)
    
    with open("src/data/mod.rs", "w") as f:
        rendered = env.from_string(country_mod).render(countries=countries)
        f.write(rendered)

    for country in countries:
        with open("src/data/{}.rs".format(mod_name(country)), "w") as f:
            holiday = getattr(holidays, country.code, {})
            rendered = env.from_string(build_country).render(
                    country=country,
                    years=years,
                    holiday=holiday)
            f.write(rendered)
