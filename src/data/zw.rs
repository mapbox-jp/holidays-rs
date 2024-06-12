//! Zimbabwe
use super::*;

/// Generate holiday map for Zimbabwe.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 22)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2000, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2000, 8, 14)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2000, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2001, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2001, 8, 13)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 14)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2001, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 3, 30)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2002, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2002, 8, 12)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 13)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2002, 12, 22)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 23)?,
                "Unity Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2003, 4, 18)?,
                "Good Friday; Independence Day",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 19)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2003, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2003, 5, 26)?,
                "Africa Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 11)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 12)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2003, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 4, 18)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2004, 4, 19)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2004, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2004, 8, 9)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2004, 8, 10)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2004, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2005, 8, 8)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2005, 8, 9)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2005, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 15)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2006, 8, 14)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2006, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2007, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2007, 8, 13)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 14)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2007, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2008, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2008, 5, 26)?,
                "Africa Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 11)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 12)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2008, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 11)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2009, 8, 10)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 11)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2009, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 4, 18)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2010, 4, 19)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2010, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2010, 8, 9)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2010, 8, 10)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2010, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2011, 8, 8)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2011, 8, 9)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2011, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 7)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2012, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2012, 8, 13)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 14)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2012, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 3, 30)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2013, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2013, 8, 12)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 13)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2013, 12, 22)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2013, 12, 23)?,
                "Unity Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2014, 4, 18)?,
                "Good Friday; Independence Day",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2014, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2014, 5, 26)?,
                "Africa Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 11)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 12)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2014, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2015, 8, 10)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 11)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2015, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 26)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2016, 8, 8)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2016, 8, 9)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2016, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2017, 8, 14)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2017, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2018, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2018, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2018, 8, 13)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 14)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2018, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2019, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 20)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2019, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 13)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2019, 12, 22)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2019, 12, 23)?,
                "Unity Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2020, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 11)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2020, 8, 10)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 11)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2020, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2021, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 22)?,
                "Robert Gabriel Mugabe National Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 3)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 4, 18)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2021, 4, 19)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2021, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2021, 8, 9)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2021, 8, 10)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2021, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 16)?, "Easter Saturday"),
            (
                NaiveDate::from_ymd_res(2022, 4, 18)?,
                "Easter Monday; Independence Day",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2022, 8, 8)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2022, 8, 9)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2022, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 8)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2023, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2023, 8, 14)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2023, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2024, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 3, 30)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2024, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2024, 8, 12)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 13)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2024, 12, 22)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2024, 12, 23)?,
                "Unity Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2025, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 18)?,
                "Good Friday; Independence Day",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2025, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2025, 5, 26)?,
                "Africa Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 11)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 12)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2025, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2026, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 4)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2026, 8, 10)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 11)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2026, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2027, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 22)?,
                "Robert Gabriel Mugabe National Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 27)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 4, 18)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2027, 4, 19)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2027, 5, 25)?, "Africa Day"),
            (NaiveDate::from_ymd_res(2027, 8, 9)?, "Zimbabwe Heroes' Day"),
            (NaiveDate::from_ymd_res(2027, 8, 10)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2027, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2028, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2028, 8, 14)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2028, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2029, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2029, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2029, 8, 13)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 14)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2029, 12, 22)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2030, 2, 21)?,
                "Robert Gabriel Mugabe National Youth Day",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 20)?, "Easter Saturday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2030, 5, 25)?, "Africa Day"),
            (
                NaiveDate::from_ymd_res(2030, 8, 12)?,
                "Zimbabwe Heroes' Day",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 13)?, "Defense Forces Day"),
            (NaiveDate::from_ymd_res(2030, 12, 22)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2030, 12, 23)?,
                "Unity Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::ZW,
        "Zimbabwe",
    );

    Ok(map)
}
