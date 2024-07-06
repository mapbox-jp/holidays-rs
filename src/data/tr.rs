//! Turkey
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Turkey";
const COUNTY_CODE: Country = Country::TR;

/// Generate holiday map for Turkey.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2000, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2000, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2000, 1, 8)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2000, 12, 27)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2000, 1, 9)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2000, 12, 28)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2000, 1, 10)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2000, 12, 29)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2000, 3, 16)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2000, 3, 17)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2000, 3, 18)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2000, 3, 19)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2001, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2001, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2001, 12, 16)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2001, 12, 17)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2001, 12, 18)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2001, 3, 5)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2001, 3, 6)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2001, 3, 7)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2001, 3, 8)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2002, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2002, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2002, 12, 5)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2002, 12, 6)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2002, 12, 7)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2002, 2, 22)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2002, 2, 23)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2002, 2, 24)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2002, 2, 25)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2003, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2003, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2003, 11, 25)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2003, 11, 26)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2003, 11, 27)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2003, 2, 11)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2003, 2, 13)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2003, 2, 14)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2004, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2004, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2004, 11, 15)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2004, 11, 16)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2004, 2, 1)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2004, 2, 2)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2004, 2, 3)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2004, 2, 4)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2005, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2005, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2005, 11, 4)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2005, 11, 5)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2005, 1, 22)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2005, 1, 23)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2006, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2006, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2006, 10, 25)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2006, 1, 11)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2006, 1, 12)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2006, 1, 13)?, "Kurban Bayramı"),
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
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Kurban Bayramı; Yılbaşı",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2007, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2007, 10, 12)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2007, 10, 14)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2007, 12, 21)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2007, 12, 22)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2007, 1, 3)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2007, 12, 23)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2008, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2008, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2008, 9, 30)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2008, 10, 2)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2008, 12, 9)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2008, 12, 10)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2008, 12, 11)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2009, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2009, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2009, 9, 22)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2009, 11, 28)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2009, 11, 29)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2009, 11, 30)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2010, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2010, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2010, 9, 9)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2010, 9, 11)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2010, 11, 16)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2010, 11, 18)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2010, 11, 19)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2011, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "Ramazan Bayramı; Zafer Bayramı",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2011, 9, 1)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2011, 11, 6)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2011, 11, 7)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2011, 11, 8)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2011, 11, 9)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2012, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2012, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2012, 8, 20)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2012, 8, 21)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2012, 10, 25)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2012, 10, 27)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2012, 10, 28)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2013, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2013, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2013, 8, 9)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2013, 8, 10)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2013, 10, 16)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2013, 10, 17)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2013, 10, 18)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2014, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2014, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2014, 7, 29)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2014, 7, 30)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2014, 10, 5)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2014, 10, 6)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2014, 10, 7)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2015, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2015, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2015, 7, 18)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2015, 7, 19)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2015, 9, 25)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2015, 9, 26)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2015, 9, 27)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2016, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2016, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2016, 7, 5)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2016, 7, 7)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2016, 9, 13)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2016, 9, 14)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2016, 9, 15)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2017, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2017, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2017, 6, 26)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2017, 6, 27)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2017, 9, 3)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2017, 9, 4)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2018, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2018, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2018, 6, 16)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2018, 6, 17)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2018, 8, 21)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2018, 8, 23)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2018, 8, 24)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2019, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2019, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2019, 6, 6)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2019, 8, 12)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2019, 8, 13)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2019, 8, 14)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2020, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2020, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2020, 5, 26)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2020, 8, 2)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2020, 8, 3)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2021, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2021, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2021, 5, 14)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2021, 5, 15)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2021, 7, 21)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2021, 7, 22)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2021, 7, 23)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2022, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2022, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2022, 5, 4)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2022, 7, 11)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2022, 7, 12)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2023, 4, 23)?,
                "Ramazan Bayramı; Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2023, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2023, 6, 30)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2023, 7, 1)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2024, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2024, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2024, 4, 11)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2024, 4, 12)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2024, 6, 16)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2024, 6, 17)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2024, 6, 18)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2024, 6, 19)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2025, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2025, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2025, 3, 30)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2025, 3, 31)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2025, 4, 1)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2025, 6, 7)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2026, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2026, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2026, 3, 20)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2026, 3, 21)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2026, 3, 22)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2026, 5, 27)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2026, 5, 28)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2026, 5, 29)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2026, 5, 30)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2027, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı; Kurban Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2027, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 9)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 10)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 11)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2027, 5, 18)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2028, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2028, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2028, 2, 26)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2028, 2, 27)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2028, 2, 28)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2028, 5, 5)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2028, 5, 6)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2028, 5, 7)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2028, 5, 8)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2029, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2029, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2029, 2, 14)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2029, 2, 15)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2029, 2, 16)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2029, 4, 24)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2029, 4, 25)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2029, 4, 26)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2029, 4, 27)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Yılbaşı"),
            (
                NaiveDate::from_ymd_res(2030, 4, 23)?,
                "Ulusal Egemenlik ve Çocuk Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Emek ve Dayanışma Günü",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 19)?,
                "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 15)?,
                "Demokrasi ve Millî Birlik Günü",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 30)?, "Zafer Bayramı"),
            (NaiveDate::from_ymd_res(2030, 10, 29)?, "Cumhuriyet Bayramı"),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2030, 2, 5)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2030, 2, 6)?, "Ramazan Bayramı"),
            (NaiveDate::from_ymd_res(2030, 4, 13)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2030, 4, 14)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2030, 4, 15)?, "Kurban Bayramı"),
            (NaiveDate::from_ymd_res(2030, 4, 16)?, "Kurban Bayramı"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
