//! Germany (Saarland)
use super::*;

/// Generate holiday map for Germany (Saarland).
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2000, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 22)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2001, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 14)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2001, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2002, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 30)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2002, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2003, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 19)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2004, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2004, 6, 10)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2004, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2005, 5, 16)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2005, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 26)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2006, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 15)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2007, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2007, 6, 7)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2007, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Ostermontag"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Christi Himmelfahrt; Erster Mai",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2008, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 22)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2009, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 11)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2009, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2010, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2010, 6, 3)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2010, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2011, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2012, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 7)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2012, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2013, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 30)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2013, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2014, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 19)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2015, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2015, 6, 4)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2015, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2016, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 26)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2017, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 31)?, "Reformationstag"),
            (NaiveDate::from_ymd_res(2017, 6, 15)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2018, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 31)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2018, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2019, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 20)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2019, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2020, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 11)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2020, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2021, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 3)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2021, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2022, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 16)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2023, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 8)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2024, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 30)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2024, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2025, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 19)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2026, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 4)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2026, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2027, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 27)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2027, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2028, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 15)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2029, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 31)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2029, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Neujahr"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Karfreitag"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Ostermontag"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Erster Mai"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Christi Himmelfahrt"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Pfingstmontag"),
            (
                NaiveDate::from_ymd_res(2030, 10, 3)?,
                "Tag der Deutschen Einheit",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 25)?,
                "Erster Weihnachtstag",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Zweiter Weihnachtstag",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 20)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2030, 8, 15)?, "Mariä Himmelfahrt"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        Country::DE_SL,
        "Germany (Saarland)",
    );

    Ok(map)
}
