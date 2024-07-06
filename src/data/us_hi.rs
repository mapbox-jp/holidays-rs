//! United States (Hawaii)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Hawaii)";
const COUNTY_CODE: Country = Country::US_HI;

/// Generate holiday map for United States (Hawaii).
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
            (
                NaiveDate::from_ymd_res(2000, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 27)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2000, 8, 18)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (
                NaiveDate::from_ymd_res(2001, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2001, 8, 17)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (
                NaiveDate::from_ymd_res(2002, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2002, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2002, 8, 16)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (
                NaiveDate::from_ymd_res(2003, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (
                NaiveDate::from_ymd_res(2004, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2004, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2004, 8, 20)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (
                NaiveDate::from_ymd_res(2005, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 25)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2005, 8, 19)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (
                NaiveDate::from_ymd_res(2006, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 27)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2006, 8, 18)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (
                NaiveDate::from_ymd_res(2007, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2007, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2007, 8, 17)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (
                NaiveDate::from_ymd_res(2008, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2008, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2009, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2009, 8, 21)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (
                NaiveDate::from_ymd_res(2010, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2010, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2010, 8, 20)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2011, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 25)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 11)?, "Kamehameha Day"),
            (
                NaiveDate::from_ymd_res(2011, 6, 10)?,
                "Kamehameha Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 19)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (
                NaiveDate::from_ymd_res(2012, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2012, 8, 17)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2013, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2013, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2013, 8, 16)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (
                NaiveDate::from_ymd_res(2014, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2015, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2015, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2015, 8, 21)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (
                NaiveDate::from_ymd_res(2016, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 25)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 11)?, "Kamehameha Day"),
            (
                NaiveDate::from_ymd_res(2016, 6, 10)?,
                "Kamehameha Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 19)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2017, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 27)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 11)?, "Kamehameha Day"),
            (
                NaiveDate::from_ymd_res(2017, 6, 12)?,
                "Kamehameha Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 18)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (
                NaiveDate::from_ymd_res(2018, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2018, 8, 17)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2019, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2019, 8, 16)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (
                NaiveDate::from_ymd_res(2020, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2020, 8, 21)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2021, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2021, 8, 20)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (
                NaiveDate::from_ymd_res(2022, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 25)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 11)?, "Kamehameha Day"),
            (
                NaiveDate::from_ymd_res(2022, 6, 10)?,
                "Kamehameha Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 19)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2023, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 27)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 11)?, "Kamehameha Day"),
            (
                NaiveDate::from_ymd_res(2023, 6, 12)?,
                "Kamehameha Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 18)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (
                NaiveDate::from_ymd_res(2024, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2024, 8, 16)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2025, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (
                NaiveDate::from_ymd_res(2026, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2026, 8, 21)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2027, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2027, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2027, 8, 20)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (
                NaiveDate::from_ymd_res(2028, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 27)?,
                "Prince Jonah Kuhio Kalanianaole Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 11)?, "Kamehameha Day"),
            (
                NaiveDate::from_ymd_res(2028, 6, 12)?,
                "Kamehameha Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 18)?, "Statehood Day"),
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
            (
                NaiveDate::from_ymd_res(2029, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2029, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2029, 8, 17)?, "Statehood Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (
                NaiveDate::from_ymd_res(2030, 3, 26)?,
                "Prince Jonah Kuhio Kalanianaole Day",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 11)?, "Kamehameha Day"),
            (NaiveDate::from_ymd_res(2030, 8, 16)?, "Statehood Day"),
            (NaiveDate::from_ymd_res(2030, 11, 5)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
