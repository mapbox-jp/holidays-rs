//! United States (Indiana)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Indiana)";
const COUNTY_CODE: Country = Country::US_IN;

/// Generate holiday map for United States (Indiana).
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
        [(NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday")],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2005,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday")],
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
            (NaiveDate::from_ymd_res(2006, 5, 2)?, "Primary Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday")],
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
            (NaiveDate::from_ymd_res(2008, 5, 6)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2008, 11, 4)?, "Election Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [(NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday")],
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
            (NaiveDate::from_ymd_res(2010, 5, 4)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "Election Day"),
            (NaiveDate::from_ymd_res(2010, 11, 26)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2011, 11, 25)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2012, 5, 8)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2012, 11, 6)?, "Election Day"),
            (NaiveDate::from_ymd_res(2012, 11, 23)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2013, 11, 29)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2014, 5, 6)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2014, 11, 4)?, "Election Day"),
            (NaiveDate::from_ymd_res(2014, 11, 28)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2015, 5, 5)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2015, 11, 3)?, "Election Day"),
            (NaiveDate::from_ymd_res(2015, 11, 27)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2016, 5, 3)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2016, 11, 8)?, "Election Day"),
            (NaiveDate::from_ymd_res(2016, 11, 25)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2017, 5, 2)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2017, 11, 7)?, "Election Day"),
            (NaiveDate::from_ymd_res(2017, 11, 24)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2018, 5, 8)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2018, 11, 6)?, "Election Day"),
            (NaiveDate::from_ymd_res(2018, 11, 23)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2019, 5, 7)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2019, 11, 5)?, "Election Day"),
            (NaiveDate::from_ymd_res(2019, 11, 29)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2020, 5, 5)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2020, 11, 3)?, "Election Day"),
            (NaiveDate::from_ymd_res(2020, 11, 27)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2021, 5, 4)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2021, 11, 2)?, "Election Day"),
            (NaiveDate::from_ymd_res(2021, 11, 26)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2022, 11, 8)?, "Election Day"),
            (NaiveDate::from_ymd_res(2022, 11, 25)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2023, 5, 2)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2023, 11, 7)?, "Election Day"),
            (NaiveDate::from_ymd_res(2023, 11, 24)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2024, 5, 7)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2024, 11, 5)?, "Election Day"),
            (NaiveDate::from_ymd_res(2024, 11, 29)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2025, 5, 6)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2025, 11, 4)?, "Election Day"),
            (NaiveDate::from_ymd_res(2025, 11, 28)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2026, 5, 5)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2026, 11, 3)?, "Election Day"),
            (NaiveDate::from_ymd_res(2026, 11, 27)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2027, 5, 4)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2027, 11, 2)?, "Election Day"),
            (NaiveDate::from_ymd_res(2027, 11, 26)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2028, 5, 2)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2028, 11, 7)?, "Election Day"),
            (NaiveDate::from_ymd_res(2028, 11, 24)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2029, 5, 8)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2029, 11, 6)?, "Election Day"),
            (NaiveDate::from_ymd_res(2029, 11, 23)?, "Lincoln's Birthday"),
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
            (NaiveDate::from_ymd_res(2030, 5, 7)?, "Primary Election Day"),
            (NaiveDate::from_ymd_res(2030, 11, 5)?, "Election Day"),
            (NaiveDate::from_ymd_res(2030, 11, 29)?, "Lincoln's Birthday"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
