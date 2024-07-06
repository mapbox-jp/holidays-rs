//! South Africa
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "South Africa";
const COUNTY_CODE: Country = Country::ZA;

/// Generate holiday map for South Africa.
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
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2000, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2000, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2000, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2000, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2000, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2000, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2000, 9, 25)?,
                "Heritage Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "Y2K changeover"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "Y2K changeover (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Day of Reconciliation",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Day of Reconciliation (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2001, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2001, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2001, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2001, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2001, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2002, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2002, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2002, 6, 16)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2002, 6, 17)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2002, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2003, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2003, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "Freedom Day"),
            (
                NaiveDate::from_ymd_res(2003, 4, 28)?,
                "Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2003, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2003, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2003, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2004, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Day of Goodwill"),
            (
                NaiveDate::from_ymd_res(2004, 12, 27)?,
                "Day of Goodwill (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 21)?, "Human Rights Day"),
            (
                NaiveDate::from_ymd_res(2004, 3, 22)?,
                "Human Rights Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2004, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2004, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2004, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2004, 4, 14)?,
                "National and provincial government elections",
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
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2005, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2005, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2005, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2005, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2006, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2006, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2006, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2006, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2006, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2006, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2006, 9, 25)?,
                "Heritage Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 1)?,
                "Local government elections",
            ),
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
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2007, 12, 16)?,
                "Day of Reconciliation",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 17)?,
                "Day of Reconciliation (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2007, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2007, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2007, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2007, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2007, 9, 24)?, "Heritage Day"),
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
                NaiveDate::from_ymd_res(2008, 3, 21)?,
                "Good Friday; Human Rights Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2008, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "Freedom Day"),
            (
                NaiveDate::from_ymd_res(2008, 4, 28)?,
                "Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2008, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2008, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2008, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2008, 5, 2)?,
                "Public holiday by presidential decree",
            ),
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
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2009, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2009, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2009, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2009, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2009, 8, 9)?, "National Women's Day"),
            (
                NaiveDate::from_ymd_res(2009, 8, 10)?,
                "National Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2009, 4, 22)?,
                "National and provincial government elections",
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
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2010, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Day of Goodwill"),
            (
                NaiveDate::from_ymd_res(2010, 12, 27)?,
                "Day of Goodwill (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 21)?, "Human Rights Day"),
            (
                NaiveDate::from_ymd_res(2010, 3, 22)?,
                "Human Rights Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2010, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2010, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2010, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2011, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2011, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2011, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2011, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 18)?,
                "Local government elections",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 27)?,
                "Public holiday by presidential decree",
            ),
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
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2012, 12, 16)?,
                "Day of Reconciliation",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 17)?,
                "Day of Reconciliation (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2012, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2012, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2012, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2012, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2012, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2013, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2013, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2013, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2013, 6, 16)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2013, 6, 17)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2013, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2014, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2014, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2014, 4, 27)?, "Freedom Day"),
            (
                NaiveDate::from_ymd_res(2014, 4, 28)?,
                "Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2014, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2014, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2014, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2014, 5, 7)?,
                "National and provincial government elections",
            ),
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
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2015, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2015, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2015, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2015, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2015, 8, 9)?, "National Women's Day"),
            (
                NaiveDate::from_ymd_res(2015, 8, 10)?,
                "National Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2016, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2016, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2016, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2016, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2016, 8, 3)?,
                "Local government elections",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 27)?,
                "Public holiday by presidential decree",
            ),
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
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2017, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2017, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2017, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2017, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2017, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2017, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2017, 9, 25)?,
                "Heritage Day (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2018, 12, 16)?,
                "Day of Reconciliation",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 17)?,
                "Day of Reconciliation (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2018, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2018, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2018, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2018, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2018, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2019, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2019, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2019, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2019, 6, 16)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2019, 6, 17)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2019, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2019, 5, 8)?,
                "National and provincial government elections",
            ),
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
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2020, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2020, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2020, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2020, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2020, 8, 9)?, "National Women's Day"),
            (
                NaiveDate::from_ymd_res(2020, 8, 10)?,
                "National Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2021, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Day of Goodwill"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Day of Goodwill (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 21)?, "Human Rights Day"),
            (
                NaiveDate::from_ymd_res(2021, 3, 22)?,
                "Human Rights Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2021, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2021, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2021, 9, 24)?, "Heritage Day"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Municipal elections"),
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
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2022, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2022, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Workers' Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Workers' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2022, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2022, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 27)?,
                "Public holiday by presidential decree",
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
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "New Year's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2023, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2023, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2023, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2023, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2023, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2023, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2023, 9, 25)?,
                "Heritage Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 15)?,
                "Public holiday by presidential decree",
            ),
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
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2024, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2024, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2024, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2024, 6, 16)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2024, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2024, 5, 29)?,
                "National and provincial government elections",
            ),
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
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2025, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2025, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2025, 4, 27)?, "Freedom Day"),
            (
                NaiveDate::from_ymd_res(2025, 4, 28)?,
                "Freedom Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2025, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2025, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2025, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2026, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2026, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2026, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2026, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2026, 8, 9)?, "National Women's Day"),
            (
                NaiveDate::from_ymd_res(2026, 8, 10)?,
                "National Women's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2027, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Day of Goodwill"),
            (
                NaiveDate::from_ymd_res(2027, 12, 27)?,
                "Day of Goodwill (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 21)?, "Human Rights Day"),
            (
                NaiveDate::from_ymd_res(2027, 3, 22)?,
                "Human Rights Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2027, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2027, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2027, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2028, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2028, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2028, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2028, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2028, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2028, 9, 24)?, "Heritage Day"),
            (
                NaiveDate::from_ymd_res(2028, 9, 25)?,
                "Heritage Day (observed)",
            ),
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
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2029, 12, 16)?,
                "Day of Reconciliation",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 17)?,
                "Day of Reconciliation (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2029, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2029, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2029, 6, 16)?, "Youth Day"),
            (NaiveDate::from_ymd_res(2029, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2029, 9, 24)?, "Heritage Day"),
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
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Family Day"),
            (
                NaiveDate::from_ymd_res(2030, 12, 16)?,
                "Day of Reconciliation",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Day of Goodwill"),
            (NaiveDate::from_ymd_res(2030, 3, 21)?, "Human Rights Day"),
            (NaiveDate::from_ymd_res(2030, 4, 27)?, "Freedom Day"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Workers' Day"),
            (NaiveDate::from_ymd_res(2030, 6, 16)?, "Youth Day"),
            (
                NaiveDate::from_ymd_res(2030, 6, 17)?,
                "Youth Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 9)?, "National Women's Day"),
            (NaiveDate::from_ymd_res(2030, 9, 24)?, "Heritage Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
