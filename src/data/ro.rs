//! Romania
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Romania";
const COUNTY_CODE: Country = Country::RO;

/// Generate holiday map for Romania.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2000, 4, 30)?, "Paștele"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Paștele; Ziua Muncii"),
            (NaiveDate::from_ymd_res(2000, 6, 18)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2000, 6, 19)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2000, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Paștele"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2001, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2002, 6, 23)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2002, 6, 24)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2002, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "Paștele"),
            (NaiveDate::from_ymd_res(2003, 4, 28)?, "Paștele"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2003, 6, 15)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2003, 6, 16)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2003, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Paștele"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Paștele"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2004, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Paștele; Ziua Muncii"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "Paștele"),
            (NaiveDate::from_ymd_res(2005, 6, 19)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2005, 6, 20)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2005, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "Paștele"),
            (NaiveDate::from_ymd_res(2006, 4, 24)?, "Paștele"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2006, 6, 11)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2006, 6, 12)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2006, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Paștele"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Paștele"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2007, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "Paștele"),
            (NaiveDate::from_ymd_res(2008, 4, 28)?, "Paștele"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2008, 6, 15)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2008, 6, 16)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2008, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "Paștele"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "Paștele"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2009, 6, 7)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2009, 6, 8)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Paștele"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Paștele"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Paștele"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "Paștele"),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2012, 6, 3)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2012, 6, 4)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2013, 6, 23)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2013, 6, 24)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Paștele"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Paștele"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "Paștele"),
            (NaiveDate::from_ymd_res(2015, 4, 13)?, "Paștele"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2015, 5, 31)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2015, 6, 1)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2016, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Paștele; Ziua Muncii"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "Paștele"),
            (NaiveDate::from_ymd_res(2016, 6, 19)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2016, 6, 20)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2017, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Paștele"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2017, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2018, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2018, 4, 8)?, "Paștele"),
            (NaiveDate::from_ymd_res(2018, 4, 9)?, "Paștele"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2018, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2018, 5, 27)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2019, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 26)?, "Paștele"),
            (NaiveDate::from_ymd_res(2019, 4, 28)?, "Paștele"),
            (NaiveDate::from_ymd_res(2019, 4, 29)?, "Paștele"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2019, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2019, 6, 16)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2019, 6, 17)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2020, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 17)?, "Paștele"),
            (NaiveDate::from_ymd_res(2020, 4, 19)?, "Paștele"),
            (NaiveDate::from_ymd_res(2020, 4, 20)?, "Paștele"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2020, 6, 7)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2020, 6, 8)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2021, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 30)?, "Paștele"),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "Paștele"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "Paștele"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2021, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2021, 6, 20)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2021, 6, 21)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2022, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 22)?, "Paștele"),
            (NaiveDate::from_ymd_res(2022, 4, 24)?, "Paștele"),
            (NaiveDate::from_ymd_res(2022, 4, 25)?, "Paștele"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2022, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2022, 6, 12)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2022, 6, 13)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2023, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 14)?, "Paștele"),
            (NaiveDate::from_ymd_res(2023, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2023, 4, 17)?, "Paștele"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2023, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2023, 6, 4)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2023, 6, 5)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2024, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 3)?, "Paștele"),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2024, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2024, 6, 23)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2024, 6, 24)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2025, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Paștele"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Paștele"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Paștele"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2025, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2026, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 10)?, "Paștele"),
            (NaiveDate::from_ymd_res(2026, 4, 12)?, "Paștele"),
            (NaiveDate::from_ymd_res(2026, 4, 13)?, "Paștele"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Ziua Muncii"),
            (
                NaiveDate::from_ymd_res(2026, 6, 1)?,
                "Rusaliile; Ziua Copilului",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 31)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2027, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 30)?, "Paștele"),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "Paștele"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "Paștele"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2027, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2027, 6, 20)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2027, 6, 21)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2028, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Paștele"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Paștele"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2028, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2029, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2029, 4, 8)?, "Paștele"),
            (NaiveDate::from_ymd_res(2029, 4, 9)?, "Paștele"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2029, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2029, 5, 27)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2029, 5, 28)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Anul Nou"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Bobotează"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "Sfântul Ion"),
            (
                NaiveDate::from_ymd_res(2030, 1, 24)?,
                "Ziua Unirii Principatelor Române",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 26)?, "Paștele"),
            (NaiveDate::from_ymd_res(2030, 4, 28)?, "Paștele"),
            (NaiveDate::from_ymd_res(2030, 4, 29)?, "Paștele"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Ziua Muncii"),
            (NaiveDate::from_ymd_res(2030, 6, 1)?, "Ziua Copilului"),
            (NaiveDate::from_ymd_res(2030, 6, 16)?, "Rusaliile"),
            (NaiveDate::from_ymd_res(2030, 6, 17)?, "Rusaliile"),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Adormirea Maicii Domnului",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 30)?,
                "Sfantul Apostol Andrei cel Intai chemat",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 1)?,
                "Ziua Națională a României",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Crăciunul"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Crăciunul"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
