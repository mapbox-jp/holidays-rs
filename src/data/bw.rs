//! Botswana
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Botswana";
const COUNTY_CODE: Country = Country::BW;

/// Generate holiday map for Botswana.
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
                NaiveDate::from_ymd_res(2000, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "New Year's Day Holiday (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 22)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2000, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2000, 7, 17)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2000, 7, 18)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2000, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2000, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 2)?,
                "Botswana Day Holiday (observed)",
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
            (
                NaiveDate::from_ymd_res(2001, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2001, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "Sir Seretse Khama Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 7, 16)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2001, 7, 17)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2001, 10, 2)?,
                "Botswana Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2002, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 3, 30)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2002, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2002, 7, 15)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2002, 7, 16)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2002, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2003, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 19)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2003, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2003, 7, 21)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2003, 7, 22)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2003, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2004, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2004, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2004, 7, 19)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2004, 7, 20)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2004, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2005, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "New Year's Day Holiday (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2005, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2005, 7, 18)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2005, 7, 19)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2005, 10, 1)?,
                "Botswana Day Holiday",
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
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 15)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2006, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2006, 7, 17)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2006, 7, 18)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2006, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 2)?,
                "Botswana Day Holiday (observed)",
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
            (
                NaiveDate::from_ymd_res(2007, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2007, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "Sir Seretse Khama Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 7, 16)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2007, 7, 17)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2007, 10, 2)?,
                "Botswana Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2008, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Ascension Day; Labour Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2008, 7, 21)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2008, 7, 22)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2009, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 11)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2009, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2009, 7, 20)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2009, 7, 21)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2009, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2010, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2010, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2010, 7, 19)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2010, 7, 20)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2010, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2011, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "New Year's Day Holiday (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2011, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2011, 7, 18)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2011, 7, 19)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2011, 10, 1)?,
                "Botswana Day Holiday",
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
                NaiveDate::from_ymd_res(2012, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 7)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2012, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "Sir Seretse Khama Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 7, 16)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2012, 7, 17)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2012, 10, 2)?,
                "Botswana Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2013, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 3, 30)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2013, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2013, 7, 15)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2013, 7, 16)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2013, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2014, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2014, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 21)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2014, 7, 22)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2014, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2015, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 20)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2015, 7, 21)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2015, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2016, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 26)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2016, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 18)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2016, 7, 19)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2016, 10, 1)?,
                "Botswana Day Holiday",
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
                NaiveDate::from_ymd_res(2017, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2017, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2017, 7, 17)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2017, 7, 18)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2017, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 2)?,
                "Botswana Day Holiday (observed)",
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
            (
                NaiveDate::from_ymd_res(2018, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2018, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "Sir Seretse Khama Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 7, 16)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2018, 7, 17)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2018, 10, 2)?,
                "Botswana Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2019, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 20)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2019, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2019, 7, 15)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2019, 7, 16)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2019, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
            (NaiveDate::from_ymd_res(2019, 7, 2)?, "Public Holiday"),
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
                NaiveDate::from_ymd_res(2020, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 11)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2020, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 20)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2020, 7, 21)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2020, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
            (NaiveDate::from_ymd_res(2020, 12, 28)?, "Boxing Day Holiday"),
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
                NaiveDate::from_ymd_res(2021, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 3)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "Labour Day Holiday"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 19)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2021, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2022, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "New Year's Day Holiday (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 16)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2022, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 18)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2022, 7, 19)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2022, 10, 1)?,
                "Botswana Day Holiday",
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
                NaiveDate::from_ymd_res(2023, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 8)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2023, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2023, 7, 17)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2023, 7, 18)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2023, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 2)?,
                "Botswana Day Holiday (observed)",
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
            (
                NaiveDate::from_ymd_res(2024, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 3, 30)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2024, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2024, 7, 15)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2024, 7, 16)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2024, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2025, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2025, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2025, 7, 21)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2025, 7, 22)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2025, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2026, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 4)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2026, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2026, 7, 20)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2026, 7, 21)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2026, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
            (NaiveDate::from_ymd_res(2026, 12, 28)?, "Boxing Day Holiday"),
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
                NaiveDate::from_ymd_res(2027, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 27)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "Labour Day Holiday"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2027, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2027, 7, 19)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2027, 7, 20)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2027, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2028, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "New Year's Day Holiday (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2028, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2028, 7, 17)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2028, 7, 18)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2028, 10, 1)?,
                "Botswana Day Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 2)?,
                "Botswana Day Holiday (observed)",
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
            (
                NaiveDate::from_ymd_res(2029, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2029, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 2)?,
                "Sir Seretse Khama Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 7, 16)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2029, 7, 17)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2029, 10, 2)?,
                "Botswana Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 1)?,
                "Botswana Day Holiday",
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
            (
                NaiveDate::from_ymd_res(2030, 1, 2)?,
                "New Year's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 20)?, "Holy Saturday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2030, 7, 1)?,
                "Sir Seretse Khama Day",
            ),
            (NaiveDate::from_ymd_res(2030, 7, 15)?, "President's Day"),
            (
                NaiveDate::from_ymd_res(2030, 7, 16)?,
                "President's Day Holiday",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 30)?, "Botswana Day"),
            (
                NaiveDate::from_ymd_res(2030, 10, 1)?,
                "Botswana Day Holiday",
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
