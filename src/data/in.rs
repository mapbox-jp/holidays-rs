//! India
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "India";
const COUNTY_CODE: Country = Country::IN;

/// Generate holiday map for India.
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
                NaiveDate::from_ymd_res(2000, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2000, 4, 15)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 14)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 16)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2001, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2001, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2001, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 11, 14)?, "Diwali"),
            (NaiveDate::from_ymd_res(2001, 3, 10)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2001, 4, 4)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 6)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 8)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2002, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2002, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2002, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 11, 4)?, "Diwali"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Good Friday; Holi"),
            (
                NaiveDate::from_ymd_res(2002, 3, 24)?,
                "Day of Ashura (estimated); Palm Sunday",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 24)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2003, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2003, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 10, 25)?, "Diwali"),
            (NaiveDate::from_ymd_res(2003, 3, 18)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2003, 3, 13)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 13)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 12)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 13)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2004, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2004, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2004, 10, 2)?, "Gandhi Jayanti"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Labour Day; Mawlid (estimated)",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 12)?, "Diwali"),
            (NaiveDate::from_ymd_res(2004, 3, 7)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2004, 3, 1)?,
                "Day of Ashura (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 4)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2005, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Diwali"),
            (NaiveDate::from_ymd_res(2005, 3, 26)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2005, 2, 19)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 21)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2005, 11, 3)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 22)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 20)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2006, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 10, 21)?, "Diwali"),
            (NaiveDate::from_ymd_res(2006, 3, 15)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2006, 2, 9)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 10)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2006, 10, 23)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 11)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 9)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2007, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2007, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2007, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 11, 9)?, "Diwali"),
            (NaiveDate::from_ymd_res(2007, 3, 4)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2007, 1, 29)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 21)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 1)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2008, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2008, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "Eid ul-Fitr (estimated); Gandhi Jayanti",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 10, 28)?, "Diwali"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2008, 1, 19)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 9)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 16)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2009, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2009, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 10, 17)?, "Diwali"),
            (NaiveDate::from_ymd_res(2009, 3, 11)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2009, 1, 7)?,
                "Day of Ashura (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 27)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 9)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2009, 9, 20)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 28)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 5)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2010, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2010, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2010, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2010, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 11, 5)?, "Diwali"),
            (NaiveDate::from_ymd_res(2010, 3, 1)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2010, 12, 16)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2010, 2, 26)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2010, 9, 10)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 17)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 28)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2011, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2011, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2011, 10, 26)?, "Diwali"),
            (NaiveDate::from_ymd_res(2011, 3, 20)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2011, 12, 5)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2011, 2, 15)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 17)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2012, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2012, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 11, 13)?, "Diwali"),
            (NaiveDate::from_ymd_res(2012, 3, 8)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2012, 11, 24)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2012, 2, 4)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2012, 8, 19)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 27)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 1)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2013, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2013, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2013, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 11, 3)?, "Diwali"),
            (NaiveDate::from_ymd_res(2013, 3, 27)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2013, 11, 13)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2013, 1, 24)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2013, 8, 8)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 16)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 24)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2014, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 10, 23)?, "Diwali"),
            (NaiveDate::from_ymd_res(2014, 3, 17)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2014, 11, 3)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2014, 1, 13)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 5)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 13)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2015, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2015, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2015, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 11, 11)?, "Diwali"),
            (NaiveDate::from_ymd_res(2015, 3, 6)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2015, 10, 23)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2015, 1, 3)?, "Mawlid (estimated)"),
            (NaiveDate::from_ymd_res(2015, 12, 23)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2015, 7, 17)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 24)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 29)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2016, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2016, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2016, 10, 30)?, "Diwali"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2016, 10, 11)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 11)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2016, 7, 6)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 20)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2017, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 10, 19)?, "Diwali"),
            (NaiveDate::from_ymd_res(2017, 3, 13)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2017, 9, 30)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 30)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2017, 6, 25)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 2)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 9)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2018, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2018, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2018, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 11, 7)?, "Diwali"),
            (NaiveDate::from_ymd_res(2018, 3, 2)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2018, 9, 20)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 20)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2018, 6, 15)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 22)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 25)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2019, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2019, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2019, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2019, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2019, 10, 27)?, "Diwali"),
            (NaiveDate::from_ymd_res(2019, 3, 21)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2019, 9, 9)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 9)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2019, 6, 4)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 5)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 11)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 14)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2020, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2020, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2020, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 11, 14)?, "Diwali"),
            (NaiveDate::from_ymd_res(2020, 3, 10)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2020, 8, 29)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 29)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2020, 5, 24)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 31)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 1)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 5)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2021, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2021, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2021, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2021, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 11, 4)?, "Diwali"),
            (NaiveDate::from_ymd_res(2021, 3, 29)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2021, 8, 18)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 18)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 28)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2022, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2022, 10, 24)?, "Diwali"),
            (NaiveDate::from_ymd_res(2022, 3, 18)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2022, 8, 8)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 8)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 9)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 10)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 10)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2023, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 11, 12)?, "Diwali"),
            (NaiveDate::from_ymd_res(2023, 3, 8)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2023, 7, 28)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 27)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2023, 4, 21)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 22)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 28)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 2)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2024, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2024, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2024, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Diwali"),
            (NaiveDate::from_ymd_res(2024, 3, 25)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2024, 7, 16)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 15)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2024, 4, 10)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 24)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2025, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2025, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 10, 20)?, "Diwali"),
            (NaiveDate::from_ymd_res(2025, 3, 14)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2025, 7, 5)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 4)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 13)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2026, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2026, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2026, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 11, 8)?, "Diwali"),
            (NaiveDate::from_ymd_res(2026, 3, 4)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2026, 6, 25)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 25)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 29)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2027, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2027, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2027, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2027, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2027, 10, 29)?, "Diwali"),
            (NaiveDate::from_ymd_res(2027, 3, 22)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2027, 6, 15)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 14)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Eid al-Adha (estimated); Feast of Pentecost",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 21)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2028, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 10, 17)?, "Diwali"),
            (NaiveDate::from_ymd_res(2028, 3, 11)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2028, 6, 3)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 3)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 9)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2029, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2029, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2029, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 11, 5)?, "Diwali"),
            (NaiveDate::from_ymd_res(2029, 3, 1)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2029, 5, 23)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2029, 7, 24)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "Eid al-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 25)?, "Palm Sunday"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Christmas Day"),
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
                NaiveDate::from_ymd_res(2030, 1, 14)?,
                "Makar Sankranti / Pongal",
            ),
            (NaiveDate::from_ymd_res(2030, 1, 26)?, "Republic Day"),
            (NaiveDate::from_ymd_res(2030, 8, 15)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 10, 2)?, "Gandhi Jayanti"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 10, 26)?, "Diwali"),
            (NaiveDate::from_ymd_res(2030, 3, 20)?, "Holi"),
            (
                NaiveDate::from_ymd_res(2030, 5, 12)?,
                "Day of Ashura (estimated)",
            ),
            (NaiveDate::from_ymd_res(2030, 7, 13)?, "Mawlid (estimated)"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Eid ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Eid al-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "Eid al-Adha (estimated); Palm Sunday",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Good Friday"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Easter Sunday"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Feast of Pentecost"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Christmas Day"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
