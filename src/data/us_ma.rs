//! United States (Massachusetts)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Massachusetts)";
const COUNTY_CODE: Country = Country::US_MA;

/// Generate holiday map for United States (Massachusetts).
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
            (NaiveDate::from_ymd_res(2000, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2000, 4, 17)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2001, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2001, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2002, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2002, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2002, 4, 15)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2003, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2004, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2004, 4, 19)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2005, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2005, 4, 18)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2006, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2007, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2007, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 16)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2008, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2008, 4, 21)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2009, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2010, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2010, 4, 19)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2011, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2011, 4, 18)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2012, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2012, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2013, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2013, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 15)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2014, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2015, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2015, 4, 20)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2016, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2016, 4, 18)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2017, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2018, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2018, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 16)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2019, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2019, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 15)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2020, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2020, 4, 20)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2021, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2021, 4, 19)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2022, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2023, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2023, 4, 17)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2024, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2024, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 15)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2025, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2026, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2026, 4, 20)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2027, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2027, 4, 19)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2028, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2029, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2029, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 16)?, "Patriots' Day"),
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
            (NaiveDate::from_ymd_res(2030, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2030, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 15)?, "Patriots' Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
