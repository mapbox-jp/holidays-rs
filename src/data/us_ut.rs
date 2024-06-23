//! United States (Utah)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Utah)";
const COUNTY_CODE: Country = Country::US_UT;

/// Generate holiday map for United States (Utah).
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
        [(NaiveDate::from_ymd_res(2000, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2001, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2002, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2003, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2004, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2004, 7, 23)?,
                "Pioneer Day (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2005, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2005, 7, 25)?,
                "Pioneer Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2006,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2006, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2007, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2008,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2008, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2009, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2010,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2010, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2010, 7, 23)?,
                "Pioneer Day (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2011, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2011, 7, 25)?,
                "Pioneer Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2012,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2012, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2013,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2013, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2014,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2014, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2015,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2015, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2016,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2016, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2016, 7, 25)?,
                "Pioneer Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2017,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2017, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2018, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2019,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2019, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2020,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2020, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2021,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2021, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 23)?,
                "Pioneer Day (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2022, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2022, 7, 25)?,
                "Pioneer Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2023,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2023, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2024,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2024, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2025,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2025, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2026,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2026, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2027,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2027, 7, 24)?, "Pioneer Day"),
            (
                NaiveDate::from_ymd_res(2027, 7, 23)?,
                "Pioneer Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2028,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2028, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2029,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2029, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2030,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2030, 7, 24)?, "Pioneer Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
