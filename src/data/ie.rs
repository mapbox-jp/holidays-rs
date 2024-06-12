//! Ireland
use super::*;

/// Generate holiday map for Ireland.
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
            (NaiveDate::from_ymd_res(2000, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2000, 6, 5)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2000, 8, 7)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2000, 10, 30)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2001, 8, 6)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2001, 10, 29)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2002, 6, 3)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2002, 8, 5)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2002, 10, 28)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2003, 6, 2)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2003, 8, 4)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2003, 10, 27)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2004, 6, 7)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2004, 8, 2)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2004, 10, 25)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2005, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2005, 6, 6)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2005, 8, 1)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2005, 10, 31)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2006, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2006, 8, 7)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2006, 10, 30)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2007, 6, 4)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2007, 8, 6)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2007, 10, 29)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2008, 6, 2)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2008, 8, 4)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2008, 10, 27)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2009, 8, 3)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2009, 10, 26)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2010, 6, 7)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2010, 8, 2)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2010, 10, 25)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2011, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2011, 8, 1)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2011, 10, 31)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "St. Stephen's Day"),
            (
                NaiveDate::from_ymd_res(2011, 9, 14)?,
                "National Day of Mourning",
            ),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2012, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2012, 6, 4)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2012, 8, 6)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2012, 10, 29)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2013, 6, 3)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2013, 8, 5)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2013, 10, 28)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2014, 6, 2)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2014, 8, 4)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2014, 10, 27)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2015, 6, 1)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2015, 8, 3)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2015, 10, 26)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2016, 6, 6)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2016, 8, 1)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2016, 10, 31)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2017, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2017, 8, 7)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2017, 10, 30)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2018, 6, 4)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2018, 8, 6)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2018, 10, 29)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2019, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2019, 6, 3)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2019, 8, 5)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2019, 10, 28)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2020, 8, 3)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2020, 10, 26)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2021, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2021, 6, 7)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2021, 8, 2)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2021, 10, 25)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2022, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "May Day"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2022, 8, 1)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2022, 10, 31)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "St. Stephen's Day"),
            (
                NaiveDate::from_ymd_res(2022, 3, 18)?,
                "Day of Remembrance and Recognition",
            ),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2023, 2, 6)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2023, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2023, 6, 5)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2023, 8, 7)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2023, 10, 30)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 2, 5)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2024, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2024, 6, 3)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2024, 8, 5)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2024, 10, 28)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 2, 3)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2025, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "May Day"),
            (NaiveDate::from_ymd_res(2025, 6, 2)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2025, 8, 4)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2025, 10, 27)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 2, 2)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2026, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 5, 4)?, "May Day"),
            (NaiveDate::from_ymd_res(2026, 6, 1)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2026, 8, 3)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2026, 10, 26)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2027, 2, 1)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2027, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "May Day"),
            (NaiveDate::from_ymd_res(2027, 6, 7)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2027, 8, 2)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2027, 10, 25)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2028, 2, 7)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2028, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "May Day"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2028, 8, 7)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2028, 10, 30)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 2, 5)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2029, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 5, 7)?, "May Day"),
            (NaiveDate::from_ymd_res(2029, 6, 4)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2029, 8, 6)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2029, 10, 29)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 2, 1)?, "St. Brigid's Day"),
            (NaiveDate::from_ymd_res(2030, 3, 17)?, "St. Patrick's Day"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 5, 6)?, "May Day"),
            (NaiveDate::from_ymd_res(2030, 6, 3)?, "June Bank Holiday"),
            (NaiveDate::from_ymd_res(2030, 8, 5)?, "August Bank Holiday"),
            (
                NaiveDate::from_ymd_res(2030, 10, 28)?,
                "October Bank Holiday",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "St. Stephen's Day"),
        ],
        &mut map,
        Country::IE,
        "Ireland",
    );

    Ok(map)
}
