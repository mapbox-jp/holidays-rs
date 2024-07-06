//! United States (Puerto Rico)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Puerto Rico)";
const COUNTY_CODE: Country = Country::US_PR;

/// Generate holiday map for United States (Puerto Rico).
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
            (NaiveDate::from_ymd_res(2000, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2000, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2000, 11, 19)?, "Discovery Day"),
            (
                NaiveDate::from_ymd_res(2000, 11, 20)?,
                "Discovery Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2001, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2001, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2002, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2002, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2003, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2003, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2004, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 7, 25)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2004, 7, 26)?,
                "Constitution Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2005, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2005, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2006, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2006, 11, 19)?, "Discovery Day"),
            (
                NaiveDate::from_ymd_res(2006, 11, 20)?,
                "Discovery Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2007, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2007, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2008, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2009, 3, 22)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2009, 3, 23)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2009, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2010, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 7, 25)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2010, 7, 26)?,
                "Constitution Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2011, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2011, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2012, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2012, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2013, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2013, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2014, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2014, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2015, 3, 22)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2015, 3, 23)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2015, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2016, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2016, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2017, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2017, 11, 19)?, "Discovery Day"),
            (
                NaiveDate::from_ymd_res(2017, 11, 20)?,
                "Discovery Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2018, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2018, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2019, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2019, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2020, 3, 22)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2020, 3, 23)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2020, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2021, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 7, 25)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 26)?,
                "Constitution Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2022, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2022, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2023, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2023, 11, 19)?, "Discovery Day"),
            (
                NaiveDate::from_ymd_res(2023, 11, 20)?,
                "Discovery Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2024, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2024, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2025, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2025, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2026, 3, 22)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2026, 3, 23)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2026, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2027, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 7, 25)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2027, 7, 26)?,
                "Constitution Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2028, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2028, 11, 19)?, "Discovery Day"),
            (
                NaiveDate::from_ymd_res(2028, 11, 20)?,
                "Discovery Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2029, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2029, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Epiphany"),
            (NaiveDate::from_ymd_res(2030, 3, 22)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 7, 25)?, "Constitution Day"),
            (NaiveDate::from_ymd_res(2030, 11, 19)?, "Discovery Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
