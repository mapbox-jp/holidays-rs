//! United States (Florida)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Florida)";
const COUNTY_CODE: Country = Country::US_FL;

/// Generate holiday map for United States (Florida).
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
            NaiveDate::from_ymd_res(2000, 11, 24)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [(
            NaiveDate::from_ymd_res(2001, 11, 23)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [(
            NaiveDate::from_ymd_res(2002, 11, 29)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [(
            NaiveDate::from_ymd_res(2003, 11, 28)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [(
            NaiveDate::from_ymd_res(2004, 11, 26)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [(
            NaiveDate::from_ymd_res(2005, 11, 25)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [(
            NaiveDate::from_ymd_res(2006, 11, 24)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [(
            NaiveDate::from_ymd_res(2007, 11, 23)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [(
            NaiveDate::from_ymd_res(2008, 11, 28)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [(
            NaiveDate::from_ymd_res(2009, 11, 27)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [(
            NaiveDate::from_ymd_res(2010, 11, 26)?,
            "Friday After Thanksgiving",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 25)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2012, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 23)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2013, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 29)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2014, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 28)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2015, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 27)?,
                "Friday After Thanksgiving",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [(
            NaiveDate::from_ymd_res(2016, 11, 25)?,
            "Friday After Thanksgiving",
        )],
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
            (
                NaiveDate::from_ymd_res(2017, 11, 24)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 23)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2019, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 29)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2020, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 27)?,
                "Friday After Thanksgiving",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [(
            NaiveDate::from_ymd_res(2021, 11, 26)?,
            "Friday After Thanksgiving",
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
            (
                NaiveDate::from_ymd_res(2022, 11, 25)?,
                "Friday After Thanksgiving",
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
            (
                NaiveDate::from_ymd_res(2023, 11, 24)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2024, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 29)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2025, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 28)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2026, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 27)?,
                "Friday After Thanksgiving",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [(
            NaiveDate::from_ymd_res(2027, 11, 26)?,
            "Friday After Thanksgiving",
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
            (
                NaiveDate::from_ymd_res(2028, 11, 24)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 23)?,
                "Friday After Thanksgiving",
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
                NaiveDate::from_ymd_res(2030, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 29)?,
                "Friday After Thanksgiving",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
