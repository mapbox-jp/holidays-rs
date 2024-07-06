//! Curaçao
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Curaçao";
const COUNTY_CODE: Country = Country::CW;

/// Generate holiday map for Curaçao.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2000, 3, 6)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2000, 4, 23)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 29)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2000, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2001, 2, 26)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2001, 4, 15)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2002, 2, 11)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2002, 3, 31)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2002, 4, 1)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2002, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2002, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2003, 3, 3)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2003, 4, 20)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 21)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2003, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2004, 2, 23)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2004, 4, 11)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2004, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2005, 2, 7)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2005, 3, 27)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 28)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2005, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2006, 2, 27)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2006, 4, 16)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 17)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 29)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2006, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2007, 2, 19)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2008, 2, 4)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2008, 3, 23)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 30)?, "Dia di la Reina"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Dia di Asenshon; Dia di Obrero",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2009, 2, 23)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2009, 4, 12)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 13)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2009, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2010, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2011, 3, 7)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2011, 4, 24)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2011, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2012, 2, 20)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2013, 2, 11)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2013, 3, 31)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 30)?, "Dia di la Reina"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2013, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2014, 3, 3)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2014, 4, 20)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 26)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2014, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2015, 2, 16)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2015, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2016, 2, 8)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2016, 3, 27)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 28)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2016, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2017, 2, 27)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2017, 4, 16)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2017, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2018, 2, 12)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2019, 3, 4)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2019, 4, 21)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 22)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2019, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2020, 2, 24)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2020, 4, 12)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 13)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2020, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2021, 4, 5)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2021, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2022, 2, 28)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2022, 4, 17)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 18)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2022, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2023, 2, 20)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2023, 4, 10)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2023, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2024, 2, 12)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2024, 3, 31)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2024, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2025, 3, 3)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2025, 4, 20)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 21)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 26)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2025, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2026, 4, 6)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2026, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2027, 3, 28)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 29)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2027, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2028, 4, 16)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 17)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2028, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2029, 2, 12)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Bièrnèsantu"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Pasku di Resurekshon"),
            (
                NaiveDate::from_ymd_res(2029, 4, 2)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2029, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
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
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Aña Nobo"),
            (
                NaiveDate::from_ymd_res(2030, 3, 4)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Bièrnèsantu"),
            (
                NaiveDate::from_ymd_res(2030, 4, 21)?,
                "Pasku di Resurekshon",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 22)?,
                "Di dos dia di Pasku di Resurekshon",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Dia di Asenshon"),
            (
                NaiveDate::from_ymd_res(2030, 7, 2)?,
                "Dia di Himno i Bandera",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 10)?, "Dia di Pais Kòrsou"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Pasku di Nasementu"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Di dos dia di Pasku di Nasementu",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
