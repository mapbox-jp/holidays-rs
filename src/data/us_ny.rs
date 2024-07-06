//! United States (New York)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (New York)";
const COUNTY_CODE: Country = Country::US_NY;

/// Generate holiday map for United States (New York).
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
            (NaiveDate::from_ymd_res(2000, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2000, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [(NaiveDate::from_ymd_res(2001, 2, 12)?, "Lincoln's Birthday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [(NaiveDate::from_ymd_res(2002, 2, 12)?, "Lincoln's Birthday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [(NaiveDate::from_ymd_res(2003, 2, 12)?, "Lincoln's Birthday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2004, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2005, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2005, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2006, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2006, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2007, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2007, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2008, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2008, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 4)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2009, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2010, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2011, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2012, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2012, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 6)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2013, 2, 15)?,
                "Susan B. Anthony Day",
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
            (NaiveDate::from_ymd_res(2014, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2014, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 4)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2015, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 3)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2016, 11, 8)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2017, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 7)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 6)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2019, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 5)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2020, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 3)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2021, 11, 2)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2022, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 8)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2023, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 7)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2024, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 5)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2025, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 4)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2026, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 3)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2027, 11, 2)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2028, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 7)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 6)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2030, 2, 15)?,
                "Susan B. Anthony Day",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 5)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
