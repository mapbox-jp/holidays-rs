//! Canada
use super::*;

/// Generate holiday map for Canada.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2000, 9, 4)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2001, 9, 3)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2002, 9, 2)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2003, 9, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2004, 9, 6)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2005, 9, 5)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2006, 9, 4)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2007, 9, 3)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2008, 9, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2009, 9, 7)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2010, 9, 6)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2011, 9, 5)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2012, 9, 3)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2013, 9, 2)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2014, 9, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2015, 9, 7)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2016, 9, 5)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2017, 9, 4)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2018, 9, 3)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2019, 9, 2)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2020, 9, 7)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2021, 9, 6)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2022, 9, 5)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2023, 9, 4)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2024, 9, 2)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2025, 9, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2026, 9, 7)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2027, 9, 6)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Christmas Day (observed)",
            ),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2028, 9, 4)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2029, 9, 3)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 7, 1)?, "Canada Day"),
            (NaiveDate::from_ymd_res(2030, 9, 2)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        Country::CA,
        "Canada",
    );

    Ok(map)
}
