//! Cuba
use super::*;

/// Generate holiday map for Cuba.
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
                NaiveDate::from_ymd_res(2000, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 11)?,
                "Inicio de las Guerras de Independencia (observado)",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Día Internacional de los Trabajadores (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Triunfo de la Revolución (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Día de Navidad"),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2007, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Día de la Victoria"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2008, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Día de la Victoria"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2009, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Día de la Victoria"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 11)?,
                "Inicio de las Guerras de Independencia (observado)",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2010, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Día de la Victoria"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Día Internacional de los Trabajadores (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2011, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "Día de la Victoria; Triunfo de la Revolución (observado)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2013, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2014, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2015, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Día Internacional de los Trabajadores (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2016, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2017, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2019, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2020, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 11)?,
                "Inicio de las Guerras de Independencia (observado)",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2021, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Día Internacional de los Trabajadores (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2022, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2023, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2024, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2025, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2026, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 11)?,
                "Inicio de las Guerras de Independencia (observado)",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2027, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2028, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2029, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 1, 1)?,
                "Triunfo de la Revolución",
            ),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Día de la Victoria"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Día Internacional de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 25)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 26)?,
                "Día de la Rebeldía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 27)?,
                "Conmemoración del asalto a Moncada",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 10)?,
                "Inicio de las Guerras de Independencia",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2030, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        Country::CU,
        "Cuba",
    );

    Ok(map)
}
