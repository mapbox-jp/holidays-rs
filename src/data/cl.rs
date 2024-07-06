//! Chile
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Chile";
const COUNTY_CODE: Country = Country::CL;

/// Generate holiday map for Chile.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 22)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2000, 6, 19)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 26)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 4)?,
                "Día de la Unidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 9)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2001, 6, 11)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 3)?,
                "Día de la Unidad Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 15)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 30)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2002, 5, 27)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 19)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2003, 6, 16)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2004, 6, 7)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 28)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 11)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2005, 5, 23)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 27)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 10)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2006, 4, 15)?, "Sábado Santo"),
            (NaiveDate::from_ymd_res(2006, 6, 12)?, "Corpus Christi"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 26)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 9)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2007, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 17)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2007, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2008, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 11)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2009, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 28)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2010, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 11)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 27)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2011, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 10)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 7)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2012, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 17)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2012, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 15)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 2)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 30)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2013, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 20)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2013, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 4)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 26)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 27)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 10)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Feriado nacional"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2017, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 9)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 27)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2018, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 17)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2018, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 15)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 2)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 20)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2019, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 20)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2019, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 11)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 3)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 21)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 28)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 17)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2021, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 11)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2022, 4, 16)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 21)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 27)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 10)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2022, 9, 16)?, "Feriado nacional"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Feriado nacional"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 8)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 21)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 26)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2023, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 9)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 27)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 30)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 20)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2024, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 20)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2024, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 20)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2025, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 4)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 21)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2026, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 27)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 21)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 28)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2027, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 17)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2027, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 11)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 20)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 26)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2028, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 9)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 27)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 20)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 2)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2029, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 17)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2029, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 15)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 2)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año Nuevo"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 20)?, "Sábado Santo"),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Día Nacional del Trabajo",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 21)?,
                "Día de las Glorias Navales",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 21)?,
                "Día Nacional de los Pueblos Indígenas",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (NaiveDate::from_ymd_res(2030, 7, 16)?, "Virgen del Carmen"),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 18)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 19)?,
                "Día de las Glorias del Ejército",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 20)?, "Fiestas Patrias"),
            (
                NaiveDate::from_ymd_res(2030, 10, 12)?,
                "Día del Encuentro de dos Mundos",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 31)?,
                "Día Nacional de las Iglesias Evangélicas y Protestantes",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 1)?,
                "Día de Todos los Santos",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "La Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
