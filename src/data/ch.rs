//! Switzerland
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Switzerland";
const COUNTY_CODE: Country = Country::CH;

/// Generate holiday map for Switzerland.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2000, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2001, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2002, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2003, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2004, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2005, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2006, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2007, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2008, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2009, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2010, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2011, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2012, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2013, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2014, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2015, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2016, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2017, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2018, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2019, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2021, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2022, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2023, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2024, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2025, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2026, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2027, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2028, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2029, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Neujahrestag"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Auffahrt"),
            (NaiveDate::from_ymd_res(2030, 8, 1)?, "Nationalfeiertag"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Weihnachten"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
