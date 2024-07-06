//! Bosnia and Herzegovina
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Bosnia and Herzegovina";
const COUNTY_CODE: Country = Country::BA;

/// Generate holiday map for Bosnia and Herzegovina.
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
            (
                NaiveDate::from_ymd_res(2000, 4, 28)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2000, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (
                NaiveDate::from_ymd_res(2001, 4, 13)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 17)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2001, 3, 6)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2001, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (
                NaiveDate::from_ymd_res(2002, 5, 3)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 4, 1)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 6)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2002, 2, 23)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2002, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (
                NaiveDate::from_ymd_res(2003, 4, 25)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 21)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 26)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2003, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (
                NaiveDate::from_ymd_res(2004, 4, 9)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2004, 2, 2)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2004, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (
                NaiveDate::from_ymd_res(2005, 4, 29)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 28)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 4)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2005, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (
                NaiveDate::from_ymd_res(2006, 4, 21)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 17)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2006, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (
                NaiveDate::from_ymd_res(2007, 4, 6)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2007, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (
                NaiveDate::from_ymd_res(2008, 4, 25)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 2)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2008, 12, 9)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (
                NaiveDate::from_ymd_res(2009, 4, 17)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 13)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2009, 11, 28)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (
                NaiveDate::from_ymd_res(2010, 4, 2)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (
                NaiveDate::from_ymd_res(2011, 4, 22)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2011, 11, 7)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (
                NaiveDate::from_ymd_res(2012, 4, 13)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (
                NaiveDate::from_ymd_res(2013, 5, 3)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (
                NaiveDate::from_ymd_res(2014, 4, 18)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (
                NaiveDate::from_ymd_res(2015, 4, 10)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 18)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (
                NaiveDate::from_ymd_res(2016, 4, 29)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 28)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 7)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2016, 9, 13)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (
                NaiveDate::from_ymd_res(2017, 4, 14)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 26)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (
                NaiveDate::from_ymd_res(2018, 4, 6)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (
                NaiveDate::from_ymd_res(2019, 4, 26)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 22)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (
                NaiveDate::from_ymd_res(2020, 4, 17)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 13)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (
                NaiveDate::from_ymd_res(2021, 4, 30)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 5)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (
                NaiveDate::from_ymd_res(2022, 4, 22)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 18)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Međunarodni praznik rada; Ramazanski Bajram",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (
                NaiveDate::from_ymd_res(2023, 4, 14)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 10)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "Ramazanski Bajram"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "Kurban Bajram"),
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (
                NaiveDate::from_ymd_res(2024, 5, 3)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Ramazanski Bajram"),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (
                NaiveDate::from_ymd_res(2025, 4, 18)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 21)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (
                NaiveDate::from_ymd_res(2026, 4, 10)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 6)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (
                NaiveDate::from_ymd_res(2027, 4, 30)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 29)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (
                NaiveDate::from_ymd_res(2028, 4, 14)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 17)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (
                NaiveDate::from_ymd_res(2029, 4, 6)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 2)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (
                NaiveDate::from_ymd_res(2030, 4, 26)?,
                "Veliki petak (Pravoslavni)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 22)?,
                "Uskrsni ponedjeljak (Katolički)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Ramazanski Bajram (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Kurban Bajram (estimated)",
            ),
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "Božić (Pravoslavni)"),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Međunarodni praznik rada",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 2)?,
                "Međunarodni praznik rada",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Božić (Katolički)"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
