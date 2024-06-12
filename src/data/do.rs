//! Dominican Republic
use super::*;

/// Generate holiday map for Dominican Republic.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2000, 1, 10)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 24)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2000, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2000, 6, 22)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2000, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2000, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2000, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2001, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2001, 1, 29)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2001, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 30)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2001, 6, 14)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2001, 8, 20)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2001, 11, 5)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2002, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2002, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2002, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2002, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2002, 4, 29)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2002, 5, 30)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2002, 8, 19)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2002, 11, 4)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2003, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2003, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2003, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2003, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2003, 11, 10)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2004, 1, 5)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2004, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2004, 6, 10)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2004, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2004, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2005, 1, 10)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 24)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2005, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2005, 5, 26)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2005, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2006, 1, 9)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 30)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2006, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2006, 6, 15)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2006, 8, 14)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2006, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2007, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2007, 1, 29)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2007, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 30)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2007, 6, 7)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2007, 8, 20)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2007, 11, 5)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2008, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2008, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2008, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2008, 5, 22)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2008, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2008, 11, 10)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2009, 1, 5)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2009, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2009, 5, 4)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2009, 6, 11)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2009, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2009, 11, 9)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2010, 1, 4)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2010, 1, 25)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2010, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2010, 6, 3)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2010, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2010, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2011, 1, 10)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 24)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2011, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2012, 1, 9)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 30)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2012, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 30)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2012, 6, 7)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2012, 11, 5)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2013, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2013, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2013, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2013, 4, 29)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2013, 5, 30)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2013, 8, 19)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2013, 11, 4)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2014, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2014, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2014, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2014, 11, 10)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2015, 1, 5)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2015, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2015, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2015, 5, 4)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2015, 6, 4)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2015, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2015, 11, 9)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2016, 1, 4)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2016, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2016, 1, 25)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2016, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2016, 5, 26)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2016, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2017, 1, 9)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 30)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2017, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2017, 6, 15)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2017, 8, 14)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2017, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2018, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2018, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2018, 1, 29)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2018, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2018, 4, 30)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2018, 5, 31)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2018, 8, 20)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2018, 11, 5)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2019, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2019, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2019, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2019, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 29)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2019, 6, 20)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2019, 8, 19)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2019, 11, 4)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2020, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2020, 5, 4)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2020, 6, 11)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2020, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2020, 11, 9)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2021, 1, 4)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2021, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2021, 1, 25)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2021, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2021, 6, 3)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2021, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2021, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2022, 1, 10)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 24)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2022, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2022, 6, 16)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2022, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2023, 1, 9)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 30)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2023, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2023, 6, 8)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2023, 8, 14)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2023, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2024, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2024, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2024, 1, 29)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2024, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2024, 4, 29)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2024, 5, 30)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2024, 8, 19)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2024, 11, 4)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2025, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2025, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2025, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2025, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2025, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2025, 11, 10)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2026, 1, 5)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2026, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2026, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2026, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2026, 5, 4)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2026, 6, 4)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2026, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2026, 11, 9)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2027, 1, 4)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2027, 1, 25)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2027, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2027, 5, 27)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2027, 8, 16)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2027, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2028, 1, 10)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 24)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2028, 6, 15)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2028, 8, 14)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2028, 11, 6)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2029, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2029, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2029, 1, 29)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2029, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2029, 4, 30)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2029, 5, 31)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2029, 8, 20)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2029, 11, 5)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2030, 1, 6)?,
                "Día de los Santos Reyes",
            ),
            (
                NaiveDate::from_ymd_res(2030, 1, 21)?,
                "Día de la Altagracia",
            ),
            (NaiveDate::from_ymd_res(2030, 1, 26)?, "Día de Duarte"),
            (
                NaiveDate::from_ymd_res(2030, 2, 27)?,
                "Día de Independencia",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 29)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2030, 6, 20)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2030, 8, 19)?,
                "Día de la Restauración",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 24)?, "Día de las Mercedes"),
            (
                NaiveDate::from_ymd_res(2030, 11, 4)?,
                "Día de la Constitución",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::DO,
        "Dominican Republic",
    );

    Ok(map)
}
