//! United States (North Carolina)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (North Carolina)";
const COUNTY_CODE: Country = Country::US_NC;

/// Generate holiday map for United States (North Carolina).
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
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2000, 11, 24)?,
                "Day After Thanksgiving",
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
        [
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2001, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2002, 11, 29)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2003, 11, 28)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2004, 11, 26)?,
                "Day After Thanksgiving",
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
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2005, 11, 25)?,
                "Day After Thanksgiving",
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
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2006, 11, 24)?,
                "Day After Thanksgiving",
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
        [
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2007, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2008, 11, 28)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2010, 11, 26)?,
                "Day After Thanksgiving",
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
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2011, 11, 25)?,
                "Day After Thanksgiving",
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
        [
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2012, 11, 23)?,
                "Day After Thanksgiving",
            ),
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
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2013, 11, 29)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2014, 11, 28)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2015, 11, 27)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Day After Christmas",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 28)?,
                "Day After Christmas (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2016, 11, 25)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2016, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Day After Christmas (observed)",
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
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2017, 11, 24)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2017, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Day After Christmas",
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
        [
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2018, 11, 23)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2019, 11, 29)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2020, 11, 27)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Day After Christmas",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 28)?,
                "Day After Christmas (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2021, 11, 26)?,
                "Day After Thanksgiving",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Day After Christmas",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Day After Christmas (observed)",
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
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2022, 11, 25)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2022, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Day After Christmas (observed)",
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
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2023, 11, 24)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2023, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Day After Christmas",
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
        [
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2024, 11, 29)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2025, 11, 28)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2026, 11, 27)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Day After Christmas",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 28)?,
                "Day After Christmas (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2027, 11, 26)?,
                "Day After Thanksgiving",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Day After Christmas",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Day After Christmas (observed)",
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
        [
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2028, 11, 24)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2028, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Day After Christmas",
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
        [
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2029, 11, 23)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Day After Christmas",
            ),
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
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2030, 11, 29)?,
                "Day After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Day After Christmas",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
