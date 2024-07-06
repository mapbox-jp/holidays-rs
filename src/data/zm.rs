//! Zambia
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Zambia";
const COUNTY_CODE: Country = Country::ZM;

/// Generate holiday map for Zambia.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 12)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2000, 3, 13)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 22)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2000, 7, 3)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2000, 7, 4)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2000, 8, 7)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2000, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2001, 7, 2)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2001, 7, 3)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2001, 8, 6)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2001, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 3, 30)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2002, 7, 1)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2002, 7, 2)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2002, 8, 5)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2002, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 19)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 5, 25)?, "Africa Freedom Day"),
            (
                NaiveDate::from_ymd_res(2003, 5, 26)?,
                "Africa Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2003, 7, 7)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2003, 7, 8)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2003, 8, 4)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2003, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2004, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2004, 7, 5)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2004, 7, 6)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2004, 8, 2)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2004, 10, 24)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2004, 10, 25)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2005, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2005, 7, 4)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2005, 7, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2005, 8, 1)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2005, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Christmas Day (observed)",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 12)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2006, 3, 13)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 15)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2006, 7, 3)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2006, 7, 4)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2006, 8, 7)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2007, 7, 2)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2007, 7, 3)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2007, 8, 6)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2007, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 5, 25)?, "Africa Freedom Day"),
            (
                NaiveDate::from_ymd_res(2008, 5, 26)?,
                "Africa Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2008, 7, 7)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2008, 7, 8)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2008, 8, 4)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2008, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "International Women's Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 11)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2009, 7, 6)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2009, 7, 7)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2009, 8, 3)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2009, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2010, 7, 5)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2010, 7, 6)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2010, 8, 2)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2010, 10, 24)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2010, 10, 25)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2011, 7, 4)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2011, 7, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2011, 8, 1)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2011, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Christmas Day (observed)",
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
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 7)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2012, 7, 2)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2012, 7, 3)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2012, 8, 6)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2012, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 3, 30)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2013, 7, 1)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2013, 7, 2)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2013, 8, 5)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2013, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 5, 25)?, "Africa Freedom Day"),
            (
                NaiveDate::from_ymd_res(2014, 5, 26)?,
                "Africa Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 7)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2014, 7, 8)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2014, 8, 4)?, "Farmers' Day"),
            (NaiveDate::from_ymd_res(2014, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "International Women's Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2015, 7, 6)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2015, 7, 7)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2015, 8, 3)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2015, 10, 18)?,
                "National Prayer Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 19)?,
                "National Prayer Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2016, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 26)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2016, 7, 4)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2016, 7, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2016, 8, 1)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2016, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 11)?,
                "General elections and referendum",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 13)?,
                "Inauguration ceremony of President-elect and Vice President-elect",
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
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 12)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2017, 3, 13)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2017, 7, 3)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2017, 7, 4)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2017, 8, 7)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2017, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2018, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2018, 7, 2)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2018, 7, 3)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2018, 8, 6)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2018, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 3, 9)?, "Public holiday"),
            (
                NaiveDate::from_ymd_res(2018, 7, 26)?,
                "Lusaka mayoral and other local government elections",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2019, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 20)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2019, 7, 1)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2019, 7, 2)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2019, 8, 5)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2019, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2020, 3, 8)?,
                "International Women's Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 11)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2020, 7, 6)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2020, 7, 7)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2020, 8, 3)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2020, 10, 18)?,
                "National Prayer Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 19)?,
                "National Prayer Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2021, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 3)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2021, 7, 5)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2021, 7, 6)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2021, 8, 2)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 24)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2021, 10, 25)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 2)?,
                "Memorial service for Kenneth Kaunda",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 7)?,
                "Funeral of Kenneth Kaunda",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 12)?, "General elections"),
            (
                NaiveDate::from_ymd_res(2021, 8, 13)?,
                "Counting in general elections",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 24)?,
                "Presidential inauguration",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 16)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2022, 7, 4)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2022, 7, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2022, 8, 1)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2022, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 18)?,
                "Funeral of Rupiah Banda",
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
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 12)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2023, 3, 13)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 8)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2023, 7, 3)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2023, 7, 4)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2023, 8, 7)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2023, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2024, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 3, 30)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 4, 28)?, "Kenneth Kaunda Day"),
            (
                NaiveDate::from_ymd_res(2024, 4, 29)?,
                "Kenneth Kaunda Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2024, 7, 1)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2024, 7, 2)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2024, 8, 5)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2024, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2025, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 5, 25)?, "Africa Freedom Day"),
            (
                NaiveDate::from_ymd_res(2025, 5, 26)?,
                "Africa Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2025, 7, 7)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2025, 7, 8)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2025, 8, 4)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2025, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2026, 3, 8)?,
                "International Women's Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 4)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2026, 7, 6)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2026, 7, 7)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2026, 8, 3)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2026, 10, 18)?,
                "National Prayer Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 19)?,
                "National Prayer Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2027, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 27)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2027, 7, 5)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2027, 7, 6)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2027, 8, 2)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2027, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 24)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2027, 10, 25)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2028, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 12)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2028, 3, 13)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2028, 7, 3)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2028, 7, 4)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2028, 8, 7)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2028, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2029, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 4, 28)?, "Kenneth Kaunda Day"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2029, 7, 2)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2029, 7, 3)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2029, 8, 6)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2029, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2030, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 12)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 20)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 4, 28)?, "Kenneth Kaunda Day"),
            (
                NaiveDate::from_ymd_res(2030, 4, 29)?,
                "Kenneth Kaunda Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 5, 25)?, "Africa Freedom Day"),
            (NaiveDate::from_ymd_res(2030, 7, 1)?, "Heroes' Day"),
            (NaiveDate::from_ymd_res(2030, 7, 2)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2030, 8, 5)?, "Farmers' Day"),
            (
                NaiveDate::from_ymd_res(2030, 10, 18)?,
                "National Prayer Day",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 24)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
