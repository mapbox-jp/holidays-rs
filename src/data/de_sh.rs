//! Germany (Schleswig-Holstein)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Germany (Schleswig-Holstein)";
const COUNTY_CODE: Country = Country::DE_SH;

/// Generate holiday map for Germany (Schleswig-Holstein).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_year(years, 2000, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2001, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2002, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2003, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2004, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2005, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2006, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2007, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2008, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2009, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2010, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2011, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2012, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2013, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2014, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2015, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2016, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(years, 2017, [], &mut map, COUNTY_CODE, COUNTY_NAME);

    build_year(
        years,
        2018,
        [(NaiveDate::from_ymd_res(2018, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [(NaiveDate::from_ymd_res(2019, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [(NaiveDate::from_ymd_res(2020, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [(NaiveDate::from_ymd_res(2021, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [(NaiveDate::from_ymd_res(2022, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [(NaiveDate::from_ymd_res(2023, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [(NaiveDate::from_ymd_res(2024, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [(NaiveDate::from_ymd_res(2025, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [(NaiveDate::from_ymd_res(2026, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [(NaiveDate::from_ymd_res(2027, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [(NaiveDate::from_ymd_res(2028, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [(NaiveDate::from_ymd_res(2029, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [(NaiveDate::from_ymd_res(2030, 10, 31)?, "Reformationstag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
