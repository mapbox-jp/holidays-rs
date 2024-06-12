//! United States (Massachusetts)
use super::*;

/// Generate holiday map for United States (Massachusetts).
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
            (NaiveDate::from_ymd_res(2000, 5, 29)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2000, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 9, 4)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2000, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2000, 11, 10)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 23)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 17)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 21)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 9)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2000, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2000, 4, 17)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2001, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 9, 3)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2001, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2001, 11, 12)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 22)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2001, 1, 15)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 2, 19)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 8)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2001, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2001, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 5, 27)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2002, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 9, 2)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2002, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2002, 11, 28)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2002, 1, 21)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 18)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 14)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2002, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2002, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2002, 4, 15)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 5, 26)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2003, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 9, 1)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2003, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2003, 11, 27)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2003, 1, 20)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 17)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 13)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2003, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 31)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2004, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2004, 7, 5)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 6)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2004, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2004, 11, 25)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 24)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 19)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 16)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 11)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2004, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2004, 4, 19)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2005, 5, 30)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2005, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 9, 5)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2005, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2005, 11, 24)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 17)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 21)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 10)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2005, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2005, 4, 18)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
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
            (NaiveDate::from_ymd_res(2006, 5, 29)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2006, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 9, 4)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2006, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2006, 11, 10)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 23)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 16)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 20)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 9)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2006, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2007, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 9, 3)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2007, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2007, 11, 12)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 22)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2007, 1, 15)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 19)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 8)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2007, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2007, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 16)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 5, 26)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2008, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2008, 9, 1)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2008, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2008, 11, 27)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2008, 1, 21)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 18)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 13)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2008, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2008, 4, 21)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2009, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2009, 7, 3)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 7)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2009, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2009, 11, 26)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2009, 1, 19)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 2, 16)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 12)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2009, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 31)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 31)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2010, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2010, 7, 5)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 6)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2010, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2010, 11, 25)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 24)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 18)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 11)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2010, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2010, 4, 19)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2011, 5, 30)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2011, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 9, 5)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2011, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2011, 11, 24)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 17)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 21)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 10)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2011, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2011, 4, 18)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
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
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2012, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 9, 3)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2012, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2012, 11, 12)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 22)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 16)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 20)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 8)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2012, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2012, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 5, 27)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2013, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 9, 2)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2013, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2013, 11, 28)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2013, 1, 21)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 18)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2013, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2013, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 15)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 5, 26)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2014, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 9, 1)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2014, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2014, 11, 27)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2014, 1, 20)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 17)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 13)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2014, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2015, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2015, 7, 3)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 7)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2015, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2015, 11, 26)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2015, 1, 19)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 16)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 12)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2015, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2015, 4, 20)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 5, 30)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2016, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 9, 5)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2016, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2016, 11, 24)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 1, 18)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 15)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 10)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2016, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2016, 4, 18)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
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
            (NaiveDate::from_ymd_res(2017, 5, 29)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2017, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 9, 4)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2017, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2017, 11, 10)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 23)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 16)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 2, 20)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 9)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2017, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2018, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 9, 3)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2018, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2018, 11, 12)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 22)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2018, 1, 15)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 19)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 8)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2018, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2018, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 16)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2019, 5, 27)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2019, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 9, 2)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2019, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2019, 11, 28)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2019, 1, 21)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 18)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 14)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2019, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2019, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 15)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Memorial Day"),
            (NaiveDate::from_ymd_res(2020, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2020, 7, 3)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 7)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2020, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2020, 11, 26)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2020, 1, 20)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 2, 17)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 12)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2020, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2020, 4, 20)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 31)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 31)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2021, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 18)?,
                "Juneteenth National Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 5)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 6)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2021, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2021, 11, 25)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 24)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 1, 18)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 11)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2021, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2021, 4, 19)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2022, 5, 30)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2022, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 20)?,
                "Juneteenth National Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 9, 5)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2022, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2022, 11, 24)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 17)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 21)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2022, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
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
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2023, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2023, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 9, 4)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2023, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2023, 11, 10)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 23)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 16)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 20)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 9)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2023, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2023, 4, 17)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 5, 27)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2024, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2024, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 9, 2)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2024, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2024, 11, 28)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2024, 1, 15)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 19)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 14)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2024, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2024, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 15)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 5, 26)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2025, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2025, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 9, 1)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2025, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2025, 11, 27)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2025, 1, 20)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 17)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 13)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2025, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2026, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2026, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2026, 7, 3)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 7)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2026, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2026, 11, 26)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2026, 1, 19)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 12)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2026, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2026, 4, 20)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 31)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 31)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2027, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 18)?,
                "Juneteenth National Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 7, 4)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2027, 7, 5)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 6)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2027, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2027, 11, 25)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 24)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 18)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 15)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 11)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2027, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2027, 4, 19)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2028, 5, 29)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2028, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2028, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 9, 4)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2028, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2028, 11, 10)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 23)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2028, 1, 17)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 21)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 9)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2028, 3, 17)?, "Evacuation Day"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 5, 28)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2029, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2029, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 9, 3)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2029, 11, 11)?, "Veterans Day"),
            (
                NaiveDate::from_ymd_res(2029, 11, 12)?,
                "Veterans Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 22)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2029, 1, 15)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 19)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 8)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2029, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2029, 3, 19)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 16)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 5, 27)?, "Memorial Day"),
            (
                NaiveDate::from_ymd_res(2030, 6, 19)?,
                "Juneteenth National Independence Day",
            ),
            (NaiveDate::from_ymd_res(2030, 7, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 9, 2)?, "Labor Day"),
            (NaiveDate::from_ymd_res(2030, 11, 11)?, "Veterans Day"),
            (NaiveDate::from_ymd_res(2030, 11, 28)?, "Thanksgiving"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2030, 1, 21)?,
                "Martin Luther King Jr. Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 18)?,
                "Washington's Birthday",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 14)?, "Columbus Day"),
            (NaiveDate::from_ymd_res(2030, 3, 17)?, "Evacuation Day"),
            (
                NaiveDate::from_ymd_res(2030, 3, 18)?,
                "Evacuation Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 15)?, "Patriots' Day"),
        ],
        &mut map,
        Country::US_MA,
        "United States (Massachusetts)",
    );

    Ok(map)
}
