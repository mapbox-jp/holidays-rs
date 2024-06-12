//! Slovakia
use super::*;

/// Generate holiday map for Slovakia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (
                NaiveDate::from_ymd_res(2000, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2000, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2000, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2000, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2001, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2001, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2002, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2002, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2002, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2002, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2003, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2003, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2003, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2004, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2004, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2004, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2005, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2005, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2005, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2006, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2006, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2006, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2007, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2007, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2007, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2007, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2008, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2008, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2009, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2009, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2009, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2010, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2010, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2011, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2011, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2011, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2012, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2012, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2013, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2013, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2014, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2015, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2016, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2016, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2016, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2016, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2017, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2017, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2018, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2018, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 30)?,
                "100. výročie prijatia Deklarácie slovenského národa",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2019, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2019, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2019, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2020, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2020, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2021, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2021, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2022, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2022, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2023, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2023, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2023, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2024, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2024, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2024, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2025, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2025, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2025, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2026, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2026, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2026, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2026, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2027, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2027, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2027, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2027, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2028, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2028, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2028, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2029, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2029, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2029, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 1, 1)?,
                "Deň vzniku Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2030, 1, 6)?,
                "Zjavenie Pána (Traja králi a vianočný sviatok pravoslávnych kresťanov)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Veľký piatok"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Veľkonočný pondelok"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Sviatok práce"),
            (
                NaiveDate::from_ymd_res(2030, 5, 8)?,
                "Deň víťazstva nad fašizmom",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 5)?,
                "Sviatok svätého Cyrila a svätého Metoda",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 29)?,
                "Výročie Slovenského národného povstania",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 1)?,
                "Deň Ústavy Slovenskej republiky",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 15)?,
                "Sedembolestná Panna Mária",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 1)?,
                "Sviatok Všetkých svätých",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 17)?,
                "Deň boja za slobodu a demokraciu",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 24)?, "Štedrý deň"),
            (
                NaiveDate::from_ymd_res(2030, 12, 25)?,
                "Prvý sviatok vianočný",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "Druhý sviatok vianočný",
            ),
        ],
        &mut map,
        Country::SK,
        "Slovakia",
    );

    Ok(map)
}
