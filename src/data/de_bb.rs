//! Germany (Brandenburg)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Germany (Brandenburg)";
const COUNTY_CODE: Country = Country::DE_BB;

/// Generate holiday map for Germany (Brandenburg).
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
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2000, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2001, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2002, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2003, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2004, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2005, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2006, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2007, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2008, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2009, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2010, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2011, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2012, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2013, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2014, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2015, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2016, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Pfingstsonntag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2018, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2019, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2020, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2021, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2022, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2023, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2024, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2025, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2026, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2027, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2028, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2029, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Ostersonntag"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Pfingstsonntag"),
            (NaiveDate::from_ymd_res(2030, 10, 31)?, "Reformationstag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
