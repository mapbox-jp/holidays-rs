//! United States (Arizona)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Arizona)";
const COUNTY_CODE: Country = Country::US_AZ;

/// Generate holiday map for United States (Arizona).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_year(years, 2000, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2001, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2002, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2003, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2004, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2005, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2006, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2007, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2008, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2009, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2010, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2011, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2012, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2013, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2014, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2015, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2016, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2017, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2018, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2019, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2020, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2021, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2022, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2023, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2024, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2025, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2026, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2027, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2028, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2029, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2030, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    Ok(map)
}
