//! Paraguay
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Paraguay";
const COUNTY_CODE: Country = Country::PY;

/// Generate holiday map for Paraguay.
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
            (
                NaiveDate::from_ymd_res(2000, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2000, 4, 23)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2000, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2001, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2001, 4, 15)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2001, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2002, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2002, 3, 31)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2002, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2003, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2003, 4, 20)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2003, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2004, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2004, 4, 11)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2004, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2005, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2005, 3, 27)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2005, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2006, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2006, 4, 16)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2006, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2007, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2007, 4, 8)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2007, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2007, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2007, 1, 29)?, "Asueto adicionale"),
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
            (
                NaiveDate::from_ymd_res(2008, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2008, 3, 23)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2008, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2009, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2009, 4, 12)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2009, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2009, 9, 10)?, "Asueto adicionale"),
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
            (
                NaiveDate::from_ymd_res(2010, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2010, 4, 4)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2010, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2010, 6, 14)?, "Asueto adicionale"),
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
            (
                NaiveDate::from_ymd_res(2011, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2011, 4, 24)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2011, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2011, 4, 19)?, "Asueto adicionale"),
            (NaiveDate::from_ymd_res(2011, 5, 14)?, "Asueto adicionale"),
            (NaiveDate::from_ymd_res(2011, 5, 16)?, "Asueto adicionale"),
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
            (
                NaiveDate::from_ymd_res(2012, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2012, 4, 8)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2012, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2013, 3, 4)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2013, 3, 31)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2013, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2013, 8, 14)?, "Asueto adicionale"),
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
            (
                NaiveDate::from_ymd_res(2014, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2014, 4, 20)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2014, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 16)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2015, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2015, 4, 5)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2015, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 28)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Navidad"),
            (NaiveDate::from_ymd_res(2015, 7, 10)?, "Asueto adicionale"),
            (NaiveDate::from_ymd_res(2015, 7, 11)?, "Asueto adicionale"),
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
            (
                NaiveDate::from_ymd_res(2016, 2, 29)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2016, 3, 27)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2016, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 3)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2017, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2017, 4, 16)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2017, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 2)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2018, 2, 26)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2018, 4, 1)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2018, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 11)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2019, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2019, 4, 21)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2019, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2020, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2020, 4, 12)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2020, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2021, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2021, 4, 4)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 27)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2022, 2, 28)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2022, 4, 17)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2022, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 3)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Navidad"),
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
            (
                NaiveDate::from_ymd_res(2023, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2023, 4, 9)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2023, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2024, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2024, 3, 31)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2024, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2025, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2025, 4, 20)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2025, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2026, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2026, 4, 5)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2026, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2027, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2027, 3, 28)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2027, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2027, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2028, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2028, 4, 16)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2028, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2029, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2029, 4, 1)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "Día de la Virgen de Caacupé",
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
            (
                NaiveDate::from_ymd_res(2030, 3, 1)?,
                "Día de los Héroes de la Patria",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Jueves Santo"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Viernes Santo"),
            (
                NaiveDate::from_ymd_res(2030, 4, 21)?,
                "Domingo de Resurrección",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajador"),
            (
                NaiveDate::from_ymd_res(2030, 5, 14)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 15)?,
                "Día de la Independencia Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 12)?,
                "Día de la Paz del Chaco",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Día de la Fundación de Asunción",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 29)?,
                "Día de la Batalla de Boquerón",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "Día de la Virgen de Caacupé",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
