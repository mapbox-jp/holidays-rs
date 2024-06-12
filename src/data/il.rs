//! Israel
use super::*;

/// Generate holiday map for Israel.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 9, 30)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2000, 10, 1)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2000, 10, 9)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2000, 10, 14)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2000, 10, 21)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "פסח"),
            (NaiveDate::from_ymd_res(2000, 4, 26)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2000, 5, 10)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2000, 6, 9)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 9, 18)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2001, 9, 19)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2001, 9, 27)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2001, 10, 2)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2001, 10, 9)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 8)?, "פסח"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2001, 4, 26)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 9, 7)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2002, 9, 8)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2002, 9, 16)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2002, 9, 21)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2002, 9, 28)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "פסח"),
            (NaiveDate::from_ymd_res(2002, 4, 3)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2002, 4, 17)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2002, 5, 17)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 9, 27)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2003, 9, 28)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2003, 10, 6)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2003, 10, 11)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2003, 10, 18)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "פסח"),
            (NaiveDate::from_ymd_res(2003, 4, 23)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2003, 5, 7)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2003, 6, 6)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 9, 16)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2004, 9, 17)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2004, 9, 25)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2004, 9, 30)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2004, 10, 7)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 6)?, "פסח"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2004, 4, 27)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2004, 5, 26)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 10, 4)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2005, 10, 5)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2005, 10, 13)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2005, 10, 18)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2005, 10, 25)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 24)?, "פסח"),
            (NaiveDate::from_ymd_res(2005, 4, 30)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2005, 5, 12)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2005, 6, 13)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 9, 23)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2006, 9, 24)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2006, 10, 2)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2006, 10, 7)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2006, 10, 14)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "פסח"),
            (NaiveDate::from_ymd_res(2006, 4, 19)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2006, 5, 3)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2006, 6, 2)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 9, 13)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2007, 9, 14)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2007, 9, 22)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2007, 9, 27)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2007, 10, 4)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 3)?, "פסח"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2007, 4, 24)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2007, 5, 23)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 9, 30)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2008, 10, 9)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2008, 10, 14)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2008, 10, 21)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 20)?, "פסח"),
            (NaiveDate::from_ymd_res(2008, 4, 26)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2008, 5, 8)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2008, 6, 9)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 9, 19)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2009, 9, 28)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2009, 10, 3)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2009, 10, 10)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "פסח"),
            (NaiveDate::from_ymd_res(2009, 4, 15)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2009, 4, 29)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2009, 5, 29)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 9, 9)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2010, 9, 18)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2010, 9, 23)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2010, 9, 30)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 30)?, "פסח"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2010, 4, 20)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2010, 5, 19)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 9, 29)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2011, 9, 30)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2011, 10, 8)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2011, 10, 13)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2011, 10, 20)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 19)?, "פסח"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2011, 5, 10)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2011, 6, 8)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 9, 17)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2012, 9, 18)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2012, 9, 26)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2012, 10, 1)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2012, 10, 8)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 7)?, "פסח"),
            (NaiveDate::from_ymd_res(2012, 4, 13)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2012, 4, 26)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 9, 5)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2013, 9, 6)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2013, 9, 14)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2013, 9, 19)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2013, 9, 26)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 26)?, "פסח"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2013, 4, 16)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2013, 5, 15)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 9, 25)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2014, 9, 26)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2014, 10, 9)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2014, 10, 16)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 15)?, "פסח"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2014, 5, 6)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2014, 6, 4)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 9, 14)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2015, 9, 15)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2015, 9, 23)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2015, 9, 28)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2015, 10, 5)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "פסח"),
            (NaiveDate::from_ymd_res(2015, 4, 10)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2015, 4, 23)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 10, 3)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2016, 10, 4)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2016, 10, 12)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2016, 10, 17)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2016, 10, 24)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 23)?, "פסח"),
            (NaiveDate::from_ymd_res(2016, 4, 29)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2016, 5, 12)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2016, 6, 12)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 9, 21)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2017, 9, 22)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2017, 9, 30)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2017, 10, 5)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2017, 10, 12)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 11)?, "פסח"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2017, 5, 2)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2017, 5, 31)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 9, 10)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2018, 9, 11)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2018, 9, 19)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2018, 10, 1)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "פסח"),
            (NaiveDate::from_ymd_res(2018, 4, 6)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2018, 4, 19)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 9, 30)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2019, 10, 1)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2019, 10, 9)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2019, 10, 14)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2019, 10, 21)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 20)?, "פסח"),
            (NaiveDate::from_ymd_res(2019, 4, 26)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2019, 5, 9)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 9, 19)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2020, 9, 20)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2020, 9, 28)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2020, 10, 3)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2020, 10, 10)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "פסח"),
            (NaiveDate::from_ymd_res(2020, 4, 15)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2020, 4, 29)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2020, 5, 29)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 9, 7)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2021, 9, 8)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2021, 9, 16)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2021, 9, 21)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2021, 9, 28)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 28)?, "פסח"),
            (NaiveDate::from_ymd_res(2021, 4, 3)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2021, 4, 15)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2021, 5, 17)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 9, 26)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2022, 9, 27)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2022, 10, 5)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2022, 10, 17)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 16)?, "פסח"),
            (NaiveDate::from_ymd_res(2022, 4, 22)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2022, 5, 5)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 9, 16)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2023, 9, 17)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2023, 9, 25)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2023, 9, 30)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2023, 10, 7)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "פסח"),
            (NaiveDate::from_ymd_res(2023, 4, 12)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2023, 4, 26)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2023, 5, 26)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 10, 3)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2024, 10, 4)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2024, 10, 12)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2024, 10, 17)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2024, 10, 24)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 23)?, "פסח"),
            (NaiveDate::from_ymd_res(2024, 4, 29)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2024, 5, 14)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2024, 6, 12)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 9, 23)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2025, 9, 24)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2025, 10, 2)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2025, 10, 7)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2025, 10, 14)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 13)?, "פסח"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2025, 6, 2)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 9, 12)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2026, 9, 13)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2026, 9, 21)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2026, 9, 26)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2026, 10, 3)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "פסח"),
            (NaiveDate::from_ymd_res(2026, 4, 8)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2026, 4, 22)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2026, 5, 22)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 10, 2)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2027, 10, 3)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2027, 10, 11)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2027, 10, 16)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2027, 10, 23)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 22)?, "פסח"),
            (NaiveDate::from_ymd_res(2027, 4, 28)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2027, 5, 12)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2027, 6, 11)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 9, 21)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2028, 9, 22)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2028, 9, 30)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2028, 10, 5)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2028, 10, 12)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 11)?, "פסח"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2028, 5, 2)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2028, 5, 31)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 9, 10)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2029, 9, 11)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2029, 9, 19)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2029, 9, 24)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2029, 10, 1)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "פסח"),
            (NaiveDate::from_ymd_res(2029, 4, 6)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2029, 4, 19)?, "(נצפה) יום העצמאות"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 9, 28)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2030, 9, 29)?, "ראש השנה"),
            (NaiveDate::from_ymd_res(2030, 10, 7)?, "יום כיפור"),
            (NaiveDate::from_ymd_res(2030, 10, 12)?, "סוכות"),
            (
                NaiveDate::from_ymd_res(2030, 10, 19)?,
                "שמחת תורה/שמיני עצרת",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "פסח"),
            (NaiveDate::from_ymd_res(2030, 4, 24)?, "שביעי של פסח"),
            (NaiveDate::from_ymd_res(2030, 5, 8)?, "יום העצמאות"),
            (NaiveDate::from_ymd_res(2030, 6, 7)?, "שבועות"),
        ],
        &mut map,
        Country::IL,
        "Israel",
    );

    Ok(map)
}
