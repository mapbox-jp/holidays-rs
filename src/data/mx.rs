//! Mexico
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Mexico";
const COUNTY_CODE: Country = Country::MX;

/// Generate holiday map for Mexico.
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
                NaiveDate::from_ymd_res(2000, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2000, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 20)?,
                "Día de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 1)?,
                "Transmisión del Poder Ejecutivo Federal",
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
                NaiveDate::from_ymd_res(2001, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2001, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2002, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2002, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2003, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2003, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2004, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2004, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2005, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2005, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2006, 2, 6)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2006, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 20)?,
                "Día de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 1)?,
                "Transmisión del Poder Ejecutivo Federal",
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
                NaiveDate::from_ymd_res(2007, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 19)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2007, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 19)?,
                "Día de la Revolución",
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
            (
                NaiveDate::from_ymd_res(2008, 2, 4)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 17)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2008, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 17)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2009, 2, 2)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 16)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2009, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 16)?,
                "Día de la Revolución",
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
            (
                NaiveDate::from_ymd_res(2010, 2, 1)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 15)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2010, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 15)?,
                "Día de la Revolución",
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
            (
                NaiveDate::from_ymd_res(2011, 2, 7)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2011, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 21)?,
                "Día de la Revolución",
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
            (
                NaiveDate::from_ymd_res(2012, 2, 6)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 19)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2012, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 19)?,
                "Día de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 1)?,
                "Transmisión del Poder Ejecutivo Federal",
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
                NaiveDate::from_ymd_res(2013, 2, 4)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 18)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2013, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 18)?,
                "Día de la Revolución",
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
            (
                NaiveDate::from_ymd_res(2014, 2, 3)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 17)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2014, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 17)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2015, 2, 2)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 16)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2015, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 16)?,
                "Día de la Revolución",
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
            (
                NaiveDate::from_ymd_res(2016, 2, 1)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2016, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 21)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2017, 2, 6)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 20)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2017, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2018, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 19)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2018, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 19)?,
                "Día de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 1)?,
                "Transmisión del Poder Ejecutivo Federal",
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
                NaiveDate::from_ymd_res(2019, 2, 4)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 18)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2019, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 18)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2020, 2, 3)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 16)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2020, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 16)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2021, 2, 1)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 15)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2021, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 15)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2022, 2, 7)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 21)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2022, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 21)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2023, 2, 6)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 20)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2023, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2024, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 18)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2024, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 18)?,
                "Día de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 1)?,
                "Transmisión del Poder Ejecutivo Federal",
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
                NaiveDate::from_ymd_res(2025, 2, 3)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 17)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2025, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 17)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2026, 2, 2)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 16)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2026, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 16)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2027, 2, 1)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 15)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2027, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 15)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2028, 2, 7)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 20)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2028, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 20)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2029, 2, 5)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 19)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2029, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 19)?,
                "Día de la Revolución",
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
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Día de la Constitución",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 18)?,
                "Natalicio de Benito Juárez",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Día del Trabajo"),
            (
                NaiveDate::from_ymd_res(2030, 9, 16)?,
                "Día de la Independencia",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 18)?,
                "Día de la Revolución",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 1)?,
                "Transmisión del Poder Ejecutivo Federal",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Navidad"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
