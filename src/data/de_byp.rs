//! Germany (Bavaria (Bayern) with more protestants)
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Germany (Bavaria (Bayern) with more protestants)";
const COUNTY_CODE: Country = Country::DE_BYP;

/// Generate holiday map for Germany (Bavaria (Bayern) with more protestants).
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    let mut national_holidays = de::build(years)?;

    build_subdivision_year(
        years,
        2000,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2000, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2000, 6, 22)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2001,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2001, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2001, 6, 14)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2002,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2002, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2002, 5, 30)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2003,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2003, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2003, 6, 19)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2004,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2004, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2004, 6, 10)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2005,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2005, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2005, 5, 26)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2006,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2006, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2006, 6, 15)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2007,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2007, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2007, 6, 7)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2008,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2008, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2008, 5, 22)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2009,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2009, 6, 11)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2010,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2010, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2010, 6, 3)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2011,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2011, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2012,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2012, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2012, 6, 7)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2013,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2013, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2013, 5, 30)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2014,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2014, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2014, 6, 19)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2015,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2015, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2015, 6, 4)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2016,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2016, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2016, 5, 26)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2017,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2017, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2017, 6, 15)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2018,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2018, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2018, 5, 31)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2019,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2019, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2019, 6, 20)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2020,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2020, 6, 11)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2021,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2021, 6, 3)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2022,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2022, 6, 16)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2023,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2023, 6, 8)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2024,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2024, 5, 30)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2025,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2025, 6, 19)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2026,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2026, 6, 4)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2027,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2027, 5, 27)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2028,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2028, 6, 15)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2029,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2029, 5, 31)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_subdivision_year(
        years,
        2030,
        &mut national_holidays,
        [
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Heilige Drei Könige"),
            (NaiveDate::from_ymd_res(2030, 6, 20)?, "Fronleichnam"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Allerheiligen"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
