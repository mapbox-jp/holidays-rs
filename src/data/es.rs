//! Spain
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Spain";
const COUNTY_CODE: Country = Country::ES;

/// Generate holiday map for Spain.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2000, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2000, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2001, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2003, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2004, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2005, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2006, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2007, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2008, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2009, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2010, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2010, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2011, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "Inmaculada Concepción",
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
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2012, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2013, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2014, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2016, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "Inmaculada Concepción",
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
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2017, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2018, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2019, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2021, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2021, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2022, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "Inmaculada Concepción",
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
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2023, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2024, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2025, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2026, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2027, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2028, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2029, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 25)?,
                "Natividad del Señor",
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
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Año nuevo"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Epifanía del Señor"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Fiesta del Trabajo"),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Asunción de la Virgen",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 12)?,
                "Fiesta Nacional de España",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Todos los Santos"),
            (
                NaiveDate::from_ymd_res(2030, 12, 6)?,
                "Día de la Constitución Española",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "Inmaculada Concepción",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 25)?,
                "Natividad del Señor",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
