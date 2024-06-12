//! United Arab Emirates
use super::*;

/// Generate holiday map for United Arab Emirates.
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
            (NaiveDate::from_ymd_res(2000, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2000, 12, 3)?, "اليوم الوطني"),
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
            (NaiveDate::from_ymd_res(2000, 3, 15)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2000, 10, 24)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2001, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2001, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2001, 12, 16)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 4)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2001, 10, 14)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2002, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2002, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2002, 12, 5)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 21)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2002, 10, 4)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2003, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2003, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2003, 11, 25)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 10)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2003, 9, 24)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2004, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2004, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 31)?, "(تقدير) وقفة عرفة"),
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
            (
                NaiveDate::from_ymd_res(2004, 9, 12)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2005, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2005, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "(تقدير) وقفة عرفة"),
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
            (NaiveDate::from_ymd_res(2005, 9, 1)?, "(تقدير) ليلة المعراج"),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2006, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2006, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 9)?, "(تقدير) وقفة عرفة"),
            (NaiveDate::from_ymd_res(2006, 12, 30)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2006, 8, 21)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 10)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "(تقدير) عطلة عيد الأضحى; رأس السنة الميلادية",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2007, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 19)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2007, 8, 10)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2008, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2008, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 7)?, "(تقدير) وقفة عرفة"),
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
            (
                NaiveDate::from_ymd_res(2008, 7, 30)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2009, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2009, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 26)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2009, 7, 20)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2010, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2010, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 12)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 15)?, "(تقدير) وقفة عرفة"),
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
            (NaiveDate::from_ymd_res(2010, 7, 9)?, "(تقدير) ليلة المعراج"),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2011, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2011, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 5)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2011, 6, 29)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2012, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2012, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 25)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2012, 6, 17)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 4)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2013, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2013, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "(تقدير) وقفة عرفة"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "(تقدير) عيد الأضحى"),
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
            (NaiveDate::from_ymd_res(2013, 6, 6)?, "(تقدير) ليلة المعراج"),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2014, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2014, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 30)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2014, 5, 26)?,
                "(تقدير) ليلة المعراج",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 13)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2015, 11, 30)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2015, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2015, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 19)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 22)?, "(تقدير) وقفة عرفة"),
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
                NaiveDate::from_ymd_res(2015, 5, 16)?,
                "(تقدير) ليلة المعراج",
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2016, 11, 30)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2016, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2016, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 10)?, "(تقدير) وقفة عرفة"),
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
            (NaiveDate::from_ymd_res(2016, 5, 4)?, "(تقدير) ليلة المعراج"),
            (
                NaiveDate::from_ymd_res(2016, 12, 11)?,
                "(تقدير) عيد المولد النبوي",
            ),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "رأس السنة الميلادية"),
            (
                NaiveDate::from_ymd_res(2017, 11, 30)?,
                "عيد المولد النبوي; يوم الشهيد",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2017, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2017, 6, 26)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2017, 6, 27)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2017, 8, 31)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2017, 9, 3)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2017, 9, 22)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2017, 4, 23)?, "ليلة المعراج"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2018, 11, 30)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2018, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2018, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2018, 6, 14)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2018, 6, 16)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2018, 8, 20)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2018, 8, 21)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2018, 8, 23)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2018, 9, 11)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2018, 4, 13)?, "ليلة المعراج"),
            (NaiveDate::from_ymd_res(2018, 11, 19)?, "عيد المولد النبوي"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2019, 12, 1)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2019, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2019, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2019, 6, 6)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2019, 6, 3)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2019, 8, 10)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2019, 8, 12)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2019, 8, 13)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2019, 8, 31)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2019, 11, 9)?, "عيد المولد النبوي"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2020, 12, 1)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2020, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2020, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2020, 5, 26)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2020, 5, 23)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2020, 7, 30)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2020, 8, 2)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2020, 8, 23)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2020, 10, 29)?, "عيد المولد النبوي"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2021, 12, 1)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2021, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2021, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2021, 5, 14)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2021, 5, 15)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2021, 5, 12)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2021, 7, 19)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2021, 7, 21)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2021, 7, 22)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2021, 8, 12)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2021, 10, 21)?, "عيد المولد النبوي"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2022, 12, 1)?, "يوم الشهيد"),
            (NaiveDate::from_ymd_res(2022, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2022, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2022, 5, 4)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2022, 7, 8)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2022, 7, 11)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2022, 7, 30)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2022, 10, 8)?, "عيد المولد النبوي"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2023, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2023, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2023, 4, 23)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2023, 4, 20)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2023, 6, 27)?, "وقفة عرفة"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "عيد الأضحى"),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2023, 6, 30)?, "عطلة عيد الأضحى"),
            (NaiveDate::from_ymd_res(2023, 7, 21)?, "رأس السنة الهجرية"),
            (NaiveDate::from_ymd_res(2023, 9, 29)?, "عيد المولد النبوي"),
        ],
        &mut map,
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2024, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2024, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "عيد الفطر"),
            (NaiveDate::from_ymd_res(2024, 4, 11)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2024, 4, 12)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2024, 4, 9)?, "عطلة عيد الفطر"),
            (NaiveDate::from_ymd_res(2024, 6, 15)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2025, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2025, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2025, 3, 30)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 29)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 5)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2026, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2026, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2026, 3, 20)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 19)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 26)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2027, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2027, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2027, 3, 9)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 15)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2028, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2028, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2028, 2, 26)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 4)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2029, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2029, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 13)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 23)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "رأس السنة الميلادية"),
            (NaiveDate::from_ymd_res(2030, 12, 2)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2030, 12, 3)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "(تقدير) عيد الفطر"),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 12)?, "(تقدير) وقفة عرفة"),
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
        Country::AE,
        "United Arab Emirates",
    );

    Ok(map)
}
