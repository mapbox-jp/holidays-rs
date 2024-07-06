//! United States (Wisconsin)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Wisconsin)";
const COUNTY_CODE: Country = Country::US_WI;

/// Generate holiday map for United States (Wisconsin).
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
        [(
            NaiveDate::from_ymd_res(2000, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [(
            NaiveDate::from_ymd_res(2001, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [(
            NaiveDate::from_ymd_res(2002, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [(
            NaiveDate::from_ymd_res(2003, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [(
            NaiveDate::from_ymd_res(2004, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [(
            NaiveDate::from_ymd_res(2005, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [(
            NaiveDate::from_ymd_res(2006, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [(
            NaiveDate::from_ymd_res(2007, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [(
            NaiveDate::from_ymd_res(2008, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [(
            NaiveDate::from_ymd_res(2009, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2010, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2011,
        [(
            NaiveDate::from_ymd_res(2011, 2, 15)?,
            "Susan B. Anthony Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (
                NaiveDate::from_ymd_res(2012, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2012, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2013, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2013, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2014, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2014, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2015, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2015, 12, 31)?, "New Year's Eve"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2016, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 31)?, "New Year's Eve"),
            (
                NaiveDate::from_ymd_res(2016, 12, 30)?,
                "New Year's Eve (observed)",
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
                NaiveDate::from_ymd_res(2017, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2017, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2018, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2019, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2019, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2020, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2020, 12, 31)?, "New Year's Eve"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [(
            NaiveDate::from_ymd_res(2021, 12, 23)?,
            "Christmas Eve (observed)",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (
                NaiveDate::from_ymd_res(2022, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2022, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 31)?, "New Year's Eve"),
            (
                NaiveDate::from_ymd_res(2022, 12, 30)?,
                "New Year's Eve (observed)",
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
                NaiveDate::from_ymd_res(2023, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2023, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2024, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2024, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2025, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2025, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2026, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2026, 12, 31)?, "New Year's Eve"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [(
            NaiveDate::from_ymd_res(2027, 12, 23)?,
            "Christmas Eve (observed)",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (
                NaiveDate::from_ymd_res(2028, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2028, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2029, 12, 31)?, "New Year's Eve"),
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
                NaiveDate::from_ymd_res(2030, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Christmas Eve"),
            (NaiveDate::from_ymd_res(2030, 12, 31)?, "New Year's Eve"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
