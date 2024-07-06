//! United States (Nevada)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Nevada)";
const COUNTY_CODE: Country = Country::US_NV;

/// Generate holiday map for United States (Nevada).
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
        [
            (NaiveDate::from_ymd_res(2000, 10, 27)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2000, 11, 24)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 10, 26)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2001, 11, 23)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 10, 25)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2002, 11, 29)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 10, 31)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2003, 11, 28)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 10, 29)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2004, 11, 26)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 10, 28)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2005, 11, 25)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 10, 27)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2006, 11, 24)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 10, 26)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2007, 11, 23)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 10, 31)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2008, 11, 28)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 10, 30)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 10, 29)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2010, 11, 26)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 10, 28)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2011, 11, 25)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2012, 11, 23)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 10, 25)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2013, 11, 29)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 10, 31)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2014, 11, 28)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 10, 30)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2015, 11, 27)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 10, 28)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2016, 11, 25)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 10, 27)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2017, 11, 24)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 10, 26)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2018, 11, 23)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 10, 25)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2019, 11, 29)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 10, 30)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2020, 11, 27)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 10, 29)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2021, 11, 26)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 10, 28)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2022, 11, 25)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 10, 27)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2023, 11, 24)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 10, 25)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2024, 11, 29)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 10, 31)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2025, 11, 28)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 10, 30)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2026, 11, 27)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 10, 29)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2027, 11, 26)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 10, 27)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2028, 11, 24)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 10, 26)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2029, 11, 23)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 10, 25)?, "Nevada Day"),
            (NaiveDate::from_ymd_res(2030, 11, 29)?, "Family Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
