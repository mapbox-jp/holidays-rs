//! New Zealand
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "New Zealand";
const COUNTY_CODE: Country = Country::NZ;

/// Generate holiday map for New Zealand.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2000, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 6, 5)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2000, 10, 23)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2001, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2001, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2001, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2001, 10, 22)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2002, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2002, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 6, 3)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2002, 10, 28)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2003, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2003, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 6, 2)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2003, 10, 27)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2004, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2004, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2004, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 6, 7)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2004, 10, 25)?, "Labour Day"),
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
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2005, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 6, 6)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2005, 10, 24)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2006, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2006, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2007, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2007, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2007, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 6, 4)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2007, 10, 22)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2008, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2008, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2008, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 6, 2)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2008, 10, 27)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2009, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2009, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2009, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2009, 10, 26)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2009, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2010, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2010, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 6, 7)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2010, 10, 25)?, "Labour Day"),
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
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 2, 6)?, "Waitangi Day"),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Anzac Day; Easter Monday",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 6, 6)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2011, 10, 24)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2012, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2012, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2012, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 6, 4)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2012, 10, 22)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2013, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2013, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2013, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 6, 3)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2013, 10, 28)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2014, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2014, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2014, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 6, 2)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2014, 10, 27)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2015, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2015, 4, 25)?, "Anzac Day"),
            (
                NaiveDate::from_ymd_res(2015, 4, 27)?,
                "Anzac Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 6, 1)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2015, 10, 26)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2015, 12, 28)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2016, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 2, 6)?, "Waitangi Day"),
            (
                NaiveDate::from_ymd_res(2016, 2, 8)?,
                "Waitangi Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 6, 6)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2016, 10, 24)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2017, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2017, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2017, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2017, 10, 23)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2018, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2018, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2018, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 6, 4)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2018, 10, 22)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2019, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2019, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2019, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 6, 3)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2019, 10, 28)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2020, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2020, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2020, 4, 25)?, "Anzac Day"),
            (
                NaiveDate::from_ymd_res(2020, 4, 27)?,
                "Anzac Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2020, 10, 26)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2020, 12, 28)?,
                "Boxing Day (observed)",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2021, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 2, 6)?, "Waitangi Day"),
            (
                NaiveDate::from_ymd_res(2021, 2, 8)?,
                "Waitangi Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 25)?, "Anzac Day"),
            (
                NaiveDate::from_ymd_res(2021, 4, 26)?,
                "Anzac Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 6, 7)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2021, 10, 25)?, "Labour Day"),
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
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 2, 6)?, "Waitangi Day"),
            (
                NaiveDate::from_ymd_res(2022, 2, 7)?,
                "Waitangi Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Queen's Birthday"),
            (NaiveDate::from_ymd_res(2022, 6, 24)?, "Matariki"),
            (NaiveDate::from_ymd_res(2022, 10, 24)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2022, 9, 26)?,
                "Queen Elizabeth II Memorial Day",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2023, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2023, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2023, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 6, 5)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2023, 7, 14)?, "Matariki"),
            (NaiveDate::from_ymd_res(2023, 10, 23)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2024, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2024, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2024, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 6, 3)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2024, 6, 28)?, "Matariki"),
            (NaiveDate::from_ymd_res(2024, 10, 28)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2025, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2025, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2025, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 6, 2)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2025, 6, 20)?, "Matariki"),
            (NaiveDate::from_ymd_res(2025, 10, 27)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2026, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2026, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2026, 4, 25)?, "Anzac Day"),
            (
                NaiveDate::from_ymd_res(2026, 4, 27)?,
                "Anzac Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 6, 1)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2026, 7, 10)?, "Matariki"),
            (NaiveDate::from_ymd_res(2026, 10, 26)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 28)?,
                "Boxing Day (observed)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2027, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 6)?, "Waitangi Day"),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "Waitangi Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 25)?, "Anzac Day"),
            (
                NaiveDate::from_ymd_res(2027, 4, 26)?,
                "Anzac Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 6, 7)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2027, 6, 25)?, "Matariki"),
            (NaiveDate::from_ymd_res(2027, 10, 25)?, "Labour Day"),
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
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "New Year's Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 2)?,
                "Day after New Year's Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 4)?,
                "Day after New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 2, 6)?, "Waitangi Day"),
            (
                NaiveDate::from_ymd_res(2028, 2, 7)?,
                "Waitangi Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2028, 7, 14)?, "Matariki"),
            (NaiveDate::from_ymd_res(2028, 10, 23)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2029, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2029, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2029, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 6, 4)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2029, 7, 6)?, "Matariki"),
            (NaiveDate::from_ymd_res(2029, 10, 22)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "New Year's Day"),
            (
                NaiveDate::from_ymd_res(2030, 1, 2)?,
                "Day after New Year's Day",
            ),
            (NaiveDate::from_ymd_res(2030, 2, 6)?, "Waitangi Day"),
            (NaiveDate::from_ymd_res(2030, 4, 25)?, "Anzac Day"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 6, 3)?, "King's Birthday"),
            (NaiveDate::from_ymd_res(2030, 6, 21)?, "Matariki"),
            (NaiveDate::from_ymd_res(2030, 10, 28)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
