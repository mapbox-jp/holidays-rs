import holidays
from dataclasses import dataclass
from jinja2 import Environment


@dataclass
class Country:
    code: str
    name: str


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
    Country("UY", "Uruguay"),
    Country("UZ", "Uzbekistan"),
    Country("VE", "Venezuela"),
    Country("VN", "Vietnam"),
    Country("ZM", "Zambia"),
    Country("ZW", "Zimbabwe"),
]

years = range(2000, 2031)

country = """
/// Two-letter country codes defined in ISO 3166-1 alpha-2 .
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Country {
{%- for country in countries %}
  #[cfg(feature = "{{country.code}}")]
  /// {{country.name}}
  {{country.code}},
{%- endfor %}
}

impl ToString for Country {
  fn to_string(&self) -> String {
    match self {
{%- for country in countries %}
      #[cfg(feature = "{{country.code}}")]
      Country::{{country.code}} => "{{country.code}}".into(),
{%- endfor %}
    }
  }
}

impl AsRef<str> for Country {
  fn as_ref(&self) -> &str {
    match self {
{%- for country in countries %}
      #[cfg(feature = "{{country.code}}")]
      Country::{{country.code}} => "{{country.code}}",
{%- endfor %}
    }
  }
}

impl std::str::FromStr for Country {
  type Err = Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
{%- for country in countries %}
      #[cfg(feature = "{{country.code}}")]
      "{{country.code}}" => Ok(Country::{{country.code}}),
{%- endfor %}
      _ => Err(Error::CountryNotAvailable),
    }
  }
}

"""

build = """
/// Generate holiday map for the specified countries and years.
fn build(countries: Option<&HashSet<Country>>, years: Option<&std::ops::Range<Year>>) -> Result<HolidayMap> {
  let mut map = HolidayMap::new();
{% for country in countries %}
  #[cfg(feature = "{{country.code}}")]
  if countries.is_none() || countries.unwrap().contains(&Country::{{country.code}}) {
      map.insert(Country::{{country.code}}, {{country.code|escape}}::build(&years)?);
  }
{% endfor %}
  Ok(map)
}

"""

build_country = """
/// {{country}}.
#[cfg(feature = "{{code}}")]
pub mod {{code|escape}} {
use super::*;

/// Generate holiday map for {{country}}.
#[allow(unused_mut, unused_variables)]
pub fn build(years: &Option<&std::ops::Range<Year>>) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
  let mut map = HashMap::new();

{%- for year in years %}
  {% if holiday(years=year) %}
  build_year(
    years,
    {{year}},
    vec![
{% for date, name in holiday(years=year).items() %}
      Holiday::new(
        Country::{{code}},
        "{{country}}",
        NaiveDate::from_ymd_res({{date|year}}, {{date|month}}, {{date|day}})?,
        "{{name}}"
      ),
{%- endfor %}
    ],
    &mut map,
  );
{%- endif %}
{%- endfor  %}

  Ok(map)
}
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


def empty_holiday(**kwargs):
    return {}


if __name__ == "__main__":
    env = Environment()
    env.filters["year"] = lambda d: d.year
    env.filters["month"] = lambda d: d.month
    env.filters["day"] = lambda d: d.day
    env.filters["escape"] = escape
    env.filters["lower"] = lower
    with open("src/data.rs", "w") as f:
        rendered = env.from_string(country).render(countries=countries)
        f.write(rendered)

        rendered = env.from_string(build).render(countries=countries)
        f.write(rendered)

        for country in countries:
            holiday = getattr(holidays, country.code, None)
            rendered = env.from_string(build_country).render(
                    code=country.code,
                    country=country.name,
                    years=years,
                    holiday=holiday or empty_holiday)
            f.write(rendered)
