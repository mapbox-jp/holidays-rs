//! Estonia
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Estonia";
const COUNTY_CODE: Country = Country::EE;

/// Generate holiday map for Estonia.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2000, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2000, 4, 23)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2000, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2000, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2000, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2001, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2001, 4, 15)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2001, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2001, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2001, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2002, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2002, 3, 31)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2002, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2002, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2002, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2003, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2003, 4, 20)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2003, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2003, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2003, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2004, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2004, 4, 11)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2004, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2004, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2004, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2005, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2005, 3, 27)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2005, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2005, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2005, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2006, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2006, 4, 16)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2006, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2006, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2006, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2007, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2007, 4, 8)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2007, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2007, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2007, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2008, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2008, 3, 23)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2008, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2008, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2008, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2009, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2009, 4, 12)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2009, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2009, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2009, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2010, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2010, 4, 4)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2010, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2010, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2010, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2011, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2011, 4, 24)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2011, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2011, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2012, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2012, 4, 8)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2012, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2013, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2013, 3, 31)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2013, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2013, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2013, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2014, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2014, 4, 20)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2014, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2014, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2014, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2015, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2015, 4, 5)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2015, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2015, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2015, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2016, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2016, 3, 27)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2016, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2016, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2016, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2017, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2017, 4, 16)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2017, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2017, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2017, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2018, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2018, 4, 1)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2018, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2018, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2018, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2019, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2019, 4, 21)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2019, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2019, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2019, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2020, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2020, 4, 12)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2020, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2020, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2020, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2021, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2021, 4, 4)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2021, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2021, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2021, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2022, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2022, 4, 17)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2022, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2022, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2022, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2023, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2023, 4, 9)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2023, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2023, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2023, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2024, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2024, 3, 31)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2024, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2024, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2024, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2025, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2025, 4, 20)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2025, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2025, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2025, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2026, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2026, 4, 5)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2026, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2026, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2026, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2027, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2027, 3, 28)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2027, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2027, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2027, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2028, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2028, 4, 16)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2028, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2028, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2028, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2029, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2029, 4, 1)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2029, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2029, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2029, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "uusaasta"),
            (NaiveDate::from_ymd_res(2030, 2, 24)?, "iseseisvuspäev"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "suur reede"),
            (
                NaiveDate::from_ymd_res(2030, 4, 21)?,
                "ülestõusmispühade 1. püha",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "kevadpüha"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "nelipühade 1. püha"),
            (NaiveDate::from_ymd_res(2030, 6, 23)?, "võidupüha"),
            (NaiveDate::from_ymd_res(2030, 6, 24)?, "jaanipäev"),
            (
                NaiveDate::from_ymd_res(2030, 8, 20)?,
                "taasiseseisvumispäev",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "jõululaupäev"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "esimene jõulupüha"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "teine jõulupüha"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
