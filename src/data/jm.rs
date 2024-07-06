//! Jamaica
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Jamaica";
const COUNTY_CODE: Country = Country::JM;

/// Generate holiday map for Jamaica.
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
            (NaiveDate::from_ymd_res(2000, 3, 8)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2000, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2000, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2000, 8, 7)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 16)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2001, 2, 28)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2001, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2001, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2001, 10, 15)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2002, 2, 13)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2002, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2002, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2002, 10, 21)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2003, 3, 5)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2003, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2003, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2003, 10, 20)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2004, 2, 25)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2004, 5, 24)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 1)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2004, 8, 2)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2004, 10, 18)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2005, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2005, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2005, 10, 17)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2006, 3, 1)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2006, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2006, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2006, 8, 7)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 16)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2007, 2, 21)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2007, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2007, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2008, 2, 6)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2008, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2008, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 20)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2009, 2, 25)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2009, 5, 25)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2009, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2009, 10, 19)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2010, 2, 17)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2010, 5, 24)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 1)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2010, 8, 2)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2010, 10, 18)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2011, 3, 9)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2011, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2011, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2011, 10, 17)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2012, 2, 22)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2012, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2012, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2012, 10, 15)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2013, 2, 13)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2013, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2013, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2013, 10, 21)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2014, 3, 5)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2014, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2014, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2014, 10, 20)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2015, 2, 18)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2015, 5, 25)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2015, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2015, 10, 19)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2016, 2, 10)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2016, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2016, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2016, 10, 17)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2017, 3, 1)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2017, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2017, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2017, 8, 7)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 16)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2018, 2, 14)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2018, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2018, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2018, 10, 15)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2019, 3, 6)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2019, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2019, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2019, 10, 21)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2020, 2, 26)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2020, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2020, 10, 19)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2021, 2, 17)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2021, 5, 24)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 1)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2021, 8, 2)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2022, 3, 2)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2022, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2022, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2022, 10, 17)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2023, 2, 22)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2023, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2023, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2023, 8, 7)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 16)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2024, 2, 14)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2024, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2024, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2024, 10, 21)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2025, 3, 5)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2025, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2025, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2025, 10, 20)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2026, 2, 18)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2026, 5, 25)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2026, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2026, 10, 19)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2027, 2, 10)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 5, 23)?, "National Labour Day"),
            (
                NaiveDate::from_ymd_res(2027, 5, 24)?,
                "National Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 1)?, "Emancipation Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 2)?,
                "Emancipation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2027, 10, 18)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2028, 3, 1)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2028, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2028, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2028, 8, 7)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 16)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2029, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2029, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2029, 10, 15)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2030, 3, 6)?, "Ash Wednesday"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 5, 23)?, "National Labour Day"),
            (NaiveDate::from_ymd_res(2030, 8, 1)?, "Emancipation Day"),
            (NaiveDate::from_ymd_res(2030, 8, 6)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2030, 10, 21)?,
                "National Heroes Day",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
