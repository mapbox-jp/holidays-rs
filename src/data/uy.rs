//! Uruguay
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Uruguay";
const COUNTY_CODE: Country = Country::UY;

/// Generate holiday map for Uruguay.
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
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Día de la Familia"),
            (
                NaiveDate::from_ymd_res(2000, 3, 1)?,
                "Inauguración del Presidente de la República",
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
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Día de la Familia"),
            (
                NaiveDate::from_ymd_res(2005, 3, 1)?,
                "Inauguración del Presidente de la República",
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
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Día de la Familia"),
            (
                NaiveDate::from_ymd_res(2010, 3, 1)?,
                "Inauguración del Presidente de la República",
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
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Día de la Familia"),
            (
                NaiveDate::from_ymd_res(2015, 3, 1)?,
                "Inauguración del Presidente de la República",
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
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Día de la Familia"),
            (
                NaiveDate::from_ymd_res(2020, 3, 1)?,
                "Inauguración del Presidente de la República",
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
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Día de la Familia"),
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
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Día de los Trabajadores",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 18)?,
                "Jura de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 25)?,
                "Declaratoria de la Independencia",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Día de la Familia"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
