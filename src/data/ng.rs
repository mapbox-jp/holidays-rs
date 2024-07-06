//! Nigeria
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Nigeria";
const COUNTY_CODE: Country = Country::NG;

/// Generate holiday map for Nigeria.
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
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2000, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2000, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2001, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2001, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 6)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2002, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2002, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2003, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 12)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Eid-el-Mawlid (estimated); Workers' Day",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2004, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "Eid-el-Kabir Holiday (estimated)",
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
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2005, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2005, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2005, 11, 3)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 22)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2006, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2006, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2006, 10, 23)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 11)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 10)?,
                "Eid-el-Mawlid (estimated)",
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
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Eid-el-Kabir Holiday (estimated); New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2007, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2007, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 21)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2008, 5, 29)?, "Democracy Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Eid-el-Fitr (estimated); Independence Day",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 9)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2009, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2009, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2009, 9, 20)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 28)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2010, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2010, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2010, 9, 10)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 17)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2011, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2011, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2012, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2012, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2012, 8, 19)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 27)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 4)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2013, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2013, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2013, 8, 8)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 16)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2014, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 5)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 13)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2015, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2015, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2015, 7, 17)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 24)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 3)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 23)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2016, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2016, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2016, 7, 6)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 11)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 30)?,
                "Democracy Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 13)?,
                "Eid-el-Kabir (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 3)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 12)?,
                "Eid-el-Mawlid (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Christmas Day (observed)",
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
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2017, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2017, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2017, 6, 25)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 2)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 30)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 27)?,
                "Eid-el-Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 4)?,
                "Eid-el-Kabir Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 2)?,
                "Independence Day (observed)",
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
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2018, 5, 29)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2018, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2018, 6, 15)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 22)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 20)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 18)?,
                "Eid-el-Fitr Holiday (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2019, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2019, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2019, 6, 4)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 5)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 11)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 13)?,
                "Eid-el-Kabir (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 11)?,
                "Eid-el-Mawlid (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 22)?,
                "Public Holiday for Elections",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 29)?,
                "Presidential Inauguration Day",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2020, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2020, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2020, 5, 24)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 31)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 1)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 29)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 26)?,
                "Eid-el-Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 3)?,
                "Eid-el-Kabir Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 28)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2021, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2021, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 3)?,
                "Workers' Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 14)?,
                "Democracy Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 28)?,
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
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2022, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2022, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 9)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 10)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 8)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 4)?,
                "Workers' Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 13)?,
                "Democracy Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 11)?,
                "Eid-el-Kabir (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 12)?,
                "Eid-el-Kabir Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 3)?,
                "Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 10)?,
                "Eid-el-Mawlid (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
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
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2023, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2023, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2023, 4, 21)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 22)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 28)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 27)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Eid-el-Fitr Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 2)?,
                "Independence Day (observed)",
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
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2024, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2024, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2024, 4, 10)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 18)?,
                "Eid-el-Kabir (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 16)?,
                "Eid-el-Mawlid (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2025, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2025, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "Eid-el-Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 9)?,
                "Eid-el-Kabir Holiday (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2026, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2026, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 23)?,
                "Eid-el-Fitr Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 28)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2027, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2027, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 3)?,
                "Workers' Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 18)?,
                "Eid-el-Kabir (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 14)?,
                "Democracy Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 16)?,
                "Eid-el-Mawlid (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 28)?,
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
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2028, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2028, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Eid-el-Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 29)?,
                "Eid-el-Fitr Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 8)?,
                "Eid-el-Kabir Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 2)?,
                "Independence Day (observed)",
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
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2029, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2029, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "Eid-el-Mawlid (estimated)",
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
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2030, 6, 12)?, "Democracy Day"),
            (NaiveDate::from_ymd_res(2030, 10, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Eid-el-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Eid-el-Fitr Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Eid-el-Kabir (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "Eid-el-Kabir Holiday (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "Eid-el-Mawlid (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 15)?,
                "Eid-el-Kabir (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 16)?,
                "Eid-el-Kabir Holiday (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 15)?,
                "Eid-el-Mawlid (estimated) (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
