//! Kazakhstan
use super::*;

/// Generate holiday map for Kazakhstan.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "New Year"),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "International Women's Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2000, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2000, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "New Year"),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "International Women's Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2001, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "New Year"),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2002, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "New Year"),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2003, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2003, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 10)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 1)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 27)?,
                "Republic Day (observed)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "New Year"),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2004, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 3)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 10)?,
                "Victory Day (observed)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "New Year"),
            (
                NaiveDate::from_ymd_res(2005, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2005, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 3)?, "New Year (observed)"),
            (NaiveDate::from_ymd_res(2005, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 19)?,
                "Kazakhstan Independence Day (observed)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2006, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2006, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2006, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 3)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2006, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 19)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "Kurban Ait (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2007, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2007, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2007, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 9)?, "Victory Day"),
            (
                NaiveDate::from_ymd_res(2008, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2008, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 10)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 1)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 27)?,
                "Republic Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 22)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2009, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2009, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 23)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 11)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 31)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2010, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2010, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2010, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2010, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2010, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2010, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 3)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 10)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2011, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2011, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2011, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 3)?, "New Year (observed)"),
            (NaiveDate::from_ymd_res(2011, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 19)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2012, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2012, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2012, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2012, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2012, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 3)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2012, 12, 3)?,
                "First President Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2013, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2013, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2013, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2013, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2013, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 8)?,
                "Capital Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 2)?,
                "First President Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2014, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2014, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2014, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2014, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2014, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 10)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 7)?,
                "Capital Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 1)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2015, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2015, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2015, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2015, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2015, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 11)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 31)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2016, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2016, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2016, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2016, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2016, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 10)?,
                "Defender of the Fatherland Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 19)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2017, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2017, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2017, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2017, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2017, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2017, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 3)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2017, 5, 8)?,
                "Defender of the Fatherland Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 19)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2018, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2018, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2018, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2018, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2018, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2018, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 3)?,
                "First President Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2019, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2019, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2019, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2019, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2019, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2019, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 8)?,
                "Capital Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 2)?,
                "First President Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 11)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2020, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2020, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2020, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2020, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2020, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2020, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 11)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 31)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 31)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2021, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2021, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2021, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2021, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2021, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 1)?, "First President Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 17)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2021, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2021, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 3)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 10)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2022, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2022, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2022, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2022, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2022, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 3)?, "New Year (observed)"),
            (NaiveDate::from_ymd_res(2022, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 10)?,
                "Defender of the Fatherland Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 9)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2023, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2023, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2023, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2023, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2023, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2023, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 3)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2023, 5, 8)?,
                "Defender of the Fatherland Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 28)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2024, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2024, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2024, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2024, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2024, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2024, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 8)?,
                "Capital Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2025, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2025, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2025, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2025, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2025, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2025, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 10)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 7)?,
                "Capital Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 1)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 27)?,
                "Republic Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2026, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2026, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2026, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2026, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2026, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 9)?,
                "International Women's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 11)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 31)?,
                "Constitution Day of the Republic of Kazakhstan (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 26)?,
                "Republic Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2027, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2027, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2027, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2027, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2027, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2027, 3, 24)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 3)?,
                "Kazakhstan People Solidarity Holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 10)?,
                "Victory Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2028, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2028, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2028, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2028, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2028, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2028, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 3)?, "New Year (observed)"),
            (NaiveDate::from_ymd_res(2028, 1, 4)?, "New Year (observed)"),
            (
                NaiveDate::from_ymd_res(2028, 5, 8)?,
                "Defender of the Fatherland Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 18)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2029, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2029, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2029, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2029, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2029, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2029, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 17)?,
                "Kazakhstan Independence Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "New Year"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "Orthodox Christmas"),
            (
                NaiveDate::from_ymd_res(2030, 3, 8)?,
                "International Women's Day",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 22)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2030, 3, 21)?, "Nauryz holiday"),
            (NaiveDate::from_ymd_res(2030, 3, 23)?, "Nauryz holiday"),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Kazakhstan People Solidarity Holiday",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 7)?,
                "Defender of the Fatherland Day",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 9)?, "Victory Day"),
            (NaiveDate::from_ymd_res(2030, 7, 6)?, "Capital Day"),
            (
                NaiveDate::from_ymd_res(2030, 8, 30)?,
                "Constitution Day of the Republic of Kazakhstan",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 25)?, "Republic Day"),
            (
                NaiveDate::from_ymd_res(2030, 12, 16)?,
                "Kazakhstan Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 25)?,
                "Nauryz holiday (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 8)?,
                "Capital Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Kurban Ait (estimated)",
            ),
        ],
        &mut map,
        Country::KZ,
        "Kazakhstan",
    );

    Ok(map)
}
