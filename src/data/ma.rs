//! Morocco
use super::*;

/// Generate holiday map for Morocco.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2000, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2000, 3, 3)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2000, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2000, 7, 9)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2000, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2000, 1, 8)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2000, 12, 27)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2000, 1, 9)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2000, 12, 28)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2000, 3, 16)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2000, 3, 17)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2000, 4, 6)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 15)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2001, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2001, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2001, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2001, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2001, 12, 16)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2001, 12, 17)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2001, 3, 5)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2001, 3, 6)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2001, 3, 26)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 5)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2002, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2002, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2002, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2002, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2002, 12, 5)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2002, 12, 6)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2002, 2, 22)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2002, 2, 23)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2002, 3, 15)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 25)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2003, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2003, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2003, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2003, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2003, 11, 25)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2003, 11, 26)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2003, 2, 11)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2003, 3, 4)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 14)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2004, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "(تقدير) عيد المولد النبوي; عيد العمال",
            ),
            (NaiveDate::from_ymd_res(2004, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2004, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2004, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2004, 11, 15)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2004, 2, 1)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2004, 2, 2)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2004, 2, 21)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 2)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2005, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2005, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2005, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2005, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2005, 11, 4)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2005, 1, 22)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 22)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2006, 1, 11)?,
                "(تقدير) عيد الأضحى; ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2006, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2006, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2006, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2006, 1, 31)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 10)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 11)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "(تقدير) عيد الأضحى; رأس السنة الميلادية",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2007, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2007, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2007, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2007, 10, 14)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2007, 12, 21)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2007, 1, 20)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 1)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2008, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2008, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2008, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2008, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2008, 10, 2)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2008, 12, 9)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2008, 1, 10)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 29)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 21)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2009, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2009, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2009, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2009, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2009, 11, 28)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2009, 12, 18)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 10)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2010, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2010, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2010, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2010, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2010, 9, 11)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2010, 11, 16)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2010, 12, 7)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 27)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2011, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2011, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2011, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "(تقدير) عيد الأضحى; ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2011, 11, 7)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2011, 11, 26)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 16)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2012, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2012, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2012, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "(تقدير) عيد الفطر; ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2012, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2012, 10, 27)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2012, 11, 15)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 5)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2013, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2013, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2013, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2013, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2013, 8, 9)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2013, 10, 16)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2013, 11, 4)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 25)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2014, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2014, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2014, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2014, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2014, 7, 29)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2014, 10, 5)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2014, 10, 25)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 13)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 14)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2015, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2015, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2015, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2015, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2015, 7, 18)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2015, 9, 23)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "(تقدير) عيد الأضحى"),
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
            (
                NaiveDate::from_ymd_res(2015, 1, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 24)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2016, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2016, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2016, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2016, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2016, 7, 7)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2016, 9, 11)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2016, 10, 2)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 11)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 12)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2017, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2017, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2017, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2017, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2017, 6, 26)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2017, 9, 21)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 30)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 1)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2018, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2018, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2018, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "(تقدير) عيد الأضحى; عيد الشباب",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2018, 6, 16)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2018, 9, 11)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 20)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 21)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2019, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2019, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2019, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2019, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2019, 8, 12)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2019, 8, 31)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 10)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2020, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2020, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2020, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 20)?,
                "(تقدير) رأس السنة الهجرية; ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2020, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2020, 10, 29)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 30)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2021, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2021, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2021, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2021, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2021, 5, 14)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2021, 7, 21)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2021, 8, 9)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 19)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2022, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "عيد العمال"),
            (
                NaiveDate::from_ymd_res(2022, 7, 30)?,
                "(تقدير) رأس السنة الهجرية; عيد العرش",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2022, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2022, 10, 8)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 9)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2023, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2023, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2023, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2023, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2023, 7, 19)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 27)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 28)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2024, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2024, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2024, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2024, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2024, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2024, 4, 11)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2024, 6, 16)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2024, 6, 17)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2024, 7, 7)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 16)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2025, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2025, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2025, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2025, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2025, 3, 30)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2025, 3, 31)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2025, 6, 7)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 5)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2026, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2026, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2026, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2026, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2026, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2026, 3, 20)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2026, 3, 21)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2026, 5, 27)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2026, 5, 28)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2026, 6, 16)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 26)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2027, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2027, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "(تقدير) عيد المولد النبوي; ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2027, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2027, 3, 9)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2027, 3, 10)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2027, 6, 6)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2028, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2028, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2028, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2028, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2028, 2, 26)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2028, 2, 27)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2028, 5, 5)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2028, 5, 6)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2029, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2029, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2029, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2029, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2029, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2029, 2, 15)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2029, 4, 24)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2029, 4, 25)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 25)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2030, 1, 11)?,
                "ذكرى تقديم وثيقة الاستقلال",
            ),
            (
                NaiveDate::from_ymd_res(2030, 1, 13)?,
                "رأس السنة الأمازيغية",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "عيد العمال"),
            (NaiveDate::from_ymd_res(2030, 7, 30)?, "عيد العرش"),
            (
                NaiveDate::from_ymd_res(2030, 8, 14)?,
                "ذكرى استرجاع إقليم وادي الذهب",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 20)?,
                "ذكرى ثورة الملك و الشعب",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 21)?, "عيد الشباب"),
            (
                NaiveDate::from_ymd_res(2030, 11, 6)?,
                "ذكرى المسيرة الخضراء",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 18)?, "عيد الإستقلال"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2030, 2, 5)?, "(تقدير) عيد الفطر"),
            (NaiveDate::from_ymd_res(2030, 4, 13)?, "(تقدير) عيد الأضحى"),
            (NaiveDate::from_ymd_res(2030, 4, 14)?, "(تقدير) عيد الأضحى"),
            (
                NaiveDate::from_ymd_res(2030, 5, 3)?,
                "(تقدير) رأس السنة الهجرية",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "(تقدير) عيد المولد النبوي",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 14)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::MA,
        "Morocco",
    );

    Ok(map)
}
