//! Kenya
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Kenya";
const COUNTY_CODE: Country = Country::KE;

/// Generate holiday map for Kenya.
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
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2000, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2001, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2002, 10, 20)?, "Kenyatta Day"),
            (
                NaiveDate::from_ymd_res(2002, 10, 21)?,
                "Kenyatta Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2003, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2003, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2004, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2004, 10, 11)?, "Moi Day (observed)"),
            (NaiveDate::from_ymd_res(2004, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2004, 12, 12)?, "Jamhuri Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 13)?,
                "Jamhuri Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
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
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2005, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2005, 12, 12)?, "Jamhuri Day"),
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
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2006, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2006, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2007, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2007, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2008, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2008, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2009, 10, 20)?, "Kenyatta Day"),
            (NaiveDate::from_ymd_res(2009, 12, 12)?, "Jamhuri Day"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2010, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2010, 12, 12)?, "Jamhuri Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 13)?,
                "Jamhuri Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
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
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2011, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2011, 12, 12)?, "Jamhuri Day"),
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
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2012, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2012, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2013, 10, 20)?, "Mashujaa Day"),
            (
                NaiveDate::from_ymd_res(2013, 10, 21)?,
                "Mashujaa Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 6, 1)?, "Madaraka Day"),
            (
                NaiveDate::from_ymd_res(2014, 6, 2)?,
                "Madaraka Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2014, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2015, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2015, 12, 12)?, "Jamhuri Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2016, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2016, 12, 12)?, "Jamhuri Day"),
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
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2017, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2017, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2018, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2018, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2018, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2019, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2019, 10, 20)?, "Mashujaa Day"),
            (
                NaiveDate::from_ymd_res(2019, 10, 21)?,
                "Mashujaa Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2020, 10, 10)?, "Moi Day"),
            (NaiveDate::from_ymd_res(2020, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2020, 12, 12)?, "Jamhuri Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2020, 2, 11)?,
                "President Moi Celebration of Life Day",
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
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2021, 10, 10)?, "Utamaduni Day"),
            (
                NaiveDate::from_ymd_res(2021, 10, 11)?,
                "Utamaduni Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2021, 12, 12)?, "Jamhuri Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 13)?,
                "Jamhuri Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
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
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2022, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2022, 12, 12)?, "Jamhuri Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Christmas Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2022, 4, 29)?,
                "State Funeral for Former President Mwai Kibaki",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 9)?, "Election Day"),
            (
                NaiveDate::from_ymd_res(2022, 9, 10)?,
                "Day of Mourning for Queen Elizabeth II",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 11)?,
                "Day of Mourning for Queen Elizabeth II",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 12)?,
                "Day of Mourning for Queen Elizabeth II",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 13)?, "Inauguration Day"),
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
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2023, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2023, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2023, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2024, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2024, 10, 20)?, "Mashujaa Day"),
            (
                NaiveDate::from_ymd_res(2024, 10, 21)?,
                "Mashujaa Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 6, 1)?, "Madaraka Day"),
            (
                NaiveDate::from_ymd_res(2025, 6, 2)?,
                "Madaraka Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2025, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2025, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2026, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2026, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2026, 12, 12)?, "Jamhuri Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Boxing Day"),
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
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2027, 10, 10)?, "Utamaduni Day"),
            (
                NaiveDate::from_ymd_res(2027, 10, 11)?,
                "Utamaduni Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2027, 12, 12)?, "Jamhuri Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 13)?,
                "Jamhuri Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Boxing Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
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
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2028, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2028, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2028, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2029, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2029, 10, 20)?, "Mashujaa Day"),
            (NaiveDate::from_ymd_res(2029, 12, 12)?, "Jamhuri Day"),
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
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Easter Monday"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 6, 1)?, "Madaraka Day"),
            (NaiveDate::from_ymd_res(2030, 10, 10)?, "Utamaduni Day"),
            (NaiveDate::from_ymd_res(2030, 10, 20)?, "Mashujaa Day"),
            (
                NaiveDate::from_ymd_res(2030, 10, 21)?,
                "Mashujaa Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 12)?, "Jamhuri Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Boxing Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
