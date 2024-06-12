//! Honduras
use super::*;

/// Generate holiday map for Honduras.
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
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 22)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2000, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2000, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2000, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2000, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2001, 4, 14)?,
                "Día de las Américas; Sábado de Gloria",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2001, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2001, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2001, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 30)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2002, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2002, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2002, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2002, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 19)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2003, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2003, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2003, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2003, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2004, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2004, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2004, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2004, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2005, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2005, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2005, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Jueves Santo"),
            (
                NaiveDate::from_ymd_res(2006, 4, 14)?,
                "Día de las Américas; Viernes Santo",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 15)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2006, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2006, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2006, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2007, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2007, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2007, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2007, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2008, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2008, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2008, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2008, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 11)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2009, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2009, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2009, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2009, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2010, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2010, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2010, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2010, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2011, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2011, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2011, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2011, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 7)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2012, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2012, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2012, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 30)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2013, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2013, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2013, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2013, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2014, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "Día de Morazán"),
            (NaiveDate::from_ymd_res(2014, 10, 12)?, "Día de la Raza"),
            (
                NaiveDate::from_ymd_res(2014, 10, 21)?,
                "Día de las Fuerzas Armadas",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2015, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2015, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2015, 10, 8)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2015, 10, 9)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 26)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2016, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2016, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2016, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2016, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Jueves Santo"),
            (
                NaiveDate::from_ymd_res(2017, 4, 14)?,
                "Día de las Américas; Viernes Santo",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2017, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2017, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2017, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2018, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2018, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 3)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2018, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2018, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 20)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2019, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2019, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 2)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2019, 10, 3)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2019, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 11)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2020, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2020, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2020, 10, 8)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2020, 10, 9)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 3)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2021, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2021, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2021, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2021, 10, 8)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año Nuevo"),
            (
                NaiveDate::from_ymd_res(2022, 4, 14)?,
                "Día de las Américas; Jueves Santo",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2022, 4, 16)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2022, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2022, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2022, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 8)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2023, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2023, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2023, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2023, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 30)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2024, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 2)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2024, 10, 3)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2024, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2025, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2025, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 1)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2025, 10, 2)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2025, 10, 3)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 4)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2026, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2026, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2026, 10, 8)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2026, 10, 9)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 27)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2027, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2027, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2027, 10, 7)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2027, 10, 8)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Jueves Santo"),
            (
                NaiveDate::from_ymd_res(2028, 4, 14)?,
                "Día de las Américas; Viernes Santo",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2028, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2028, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2028, 10, 6)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2029, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2029, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 3)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2029, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2029, 10, 5)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 20)?, "Sábado de Gloria"),
            (NaiveDate::from_ymd_res(2030, 4, 14)?, "Día de las Américas"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2030, 9, 15)?,
                "Día de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 2)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2030, 10, 3)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2030, 10, 4)?, "Semana Morazánica"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
        ],
        &mut map,
        Country::HN,
        "Honduras",
    );

    Ok(map)
}
