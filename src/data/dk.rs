//! Denmark
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Denmark";
const COUNTY_CODE: Country = Country::DK;

/// Generate holiday map for Denmark.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2000, 5, 19)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2000, 6, 1)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2001, 5, 11)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2001, 5, 24)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2002, 4, 26)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2003, 5, 16)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2003, 5, 29)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2004, 5, 7)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2004, 5, 20)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2005, 4, 22)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2005, 5, 16)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2006, 5, 12)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2007, 5, 4)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2007, 5, 17)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2008, 4, 18)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2009, 5, 8)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2009, 5, 21)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2010, 4, 30)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2010, 5, 13)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2011, 5, 20)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2011, 6, 2)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2012, 5, 4)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2012, 5, 17)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2013, 4, 26)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2014, 5, 16)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2014, 5, 29)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2015, 5, 14)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2016, 4, 22)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2016, 5, 5)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2017, 5, 12)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2018, 4, 27)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2018, 5, 10)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2019, 5, 17)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2019, 5, 30)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2020, 5, 8)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2020, 5, 21)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2021, 4, 30)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2022, 5, 13)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2022, 5, 26)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Anden påskedag"),
            (NaiveDate::from_ymd_res(2023, 5, 5)?, "Store bededag"),
            (
                NaiveDate::from_ymd_res(2023, 5, 18)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2025, 5, 29)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2026, 5, 14)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2027, 5, 6)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2029, 5, 10)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nytårsdag"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Skærtorsdag"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Påskedag"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Anden påskedag"),
            (
                NaiveDate::from_ymd_res(2030, 5, 30)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Pinsedag"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Anden pinsedag"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Juledag"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Anden juledag"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
