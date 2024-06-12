//! Malta
use super::*;

/// Generate holiday map for Malta.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2000, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2000, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2000, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2000, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2000, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2001, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2001, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2001, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2001, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2002, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2002, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2002, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2002, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2003, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2003, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2003, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2003, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2004, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2004, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2004, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2004, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2005, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2005, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2005, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2006, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2006, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2006, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2006, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2007, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2007, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2007, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2007, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2008, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2008, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2008, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2008, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2009, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2009, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2009, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2010, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2010, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2010, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2010, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2011, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2011, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2011, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2011, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2012, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2012, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2012, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2012, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2013, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2013, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2013, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2013, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2014, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2014, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2014, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2014, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2015, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2015, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2015, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2015, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2016, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2016, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2016, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2016, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2017, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2017, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2017, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2017, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2018, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2018, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2018, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2018, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2019, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2019, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2019, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2019, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2020, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2020, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2020, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2020, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2021, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2021, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2021, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2021, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2022, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2022, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2022, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2022, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2023, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2023, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2023, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2024, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2024, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2024, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2024, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2025, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2025, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2025, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2025, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2026, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2026, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2026, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2026, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2027, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2027, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2027, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2027, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2028, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2028, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2028, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2028, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2029, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2029, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2029, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2029, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "L-Ewwel tas-Sena"),
            (
                NaiveDate::from_ymd_res(2030, 2, 10)?,
                "Il-Festa tan-Nawfraġju ta' San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 19)?,
                "Il-Festa ta' San Ġużepp",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 31)?, "Jum il-Ħelsien"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Il-Ġimgħa l-Kbira"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Jum il-Ħaddiem"),
            (NaiveDate::from_ymd_res(2030, 6, 7)?, "Sette Giugno"),
            (
                NaiveDate::from_ymd_res(2030, 6, 29)?,
                "Il-Festa ta' San Pietru u San Pawl",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Il-Festa ta' Santa Marija",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 8)?, "Jum il-Vitorja"),
            (NaiveDate::from_ymd_res(2030, 9, 21)?, "Jum l-Indipendenza"),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "Il-Festa tal-Immakulata Kunċizzjoni",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 13)?, "Jum ir-Repubblika"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Il-Milied"),
        ],
        &mut map,
        Country::MT,
        "Malta",
    );

    Ok(map)
}
