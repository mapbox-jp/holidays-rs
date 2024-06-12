//! Aruba
use super::*;

/// Generate holiday map for Aruba.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2000, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2000, 3, 6)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 29)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2001, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2001, 2, 26)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2002, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2002, 2, 11)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2002, 4, 1)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2002, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2003, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2003, 3, 3)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2003, 4, 21)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2004, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2004, 2, 23)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2005, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2005, 2, 7)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2005, 3, 28)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2006, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2006, 2, 27)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2006, 4, 17)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 29)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2007, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2007, 2, 19)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2008, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2008, 2, 4)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 30)?, "Aña di La Reina"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Dia di Asuncion; Dia di Obrero",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2009, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2009, 2, 23)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2009, 4, 13)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2010, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2011, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2011, 3, 7)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2012, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2012, 2, 20)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2013, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2013, 2, 11)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2013, 4, 30)?, "Aña di La Reina"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2014, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2014, 3, 3)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 26)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2015, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2015, 2, 16)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 27)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2016, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2016, 2, 8)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2016, 3, 28)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 27)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2017, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2017, 2, 27)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 27)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2018, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2018, 2, 12)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 27)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2019, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2019, 3, 4)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2019, 4, 22)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 27)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2020, 2, 24)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2020, 4, 13)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 27)?, "Aña di Rey"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2021, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2021, 4, 5)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2022, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2022, 2, 28)?,
                "Dialuna despues di Carnaval Grandi",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2022, 4, 18)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2023, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2023, 2, 20)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2023, 4, 10)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2024, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2024, 2, 12)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2025, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2025, 3, 3)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2025, 4, 21)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 26)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2026, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2026, 4, 6)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2027, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2027, 3, 29)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2028, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2028, 4, 17)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2029, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2029, 2, 12)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2029, 4, 2)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Aña Nobo"),
            (NaiveDate::from_ymd_res(2030, 1, 25)?, "Dia di Betico"),
            (
                NaiveDate::from_ymd_res(2030, 3, 4)?,
                "Dialuna prome cu diaranson di shinish",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 18)?,
                "Dia di Himno y Bandera",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Bierna Santo"),
            (
                NaiveDate::from_ymd_res(2030, 4, 22)?,
                "Di dos dia di Pasco di Resureccion",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 27)?, "Dia di Rey"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Dia di Obrero"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Dia di Asuncion"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Pasco di Nacemento"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Di dos dia di Pasco di Nacemento",
            ),
        ],
        &mut map,
        Country::AW,
        "Aruba",
    );

    Ok(map)
}
