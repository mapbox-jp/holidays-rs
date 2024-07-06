//! United States (Nebraska)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Nebraska)";
const COUNTY_CODE: Country = Country::US_NE;

/// Generate holiday map for United States (Nebraska).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_year(
        years,
        2000,
        [(NaiveDate::from_ymd_res(2000, 4, 28)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [(NaiveDate::from_ymd_res(2001, 4, 27)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [(NaiveDate::from_ymd_res(2002, 4, 26)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [(NaiveDate::from_ymd_res(2003, 4, 25)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [(NaiveDate::from_ymd_res(2004, 4, 30)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [(NaiveDate::from_ymd_res(2005, 4, 29)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [(NaiveDate::from_ymd_res(2006, 4, 28)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [(NaiveDate::from_ymd_res(2007, 4, 27)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [(NaiveDate::from_ymd_res(2008, 4, 25)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [(NaiveDate::from_ymd_res(2009, 4, 24)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [(NaiveDate::from_ymd_res(2010, 4, 30)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [(NaiveDate::from_ymd_res(2011, 4, 29)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [(NaiveDate::from_ymd_res(2012, 4, 27)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [(NaiveDate::from_ymd_res(2013, 4, 26)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [(NaiveDate::from_ymd_res(2014, 4, 25)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [(NaiveDate::from_ymd_res(2015, 4, 24)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [(NaiveDate::from_ymd_res(2016, 4, 29)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [(NaiveDate::from_ymd_res(2017, 4, 28)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [(NaiveDate::from_ymd_res(2018, 4, 27)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [(NaiveDate::from_ymd_res(2019, 4, 26)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [(NaiveDate::from_ymd_res(2020, 4, 24)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [(NaiveDate::from_ymd_res(2021, 4, 30)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [(NaiveDate::from_ymd_res(2022, 4, 29)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [(NaiveDate::from_ymd_res(2023, 4, 28)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [(NaiveDate::from_ymd_res(2024, 4, 26)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [(NaiveDate::from_ymd_res(2025, 4, 25)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [(NaiveDate::from_ymd_res(2026, 4, 24)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [(NaiveDate::from_ymd_res(2027, 4, 30)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [(NaiveDate::from_ymd_res(2028, 4, 28)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [(NaiveDate::from_ymd_res(2029, 4, 27)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [(NaiveDate::from_ymd_res(2030, 4, 26)?, "Arbor Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
