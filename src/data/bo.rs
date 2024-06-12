//! Bolivia
use super::*;

/// Generate holiday map for Bolivia.
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
            (NaiveDate::from_ymd_res(2000, 3, 6)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2000, 3, 7)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2000, 6, 22)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2000, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 7)?,
                "Día de la Independencia de Bolivia (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2001, 2, 26)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2001, 2, 27)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2001, 6, 14)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2001, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2002, 2, 11)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2002, 5, 30)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2002, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2003, 3, 3)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2003, 3, 4)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2003, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2003, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 3)?,
                "Día de Todos los Difuntos (observado)",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2004, 2, 23)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2004, 2, 24)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2004, 6, 10)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2004, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2005, 2, 7)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Día del Trabajo (observado)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 26)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2005, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Navidad (observado)",
            ),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Año Nuevo (observado)",
            ),
            (NaiveDate::from_ymd_res(2006, 2, 27)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2006, 2, 28)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2006, 6, 15)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2006, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 7)?,
                "Día de la Independencia de Bolivia (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2007, 2, 20)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2007, 6, 7)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2007, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2008, 2, 4)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2008, 2, 5)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2008, 5, 22)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2008, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 3)?,
                "Día de Todos los Difuntos (observado)",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2009, 2, 23)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2009, 2, 24)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2009, 6, 11)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2009, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 22)?,
                "Año Nuevo Aymara Amazónico (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2010, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2010, 2, 16)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2010, 6, 3)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2010, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2011, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 7)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Día del Trabajo (observado)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2011, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Navidad (observado)",
            ),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "Año Nuevo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 23)?,
                "Día de la Creación del Estado Plurinacional de Bolivia (observado)",
            ),
            (NaiveDate::from_ymd_res(2012, 2, 20)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2012, 2, 21)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 4, 30)?,
                "Día del Trabajo (observado)",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 7)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2012, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2013, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2013, 5, 30)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2013, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2014, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 3)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2014, 3, 4)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "Día del Trabajo (observado)",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2014, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 3)?,
                "Día de Todos los Difuntos (observado)",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2015, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2015, 2, 16)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2015, 2, 17)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2015, 6, 4)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2015, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 22)?,
                "Año Nuevo Aymara Amazónico (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2016, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Día del Trabajo (observado)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 26)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2016, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Navidad (observado)",
            ),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "Año Nuevo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 23)?,
                "Día de la Creación del Estado Plurinacional de Bolivia (observado)",
            ),
            (NaiveDate::from_ymd_res(2017, 2, 27)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2017, 2, 28)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2017, 6, 15)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2017, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 7)?,
                "Día de la Independencia de Bolivia (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2018, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2018, 2, 12)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2018, 2, 13)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2018, 5, 31)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2018, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2019, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 4)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2019, 3, 5)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2019, 6, 20)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2019, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2020, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2020, 2, 24)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2020, 2, 25)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2020, 6, 11)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2020, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 22)?,
                "Año Nuevo Aymara Amazónico (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2021, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2021, 2, 15)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2021, 6, 3)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2021, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2022, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2022, 2, 28)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Día del Trabajo (observado)",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 16)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2022, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Navidad (observado)",
            ),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "Año Nuevo (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 23)?,
                "Día de la Creación del Estado Plurinacional de Bolivia (observado)",
            ),
            (NaiveDate::from_ymd_res(2023, 2, 20)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2023, 2, 21)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2023, 6, 8)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2023, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 7)?,
                "Día de la Independencia de Bolivia (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2024, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2024, 5, 30)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2024, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2025, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2025, 3, 4)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2025, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2025, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2026, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2026, 2, 16)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2026, 6, 4)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2026, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 22)?,
                "Año Nuevo Aymara Amazónico (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2027, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2027, 5, 27)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2027, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2028, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2028, 2, 28)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2028, 2, 29)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2028, 6, 15)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2028, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 7)?,
                "Día de la Independencia de Bolivia (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2029, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2029, 5, 31)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2029, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2030, 1, 22)?,
                "Día de la Creación del Estado Plurinacional de Bolivia",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 4)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2030, 3, 5)?, "Carnaval"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajo"),
            (NaiveDate::from_ymd_res(2030, 6, 20)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2030, 6, 21)?,
                "Año Nuevo Aymara Amazónico",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 6)?,
                "Día de la Independencia de Bolivia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 17)?,
                "Día de la Dignidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 2)?,
                "Día de Todos los Difuntos",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::BO,
        "Bolivia",
    );

    Ok(map)
}
