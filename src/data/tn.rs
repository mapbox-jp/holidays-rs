//! Tunisia
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Tunisia";
const COUNTY_CODE: Country = Country::TN;

/// Generate holiday map for Tunisia.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2000, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2000, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2000, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2000, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2000, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2000, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2000, 1, 8)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2000, 12, 27)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 29)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 15)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2000, 3, 16)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 6)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2001, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2001, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2001, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2001, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2001, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2001, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2001, 12, 16)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 4)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2001, 3, 5)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2001, 3, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 26)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2002, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2002, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2002, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2002, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2002, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2002, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2002, 12, 5)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 21)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2002, 2, 22)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 24)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 15)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2003, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2003, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2003, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2003, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2003, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2003, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2003, 11, 25)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 10)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2003, 2, 11)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2003, 2, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 13)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 4)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2004, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2004, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "عيد الشهداء"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "(تقدير) عيد المولد النبوي; عيد العمال",
            ),
            (NaiveDate::from_ymd_res(2004, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2004, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2004, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 31)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2004, 2, 1)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 3)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 21)?,
                "(تقدير) رأس السنة الهجرية",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2005, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2005, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2005, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2005, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2005, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2005, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2005, 1, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2006, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2006, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2006, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2006, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2006, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2006, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 9)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2006, 12, 30)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2006, 1, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 31)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 10)?,
                "(تقدير) عيد المولد النبوي",
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
                "(تقدير) عطلة عيد الأضحى; رأس السنة الميلادية",
            ),
            (NaiveDate::from_ymd_res(2007, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2007, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2007, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2007, 8, 13)?, "عيد المرأة"),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "(تقدير) عطلة عيد الفطر; عيد الجلاء",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 19)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2007, 12, 21)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 20)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2008, 1, 14)?, "عيد الثورة والشباب"),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "(تقدير) عيد المولد النبوي; عيد الإستقلال",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2008, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2008, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2008, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 7)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2008, 12, 9)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 10)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 10)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 29)?,
                "(تقدير) رأس السنة الهجرية",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2009, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2009, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2009, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2009, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2009, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 26)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2009, 11, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 29)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 18)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2010, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2010, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2010, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2010, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2010, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2010, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 12)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 15)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2010, 11, 16)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2010, 11, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 7)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2011, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2011, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2011, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2011, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2011, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2011, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 5)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2011, 11, 6)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 8)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 26)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2012, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2012, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2012, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2012, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2012, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 25)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2012, 10, 27)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 15)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 4)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2013, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2013, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2013, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2013, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2013, 8, 13)?, "عيد المرأة"),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "(تقدير) عيد الأضحى; عيد الجلاء",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2013, 10, 16)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 4)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2014, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2014, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2014, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2014, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2014, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2014, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 30)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2014, 10, 5)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 25)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 13)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2015, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2015, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2015, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2015, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2015, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2015, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 19)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 22)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2015, 9, 23)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2015, 9, 24)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 25)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 14)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 3)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 23)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2016, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2016, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2016, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2016, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2016, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2016, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 10)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2016, 9, 11)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 13)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 2)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 11)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2017, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2017, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2017, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2017, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2017, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2017, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 31)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2017, 9, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 3)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 21)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 30)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2018, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2018, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2018, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2018, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2018, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2018, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 20)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2018, 8, 21)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2018, 8, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 11)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 20)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2019, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2019, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2019, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2019, 7, 25)?, "عيد الجمهورية"),
            (
                NaiveDate::from_ymd_res(2019, 8, 13)?,
                "(تقدير) عطلة عيد الأضحى; عيد المرأة",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2019, 6, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 10)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 31)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2020, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2020, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2020, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2020, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2020, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 30)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2020, 8, 1)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 20)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 29)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2021, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2021, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2021, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2021, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2021, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2021, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 19)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 9)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2022, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2022, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2022, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2022, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2022, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2022, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 8)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2022, 7, 10)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 30)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 8)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2023, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2023, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2023, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2023, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2023, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2023, 4, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 23)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 27)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 30)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 19)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 27)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2024, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2024, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2024, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2024, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2024, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2024, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 12)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 15)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2024, 6, 16)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 7)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2025, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2025, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2025, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2025, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2025, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2025, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2025, 3, 30)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 5)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 8)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2026, 1, 14)?, "عيد الثورة والشباب"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "(تقدير) عيد الفطر; عيد الإستقلال",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2026, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2026, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2026, 10, 15)?, "عيد الجلاء"),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 26)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2026, 5, 27)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 29)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 16)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2027, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2027, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2027, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2027, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2027, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2027, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2027, 3, 9)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 15)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 6)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2028, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2028, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2028, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2028, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2028, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2028, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2028, 2, 26)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 4)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2028, 5, 5)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2029, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2029, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2029, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2029, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2029, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2029, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 23)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2029, 4, 24)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 26)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "(تقدير) عيد المولد النبوي",
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
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2030, 1, 14)?, "عيد الثورة والشباب"),
            (NaiveDate::from_ymd_res(2030, 3, 20)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2030, 4, 9)?, "عيد الشهداء"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2030, 7, 25)?, "عيد الجمهورية"),
            (NaiveDate::from_ymd_res(2030, 8, 13)?, "عيد المرأة"),
            (NaiveDate::from_ymd_res(2030, 10, 15)?, "عيد الجلاء"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 12)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2030, 4, 13)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 15)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 3)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
