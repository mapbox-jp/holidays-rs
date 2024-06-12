//! Saudi Arabia
use super::*;

/// Generate holiday map for Saudi Arabia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
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
            (
                NaiveDate::from_ymd_res(2000, 1, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 30)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 31)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 15)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 19)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 20)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 19)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 4)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "(ملاحظة) عطلة عيد الفطر",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 9)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 10)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 21)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 24)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 25)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 26)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 29)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 30)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 10)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 13)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 15)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 31)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 3)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 11, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 7)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 8)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 24)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 25)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2005, 9, 24)?,
                "(ملاحظة) اليوم الوطني",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 10, 23)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 28)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 9)?, "(تقدير) يوم عرفة"),
            (NaiveDate::from_ymd_res(2006, 12, 30)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 14)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 19)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
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
                NaiveDate::from_ymd_res(2007, 12, 23)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 24)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 5)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 6)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 7)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 9)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 10)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 9, 20)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 23)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 26)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 29)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 30)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 1)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 9, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 12)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 13)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 14)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 15)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 20)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2010, 9, 22)?,
                "(ملاحظة) اليوم الوطني",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 2)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 3)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 4)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 5)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 8)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2011, 9, 24)?,
                "(ملاحظة) اليوم الوطني",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 8, 19)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 25)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 27)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 29)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 30)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 8, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 12)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 13)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 16)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 30)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 5)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 7)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 8)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 7, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 19)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 20)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 21)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 22)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 22)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 24)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 25)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 27)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 7, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 8)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 9)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 10)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 11)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 10)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 13)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 14)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2016, 9, 22)?,
                "(ملاحظة) اليوم الوطني",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 6, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 31)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 3)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 4)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 5)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2017, 9, 24)?,
                "(ملاحظة) اليوم الوطني",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 6, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 18)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 19)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 20)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 20)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 23)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 6, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 9)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 10)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2019, 8, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 13)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 14)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 5, 24)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 30)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2020, 7, 31)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 1)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 2)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 3)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 4)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 17)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 18)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 19)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 22)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 23)?, "اليوم الوطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 8)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2022, 7, 9)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 10)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 11)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 12)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 13)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2022, 9, 22)?,
                "(ملاحظة) اليوم الوطني",
            ),
            (NaiveDate::from_ymd_res(2022, 2, 22)?, "يوم التأسيسي"),
            (NaiveDate::from_ymd_res(2022, 11, 23)?, "يوم وطني"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 4, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 23)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 25)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 26)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 27)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2023, 6, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 30)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 2)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2023, 9, 24)?,
                "(ملاحظة) اليوم الوطني",
            ),
            (NaiveDate::from_ymd_res(2023, 2, 22)?, "يوم التأسيسي"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 4, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 12)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 13)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 14)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 15)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 15)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 19)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 23)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2024, 2, 22)?, "يوم التأسيسي"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 2)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 5)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 8)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 9)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 10)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 23)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2025, 2, 22)?, "يوم التأسيسي"),
            (
                NaiveDate::from_ymd_res(2025, 2, 23)?,
                "(ملاحظة) يوم التأسيسي",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 22)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 23)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 24)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 25)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 26)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 29)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 31)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 23)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2026, 2, 22)?, "يوم التأسيسي"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 11)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 12)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 14)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 15)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 18)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 19)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 23)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2027, 2, 22)?, "يوم التأسيسي"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 29)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 1)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 4)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 7)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 8)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 23)?, "اليوم الوطني"),
            (
                NaiveDate::from_ymd_res(2028, 9, 24)?,
                "(ملاحظة) اليوم الوطني",
            ),
            (NaiveDate::from_ymd_res(2028, 2, 22)?, "يوم التأسيسي"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 17)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 18)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 19)?,
                "(تقدير ملاحظة) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 23)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 26)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 23)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2029, 2, 22)?, "يوم التأسيسي"),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 7)?,
                "(تقدير) عطلة عيد الفطر",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 12)?, "(تقدير) يوم عرفة"),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 15)?,
                "(تقدير) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 16)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 17)?,
                "(تقدير ملاحظة) عطلة عيد الأضحى",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 23)?, "اليوم الوطني"),
            (NaiveDate::from_ymd_res(2030, 2, 22)?, "يوم التأسيسي"),
            (
                NaiveDate::from_ymd_res(2030, 2, 21)?,
                "(ملاحظة) يوم التأسيسي",
            ),
        ],
        &mut map,
        Country::SA,
        "Saudi Arabia",
    );

    Ok(map)
}
