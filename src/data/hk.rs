//! Hong Kong
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Hong Kong";
const COUNTY_CODE: Country = Country::HK;

/// Generate holiday map for Hong Kong.
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
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 4)?,
                "The day preceding Lunar New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2000, 2, 5)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2000, 2, 7)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 6, 6)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2000, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 13)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 6)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2000, 10, 2)?,
                "National Day (observed)",
            ),
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
            (
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 24)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 25)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 26)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 6, 25)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2001, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "Hong Kong Special Administrative Region Establishment Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 2)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 25)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2002, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 12)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 13)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 14)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2002, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 6, 15)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2002, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 21)?,
                "Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 14)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2003, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 31)?,
                "The day preceding Lunar New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2003, 2, 3)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 6, 4)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2003, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 12)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 4)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2004, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 22)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 23)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 24)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 4)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2004, 4, 5)?,
                "Ching Ming Festival (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2004, 6, 22)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2004, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 29)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 22)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2005, 1, 1)?,
                "The first day of January",
            ),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 11)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 11)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2005, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 19)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 11)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2006, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "The first day of January (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 28)?,
                "The day preceding Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 30)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 31)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 5, 31)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2006, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 7)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 30)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2006, 10, 2)?,
                "National Day (observed)",
            ),
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
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 17)?,
                "The day preceding Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 19)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 20)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 6, 19)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2007, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "Hong Kong Special Administrative Region Establishment Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 9, 26)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 19)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2008, 1, 1)?,
                "The first day of January",
            ),
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2008, 2, 8)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 9)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 6, 8)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2008, 6, 9)?,
                "Tuen Ng Festival (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 15)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 7)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2009, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 26)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 27)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 28)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 5, 28)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2009, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 3)?,
                "Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 26)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2010, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 13)?,
                "The day preceding Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 16)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 6, 16)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2010, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 23)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 16)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2011, 1, 1)?,
                "The first day of January",
            ),
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2011, 2, 4)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 5)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2011, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 13)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 5)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2012, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "The first day of January (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 23)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 24)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 25)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2012, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "Hong Kong Special Administrative Region Establishment Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 1)?,
                "National Day; The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 23)?,
                "Chung Yeung Festival",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 2)?,
                "National Day (observed)",
            ),
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
            (
                NaiveDate::from_ymd_res(2013, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 13)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 11)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 12)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 6, 12)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2013, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 20)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 13)?,
                "Chung Yeung Festival",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 14)?,
                "Chung Yeung Festival (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2014, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 3)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 31)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 1)?,
                "The second day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 6, 2)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2014, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 9)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 2)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 1)?, "National Day"),
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "The first day of January"),
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "Lunar New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 2, 20)?, "The second day of Lunar New Year"),
            (NaiveDate::from_ymd_res(2015, 2, 21)?, "The third day of Lunar New Year"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Ching Ming Festival (observed)"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 6, 20)?, "Tuen Ng Festival"),
            (NaiveDate::from_ymd_res(2015, 7, 1)?, "Hong Kong Special Administrative Region Establishment Day"),
            (NaiveDate::from_ymd_res(2015, 9, 28)?, "The day following the Chinese Mid-Autumn Festival"),
            (NaiveDate::from_ymd_res(2015, 10, 21)?, "Chung Yeung Festival"),
            (NaiveDate::from_ymd_res(2015, 10, 1)?, "National Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 9, 3)?, "The 70th anniversary day of the victory of the Chinese people's war of resistance against Japanese aggression"),
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
                "The first day of January",
            ),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2016, 2, 9)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 10)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 9)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2016, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 16)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 9)?,
                "Chung Yeung Festival",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 10)?,
                "Chung Yeung Festival (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 1)?, "National Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
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
            (
                NaiveDate::from_ymd_res(2017, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "The first day of January (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 31)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 28)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 30)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 5, 30)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2017, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 5)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 28)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2017, 10, 2)?,
                "National Day (observed)",
            ),
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
            (
                NaiveDate::from_ymd_res(2018, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 19)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 16)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 17)?,
                "The second day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 6, 18)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2018, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "Hong Kong Special Administrative Region Establishment Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 25)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 17)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 1)?, "National Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
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
                "The first day of January",
            ),
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2019, 2, 6)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 7)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 5)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 6, 7)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2019, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 14)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 7)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2020, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 28)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 25)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 27)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 4)?, "Ching Ming Festival"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 6, 25)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2020, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 2)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 25)?,
                "Chung Yeung Festival",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 26)?,
                "Chung Yeung Festival (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 1)?, "National Day"),
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
            (
                NaiveDate::from_ymd_res(2021, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 12)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 13)?,
                "The second day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2021, 4, 5)?,
                "Ching Ming Festival (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 6, 14)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2021, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 22)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 14)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 1)?, "National Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
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
                "The first day of January",
            ),
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 2, 2)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 3)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 5)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2022, 5, 8)?,
                "The Birthday of the Buddha",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 9)?,
                "The Birthday of the Buddha (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 3)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2022, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 12)?,
                "The second day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 4)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 1)?, "National Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
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
            (
                NaiveDate::from_ymd_res(2023, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "The first day of January (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 25)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 23)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 24)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 5)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2023, 5, 26)?,
                "The Birthday of the Buddha",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 6, 22)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2023, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 30)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 23)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2023, 10, 2)?,
                "National Day (observed)",
            ),
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
            (
                NaiveDate::from_ymd_res(2024, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 13)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 10)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 12)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 4)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2024, 5, 15)?,
                "The Birthday of the Buddha",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 6, 10)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2024, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 18)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 11)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
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
            (
                NaiveDate::from_ymd_res(2025, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 29)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 30)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 31)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 4)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2025, 5, 4)?,
                "The Birthday of the Buddha",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 5)?,
                "The Birthday of the Buddha (observed)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 5, 31)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2025, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 7)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 29)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
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
            (
                NaiveDate::from_ymd_res(2026, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 17)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 18)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 19)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2026, 4, 7)?,
                "Ching Ming Festival (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 24)?,
                "The Birthday of the Buddha",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 25)?,
                "The Birthday of the Buddha (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 6, 19)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2026, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 26)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 18)?,
                "Chung Yeung Festival",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 19)?,
                "Chung Yeung Festival (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
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
            (
                NaiveDate::from_ymd_res(2027, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 9)?,
                "The fourth day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 6)?, "Lunar New Year's Day"),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 4, 5)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2027, 5, 13)?,
                "The Birthday of the Buddha",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 6, 9)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2027, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 9, 16)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 8)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "The first weekday after Christmas Day (observed)",
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
            (
                NaiveDate::from_ymd_res(2028, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 26)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 27)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 28)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 4, 4)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2028, 5, 2)?,
                "The Birthday of the Buddha",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 5, 28)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2028, 5, 29)?,
                "Tuen Ng Festival (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 4)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 26)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2028, 10, 2)?,
                "National Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
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
            (
                NaiveDate::from_ymd_res(2029, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 13)?,
                "Lunar New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 4, 4)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2029, 5, 20)?,
                "The Birthday of the Buddha",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 21)?,
                "The Birthday of the Buddha (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 6, 16)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2029, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 2)?,
                "Hong Kong Special Administrative Region Establishment Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 24)?,
                "The second day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 16)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
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
            (
                NaiveDate::from_ymd_res(2030, 1, 1)?,
                "The first day of January",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "The fourth day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "The second day of Lunar New Year",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "The third day of Lunar New Year",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (
                NaiveDate::from_ymd_res(2030, 4, 20)?,
                "The day following Good Friday",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 4, 5)?, "Ching Ming Festival"),
            (
                NaiveDate::from_ymd_res(2030, 5, 9)?,
                "The Birthday of the Buddha",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 6, 5)?, "Tuen Ng Festival"),
            (
                NaiveDate::from_ymd_res(2030, 7, 1)?,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 13)?,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 5)?,
                "Chung Yeung Festival",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 1)?, "National Day"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "The first weekday after Christmas Day",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
