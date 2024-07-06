//! United States (Virgin Islands, U.S..)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Virgin Islands, U.S..)";
const COUNTY_CODE: Country = Country::US_VI;

/// Generate holiday map for United States (Virgin Islands, U.S..).
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
            (NaiveDate::from_ymd_res(2000, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2000, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2001, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2003, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2004, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2005, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Liberty Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2006, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2008, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2009, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2010, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2011, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Liberty Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2012, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2014, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2015, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2016, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Liberty Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2017, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2019, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2020, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2021, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2022, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Liberty Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2023, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2025, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2026, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2027, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2028, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Christmas Second Day",
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
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Three Kings Day"),
            (NaiveDate::from_ymd_res(2030, 3, 31)?, "Transfer Day"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Holy Thursday"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 7, 3)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Liberty Day"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Christmas Second Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
