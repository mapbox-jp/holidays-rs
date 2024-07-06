//! Venezuela
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Venezuela";
const COUNTY_CODE: Country = Country::VE;

/// Generate holiday map for Venezuela.
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
            (NaiveDate::from_ymd_res(2000, 3, 6)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2000, 3, 7)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2000, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2000, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 12)?, "Día de la Raza"),
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2000, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2001, 2, 26)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2001, 2, 27)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2001, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2001, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 12)?, "Día de la Raza"),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2001, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2002, 2, 11)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2002, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2002, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2002, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2002, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2003, 3, 3)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2003, 3, 4)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2003, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2003, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2003, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2004, 2, 23)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2004, 2, 24)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2004, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2004, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2004, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2004, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2005, 2, 7)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2005, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2005, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2005, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2006, 2, 27)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2006, 2, 28)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2006, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2006, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2007, 2, 20)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2007, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2007, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2007, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2007, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2008, 2, 4)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2008, 2, 5)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2008, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2008, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2008, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2008, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2009, 2, 23)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2009, 2, 24)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2009, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2009, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2009, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2010, 2, 16)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2010, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2010, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2010, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2010, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2011, 3, 7)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2011, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2011, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2011, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2012, 2, 20)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2012, 2, 21)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2012, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2012, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2013, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2013, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2013, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2013, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2014, 3, 3)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2014, 3, 4)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2014, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2014, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2014, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2015, 2, 16)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2015, 2, 17)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2015, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2015, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2015, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2015, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2016, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2016, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2016, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2017, 2, 27)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2017, 2, 28)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2017, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2017, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2017, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2018, 2, 12)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2018, 2, 13)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2018, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2018, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2019, 3, 4)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2019, 3, 5)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Jueves Santo"),
            (
                NaiveDate::from_ymd_res(2019, 4, 19)?,
                "Declaración de la Independencia; Viernes Santo",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2019, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2019, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2020, 2, 24)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2020, 2, 25)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2020, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2020, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2020, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2021, 2, 15)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2021, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2021, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2021, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2022, 2, 28)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2022, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2022, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2022, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2023, 2, 20)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2023, 2, 21)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2023, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2023, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2023, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2024, 2, 12)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2024, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2024, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2024, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2025, 3, 4)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2025, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2025, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2025, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2026, 2, 16)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2026, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2026, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2026, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2027, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2027, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2027, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2027, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2028, 2, 28)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2028, 2, 29)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2028, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2028, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2028, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2029, 2, 12)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2029, 4, 19)?,
                "Declaración de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2029, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2029, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2029, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
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
            (NaiveDate::from_ymd_res(2030, 3, 4)?, "Lunes de Carnaval"),
            (NaiveDate::from_ymd_res(2030, 3, 5)?, "Martes de Carnaval"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Jueves Santo"),
            (
                NaiveDate::from_ymd_res(2030, 4, 19)?,
                "Declaración de la Independencia; Viernes Santo",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Dia Mundial del Trabajador",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 24)?, "Batalla de Carabobo"),
            (
                NaiveDate::from_ymd_res(2030, 7, 5)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 24)?,
                "Natalicio de Simón Bolívar",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 12)?,
                "Día de la Resistencia Indígena",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Nochebuena"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Día de Navidad"),
            (
                NaiveDate::from_ymd_res(2030, 12, 31)?,
                "Fiesta de Fin de Año",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
