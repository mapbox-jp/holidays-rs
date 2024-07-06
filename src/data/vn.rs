//! Vietnam
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Vietnam";
const COUNTY_CODE: Country = Country::VN;

/// Generate holiday map for Vietnam.
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
            (
                NaiveDate::from_ymd_res(2000, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 4)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2000, 2, 5)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2000, 2, 6)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 7)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 8)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 9)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2000, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 4)?,
                "Independence Day (observed)",
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
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 23)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2001, 1, 24)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2001, 1, 25)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 26)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 27)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 28)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2001, 9, 3)?,
                "Independence Day (observed)",
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
                NaiveDate::from_ymd_res(2002, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 11)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2002, 2, 13)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 14)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 15)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 16)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2002, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2003, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 31)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2003, 2, 2)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 3)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 4)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 5)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2004, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 21)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 22)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2004, 1, 23)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 24)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 25)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 26)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2004, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2005, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 8)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 11)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 12)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 13)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2006, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 28)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 29)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2006, 1, 30)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 31)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 1)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 2)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 4)?,
                "Independence Day (observed)",
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
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 17)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2007, 2, 18)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2007, 2, 19)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 20)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 21)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 22)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 26)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2007, 9, 3)?,
                "Independence Day (observed)",
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
                NaiveDate::from_ymd_res(2008, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 6)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2008, 2, 8)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 9)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 10)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 11)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 15)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2009, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 25)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2009, 1, 27)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 28)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 29)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 30)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 5)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2009, 4, 6)?,
                "Hung Kings Commemoration Day (observed)",
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
                NaiveDate::from_ymd_res(2010, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 13)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2010, 2, 14)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 16)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 17)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 18)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 23)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2010, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2011, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 2)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2011, 2, 4)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 5)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 6)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 7)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 12)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2012, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 22)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 23)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2012, 1, 24)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 25)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 26)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 27)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 31)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 2)?,
                "Hung Kings Commemoration Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 3)?,
                "Independence Day (observed)",
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
                NaiveDate::from_ymd_res(2013, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 9)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2013, 2, 10)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2013, 2, 11)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 12)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 13)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 14)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 19)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2014, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 30)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 31)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2014, 2, 1)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 2)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 3)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 4)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 9)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2015, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 18)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2015, 2, 20)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 21)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 22)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 23)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 28)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2016, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 7)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2016, 2, 9)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 10)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 11)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 12)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2016, 4, 16)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2016, 4, 18)?,
                "Hung Kings Commemoration Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2017, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 27)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2017, 1, 29)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 30)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 31)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2017, 2, 1)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 6)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 4)?,
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
            (
                NaiveDate::from_ymd_res(2018, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2018, 2, 16)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2018, 2, 17)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 18)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 19)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 20)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 25)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2018, 9, 3)?,
                "Independence Day (observed)",
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
                NaiveDate::from_ymd_res(2019, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 4)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2019, 2, 6)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 7)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 8)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 9)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 14)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2019, 4, 15)?,
                "Hung Kings Commemoration Day (observed)",
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
                NaiveDate::from_ymd_res(2020, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 24)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2020, 1, 26)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 27)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 28)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 29)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 2)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2021, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 11)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2021, 2, 13)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 14)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 16)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 21)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2021, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2022, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 31)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2022, 2, 2)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 3)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 4)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 5)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 10)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 11)?,
                "Hung Kings Commemoration Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2023, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 21)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 22)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2023, 1, 23)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 24)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 25)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 26)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 29)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 2)?,
                "Hung Kings Commemoration Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 3)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 4)?,
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
            (
                NaiveDate::from_ymd_res(2024, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 9)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2024, 2, 10)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2024, 2, 11)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 12)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 13)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 14)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 18)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2025, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 28)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2025, 1, 29)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2025, 1, 30)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 31)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 1)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 2)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 7)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 2)?, "Independence Day"),
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
                NaiveDate::from_ymd_res(2026, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2026, 2, 18)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 19)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 20)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 21)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 26)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2026, 4, 27)?,
                "Hung Kings Commemoration Day (observed)",
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
                NaiveDate::from_ymd_res(2027, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 5)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 6)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2027, 2, 7)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 9)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 10)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2027, 4, 16)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2027, 5, 3)?,
                "International Labor Day (observed)",
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
                NaiveDate::from_ymd_res(2028, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 25)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 26)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2028, 1, 27)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 28)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 29)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 30)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 4)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "International New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 2)?,
                "Liberation Day/Reunification Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 4)?,
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
            (
                NaiveDate::from_ymd_res(2029, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 12)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 17)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 23)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 2)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2029, 9, 3)?,
                "Independence Day (observed)",
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
                NaiveDate::from_ymd_res(2030, 1, 1)?,
                "International New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 2)?,
                "Vietnamese New Year's Eve",
            ),
            (NaiveDate::from_ymd_res(2030, 2, 3)?, "Vietnamese New Year"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "The second day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "The third day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "The forth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 7)?,
                "The fifth day of Tet Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 12)?,
                "Hung Kings Commemoration Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 30)?,
                "Liberation Day/Reunification Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "International Labor Day",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 2)?, "Independence Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
