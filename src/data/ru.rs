//! Russia
use super::*;

/// Generate holiday map for Russia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Новый год"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "Новый год"),
            (NaiveDate::from_ymd_res(2000, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 2)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 9)?, "День Победы"),
            (
                NaiveDate::from_ymd_res(2000, 6, 12)?,
                "День принятия Декларации о государственном суверенитете Российской Федерации",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 7)?,
                "День согласия и примирения",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Новый год"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "Новый год"),
            (NaiveDate::from_ymd_res(2001, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 2)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 9)?, "День Победы"),
            (
                NaiveDate::from_ymd_res(2001, 6, 12)?,
                "День принятия Декларации о государственном суверенитете Российской Федерации",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 7)?,
                "День согласия и примирения",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Новый год"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "Новый год"),
            (NaiveDate::from_ymd_res(2002, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 2)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2002, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2002, 11, 7)?,
                "День согласия и примирения",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Новый год"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "Новый год"),
            (NaiveDate::from_ymd_res(2003, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2003, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 2)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2003, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2003, 11, 7)?,
                "День согласия и примирения",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Новый год"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "Новый год"),
            (NaiveDate::from_ymd_res(2004, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2004, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 2)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2004, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2004, 11, 7)?,
                "День согласия и примирения",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2005, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2005, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2005, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2005, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2005, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2005, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2006, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2006, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2006, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2006, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2006, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2006, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2006, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2007, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2007, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2007, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2007, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2007, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2007, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2007, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2008, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2008, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2008, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2008, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2008, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2008, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2009, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2009, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2009, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2009, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2009, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2009, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2010, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2010, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2010, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2010, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2010, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2010, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2011, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2011, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2011, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2011, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2011, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2012, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2012, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2012, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2012, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2012, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2012, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2013, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2013, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2013, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2014, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2014, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2014, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2015, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2015, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2015, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2016, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2016, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2016, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2017, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2017, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2017, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2018, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2018, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2018, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2019, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2019, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2019, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2020, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2020, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2020, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2021, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2021, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2021, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2022, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2022, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2022, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2023, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2023, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2023, 11, 4)?,
                "День народного единства",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 24)?,
                "Выходной (перенесено с 01.01.2023)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 8)?,
                "Выходной (перенесено с 08.01.2023)",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2024, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2024, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2024, 11, 4)?,
                "День народного единства",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 29)?,
                "Выходной (перенесено с 27.04.2024)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 30)?,
                "Выходной (перенесено с 02.11.2024)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 10)?,
                "Выходной (перенесено с 06.01.2024)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 30)?,
                "Выходной (перенесено с 28.12.2024)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 31)?,
                "Выходной (перенесено с 07.01.2024)",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2025, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2025, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2025, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2026, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2026, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2026, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2027, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2027, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2027, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2028, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2028, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2028, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2029, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2029, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2029, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 3)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 4)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 5)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 8)?, "Новогодние каникулы"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "Рождество Христово"),
            (
                NaiveDate::from_ymd_res(2030, 2, 23)?,
                "День защитника Отечества",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 8)?,
                "Международный женский день",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Праздник Весны и Труда",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 9)?, "День Победы"),
            (NaiveDate::from_ymd_res(2030, 6, 12)?, "День России"),
            (
                NaiveDate::from_ymd_res(2030, 11, 4)?,
                "День народного единства",
            ),
        ],
        &mut map,
        Country::RU,
        "Russia",
    );

    Ok(map)
}
