//! United States (South Carolina)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (South Carolina)";
const COUNTY_CODE: Country = Country::US_SC;

/// Generate holiday map for United States (South Carolina).
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
        [(
            NaiveDate::from_ymd_res(2000, 4, 24)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2001, 4, 23)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2002, 4, 22)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2003, 4, 28)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2004, 4, 26)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2005,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2005, 4, 25)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2006,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2006, 4, 24)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2007, 4, 23)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2008,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2008, 4, 28)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2009, 4, 27)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2010,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2010, 4, 26)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2011,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2011, 4, 25)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2012,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2012, 4, 23)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2013,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2013, 4, 22)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2014,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2014, 4, 28)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2015,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2015, 4, 27)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2016,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2016, 4, 25)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2017,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2017, 4, 24)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2018, 4, 23)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2019,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2019, 4, 22)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2020,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2020, 4, 27)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2021,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2021, 4, 26)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2022,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2022, 4, 25)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2023,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2023, 4, 24)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2024,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2024, 4, 22)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2025,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2025, 4, 28)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2026,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2026, 4, 27)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2027,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2027, 4, 26)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2028,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2028, 4, 24)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2029,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2029, 4, 23)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2030,
        &mut national_holidays,
        [(
            NaiveDate::from_ymd_res(2030, 4, 22)?,
            "Confederate Memorial Day",
        )],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
