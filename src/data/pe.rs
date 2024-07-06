//! Peru
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Peru";
const COUNTY_CODE: Country = Country::PE;

/// Generate holiday map for Peru.
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
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2000, 4, 23)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2000, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2000, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2001, 4, 15)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2001, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2001, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2002, 3, 31)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2002, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2002, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2003, 4, 20)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2003, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2003, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2004, 4, 11)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2004, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2004, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2005, 3, 27)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2005, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2006, 4, 16)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2006, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2006, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2007, 4, 8)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2007, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2007, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2008, 3, 23)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2008, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2008, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2009, 4, 12)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2009, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2009, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2010, 4, 4)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2010, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2010, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2011, 4, 24)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2011, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2011, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2012, 4, 8)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2012, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2013, 3, 31)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2013, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2013, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2014, 4, 20)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2014, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2015, 4, 5)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2015, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2015, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2016, 3, 27)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2016, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2016, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2017, 4, 16)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2017, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2017, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2018, 4, 1)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2018, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2018, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2019, 4, 21)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2019, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2019, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2020, 4, 12)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2020, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2020, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2021, 4, 4)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2021, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2021, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2022, 4, 17)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2022, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2022, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2022, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2023, 4, 9)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2023, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2023, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2024, 3, 31)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2024, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2024, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2024, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2025, 4, 20)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2025, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2025, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2025, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2026, 4, 5)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2026, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2026, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2026, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2027, 3, 28)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2027, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2027, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2027, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2028, 4, 16)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2028, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2028, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2028, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2029, 4, 1)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2029, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2029, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2029, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Navidad del Señor"),
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
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2030, 4, 21)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2030, 6, 29)?,
                "San Pedro y San Pablo",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 28)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 29)?,
                "Día de la Gran Parada Militar",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 6)?, "Batalla de Junín"),
            (NaiveDate::from_ymd_res(2030, 8, 30)?, "Santa Rosa de Lima"),
            (NaiveDate::from_ymd_res(2030, 10, 8)?, "Combate de Angamos"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Todos Los Santos"),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 9)?, "Batalla de Ayacucho"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad del Señor"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
