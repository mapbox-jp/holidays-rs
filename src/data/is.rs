//! Iceland
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Iceland";
const COUNTY_CODE: Country = Country::IS;

/// Generate holiday map for Iceland.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Nýársdagur"),
            (
                NaiveDate::from_ymd_res(2000, 4, 20)?,
                "Skírdagur; Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Annar í páskum"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2000, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2000, 8, 7)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2000, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2001, 4, 19)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2001, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2001, 8, 6)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2001, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2002, 4, 25)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2002, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2002, 8, 5)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2002, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2003, 4, 24)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2003, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2003, 8, 4)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2003, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2004, 4, 22)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2004, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2004, 8, 2)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2004, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2005, 5, 16)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2005, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2005, 8, 1)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2005, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2006, 4, 20)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2006, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2006, 8, 7)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2007, 4, 19)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2007, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2007, 8, 6)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2007, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2008, 4, 24)?,
                "Sumardagurinn fyrsti",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Uppstigningardagur; Verkalýðsdagurinn",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2008, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2008, 8, 4)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2008, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2009, 4, 23)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2009, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2009, 8, 3)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2009, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2010, 4, 22)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2010, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2010, 8, 2)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2010, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nýársdagur"),
            (
                NaiveDate::from_ymd_res(2011, 4, 21)?,
                "Skírdagur; Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Annar í páskum"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2011, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2011, 8, 1)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2011, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2012, 4, 19)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2012, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2012, 8, 6)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2012, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2013, 4, 25)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2013, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2013, 8, 5)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2013, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2014, 4, 24)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2014, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2014, 8, 4)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2014, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2015, 4, 23)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2015, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2015, 8, 3)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2015, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2016, 4, 21)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2016, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2016, 8, 1)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2016, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2017, 4, 20)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2017, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2017, 8, 7)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2017, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2018, 4, 19)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2018, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2018, 8, 6)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2018, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2019, 4, 25)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2019, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2019, 8, 5)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2019, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2020, 4, 23)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2020, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2020, 8, 3)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2020, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2021, 4, 22)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2021, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2021, 8, 2)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2021, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2022, 4, 21)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2022, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2022, 8, 1)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2022, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2023, 4, 20)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2023, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2023, 8, 7)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2023, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2024, 4, 25)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2024, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2024, 8, 5)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2024, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2025, 4, 24)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2025, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2025, 8, 4)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2025, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2026, 4, 23)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2026, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2026, 8, 3)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2026, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2027, 4, 22)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2027, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2027, 8, 2)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2027, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2028, 4, 20)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2028, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2028, 8, 7)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2028, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2029, 4, 19)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2029, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2029, 8, 6)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2029, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nýársdagur"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Skírdagur"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Föstudagurinn langi"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Páskadagur"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Annar í páskum"),
            (
                NaiveDate::from_ymd_res(2030, 4, 25)?,
                "Sumardagurinn fyrsti",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Verkalýðsdagurinn"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Uppstigningardagur"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Hvítasunnudagur"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Annar í hvítasunnu"),
            (NaiveDate::from_ymd_res(2030, 6, 17)?, "Þjóðhátíðardagurinn"),
            (
                NaiveDate::from_ymd_res(2030, 8, 5)?,
                "Frídagur verslunarmanna",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Aðfangadagur"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Jóladagur"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Annar í jólum"),
            (NaiveDate::from_ymd_res(2030, 12, 31)?, "Gamlársdagur"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
