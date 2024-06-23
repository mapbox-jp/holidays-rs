//! Germany (Saxony (Sachsen))
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Germany (Saxony (Sachsen))";
const COUNTY_CODE: Country = Country::DE_SN;

/// Generate holiday map for Germany (Saxony (Sachsen)).
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
            (NaiveDate::from_ymd_res(2000, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2000, 11, 22)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2001, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2001, 11, 21)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2002, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2002, 11, 20)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2003, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2003, 11, 19)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2004, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2004, 11, 17)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2005, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2005, 11, 16)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2006, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2006, 11, 22)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2007, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2007, 11, 21)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2008, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2008, 11, 19)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2009, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2009, 11, 18)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2010, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2011, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2011, 11, 16)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2012, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2012, 11, 21)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2013, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2013, 11, 20)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2014, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2014, 11, 19)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2015, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2015, 11, 18)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2016, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2016, 11, 16)?, "Buß- und Bettag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2017,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2017, 11, 22)?, "Buß- und Bettag")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2018, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2018, 11, 21)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2019, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2019, 11, 20)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2020, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2020, 11, 18)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2021, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2021, 11, 17)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2022, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2022, 11, 16)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2023, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2023, 11, 22)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2024, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2024, 11, 20)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2025, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2025, 11, 19)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2026, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2026, 11, 18)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2027, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2027, 11, 17)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2028, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2028, 11, 22)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2029, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2029, 11, 21)?, "Buß- und Bettag"),
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
            (NaiveDate::from_ymd_res(2030, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2030, 11, 20)?, "Buß- und Bettag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
