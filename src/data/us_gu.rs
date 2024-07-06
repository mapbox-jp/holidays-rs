//! United States (Guam)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Guam)";
const COUNTY_CODE: Country = Country::US_GU;

/// Generate holiday map for United States (Guam).
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
            (NaiveDate::from_ymd_res(2000, 3, 6)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2000, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2000, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 3, 5)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2001, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2001, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 3, 4)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2002, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2002, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 3, 3)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2003, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2003, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 3, 1)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2004, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2004, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 3, 7)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2005, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2005, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 3, 6)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2006, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2006, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 3, 5)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2007, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2007, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 3, 3)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2008, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 3, 2)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2009, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2009, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 3, 1)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2010, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2010, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 3, 7)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2011, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2011, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 3, 5)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2012, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2012, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 3, 4)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2013, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2013, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 3, 3)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2014, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2014, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 3, 2)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2015, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2015, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 3, 7)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2016, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2016, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 3, 6)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2017, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2017, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 3, 5)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2018, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2018, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 3, 4)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2019, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2019, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 3, 2)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2020, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2020, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 3, 1)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2021, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 3, 7)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2022, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2022, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 3, 6)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2023, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2023, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 3, 4)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2024, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2024, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2025, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2025, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 3, 2)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2026, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2026, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 3, 1)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2027, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2027, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 3, 6)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2028, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2028, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 3, 5)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2029, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2029, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 3, 4)?, "Guam Discovery Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2030, 7, 21)?,
                "Liberation Day (Guam)",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 2)?, "All Souls' Day"),
            (NaiveDate::from_ymd_res(2030, 12, 8)?, "Lady of Camarin Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
