//! China
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "China";
const COUNTY_CODE: Country = Country::CN;

/// Generate holiday map for China.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2000, 2, 5)?, "春节"),
            (NaiveDate::from_ymd_res(2000, 2, 6)?, "春节"),
            (NaiveDate::from_ymd_res(2000, 2, 7)?, "春节"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2000, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2000, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2000, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2000, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2000, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2000, 1, 3)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2000, 2, 8)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2000, 2, 9)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2000, 10, 4)?, "国庆节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2001, 1, 24)?, "春节"),
            (NaiveDate::from_ymd_res(2001, 1, 25)?, "春节"),
            (NaiveDate::from_ymd_res(2001, 1, 26)?, "春节"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2001, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2001, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2001, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2001, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2001, 10, 3)?, "国庆节"),
            (
                NaiveDate::from_ymd_res(2001, 1, 29)?,
                "休息日（2001-01-20日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 30)?,
                "休息日（2001-01-21日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 4)?,
                "休息日（2001-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 7)?,
                "休息日（2001-04-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 4)?,
                "休息日（2001-09-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 5)?,
                "休息日（2001-09-30日起取代）",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "春节"),
            (NaiveDate::from_ymd_res(2002, 2, 13)?, "春节"),
            (NaiveDate::from_ymd_res(2002, 2, 14)?, "春节"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2002, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2002, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2002, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2002, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2002, 10, 3)?, "国庆节"),
            (
                NaiveDate::from_ymd_res(2002, 1, 2)?,
                "休息日（2001-12-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 1, 3)?,
                "休息日（2001-12-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 15)?,
                "休息日（2002-02-09日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 18)?,
                "休息日（2002-02-10日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 6)?,
                "休息日（2002-04-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 7)?,
                "休息日（2002-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 4)?,
                "休息日（2002-09-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 7)?,
                "休息日（2002-09-29日起取代）",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "春节"),
            (NaiveDate::from_ymd_res(2003, 2, 2)?, "春节"),
            (NaiveDate::from_ymd_res(2003, 2, 3)?, "春节"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2003, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2003, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2003, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2003, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2003, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2003, 2, 4)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2003, 2, 5)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "劳动节（观察日）"),
            (
                NaiveDate::from_ymd_res(2003, 2, 6)?,
                "休息日（2003-02-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 7)?,
                "休息日（2003-02-09日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 6)?,
                "休息日（2003-04-26日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 7)?,
                "休息日（2003-04-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 6)?,
                "休息日（2003-09-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 7)?,
                "休息日（2003-09-28日起取代）",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2004, 1, 22)?, "春节"),
            (NaiveDate::from_ymd_res(2004, 1, 23)?, "春节"),
            (NaiveDate::from_ymd_res(2004, 1, 24)?, "春节"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2004, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2004, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2004, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2004, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2004, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2004, 1, 26)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2004, 5, 4)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2004, 5, 5)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2004, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2004, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2004, 1, 27)?,
                "休息日（2004-01-17日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 28)?,
                "休息日（2004-01-18日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 6)?,
                "休息日（2004-05-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 7)?,
                "休息日（2004-05-09日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 6)?,
                "休息日（2004-10-09日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 7)?,
                "休息日（2004-10-10日起取代）",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "春节"),
            (NaiveDate::from_ymd_res(2005, 2, 10)?, "春节"),
            (NaiveDate::from_ymd_res(2005, 2, 11)?, "春节"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2005, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2005, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2005, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2005, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2005, 1, 3)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2005, 5, 4)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2005, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2005, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2005, 2, 14)?,
                "休息日（2005-02-05日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 15)?,
                "休息日（2005-02-06日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "休息日（2005-04-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 6)?,
                "休息日（2005-05-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 6)?,
                "休息日（2005-10-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 7)?,
                "休息日（2005-10-09日起取代）",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2006, 1, 29)?, "春节"),
            (NaiveDate::from_ymd_res(2006, 1, 30)?, "春节"),
            (NaiveDate::from_ymd_res(2006, 1, 31)?, "春节"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2006, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2006, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2006, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2006, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2006, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2006, 2, 1)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2006, 10, 4)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "休息日（2005-12-31日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 2)?,
                "休息日（2006-01-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 3)?,
                "休息日（2006-02-05日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 4)?,
                "休息日（2006-04-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 5)?,
                "休息日（2006-04-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 5)?,
                "休息日（2006-09-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 6)?,
                "休息日（2006-10-08日起取代）",
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
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2007, 2, 18)?, "春节"),
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "春节"),
            (NaiveDate::from_ymd_res(2007, 2, 20)?, "春节"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2007, 5, 2)?, "劳动节"),
            (NaiveDate::from_ymd_res(2007, 5, 3)?, "劳动节"),
            (NaiveDate::from_ymd_res(2007, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2007, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2007, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2007, 2, 21)?, "春节（观察日）"),
            (
                NaiveDate::from_ymd_res(2007, 1, 2)?,
                "休息日（2006-12-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 3)?,
                "休息日（2006-12-31日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 22)?,
                "休息日（2007-02-17日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 23)?,
                "休息日（2007-02-25日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 4)?,
                "休息日（2007-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 7)?,
                "休息日（2007-04-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 4)?,
                "休息日（2007-09-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 5)?,
                "休息日（2007-09-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 31)?,
                "休息日（2007-12-29日起取代）",
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
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "春节"),
            (NaiveDate::from_ymd_res(2008, 2, 8)?, "春节"),
            (NaiveDate::from_ymd_res(2008, 2, 6)?, "农历除夕"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2008, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2008, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2008, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2008, 6, 8)?, "端午节"),
            (NaiveDate::from_ymd_res(2008, 9, 14)?, "中秋节"),
            (NaiveDate::from_ymd_res(2008, 6, 9)?, "端午节（观察日）"),
            (NaiveDate::from_ymd_res(2008, 9, 15)?, "中秋节（观察日）"),
            (
                NaiveDate::from_ymd_res(2008, 2, 11)?,
                "休息日（2008-02-02日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 12)?,
                "休息日（2008-02-03日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 2)?,
                "休息日（2008-05-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 29)?,
                "休息日（2008-09-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 30)?,
                "休息日（2008-09-28日起取代）",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "春节"),
            (NaiveDate::from_ymd_res(2009, 1, 27)?, "春节"),
            (NaiveDate::from_ymd_res(2009, 1, 25)?, "农历除夕"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2009, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2009, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2009, 10, 3)?, "中秋节; 国庆节"),
            (NaiveDate::from_ymd_res(2009, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2009, 5, 28)?, "端午节"),
            (NaiveDate::from_ymd_res(2009, 1, 28)?, "农历除夕（观察日）"),
            (NaiveDate::from_ymd_res(2009, 4, 6)?, "清明节（观察日）"),
            (NaiveDate::from_ymd_res(2009, 10, 5)?, "中秋节（观察日）"),
            (NaiveDate::from_ymd_res(2009, 10, 6)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2009, 1, 2)?,
                "休息日（2009-01-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 29)?,
                "休息日（2009-01-24日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 30)?,
                "休息日（2009-02-01日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 29)?,
                "休息日（2009-05-31日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 7)?,
                "休息日（2009-09-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 8)?,
                "休息日（2009-10-10日起取代）",
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
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2010, 2, 14)?, "春节"),
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "春节"),
            (NaiveDate::from_ymd_res(2010, 2, 13)?, "农历除夕"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2010, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2010, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2010, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2010, 6, 16)?, "端午节"),
            (NaiveDate::from_ymd_res(2010, 9, 22)?, "中秋节"),
            (NaiveDate::from_ymd_res(2010, 2, 16)?, "农历除夕（观察日）"),
            (NaiveDate::from_ymd_res(2010, 2, 17)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2010, 5, 3)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2010, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2010, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2010, 2, 18)?,
                "休息日（2010-02-20日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 19)?,
                "休息日（2010-02-21日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 14)?,
                "休息日（2010-06-12日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 15)?,
                "休息日（2010-06-13日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 23)?,
                "休息日（2010-09-19日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 24)?,
                "休息日（2010-09-25日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 6)?,
                "休息日（2010-09-26日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 7)?,
                "休息日（2010-10-09日起取代）",
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
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "春节"),
            (NaiveDate::from_ymd_res(2011, 2, 4)?, "春节"),
            (NaiveDate::from_ymd_res(2011, 2, 2)?, "农历除夕"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2011, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2011, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2011, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2011, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "端午节"),
            (NaiveDate::from_ymd_res(2011, 9, 12)?, "中秋节"),
            (NaiveDate::from_ymd_res(2011, 1, 3)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2011, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2011, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2011, 2, 7)?,
                "休息日（2011-01-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 8)?,
                "休息日（2011-02-12日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 4)?,
                "休息日（2011-04-02日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 6)?,
                "休息日（2011-10-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 7)?,
                "休息日（2011-10-09日起取代）",
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
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2012, 1, 23)?, "春节"),
            (NaiveDate::from_ymd_res(2012, 1, 24)?, "春节"),
            (NaiveDate::from_ymd_res(2012, 1, 22)?, "农历除夕"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2012, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2012, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2012, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2012, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "端午节"),
            (NaiveDate::from_ymd_res(2012, 9, 30)?, "中秋节"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2012, 1, 25)?, "农历除夕（观察日）"),
            (NaiveDate::from_ymd_res(2012, 10, 4)?, "中秋节（观察日）"),
            (
                NaiveDate::from_ymd_res(2012, 1, 3)?,
                "休息日（2011-12-31日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 26)?,
                "休息日（2012-01-21日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 27)?,
                "休息日（2012-01-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 2)?,
                "休息日（2012-03-31日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 3)?,
                "休息日（2012-04-01日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 30)?,
                "休息日（2012-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 5)?,
                "休息日（2012-09-29日起取代）",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 22)?, "端午节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2013, 2, 10)?, "春节"),
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "春节"),
            (NaiveDate::from_ymd_res(2013, 2, 9)?, "农历除夕"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2013, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2013, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2013, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2013, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2013, 6, 12)?, "端午节"),
            (NaiveDate::from_ymd_res(2013, 9, 19)?, "中秋节"),
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "农历除夕（观察日）"),
            (NaiveDate::from_ymd_res(2013, 2, 13)?, "春节（观察日）"),
            (
                NaiveDate::from_ymd_res(2013, 1, 2)?,
                "休息日（2013-01-05日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 3)?,
                "休息日（2013-01-06日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 14)?,
                "休息日（2013-02-16日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 15)?,
                "休息日（2013-02-17日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 5)?,
                "休息日（2013-04-07日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 29)?,
                "休息日（2013-04-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 30)?,
                "休息日（2013-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 10)?,
                "休息日（2013-06-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 11)?,
                "休息日（2013-06-09日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 20)?,
                "休息日（2013-09-22日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 4)?,
                "休息日（2013-09-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 7)?,
                "休息日（2013-10-12日起取代）",
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
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2014, 1, 31)?, "春节"),
            (NaiveDate::from_ymd_res(2014, 2, 1)?, "春节"),
            (NaiveDate::from_ymd_res(2014, 2, 2)?, "春节"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2014, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2014, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2014, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2014, 6, 2)?, "端午节"),
            (NaiveDate::from_ymd_res(2014, 9, 8)?, "中秋节"),
            (NaiveDate::from_ymd_res(2014, 2, 3)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2014, 2, 4)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2014, 4, 7)?, "清明节（观察日）"),
            (
                NaiveDate::from_ymd_res(2014, 2, 5)?,
                "休息日（2014-01-26日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 6)?,
                "休息日（2014-02-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "休息日（2014-05-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 6)?,
                "休息日（2014-09-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 7)?,
                "休息日（2014-10-11日起取代）",
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "春节"),
            (NaiveDate::from_ymd_res(2015, 2, 20)?, "春节"),
            (NaiveDate::from_ymd_res(2015, 2, 21)?, "春节"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2015, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2015, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2015, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2015, 6, 20)?, "端午节"),
            (NaiveDate::from_ymd_res(2015, 9, 27)?, "中秋节"),
            (NaiveDate::from_ymd_res(2015, 2, 23)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "清明节（观察日）"),
            (NaiveDate::from_ymd_res(2015, 6, 22)?, "端午节（观察日）"),
            (NaiveDate::from_ymd_res(2015, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "休息日（2015-01-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 18)?,
                "休息日（2015-02-15日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 24)?,
                "休息日（2015-02-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 3)?,
                "中国人民抗日战争暨世界反法西斯战争胜利70周年纪念日",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 4)?,
                "休息日（2015-09-06日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 7)?,
                "休息日（2015-10-10日起取代）",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 6)?, "中秋节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "春节"),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "春节"),
            (NaiveDate::from_ymd_res(2016, 2, 10)?, "春节"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2016, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2016, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2016, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2016, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2016, 6, 9)?, "端午节"),
            (NaiveDate::from_ymd_res(2016, 9, 15)?, "中秋节"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2016, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2016, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2016, 2, 11)?,
                "休息日（2016-02-06日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 12)?,
                "休息日（2016-02-14日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 10)?,
                "休息日（2016-06-12日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 16)?,
                "休息日（2016-09-18日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 6)?,
                "休息日（2016-10-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 7)?,
                "休息日（2016-10-09日起取代）",
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
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "春节"),
            (NaiveDate::from_ymd_res(2017, 1, 29)?, "春节"),
            (NaiveDate::from_ymd_res(2017, 1, 30)?, "春节"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2017, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2017, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2017, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2017, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2017, 5, 30)?, "端午节"),
            (NaiveDate::from_ymd_res(2017, 10, 4)?, "中秋节"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2017, 1, 31)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2017, 2, 1)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2017, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2017, 1, 27)?,
                "休息日（2017-01-22日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2017, 2, 2)?,
                "休息日（2017-02-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 3)?,
                "休息日（2017-04-01日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 29)?,
                "休息日（2017-05-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 6)?,
                "休息日（2017-09-30日起取代）",
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
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2018, 2, 16)?, "春节"),
            (NaiveDate::from_ymd_res(2018, 2, 17)?, "春节"),
            (NaiveDate::from_ymd_res(2018, 2, 18)?, "春节"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2018, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2018, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2018, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2018, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2018, 6, 18)?, "端午节"),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "中秋节"),
            (NaiveDate::from_ymd_res(2018, 2, 19)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2018, 2, 20)?, "春节（观察日）"),
            (
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "休息日（2018-02-11日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 21)?,
                "休息日（2018-02-24日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 6)?,
                "休息日（2018-04-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "休息日（2018-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 4)?,
                "休息日（2018-09-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 5)?,
                "休息日（2018-09-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "休息日（2018-12-29日起取代）",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "春节"),
            (NaiveDate::from_ymd_res(2019, 2, 6)?, "春节"),
            (NaiveDate::from_ymd_res(2019, 2, 7)?, "春节"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2019, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2019, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2019, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2019, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2019, 6, 7)?, "端午节"),
            (NaiveDate::from_ymd_res(2019, 9, 13)?, "中秋节"),
            (
                NaiveDate::from_ymd_res(2019, 2, 4)?,
                "休息日（2019-02-02日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 8)?,
                "休息日（2019-02-03日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 4)?,
                "休息日（2019-09-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 7)?,
                "休息日（2019-10-12日起取代）",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "春节"),
            (NaiveDate::from_ymd_res(2020, 1, 26)?, "春节"),
            (NaiveDate::from_ymd_res(2020, 1, 27)?, "春节"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2020, 10, 1)?, "中秋节; 国庆节"),
            (NaiveDate::from_ymd_res(2020, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2020, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2020, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2020, 6, 25)?, "端午节"),
            (NaiveDate::from_ymd_res(2020, 1, 28)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2020, 1, 29)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2020, 4, 6)?, "清明节（观察日）"),
            (NaiveDate::from_ymd_res(2020, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2020, 1, 24)?,
                "休息日（2020-01-19日起取代）",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 31)?, "春节延长假期"),
            (NaiveDate::from_ymd_res(2020, 2, 1)?, "春节延长假期"),
            (NaiveDate::from_ymd_res(2020, 2, 2)?, "春节延长假期"),
            (
                NaiveDate::from_ymd_res(2020, 5, 4)?,
                "休息日（2020-04-26日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 5)?,
                "休息日（2020-05-09日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 26)?,
                "休息日（2020-06-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 7)?,
                "休息日（2020-09-27日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 8)?,
                "休息日（2020-10-10日起取代）",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 30)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2020, 10, 6)?, "中秋节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "春节"),
            (NaiveDate::from_ymd_res(2021, 2, 13)?, "春节"),
            (NaiveDate::from_ymd_res(2021, 2, 14)?, "春节"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2021, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2021, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2021, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2021, 6, 14)?, "端午节"),
            (NaiveDate::from_ymd_res(2021, 9, 21)?, "中秋节"),
            (NaiveDate::from_ymd_res(2021, 2, 15)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "清明节（观察日）"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2021, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2021, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2021, 2, 11)?,
                "休息日（2021-02-07日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 17)?,
                "休息日（2021-02-20日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 4)?,
                "休息日（2021-04-25日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 5)?,
                "休息日（2021-05-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 20)?,
                "休息日（2021-09-18日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 6)?,
                "休息日（2021-09-26日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 7)?,
                "休息日（2021-10-09日起取代）",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "春节"),
            (NaiveDate::from_ymd_res(2022, 2, 2)?, "春节"),
            (NaiveDate::from_ymd_res(2022, 2, 3)?, "春节"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2022, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2022, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2022, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2022, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2022, 6, 3)?, "端午节"),
            (NaiveDate::from_ymd_res(2022, 9, 10)?, "中秋节"),
            (NaiveDate::from_ymd_res(2022, 1, 3)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2022, 9, 12)?, "中秋节（观察日）"),
            (NaiveDate::from_ymd_res(2022, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2022, 10, 5)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2022, 1, 31)?,
                "休息日（2022-01-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 4)?,
                "休息日（2022-01-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 4)?,
                "休息日（2022-04-02日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "休息日（2022-04-24日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 4)?,
                "休息日（2022-05-07日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 6)?,
                "休息日（2022-10-08日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 7)?,
                "休息日（2022-10-09日起取代）",
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
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2023, 1, 22)?, "春节"),
            (NaiveDate::from_ymd_res(2023, 1, 23)?, "春节"),
            (NaiveDate::from_ymd_res(2023, 1, 24)?, "春节"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2023, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2023, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2023, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2023, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2023, 6, 22)?, "端午节"),
            (NaiveDate::from_ymd_res(2023, 9, 29)?, "中秋节"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2023, 1, 25)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2023, 10, 4)?, "国庆节（观察日）"),
            (
                NaiveDate::from_ymd_res(2023, 1, 26)?,
                "休息日（2023-01-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 27)?,
                "休息日（2023-01-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 2)?,
                "休息日（2023-04-23日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 3)?,
                "休息日（2023-05-06日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 23)?,
                "休息日（2023-06-25日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 5)?,
                "休息日（2023-10-07日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 6)?,
                "休息日（2023-10-08日起取代）",
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
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2024, 2, 10)?, "春节"),
            (NaiveDate::from_ymd_res(2024, 2, 11)?, "春节"),
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "春节"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2024, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2024, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2024, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2024, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2024, 6, 10)?, "端午节"),
            (NaiveDate::from_ymd_res(2024, 9, 17)?, "中秋节"),
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2024, 2, 14)?, "春节（观察日）"),
            (
                NaiveDate::from_ymd_res(2024, 2, 15)?,
                "休息日（2024-02-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 16)?,
                "休息日（2024-02-18日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 5)?,
                "休息日（2024-04-07日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 2)?,
                "休息日（2024-04-28日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 3)?,
                "休息日（2024-05-11日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 16)?,
                "休息日（2024-09-14日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 4)?,
                "休息日（2024-09-29日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 7)?,
                "休息日（2024-10-12日起取代）",
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
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2025, 1, 29)?, "春节"),
            (NaiveDate::from_ymd_res(2025, 1, 30)?, "春节"),
            (NaiveDate::from_ymd_res(2025, 1, 31)?, "春节"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2025, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2025, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2025, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2025, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2025, 5, 31)?, "端午节"),
            (NaiveDate::from_ymd_res(2025, 10, 6)?, "中秋节"),
            (NaiveDate::from_ymd_res(2025, 6, 2)?, "端午节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "春节"),
            (NaiveDate::from_ymd_res(2026, 2, 18)?, "春节"),
            (NaiveDate::from_ymd_res(2026, 2, 19)?, "春节"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2026, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2026, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2026, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2026, 6, 19)?, "端午节"),
            (NaiveDate::from_ymd_res(2026, 9, 25)?, "中秋节"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "清明节（观察日）"),
            (NaiveDate::from_ymd_res(2026, 10, 5)?, "国庆节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2027, 2, 6)?, "春节"),
            (NaiveDate::from_ymd_res(2027, 2, 7)?, "春节"),
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "春节"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2027, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2027, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2027, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2027, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2027, 6, 9)?, "端午节"),
            (NaiveDate::from_ymd_res(2027, 9, 15)?, "中秋节"),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2027, 2, 10)?, "春节（观察日）"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "劳动节（观察日）"),
            (NaiveDate::from_ymd_res(2027, 10, 4)?, "国庆节（观察日）"),
            (NaiveDate::from_ymd_res(2027, 10, 5)?, "国庆节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2028, 1, 26)?, "春节"),
            (NaiveDate::from_ymd_res(2028, 1, 27)?, "春节"),
            (NaiveDate::from_ymd_res(2028, 1, 28)?, "春节"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2028, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2028, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2028, 10, 3)?, "中秋节; 国庆节"),
            (NaiveDate::from_ymd_res(2028, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2028, 5, 28)?, "端午节"),
            (NaiveDate::from_ymd_res(2028, 1, 3)?, "元旦（观察日）"),
            (NaiveDate::from_ymd_res(2028, 5, 29)?, "端午节（观察日）"),
            (NaiveDate::from_ymd_res(2028, 10, 4)?, "国庆节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "春节"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "春节"),
            (NaiveDate::from_ymd_res(2029, 2, 15)?, "春节"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2029, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2029, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2029, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2029, 4, 4)?, "清明节"),
            (NaiveDate::from_ymd_res(2029, 6, 16)?, "端午节"),
            (NaiveDate::from_ymd_res(2029, 9, 22)?, "中秋节"),
            (NaiveDate::from_ymd_res(2029, 6, 18)?, "端午节（观察日）"),
            (NaiveDate::from_ymd_res(2029, 9, 24)?, "中秋节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "元旦"),
            (NaiveDate::from_ymd_res(2030, 2, 3)?, "春节"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "春节"),
            (NaiveDate::from_ymd_res(2030, 2, 5)?, "春节"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "劳动节"),
            (NaiveDate::from_ymd_res(2030, 10, 1)?, "国庆节"),
            (NaiveDate::from_ymd_res(2030, 10, 2)?, "国庆节"),
            (NaiveDate::from_ymd_res(2030, 10, 3)?, "国庆节"),
            (NaiveDate::from_ymd_res(2030, 4, 5)?, "清明节"),
            (NaiveDate::from_ymd_res(2030, 6, 5)?, "端午节"),
            (NaiveDate::from_ymd_res(2030, 9, 12)?, "中秋节"),
            (NaiveDate::from_ymd_res(2030, 2, 6)?, "春节（观察日）"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
