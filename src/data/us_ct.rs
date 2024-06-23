//! United States (Connecticut)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Connecticut)";
const COUNTY_CODE: Country = Country::US_CT;

/// Generate holiday map for United States (Connecticut).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_subdivision_year(
        years,
        2000,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2000, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2000, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2001, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2004, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2005,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2005, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2005, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2006,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2006, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2006, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2007, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2008,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2008, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2009, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2010,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2010, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2011,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2011, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2011, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2012,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2012, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2012, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2013,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2014,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2014, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2015,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2015, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2016,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2016, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2017,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2017, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2017, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2018, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2019,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2019, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2020,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2020, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2021,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2022,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2022, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2022, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2023,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2023, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2023, 2, 13)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2024,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2025,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2025, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2026,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2026, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2027,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2027, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2028,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2028, 2, 12)?, "Lincoln's Birthday"),
            (
                NaiveDate::from_ymd_res(2028, 2, 11)?,
                "Lincoln's Birthday (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2029,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2030,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2030, 2, 12)?, "Lincoln's Birthday"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
