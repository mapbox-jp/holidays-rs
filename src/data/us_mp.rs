//! United States (Northern Mariana Islands)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Northern Mariana Islands)";
const COUNTY_CODE: Country = Country::US_MP;

/// Generate holiday map for United States (Northern Mariana Islands).
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
                NaiveDate::from_ymd_res(2000, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2000, 11, 3)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2001, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 23)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2001, 11, 5)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 7)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2002, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 25)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2002, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 9)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2003, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2003, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2004, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2004, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2005, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2005, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2006, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2006, 11, 3)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2007, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 23)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2007, 11, 5)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2007, 12, 7)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2008, 11, 4)?,
                "Citizenship Day; Election Day",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2009, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2009, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2010, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "Election Day"),
            (NaiveDate::from_ymd_res(2010, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2010, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2011, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2011, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2012, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 23)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 11, 6)?, "Election Day"),
            (NaiveDate::from_ymd_res(2012, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2012, 11, 5)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2012, 12, 7)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2013, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 25)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2013, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2013, 12, 9)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2014, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2014, 11, 4)?,
                "Citizenship Day; Election Day",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2015, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2015, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2016, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 11, 8)?, "Election Day"),
            (NaiveDate::from_ymd_res(2016, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2016, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2017, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2017, 11, 3)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2018, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 23)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 11, 6)?, "Election Day"),
            (NaiveDate::from_ymd_res(2018, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2018, 11, 5)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2018, 12, 7)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2019, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 25)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2019, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2019, 12, 9)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2020, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 11, 3)?, "Election Day"),
            (NaiveDate::from_ymd_res(2020, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2020, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2021, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2021, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2022, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 11, 8)?, "Election Day"),
            (NaiveDate::from_ymd_res(2022, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2022, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2023, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2023, 11, 3)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2024, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 25)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 11, 5)?, "Election Day"),
            (NaiveDate::from_ymd_res(2024, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2024, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2024, 12, 9)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2025, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2025, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2026, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 11, 3)?, "Election Day"),
            (NaiveDate::from_ymd_res(2026, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2026, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2027, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2027, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2028, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 11, 7)?, "Election Day"),
            (NaiveDate::from_ymd_res(2028, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2028, 11, 3)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 8)?, "Constitution Day"),
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
                NaiveDate::from_ymd_res(2029, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 23)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 11, 4)?, "Citizenship Day"),
            (
                NaiveDate::from_ymd_res(2029, 11, 5)?,
                "Citizenship Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2029, 12, 7)?,
                "Constitution Day (observed)",
            ),
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
                NaiveDate::from_ymd_res(2030, 3, 24)?,
                "Commonwealth Covenant Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 25)?,
                "Commonwealth Covenant Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 11, 5)?, "Election Day"),
            (NaiveDate::from_ymd_res(2030, 11, 4)?, "Citizenship Day"),
            (NaiveDate::from_ymd_res(2030, 12, 8)?, "Constitution Day"),
            (
                NaiveDate::from_ymd_res(2030, 12, 9)?,
                "Constitution Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
