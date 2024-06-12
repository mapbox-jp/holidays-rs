//! Taiwan
use super::*;

/// Generate holiday map for Taiwan.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2000, 2, 4)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2000, 2, 5)?, "春節"),
            (NaiveDate::from_ymd_res(2000, 2, 6)?, "春節"),
            (NaiveDate::from_ymd_res(2000, 2, 7)?, "春節"),
            (NaiveDate::from_ymd_res(2000, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2000, 4, 4)?, "清明節"),
            (NaiveDate::from_ymd_res(2000, 6, 6)?, "端午節"),
            (NaiveDate::from_ymd_res(2000, 9, 12)?, "中秋節"),
            (NaiveDate::from_ymd_res(2000, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2001, 1, 23)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2001, 1, 24)?, "春節"),
            (NaiveDate::from_ymd_res(2001, 1, 25)?, "春節"),
            (NaiveDate::from_ymd_res(2001, 1, 26)?, "春節"),
            (NaiveDate::from_ymd_res(2001, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2001, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2001, 6, 25)?, "端午節"),
            (NaiveDate::from_ymd_res(2001, 10, 1)?, "中秋節"),
            (NaiveDate::from_ymd_res(2001, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2002, 2, 11)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "春節"),
            (NaiveDate::from_ymd_res(2002, 2, 13)?, "春節"),
            (NaiveDate::from_ymd_res(2002, 2, 14)?, "春節"),
            (NaiveDate::from_ymd_res(2002, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2002, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2002, 6, 15)?, "端午節"),
            (NaiveDate::from_ymd_res(2002, 9, 21)?, "中秋節"),
            (NaiveDate::from_ymd_res(2002, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2003, 1, 31)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "春節"),
            (NaiveDate::from_ymd_res(2003, 2, 2)?, "春節"),
            (NaiveDate::from_ymd_res(2003, 2, 3)?, "春節"),
            (NaiveDate::from_ymd_res(2003, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2003, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2003, 6, 4)?, "端午節"),
            (NaiveDate::from_ymd_res(2003, 9, 11)?, "中秋節"),
            (NaiveDate::from_ymd_res(2003, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2004, 1, 21)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2004, 1, 22)?, "春節"),
            (NaiveDate::from_ymd_res(2004, 1, 23)?, "春節"),
            (NaiveDate::from_ymd_res(2004, 1, 24)?, "春節"),
            (NaiveDate::from_ymd_res(2004, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2004, 4, 4)?, "清明節"),
            (NaiveDate::from_ymd_res(2004, 6, 22)?, "端午節"),
            (NaiveDate::from_ymd_res(2004, 9, 28)?, "中秋節"),
            (NaiveDate::from_ymd_res(2004, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "春節"),
            (NaiveDate::from_ymd_res(2005, 2, 10)?, "春節"),
            (NaiveDate::from_ymd_res(2005, 2, 11)?, "春節"),
            (NaiveDate::from_ymd_res(2005, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2005, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2005, 6, 11)?, "端午節"),
            (NaiveDate::from_ymd_res(2005, 9, 18)?, "中秋節"),
            (NaiveDate::from_ymd_res(2005, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2006, 1, 28)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2006, 1, 29)?, "春節"),
            (NaiveDate::from_ymd_res(2006, 1, 30)?, "春節"),
            (NaiveDate::from_ymd_res(2006, 1, 31)?, "春節"),
            (NaiveDate::from_ymd_res(2006, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2006, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2006, 5, 31)?, "端午節"),
            (NaiveDate::from_ymd_res(2006, 10, 6)?, "中秋節"),
            (NaiveDate::from_ymd_res(2006, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2007, 2, 17)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2007, 2, 18)?, "春節"),
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "春節"),
            (NaiveDate::from_ymd_res(2007, 2, 20)?, "春節"),
            (NaiveDate::from_ymd_res(2007, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2007, 6, 19)?, "端午節"),
            (NaiveDate::from_ymd_res(2007, 9, 25)?, "中秋節"),
            (NaiveDate::from_ymd_res(2007, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2008, 2, 6)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "春節"),
            (NaiveDate::from_ymd_res(2008, 2, 8)?, "春節"),
            (NaiveDate::from_ymd_res(2008, 2, 9)?, "春節"),
            (NaiveDate::from_ymd_res(2008, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2008, 4, 4)?, "清明節"),
            (NaiveDate::from_ymd_res(2008, 6, 8)?, "端午節"),
            (NaiveDate::from_ymd_res(2008, 9, 14)?, "中秋節"),
            (NaiveDate::from_ymd_res(2008, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2009, 1, 25)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "春節"),
            (NaiveDate::from_ymd_res(2009, 1, 27)?, "春節"),
            (NaiveDate::from_ymd_res(2009, 1, 28)?, "春節"),
            (NaiveDate::from_ymd_res(2009, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2009, 4, 4)?, "清明節"),
            (NaiveDate::from_ymd_res(2009, 5, 28)?, "端午節"),
            (NaiveDate::from_ymd_res(2009, 10, 3)?, "中秋節"),
            (NaiveDate::from_ymd_res(2009, 10, 10)?, "中華民國國慶日"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2010, 2, 13)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2010, 2, 14)?, "春節"),
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "春節"),
            (NaiveDate::from_ymd_res(2010, 2, 16)?, "春節"),
            (NaiveDate::from_ymd_res(2010, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2010, 6, 16)?, "端午節"),
            (NaiveDate::from_ymd_res(2010, 9, 22)?, "中秋節"),
            (NaiveDate::from_ymd_res(2010, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2010, 2, 17)?, "農曆除夕（慶祝）"),
            (NaiveDate::from_ymd_res(2010, 2, 18)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2010, 2, 19)?,
                "休息日（2010-02-06日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2011, 2, 2)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "春節"),
            (NaiveDate::from_ymd_res(2011, 2, 4)?, "春節"),
            (NaiveDate::from_ymd_res(2011, 2, 5)?, "春節"),
            (NaiveDate::from_ymd_res(2011, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2011, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2011, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "端午節"),
            (NaiveDate::from_ymd_res(2011, 9, 12)?, "中秋節"),
            (NaiveDate::from_ymd_res(2011, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2011, 2, 7)?, "春節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2012, 1, 22)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2012, 1, 23)?, "春節"),
            (NaiveDate::from_ymd_res(2012, 1, 24)?, "春節"),
            (NaiveDate::from_ymd_res(2012, 1, 25)?, "春節"),
            (NaiveDate::from_ymd_res(2012, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2012, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "端午節"),
            (NaiveDate::from_ymd_res(2012, 9, 30)?, "中秋節"),
            (NaiveDate::from_ymd_res(2012, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2012, 1, 26)?, "農曆除夕（慶祝）"),
            (
                NaiveDate::from_ymd_res(2012, 1, 27)?,
                "休息日（2012-02-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 27)?,
                "休息日（2012-03-03日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "休息日（2012-12-22日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2013, 2, 9)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2013, 2, 10)?, "春節"),
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "春節"),
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "春節"),
            (NaiveDate::from_ymd_res(2013, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2013, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2013, 6, 12)?, "端午節"),
            (NaiveDate::from_ymd_res(2013, 9, 19)?, "中秋節"),
            (NaiveDate::from_ymd_res(2013, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2013, 2, 13)?, "農曆除夕（慶祝）"),
            (NaiveDate::from_ymd_res(2013, 2, 14)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2013, 2, 15)?,
                "休息日（2013-02-23日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 20)?,
                "休息日（2013-09-14日起取代）",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 5)?, "兒童節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2014, 1, 30)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2014, 1, 31)?, "春節"),
            (NaiveDate::from_ymd_res(2014, 2, 1)?, "春節"),
            (NaiveDate::from_ymd_res(2014, 2, 2)?, "春節"),
            (NaiveDate::from_ymd_res(2014, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2014, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2014, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2014, 6, 2)?, "端午節"),
            (NaiveDate::from_ymd_res(2014, 9, 8)?, "中秋節"),
            (NaiveDate::from_ymd_res(2014, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2014, 2, 3)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2014, 2, 4)?, "春節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2015, 2, 18)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "春節"),
            (NaiveDate::from_ymd_res(2015, 2, 20)?, "春節"),
            (NaiveDate::from_ymd_res(2015, 2, 21)?, "春節"),
            (NaiveDate::from_ymd_res(2015, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2015, 6, 20)?, "端午節"),
            (NaiveDate::from_ymd_res(2015, 9, 27)?, "中秋節"),
            (NaiveDate::from_ymd_res(2015, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2015, 2, 27)?, "和平紀念日（慶祝）"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "兒童節（慶祝）"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "清明節（慶祝）"),
            (NaiveDate::from_ymd_res(2015, 6, 19)?, "端午節（慶祝）"),
            (NaiveDate::from_ymd_res(2015, 9, 28)?, "中秋節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2015, 10, 9)?,
                "中華民國國慶日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2015, 2, 23)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "休息日（2014-12-27日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2016, 2, 7)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "春節"),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "春節"),
            (NaiveDate::from_ymd_res(2016, 2, 10)?, "春節"),
            (NaiveDate::from_ymd_res(2016, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2016, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2016, 6, 9)?, "端午節"),
            (NaiveDate::from_ymd_res(2016, 9, 15)?, "中秋節"),
            (NaiveDate::from_ymd_res(2016, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2016, 2, 29)?, "和平紀念日（慶祝）"),
            (NaiveDate::from_ymd_res(2016, 2, 11)?, "農曆除夕（慶祝）"),
            (
                NaiveDate::from_ymd_res(2016, 2, 12)?,
                "休息日（2016-01-30日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 10)?,
                "休息日（2016-06-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 16)?,
                "休息日（2016-09-10日起取代）",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 5)?, "兒童節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2017, 1, 27)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "春節"),
            (NaiveDate::from_ymd_res(2017, 1, 29)?, "春節"),
            (NaiveDate::from_ymd_res(2017, 1, 30)?, "春節"),
            (NaiveDate::from_ymd_res(2017, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2017, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2017, 5, 30)?, "端午節"),
            (NaiveDate::from_ymd_res(2017, 10, 4)?, "中秋節"),
            (NaiveDate::from_ymd_res(2017, 10, 10)?, "中華民國國慶日"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "中華民國開國紀念日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 31)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2017, 2, 1)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2017, 2, 27)?,
                "休息日（2017-02-18日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 29)?,
                "休息日（2017-06-03日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 9)?,
                "休息日（2017-09-30日起取代）",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 3)?, "兒童節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2018, 2, 15)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2018, 2, 16)?, "春節"),
            (NaiveDate::from_ymd_res(2018, 2, 17)?, "春節"),
            (NaiveDate::from_ymd_res(2018, 2, 18)?, "春節"),
            (NaiveDate::from_ymd_res(2018, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2018, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2018, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2018, 6, 18)?, "端午節"),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "中秋節"),
            (NaiveDate::from_ymd_res(2018, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2018, 2, 19)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2018, 2, 20)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2018, 4, 6)?,
                "休息日（2018-03-31日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "休息日（2018-12-22日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2019, 2, 4)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "春節"),
            (NaiveDate::from_ymd_res(2019, 2, 6)?, "春節"),
            (NaiveDate::from_ymd_res(2019, 2, 7)?, "春節"),
            (NaiveDate::from_ymd_res(2019, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2019, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2019, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2019, 6, 7)?, "端午節"),
            (NaiveDate::from_ymd_res(2019, 9, 13)?, "中秋節"),
            (NaiveDate::from_ymd_res(2019, 10, 10)?, "中華民國國慶日"),
            (
                NaiveDate::from_ymd_res(2019, 2, 8)?,
                "休息日（2019-01-19日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 1)?,
                "休息日（2019-02-23日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 11)?,
                "休息日（2019-10-05日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2020, 1, 24)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "春節"),
            (NaiveDate::from_ymd_res(2020, 1, 26)?, "春節"),
            (NaiveDate::from_ymd_res(2020, 1, 27)?, "春節"),
            (NaiveDate::from_ymd_res(2020, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2020, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2020, 6, 25)?, "端午節"),
            (NaiveDate::from_ymd_res(2020, 10, 1)?, "中秋節"),
            (NaiveDate::from_ymd_res(2020, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2020, 4, 3)?, "兒童節（慶祝）"),
            (NaiveDate::from_ymd_res(2020, 4, 2)?, "清明節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2020, 10, 9)?,
                "中華民國國慶日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 28)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2020, 1, 29)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2020, 1, 23)?,
                "休息日（2020-02-15日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 26)?,
                "休息日（2020-06-20日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 2)?,
                "休息日（2020-09-26日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "中華民國開國紀念日"),
            (
                NaiveDate::from_ymd_res(2021, 12, 31)?,
                "中華民國開國紀念日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2021, 2, 11)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "春節"),
            (NaiveDate::from_ymd_res(2021, 2, 13)?, "春節"),
            (NaiveDate::from_ymd_res(2021, 2, 14)?, "春節"),
            (NaiveDate::from_ymd_res(2021, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2021, 6, 14)?, "端午節"),
            (NaiveDate::from_ymd_res(2021, 9, 21)?, "中秋節"),
            (NaiveDate::from_ymd_res(2021, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2021, 3, 1)?, "和平紀念日（慶祝）"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "兒童節（慶祝）"),
            (NaiveDate::from_ymd_res(2021, 4, 6)?, "清明節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2021, 10, 11)?,
                "中華民國國慶日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2021, 2, 15)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2021, 2, 10)?,
                "休息日（2021-02-20日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 20)?,
                "休息日（2021-09-11日起取代）",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "兒童節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2022, 1, 31)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "春節"),
            (NaiveDate::from_ymd_res(2022, 2, 2)?, "春節"),
            (NaiveDate::from_ymd_res(2022, 2, 3)?, "春節"),
            (NaiveDate::from_ymd_res(2022, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2022, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2022, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2022, 6, 3)?, "端午節"),
            (NaiveDate::from_ymd_res(2022, 9, 10)?, "中秋節"),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2022, 9, 9)?, "中秋節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2022, 2, 4)?,
                "休息日（2022-01-22日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2023, 1, 21)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2023, 1, 22)?, "春節"),
            (NaiveDate::from_ymd_res(2023, 1, 23)?, "春節"),
            (NaiveDate::from_ymd_res(2023, 1, 24)?, "春節"),
            (NaiveDate::from_ymd_res(2023, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2023, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2023, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2023, 6, 22)?, "端午節"),
            (NaiveDate::from_ymd_res(2023, 9, 29)?, "中秋節"),
            (NaiveDate::from_ymd_res(2023, 10, 10)?, "中華民國國慶日"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "中華民國開國紀念日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 25)?, "農曆除夕（慶祝）"),
            (NaiveDate::from_ymd_res(2023, 1, 26)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2023, 1, 20)?,
                "休息日（2023-01-07日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 27)?,
                "休息日（2023-02-04日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 27)?,
                "休息日（2023-02-18日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 3)?,
                "休息日（2023-03-25日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 23)?,
                "休息日（2023-06-17日起取代）",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 9)?,
                "休息日（2023-09-23日起取代）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2024, 2, 9)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2024, 2, 10)?, "春節"),
            (NaiveDate::from_ymd_res(2024, 2, 11)?, "春節"),
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "春節"),
            (NaiveDate::from_ymd_res(2024, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2024, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2024, 6, 10)?, "端午節"),
            (NaiveDate::from_ymd_res(2024, 9, 17)?, "中秋節"),
            (NaiveDate::from_ymd_res(2024, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2024, 2, 14)?, "春節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2024, 2, 8)?,
                "休息日（2024-02-17日起取代）",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 5)?, "兒童節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2025, 1, 28)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2025, 1, 29)?, "春節"),
            (NaiveDate::from_ymd_res(2025, 1, 30)?, "春節"),
            (NaiveDate::from_ymd_res(2025, 1, 31)?, "春節"),
            (NaiveDate::from_ymd_res(2025, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2025, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2025, 5, 31)?, "端午節"),
            (NaiveDate::from_ymd_res(2025, 10, 6)?, "中秋節"),
            (NaiveDate::from_ymd_res(2025, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2025, 5, 30)?, "端午節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2026, 2, 16)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "春節"),
            (NaiveDate::from_ymd_res(2026, 2, 18)?, "春節"),
            (NaiveDate::from_ymd_res(2026, 2, 19)?, "春節"),
            (NaiveDate::from_ymd_res(2026, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2026, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2026, 6, 19)?, "端午節"),
            (NaiveDate::from_ymd_res(2026, 9, 25)?, "中秋節"),
            (NaiveDate::from_ymd_res(2026, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2026, 2, 27)?, "和平紀念日（慶祝）"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "兒童節（慶祝）"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "清明節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2026, 10, 9)?,
                "中華民國國慶日（慶祝）",
            ),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "中華民國開國紀念日"),
            (
                NaiveDate::from_ymd_res(2027, 12, 31)?,
                "中華民國開國紀念日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 5)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2027, 2, 6)?, "春節"),
            (NaiveDate::from_ymd_res(2027, 2, 7)?, "春節"),
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "春節"),
            (NaiveDate::from_ymd_res(2027, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2027, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2027, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2027, 6, 9)?, "端午節"),
            (NaiveDate::from_ymd_res(2027, 9, 15)?, "中秋節"),
            (NaiveDate::from_ymd_res(2027, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2027, 3, 1)?, "和平紀念日（慶祝）"),
            (NaiveDate::from_ymd_res(2027, 4, 6)?, "兒童節（慶祝）"),
            (
                NaiveDate::from_ymd_res(2027, 10, 11)?,
                "中華民國國慶日（慶祝）",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "春節（慶祝）"),
            (NaiveDate::from_ymd_res(2027, 2, 10)?, "春節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2028, 1, 25)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2028, 1, 26)?, "春節"),
            (NaiveDate::from_ymd_res(2028, 1, 27)?, "春節"),
            (NaiveDate::from_ymd_res(2028, 1, 28)?, "春節"),
            (NaiveDate::from_ymd_res(2028, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2028, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2028, 5, 28)?, "端午節"),
            (NaiveDate::from_ymd_res(2028, 10, 3)?, "中秋節"),
            (NaiveDate::from_ymd_res(2028, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2028, 5, 29)?, "端午節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "春節"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "春節"),
            (NaiveDate::from_ymd_res(2029, 2, 15)?, "春節"),
            (NaiveDate::from_ymd_res(2029, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2029, 4, 4)?, "兒童節; 清明節"),
            (NaiveDate::from_ymd_res(2029, 6, 16)?, "端午節"),
            (NaiveDate::from_ymd_res(2029, 9, 22)?, "中秋節"),
            (NaiveDate::from_ymd_res(2029, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2029, 6, 15)?, "端午節（慶祝）"),
            (NaiveDate::from_ymd_res(2029, 9, 21)?, "中秋節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "中華民國開國紀念日"),
            (NaiveDate::from_ymd_res(2030, 2, 2)?, "農曆除夕"),
            (NaiveDate::from_ymd_res(2030, 2, 3)?, "春節"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "春節"),
            (NaiveDate::from_ymd_res(2030, 2, 5)?, "春節"),
            (NaiveDate::from_ymd_res(2030, 2, 28)?, "和平紀念日"),
            (NaiveDate::from_ymd_res(2030, 4, 4)?, "兒童節"),
            (NaiveDate::from_ymd_res(2030, 4, 5)?, "清明節"),
            (NaiveDate::from_ymd_res(2030, 6, 5)?, "端午節"),
            (NaiveDate::from_ymd_res(2030, 9, 12)?, "中秋節"),
            (NaiveDate::from_ymd_res(2030, 10, 10)?, "中華民國國慶日"),
            (NaiveDate::from_ymd_res(2030, 2, 6)?, "農曆除夕（慶祝）"),
            (NaiveDate::from_ymd_res(2030, 2, 7)?, "春節（慶祝）"),
        ],
        &mut map,
        Country::TW,
        "Taiwan",
    );

    Ok(map)
}
