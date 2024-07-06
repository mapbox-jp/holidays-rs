//! Burundi
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Burundi";
const COUNTY_CODE: Country = Country::BI;

/// Generate holiday map for Burundi.
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
            (NaiveDate::from_ymd_res(2000, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2000, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2000, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2000, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2001, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2001, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2001, 7, 1)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2001, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 22)?,
                "President Ndadaye's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2002, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2002, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2002, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2002, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 14)?,
                "Prince Louis Rwagasore Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2003, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2003, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 7)?,
                "President Ntaryamira Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2003, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2003, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2004, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2004, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2004, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2004, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2004, 8, 16)?,
                "Assumption Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "Eid al Adha (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2005, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2005, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2005, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2005, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 3)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2006, 2, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2006, 2, 6)?, "Unity Day (observed)"),
            (
                NaiveDate::from_ymd_res(2006, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2006, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2006, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2006, 10, 23)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2007, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2007, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2007, 7, 1)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "Eid ul Fitr (estimated); Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 22)?,
                "President Ndadaye's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2008, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2008, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 7)?,
                "President Ntaryamira Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Ascension Day; Labour Day",
            ),
            (NaiveDate::from_ymd_res(2008, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2009, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2009, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2009, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2009, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "All Saints' Day"),
            (
                NaiveDate::from_ymd_res(2009, 11, 2)?,
                "All Saints' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2009, 9, 20)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2010, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2010, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2010, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2010, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2010, 8, 16)?,
                "Assumption Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2010, 9, 10)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2011, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2011, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2011, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2011, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "Eid al Adha (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2012, 2, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2012, 2, 6)?, "Unity Day (observed)"),
            (
                NaiveDate::from_ymd_res(2012, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2012, 7, 1)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2012, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 22)?,
                "President Ndadaye's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2012, 8, 19)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2013, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2013, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2013, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2013, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 14)?,
                "Prince Louis Rwagasore Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2013, 8, 8)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2014, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2014, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 7)?,
                "President Ntaryamira Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2014, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2014, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2015, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2015, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2015, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "All Saints' Day"),
            (
                NaiveDate::from_ymd_res(2015, 11, 2)?,
                "All Saints' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2015, 7, 17)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2016, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2016, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2016, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2016, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 6)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "Eid al Adha (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2017, 2, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2017, 2, 6)?, "Unity Day (observed)"),
            (
                NaiveDate::from_ymd_res(2017, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2017, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2017, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2017, 6, 25)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2018, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2018, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2018, 7, 1)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2018, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 22)?,
                "President Ndadaye's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2018, 6, 15)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2019, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2019, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2019, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 14)?,
                "Prince Louis Rwagasore Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2019, 6, 4)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 11)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "Eid al Adha (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2020, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2020, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Ascension Day"),
            (NaiveDate::from_ymd_res(2020, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2020, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "All Saints' Day"),
            (
                NaiveDate::from_ymd_res(2020, 11, 2)?,
                "All Saints' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2020, 5, 24)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 31)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2021, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2021, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Ascension Day; Eid ul Fitr (estimated)",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2021, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2021, 8, 16)?,
                "Assumption Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2022, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2022, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Eid ul Fitr (estimated); Labour Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2022, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2022, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Christmas Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 9)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2023, 2, 5)?, "Unity Day"),
            (NaiveDate::from_ymd_res(2023, 2, 6)?, "Unity Day (observed)"),
            (
                NaiveDate::from_ymd_res(2023, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2023, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2023, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2023, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2023, 4, 21)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 28)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2024, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2024, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2024, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2024, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2024, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 14)?,
                "Prince Louis Rwagasore Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2024, 4, 10)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Eid al Adha (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2025, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2025, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 7)?,
                "President Ntaryamira Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2025, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 9)?,
                "President Nkurunziza Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2025, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2025, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Eid ul Fitr (estimated) (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2026, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2026, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2026, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2026, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2026, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "All Saints' Day"),
            (
                NaiveDate::from_ymd_res(2026, 11, 2)?,
                "All Saints' Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2027, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2027, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2027, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2027, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2027, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 16)?,
                "Assumption Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Eid al Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Eid al Adha (estimated) (observed)",
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
            (NaiveDate::from_ymd_res(2028, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2028, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2028, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2028, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2028, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Eid al Adha (estimated)",
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
            (NaiveDate::from_ymd_res(2029, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2029, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2029, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2029, 7, 1)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2029, 7, 2)?,
                "Independence Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2029, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 22)?,
                "President Ndadaye's Day (observed)",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Eid al Adha (estimated)",
            ),
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
            (NaiveDate::from_ymd_res(2030, 2, 5)?, "Unity Day"),
            (
                NaiveDate::from_ymd_res(2030, 4, 6)?,
                "President Ntaryamira Day",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Ascension Day"),
            (
                NaiveDate::from_ymd_res(2030, 6, 8)?,
                "President Nkurunziza Day",
            ),
            (NaiveDate::from_ymd_res(2030, 7, 1)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 8, 15)?, "Assumption Day"),
            (
                NaiveDate::from_ymd_res(2030, 10, 13)?,
                "Prince Louis Rwagasore Day",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 14)?,
                "Prince Louis Rwagasore Day (observed)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 21)?,
                "President Ndadaye's Day",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "All Saints' Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Eid ul Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Eid al Adha (estimated)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
