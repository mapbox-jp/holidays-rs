//! United States (American Samoa)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (American Samoa)";
const COUNTY_CODE: Country = Country::US_AS;

/// Generate holiday map for United States (American Samoa).
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
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2000, 12, 22)?,
                "Christmas Eve (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2001, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2002, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2003, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2004, 12, 23)?,
            "Christmas Eve (observed)",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2005,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2005, 12, 23)?,
                "Christmas Eve (observed)",
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
        [
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2006, 12, 22)?,
                "Christmas Eve (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2007, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2008,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2008, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2009, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2010,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2010, 12, 23)?,
            "Christmas Eve (observed)",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2011,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2011, 12, 23)?,
                "Christmas Eve (observed)",
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
        [(NaiveDate::from_ymd_res(2012, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2013,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2013, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2014,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2014, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2015,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2015, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2016,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2016, 12, 23)?,
                "Christmas Eve (observed)",
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
        [
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2017, 12, 22)?,
                "Christmas Eve (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2018, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2019,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2019, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2020,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2020, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2021,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2021, 12, 23)?,
            "Christmas Eve (observed)",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2022,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2022, 12, 23)?,
                "Christmas Eve (observed)",
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
        [
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2023, 12, 22)?,
                "Christmas Eve (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2024,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2024, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2025,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2025, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2026,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2026, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2027,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2027, 12, 23)?,
            "Christmas Eve (observed)",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2028,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2028, 12, 22)?,
                "Christmas Eve (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2029,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2029, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2030,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2030, 12, 24)?, "Christmas Eve")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
