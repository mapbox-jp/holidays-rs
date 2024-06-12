//! Switzerland
use super::*;

/// Generate holiday map for Switzerland.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2000, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2001, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2002, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2003, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2004, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2005, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2006, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2007, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2008, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2009, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2010, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2011, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2012, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2013, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2014, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2015, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2016, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2017, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2018, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2019, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2021, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2022, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2023, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2024, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2025, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2026, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2027, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2028, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2029, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2030, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        Country::CH,
        "Switzerland",
    );

    Ok(map)
}
