//! Lesotho
use super::*;

/// Generate holiday map for Lesotho.
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
            (NaiveDate::from_ymd_res(2000, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2000, 4, 4)?, "Heroes Day"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2000, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2000, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2001, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2001, 4, 4)?, "Heroes Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2001, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2001, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2002, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2002, 4, 4)?, "Heroes Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2002, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2002, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
            (NaiveDate::from_ymd_res(2002, 5, 25)?, "Africa Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2003, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2003, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2003, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2003, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2004, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2004, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2004, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2004, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2005, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2005, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2005, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2005, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2006, 3, 11)?, "Moshoeshoe's Day"),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Africa/Heroes Day; Ascension Day",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2006, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2006, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2007, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2007, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2007, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2007, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2008, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2008, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Ascension Day; Workers' Day",
            ),
            (NaiveDate::from_ymd_res(2008, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2008, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2009, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2009, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2009, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2010, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2010, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2010, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2010, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2011, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2011, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2011, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2011, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2012, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2012, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2012, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2012, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2013, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2013, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2013, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2013, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2014, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2014, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2014, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2015, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2015, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2016, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2016, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2016, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2016, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2017, 3, 11)?, "Moshoeshoe's Day"),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Africa/Heroes Day; Ascension Day",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2017, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2017, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2018, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2018, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2018, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2018, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2019, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2019, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2019, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2019, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2020, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2020, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2020, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2021, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2021, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2021, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2021, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2022, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2022, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2022, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2022, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2023, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2023, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2023, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2023, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2024, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2024, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2024, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2024, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2025, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2025, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2025, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2025, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2026, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2026, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2026, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2027, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2027, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2027, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2027, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2028, 3, 11)?, "Moshoeshoe's Day"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Africa/Heroes Day; Ascension Day",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2028, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2028, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2029, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2029, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2029, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2029, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (NaiveDate::from_ymd_res(2030, 3, 11)?, "Moshoeshoe's Day"),
            (NaiveDate::from_ymd_res(2030, 5, 25)?, "Africa/Heroes Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2030, 7, 17)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2030, 10, 4)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        Country::LS,
        "Lesotho",
    );

    Ok(map)
}
