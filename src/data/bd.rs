//! Bangladesh
use super::*;

/// Generate holiday map for Bangladesh.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (
                NaiveDate::from_ymd_res(2000, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2000, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2001, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2002, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2003, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2004, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2005, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2006, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2007, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2008, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2009, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2010, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2011, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2012, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2013, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2014, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2015, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2016, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2017, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2018, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2019, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2020, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2021, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2022, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2023, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2024, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2025, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2026, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2027, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2028, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2029, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 2, 21)?,
                "International Mother's language Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 17)?,
                "Sheikh Mujibur Rahman's Birthday and Children's Day",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 26)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "Bengali New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "May Day"),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "National Mourning Day",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 16)?, "Victory Day"),
        ],
        &mut map,
        Country::BD,
        "Bangladesh",
    );

    Ok(map)
}
