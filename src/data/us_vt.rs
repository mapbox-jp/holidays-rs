//! United States (Vermont)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Vermont)";
const COUNTY_CODE: Country = Country::US_VT;

/// Generate holiday map for United States (Vermont).
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
            (NaiveDate::from_ymd_res(2000, 3, 7)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2000, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2001, 3, 6)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2001, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2002, 3, 5)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2002, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2003, 3, 4)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2003, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2004, 3, 2)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2004, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2005, 3, 1)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2005, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2006, 3, 7)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2006, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2007, 3, 6)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2007, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2008, 3, 4)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2008, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2009, 3, 3)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2009, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 17)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2010, 3, 2)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2010, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2011, 3, 1)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2011, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2012, 3, 6)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2012, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2013, 3, 5)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2013, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2014, 3, 4)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2014, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2015, 3, 3)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2015, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 17)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2016, 3, 1)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2016, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2017, 3, 7)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2017, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2018, 3, 6)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2018, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2019, 3, 5)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2019, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2020, 3, 3)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2020, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 17)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2021, 3, 2)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2021, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2022, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2023, 3, 7)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2023, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2024, 3, 5)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2024, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2025, 3, 4)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2025, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2026, 3, 3)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2026, 8, 16)?,
                "Bennington Battle Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 17)?,
                "Bennington Battle Day (observed)",
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
            (NaiveDate::from_ymd_res(2027, 3, 2)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2028, 3, 7)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2028, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2029, 3, 6)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2029, 8, 16)?,
                "Bennington Battle Day",
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
            (NaiveDate::from_ymd_res(2030, 3, 5)?, "Town Meeting Day"),
            (
                NaiveDate::from_ymd_res(2030, 8, 16)?,
                "Bennington Battle Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
