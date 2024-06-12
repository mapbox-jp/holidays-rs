//! Lithuania
use super::*;

/// Generate holiday map for Lithuania.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2000, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 7)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2000, 6, 4)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2000, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2000, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2001, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 6)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2001, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2001, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2002, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2002, 4, 1)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2002, 6, 2)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2002, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2002, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2002, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2003, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2003, 4, 21)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 4)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2003, 6, 1)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2003, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2003, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2003, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2004, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 2)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2004, 6, 6)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2004, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2004, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2004, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2005, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2005, 3, 28)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Motinos diena; Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 5)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2005, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2005, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2006, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2006, 4, 17)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 7)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2006, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2006, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2007, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 6)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2007, 6, 3)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2007, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2007, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2008, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 4)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2008, 6, 1)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2008, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2008, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2009, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2009, 4, 13)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 3)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2009, 6, 7)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2009, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2009, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2010, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 2)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2010, 6, 6)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2010, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2010, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2010, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2011, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Motinos diena; Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 5)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2011, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2011, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2012, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 6)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2012, 6, 3)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2012, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2012, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2013, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2013, 6, 2)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2013, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2014, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 4)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2014, 6, 1)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2014, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2015, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 3)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2015, 6, 7)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2015, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2016, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2016, 3, 28)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Motinos diena; Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 5)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2016, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2016, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2017, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 7)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2017, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2018, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 6)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2018, 6, 3)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2018, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2019, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2019, 4, 22)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 5)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2019, 6, 2)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2019, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Visų Šventųjų diena"),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2020, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2020, 4, 13)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 3)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2020, 6, 7)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2020, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2020, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2021, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2021, 4, 5)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2021, 6, 6)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2021, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2021, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2022, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2022, 4, 18)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Motinos diena; Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2022, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2022, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2022, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2023, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2023, 4, 10)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 7)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2023, 6, 4)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2023, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2023, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2023, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2024, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2024, 6, 2)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2024, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2024, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2024, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2025, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2025, 4, 21)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 4)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2025, 6, 1)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2025, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2025, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2025, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2026, 4, 6)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 3)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2026, 6, 7)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2026, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2026, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2026, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2027, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2027, 3, 29)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2027, 6, 6)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2027, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2027, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2028, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2028, 4, 17)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 7)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2028, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2028, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2028, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2029, 4, 2)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 6)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2029, 6, 3)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2029, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2029, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2029, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Naujųjų metų diena"),
            (
                NaiveDate::from_ymd_res(2030, 2, 16)?,
                "Lietuvos valstybės atkūrimo diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 11)?,
                "Lietuvos nepriklausomybės atkūrimo diena",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Šv. Velykos"),
            (
                NaiveDate::from_ymd_res(2030, 4, 22)?,
                "Antroji šv. Velykų diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Tarptautinė darbo diena",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 5)?, "Motinos diena"),
            (NaiveDate::from_ymd_res(2030, 6, 2)?, "Tėvo diena"),
            (
                NaiveDate::from_ymd_res(2030, 6, 24)?,
                "Rasos ir Joninių diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 6)?,
                "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) ir Tautiškos giesmės diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Visų Šventųjų diena"),
            (
                NaiveDate::from_ymd_res(2030, 11, 2)?,
                "Mirusiųjų atminimo (Vėlinių) diena",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Kūčių diena"),
            (
                NaiveDate::from_ymd_res(2030, 12, 25)?,
                "Šv. Kalėdų pirma diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Šv. Kalėdų antra diena",
            ),
        ],
        &mut map,
        Country::LT,
        "Lithuania",
    );

    Ok(map)
}
