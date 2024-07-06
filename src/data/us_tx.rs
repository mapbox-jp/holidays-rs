//! United States (Texas)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "United States (Texas)";
const COUNTY_CODE: Country = Country::US_TX;

/// Generate holiday map for United States (Texas).
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
                NaiveDate::from_ymd_res(2000, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 31)?, "Cesar Chavez Day"),
            (
                NaiveDate::from_ymd_res(2000, 4, 21)?,
                "Good Friday; San Jacinto Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 24)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2000, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2001, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2001, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 23)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Day After Christmas",
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
                NaiveDate::from_ymd_res(2002, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2002, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 29)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Day After Christmas",
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
                NaiveDate::from_ymd_res(2003, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2003, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 28)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2004, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2004, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 26)?,
                "Friday After Thanksgiving",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2005, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2005, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 25)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2005, 12, 23)?,
                "Christmas Eve (observed)",
            ),
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
                NaiveDate::from_ymd_res(2006, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2006, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 24)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2006, 12, 22)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2007, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2007, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 23)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Day After Christmas",
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
                NaiveDate::from_ymd_res(2008, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2008, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 28)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2009, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2009, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2010, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2010, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 26)?,
                "Friday After Thanksgiving",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2011, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2011, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 25)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2011, 12, 23)?,
                "Christmas Eve (observed)",
            ),
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
                NaiveDate::from_ymd_res(2012, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2012, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 23)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Day After Christmas",
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
                NaiveDate::from_ymd_res(2013, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2013, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 29)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2014,
        [
            (
                NaiveDate::from_ymd_res(2014, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2014, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 28)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2015,
        [
            (
                NaiveDate::from_ymd_res(2015, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2015, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 27)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2016, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2016, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 25)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2016, 12, 23)?,
                "Christmas Eve (observed)",
            ),
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
                NaiveDate::from_ymd_res(2017, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2017, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 24)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2018,
        [
            (
                NaiveDate::from_ymd_res(2018, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2018, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 23)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2019,
        [
            (
                NaiveDate::from_ymd_res(2019, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2019, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 29)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2020,
        [
            (
                NaiveDate::from_ymd_res(2020, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2020, 6, 19)?,
                "Emancipation Day In Texas",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 27)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2021, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2021, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 26)?,
                "Friday After Thanksgiving",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2022, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2022, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 25)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2022, 12, 23)?,
                "Christmas Eve (observed)",
            ),
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
                NaiveDate::from_ymd_res(2023, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2023, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 24)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2024,
        [
            (
                NaiveDate::from_ymd_res(2024, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2024, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 29)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2025,
        [
            (
                NaiveDate::from_ymd_res(2025, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2025, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 28)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2026,
        [
            (
                NaiveDate::from_ymd_res(2026, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2026, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 27)?,
                "Friday After Thanksgiving",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Christmas Eve"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2027, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 26)?,
                "Friday After Thanksgiving",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 23)?,
                "Christmas Eve (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Day After Christmas",
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
            (
                NaiveDate::from_ymd_res(2028, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2028, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 24)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2029,
        [
            (
                NaiveDate::from_ymd_res(2029, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2029, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 23)?,
                "Friday After Thanksgiving",
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

    build_year(
        years,
        2030,
        [
            (
                NaiveDate::from_ymd_res(2030, 1, 19)?,
                "Confederate Memorial Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 2)?,
                "Texas Independence Day",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 31)?, "Cesar Chavez Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "San Jacinto Day"),
            (
                NaiveDate::from_ymd_res(2030, 8, 27)?,
                "Lyndon Baines Johnson Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 29)?,
                "Friday After Thanksgiving",
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
