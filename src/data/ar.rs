//! Argentina
use super::*;

/// Generate holiday map for Argentina.
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
        (NaiveDate::from_ymd_res(2000, 4, 2)?, "Día del Veterano de Guerra"),
        (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
        (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
        (NaiveDate::from_ymd_res(2000, 5, 25)?, "Día de la Revolución de Mayo"),
        (NaiveDate::from_ymd_res(2000, 6, 10)?, "Día de los Derechos Argentinos sobre las Islas Malvinas, Sandwich y del Atlántico Sur"),
        (NaiveDate::from_ymd_res(2000, 6, 19)?, "Paso a la Inmortalidad del General Don Manuel Belgrano"),
        (NaiveDate::from_ymd_res(2000, 7, 9)?, "Día de la Independencia"),
        (NaiveDate::from_ymd_res(2000, 12, 8)?, "Inmaculada Concepción de María"),
        (NaiveDate::from_ymd_res(2000, 12, 25)?, "Navidad"),
        (NaiveDate::from_ymd_res(2000, 8, 21)?, "Paso a la Inmortalidad del General Don José de San Martin"),
        (NaiveDate::from_ymd_res(2000, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2001, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2001, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 18)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2001, 8, 20)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2002, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2002, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 17)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2002, 8, 19)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2003, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2003, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 16)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2003, 8, 18)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2004, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2004, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 21)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2004, 8, 16)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2005, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2006, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 19)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2006, 8, 21)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2007, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2007, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 18)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2007, 8, 20)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2008, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 16)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2008, 8, 18)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2009, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2009, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 15)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2009, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 12)?, "Día de la Raza"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2010, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas; Viernes Santo",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2010, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 21)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2010, 8, 16)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 11)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2011, 3, 7)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2011, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2011, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2011, 8, 22)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 10)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 25)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 9)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2012, 2, 20)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2012, 2, 21)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2012, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 15)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 19)?,
                "Día de la Soberanía Nacional (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 27)?,
                "Bicentenario de la creación y primera jura de la bandera nacional",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 30)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 24)?,
                "Bicentenario de la Batalla de Tucumán",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 24)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2013,
        vec![

        (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año Nuevo"),
        (NaiveDate::from_ymd_res(2013, 2, 11)?, "Día de Carnaval"),
        (NaiveDate::from_ymd_res(2013, 2, 12)?, "Día de Carnaval"),
        (NaiveDate::from_ymd_res(2013, 3, 24)?, "Día Nacional de la Memoria por la Verdad y la Justicia"),
        (NaiveDate::from_ymd_res(2013, 4, 2)?, "Día del Veterano y de los Caidos en la Guerra de Malvinas"),
        (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
        (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajo"),
        (NaiveDate::from_ymd_res(2013, 5, 25)?, "Día de la Revolución de Mayo"),
        (NaiveDate::from_ymd_res(2013, 6, 20)?, "Paso a la Inmortalidad del General Don Manuel Belgrano"),
        (NaiveDate::from_ymd_res(2013, 7, 9)?, "Día de la Independencia"),
        (NaiveDate::from_ymd_res(2013, 12, 8)?, "Inmaculada Concepción de María"),
        (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad"),
        (NaiveDate::from_ymd_res(2013, 8, 17)?, "Paso a la Inmortalidad del General Don José de San Martin"),
        (NaiveDate::from_ymd_res(2013, 10, 12)?, "Día del Respeto a la Diversidad Cultural"),
        (NaiveDate::from_ymd_res(2013, 11, 18)?, "Día de la Soberanía Nacional (observado)"),
        (NaiveDate::from_ymd_res(2013, 1, 31)?, "Bicentenario de la sesión inaugural de la Asamblea Nacional Constituyente del año 1813"),
        (NaiveDate::from_ymd_res(2013, 2, 20)?, "Bicentenario de la Batalla de Salta"),
        (NaiveDate::from_ymd_res(2013, 4, 1)?, "Feriado con fines turísticos"),
        (NaiveDate::from_ymd_res(2013, 6, 21)?, "Feriado con fines turísticos"),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2014, 3, 3)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2014, 3, 4)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2014, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2014, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 24)?,
                "Día de la Soberanía Nacional (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2015, 2, 16)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2015, 2, 17)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2015, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2015, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2015, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 27)?,
                "Día de la Soberanía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 23)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 7)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2016, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2016, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2016, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 10)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 28)?,
                "Día de la Soberanía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 8)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 9)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2017, 2, 27)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2017, 2, 28)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2017, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2017, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 21)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 16)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2018, 2, 12)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2018, 2, 13)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2018, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2018, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2018, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 20)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 15)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 19)?,
                "Día de la Soberanía Nacional (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2019, 3, 4)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2019, 3, 5)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2019, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2019, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2019, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 18)?,
                "Día de la Soberanía Nacional (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 8)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 19)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 14)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2020, 2, 24)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2020, 2, 25)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2020, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 31)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2020, 6, 15)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 23)?,
                "Día de la Soberanía Nacional (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 23)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 10)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 7)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2021, 2, 15)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2021, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas; Viernes Santo",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2021, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2021, 6, 21)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 16)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 11)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 24)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 8)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 22)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2022, 2, 28)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2022, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2022, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2022, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 10)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 18)?, "Censo nacional 2022"),
            (
                NaiveDate::from_ymd_res(2022, 10, 7)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 21)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 9)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2023, 2, 20)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2023, 2, 21)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2023, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2023, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2023, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 21)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 16)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 26)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 19)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 13)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2024, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2024, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 18)?,
                "Día de la Soberanía Nacional (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 21)?,
                "Feriado con fines turísticos",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 11)?,
                "Feriado con fines turísticos",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2025, 3, 4)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2025, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2025, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2025, 6, 16)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 24)?,
                "Día de la Soberanía Nacional (observado)",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2026, 2, 16)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2026, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2026, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2026, 6, 15)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 23)?,
                "Día de la Soberanía Nacional (observado)",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2027, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2027, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2027, 6, 21)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 16)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 11)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2028, 2, 28)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2028, 2, 29)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2028, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2028, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 21)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 16)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 20)?,
                "Día de la Soberanía Nacional",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2029, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2029, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2029, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 20)?,
                "Paso a la Inmortalidad del General Don José de San Martin (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 15)?,
                "Día del Respeto a la Diversidad Cultural (observado)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 19)?,
                "Día de la Soberanía Nacional (observado)",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2030, 3, 4)?, "Día de Carnaval"),
            (NaiveDate::from_ymd_res(2030, 3, 5)?, "Día de Carnaval"),
            (
                NaiveDate::from_ymd_res(2030, 3, 24)?,
                "Día Nacional de la Memoria por la Verdad y la Justicia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 2)?,
                "Día del Veterano y de los Caidos en la Guerra de Malvinas",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2030, 5, 25)?,
                "Día de la Revolución de Mayo",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 20)?,
                "Paso a la Inmortalidad del General Don Manuel Belgrano",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 9)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "Inmaculada Concepción de María",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
            (
                NaiveDate::from_ymd_res(2030, 6, 17)?,
                "Paso a la Inmortalidad del General Don Martín Miguel de Güemes",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 17)?,
                "Paso a la Inmortalidad del General Don José de San Martin",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 12)?,
                "Día del Respeto a la Diversidad Cultural",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 18)?,
                "Día de la Soberanía Nacional (observado)",
            ),
        ],
        &mut map,
        Country::AR,
        "Argentina",
    );

    Ok(map)
}
