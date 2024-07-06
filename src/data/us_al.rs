//! United States (Alabama)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Alabama)";
const COUNTY_CODE: Country = Country::US_AL;

/// Generate holiday map for United States (Alabama).
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
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 5)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (
                NaiveDate::from_ymd_res(2001, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (
                NaiveDate::from_ymd_res(2002, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 3)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (
                NaiveDate::from_ymd_res(2003, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 2)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (
                NaiveDate::from_ymd_res(2004, 4, 26)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 7)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (
                NaiveDate::from_ymd_res(2005, 4, 25)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 6)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (
                NaiveDate::from_ymd_res(2006, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 5)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (
                NaiveDate::from_ymd_res(2007, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 4)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (
                NaiveDate::from_ymd_res(2008, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 2)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (
                NaiveDate::from_ymd_res(2009, 4, 27)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 1)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (
                NaiveDate::from_ymd_res(2010, 4, 26)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 7)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 6)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (
                NaiveDate::from_ymd_res(2012, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 4)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (
                NaiveDate::from_ymd_res(2013, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 3)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (
                NaiveDate::from_ymd_res(2014, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 2)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (
                NaiveDate::from_ymd_res(2015, 4, 27)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 1)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (
                NaiveDate::from_ymd_res(2016, 4, 25)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 6)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (
                NaiveDate::from_ymd_res(2017, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 5)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (
                NaiveDate::from_ymd_res(2018, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 4)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (
                NaiveDate::from_ymd_res(2019, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 3)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (
                NaiveDate::from_ymd_res(2020, 4, 27)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 1)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (
                NaiveDate::from_ymd_res(2021, 4, 26)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 7)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (
                NaiveDate::from_ymd_res(2022, 4, 25)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 6)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 5)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (
                NaiveDate::from_ymd_res(2024, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 3)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (
                NaiveDate::from_ymd_res(2025, 4, 28)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 2)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (
                NaiveDate::from_ymd_res(2026, 4, 27)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 1)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (
                NaiveDate::from_ymd_res(2027, 4, 26)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 7)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (
                NaiveDate::from_ymd_res(2028, 4, 24)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 5)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (
                NaiveDate::from_ymd_res(2029, 4, 23)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 4)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (
                NaiveDate::from_ymd_res(2030, 4, 22)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 3)?,
                "Jefferson Davis Birthday",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
