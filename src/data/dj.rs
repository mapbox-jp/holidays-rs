//! Djibouti
use super::*;

/// Generate holiday map for Djibouti.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2000, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2000, 10, 24)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 8)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 15)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 6)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2001, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2001, 10, 14)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 4)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2001, 3, 5)?, "Eid al-Adha (estimé)"),
            (
                NaiveDate::from_ymd_res(2001, 3, 6)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 26)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2002, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2002, 10, 4)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 21)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 15)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2003, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2003, 9, 24)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 10)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 12)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 4)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nouvel an"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Anniversaire du prophète Muhammad (estimé); Fête du travail",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2004, 9, 12)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2004, 1, 31)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2004, 2, 1)?, "Eid al-Adha (estimé)"),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 21)?,
                "Nouvel an musulman (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2005, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2005, 9, 1)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 3)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 22)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2006, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2006, 8, 21)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 23)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 9)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2006, 12, 30)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2006, 1, 10)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 11)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 31)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 10)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Eid al-Adha deuxième jour (estimé); Nouvel an",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2007, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2007, 8, 10)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 19)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2007, 12, 20)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 21)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 20)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2008, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2008, 7, 30)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 7)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 9)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 10)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 29)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2009, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2009, 7, 20)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 20)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 26)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2009, 11, 27)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 28)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 18)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2010, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2010, 7, 9)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 10)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 15)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2010, 11, 16)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 17)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 7)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2011, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2011, 6, 29)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 30)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 5)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2011, 11, 6)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 26)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2012, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2012, 6, 17)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 19)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 25)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2012, 10, 26)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 27)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 15)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 4)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2013, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2013, 6, 6)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2013, 10, 15)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 16)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 4)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2014, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2014, 5, 26)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 28)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 3)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 5)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 25)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 13)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2015, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2015, 5, 16)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 17)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 22)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2015, 9, 23)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 24)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 14)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 3)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 23)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2016, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2016, 5, 4)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 10)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2016, 9, 11)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 12)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 2)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 11)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2017, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2017, 4, 24)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 25)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 31)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "Eid al-Adha (estimé)"),
            (
                NaiveDate::from_ymd_res(2017, 9, 2)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 21)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 30)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2018, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2018, 4, 13)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 15)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 20)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2018, 8, 21)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 22)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 11)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 20)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2019, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2019, 4, 3)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 4)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2019, 6, 5)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 10)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2019, 8, 11)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 31)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2020, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2020, 3, 22)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 24)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 30)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2020, 7, 31)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 1)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 20)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 29)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2021, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2021, 3, 11)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 19)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2021, 7, 20)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 9)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 18)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2022, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2022, 2, 28)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 8)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "Eid al-Adha (estimé)"),
            (
                NaiveDate::from_ymd_res(2022, 7, 10)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 30)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 8)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2023, 6, 27)?,
                "Arafat (estimé); Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 28)?,
                "Eid al-Adha (estimé); Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2023, 2, 18)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 21)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 22)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 19)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 27)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2024, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2024, 2, 8)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 10)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 15)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 7)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2025, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2025, 1, 27)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 5)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2025, 6, 6)?, "Eid al-Adha (estimé)"),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2026, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2026, 1, 16)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 26)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 16)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2027, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Al Isra et Al Mirague (estimé); Noël",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 5)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 9)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 15)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 6)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2028, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2028, 12, 14)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 4)?, "Arafat (estimé)"),
            (NaiveDate::from_ymd_res(2028, 5, 5)?, "Eid al-Adha (estimé)"),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2029, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2029, 12, 3)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Eid al-Fitr (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 23)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nouvel an"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Fête du travail"),
            (
                NaiveDate::from_ymd_res(2030, 6, 27)?,
                "Fête de l'indépendance",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 28)?,
                "Fête de l'indépendance deuxième jour",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Noël"),
            (
                NaiveDate::from_ymd_res(2030, 11, 23)?,
                "Al Isra et Al Mirague (estimé)",
            ),
            (NaiveDate::from_ymd_res(2030, 2, 4)?, "Eid al-Fitr (estimé)"),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Eid al-Fitr deuxième jour (estimé)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 12)?, "Arafat (estimé)"),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Eid al-Adha (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "Eid al-Adha deuxième jour (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 3)?,
                "Nouvel an musulman (estimé)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "Anniversaire du prophète Muhammad (estimé)",
            ),
        ],
        &mut map,
        Country::DJ,
        "Djibouti",
    );

    Ok(map)
}
