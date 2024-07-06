//! United States (Virginia)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Virginia)";
const COUNTY_CODE: Country = Country::US_VA;

/// Generate holiday map for United States (Virginia).
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
        [(NaiveDate::from_ymd_res(2000, 1, 14)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 12)?, "Lee Jackson Day"),
            (NaiveDate::from_ymd_res(2001, 1, 20)?, "Inauguration Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [(NaiveDate::from_ymd_res(2002, 1, 18)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [(NaiveDate::from_ymd_res(2003, 1, 17)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [(NaiveDate::from_ymd_res(2004, 1, 16)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 14)?, "Lee Jackson Day"),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "Inauguration Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [(NaiveDate::from_ymd_res(2006, 1, 13)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [(NaiveDate::from_ymd_res(2007, 1, 12)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [(NaiveDate::from_ymd_res(2008, 1, 18)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 16)?, "Lee Jackson Day"),
            (NaiveDate::from_ymd_res(2009, 1, 20)?, "Inauguration Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [(NaiveDate::from_ymd_res(2010, 1, 15)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [(NaiveDate::from_ymd_res(2011, 1, 14)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [(NaiveDate::from_ymd_res(2012, 1, 13)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 18)?, "Lee Jackson Day"),
            (NaiveDate::from_ymd_res(2013, 1, 20)?, "Inauguration Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [(NaiveDate::from_ymd_res(2014, 1, 17)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [(NaiveDate::from_ymd_res(2015, 1, 16)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [(NaiveDate::from_ymd_res(2016, 1, 15)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 13)?, "Lee Jackson Day"),
            (NaiveDate::from_ymd_res(2017, 1, 20)?, "Inauguration Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [(NaiveDate::from_ymd_res(2018, 1, 12)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [(NaiveDate::from_ymd_res(2019, 1, 18)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [(NaiveDate::from_ymd_res(2020, 1, 17)?, "Lee Jackson Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [(NaiveDate::from_ymd_res(2021, 1, 20)?, "Inauguration Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2022, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2023, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2024, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2025, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2026, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2027, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2028, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2029,
        [(NaiveDate::from_ymd_res(2029, 1, 20)?, "Inauguration Day")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(years, 2030, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    Ok(map)
}
