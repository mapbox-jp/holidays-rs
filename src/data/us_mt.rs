//! United States (Montana)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Montana)";
const COUNTY_CODE: Country = Country::US_MT;

/// Generate holiday map for United States (Montana).
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

    build_year(
        years,
        2008,
        [(NaiveDate::from_ymd_res(2008, 11, 4)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2009, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2010,
        [(NaiveDate::from_ymd_res(2010, 11, 2)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2011, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2012,
        [(NaiveDate::from_ymd_res(2012, 11, 6)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2013, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2014,
        [(NaiveDate::from_ymd_res(2014, 11, 4)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2015, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2016,
        [(NaiveDate::from_ymd_res(2016, 11, 8)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2017, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2018,
        [(NaiveDate::from_ymd_res(2018, 11, 6)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2019, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2020,
        [(NaiveDate::from_ymd_res(2020, 11, 3)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2021, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2022,
        [(NaiveDate::from_ymd_res(2022, 11, 8)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2023, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2024,
        [(NaiveDate::from_ymd_res(2024, 11, 5)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2025, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2026,
        [(NaiveDate::from_ymd_res(2026, 11, 3)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2027, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2028,
        [(NaiveDate::from_ymd_res(2028, 11, 7)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2029, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2030,
        [(NaiveDate::from_ymd_res(2030, 11, 5)?, "Election Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
