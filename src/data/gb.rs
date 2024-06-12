//! United Kingdom
use super::*;

/// Generate holiday map for United Kingdom.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2000, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2002, 6, 4)?, "Spring Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2002, 6, 3)?,
                "Golden Jubilee of Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2003, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2005, 5, 30)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2006, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2008, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2009, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2010, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2011, 5, 30)?, "Spring Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2011, 4, 29)?,
                "Wedding of William and Catherine",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2012, 6, 4)?, "Spring Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2012, 6, 5)?,
                "Diamond Jubilee of Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2013, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2014, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2015, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2016, 5, 30)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2017, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2019, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 5, 8)?, "May Day"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2020, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2021, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2022, 6, 2)?, "Spring Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2022, 6, 3)?,
                "Platinum Jubilee of Elizabeth II",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 19)?,
                "State Funeral of Queen Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Spring Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2023, 5, 8)?,
                "Coronation of Charles III",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2024, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2025, 5, 26)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2027, 5, 31)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2028, 5, 29)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2029, 5, 28)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2030, 5, 27)?, "Spring Bank Holiday"),
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::GB,
        "United Kingdom",
    );

    Ok(map)
}
