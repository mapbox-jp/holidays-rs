//! Mozambique
use super::*;

/// Generate holiday map for Mozambique.
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
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 26)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2000, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2000, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2001, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2001, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 4)?,
                "Dia dos Heróis Moçambicanos (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2002, 4, 8)?,
                "Dia da Mulher Moçambicana (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2002, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2003, 9, 8)?,
                "Dia da Vitória (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2004, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Dia Internacional dos Trabalhadores (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2005, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 26)?,
                "Dia das Forças Armadas de Libertação Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Dia da Família"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Dia da Família (ponte)",
            ),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Dia da Fraternidade universal (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 26)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2006, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2007, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 4)?,
                "Dia dos Heróis Moçambicanos (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2008, 9, 8)?,
                "Dia da Vitória (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2009, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2009, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 5)?,
                "Dia da Paz e Reconciliação (ponte)",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2010, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Dia Internacional dos Trabalhadores (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2011, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 26)?,
                "Dia das Forças Armadas de Libertação Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Dia da Família"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "Dia da Família (ponte)",
            ),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 2)?,
                "Dia da Fraternidade universal (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2012, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 4)?,
                "Dia dos Heróis Moçambicanos (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 8)?,
                "Dia da Mulher Moçambicana (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2013, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2014, 9, 8)?,
                "Dia da Vitória (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2015, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 5)?,
                "Dia da Paz e Reconciliação (ponte)",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2016, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Dia Internacional dos Trabalhadores (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2016, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 26)?,
                "Dia das Forças Armadas de Libertação Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Dia da Família"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "Dia da Família (ponte)",
            ),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "Dia da Fraternidade universal (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2017, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2018, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 4)?,
                "Dia dos Heróis Moçambicanos (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 8)?,
                "Dia da Mulher Moçambicana (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2019, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2020, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2020, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 5)?,
                "Dia da Paz e Reconciliação (ponte)",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2021, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Dia Internacional dos Trabalhadores (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2022, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 26)?,
                "Dia das Forças Armadas de Libertação Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Dia da Família"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "Dia da Família (ponte)",
            ),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "Dia da Fraternidade universal (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 26)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2023, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 8)?,
                "Dia da Mulher Moçambicana (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2024, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2025, 9, 8)?,
                "Dia da Vitória (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2026, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 5)?,
                "Dia da Paz e Reconciliação (ponte)",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2027, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2027, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 26)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2028, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2029, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 1, 1)?,
                "Dia da Fraternidade universal",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 3)?,
                "Dia dos Heróis Moçambicanos",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Dia dos Heróis Moçambicanos (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 7)?,
                "Dia da Mulher Moçambicana",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 8)?,
                "Dia da Mulher Moçambicana (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Dia Internacional dos Trabalhadores",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 25)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 7)?, "Dia da Vitória"),
            (
                NaiveDate::from_ymd_res(2030, 9, 25)?,
                "Dia das Forças Armadas de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 4)?,
                "Dia da Paz e Reconciliação",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Dia da Família"),
        ],
        &mut map,
        Country::MZ,
        "Mozambique",
    );

    Ok(map)
}
