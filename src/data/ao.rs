//! Angola
use super::*;

/// Generate holiday map for Angola.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2000, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 7)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2000, 9, 18)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2000, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2001, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2001, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (
                NaiveDate::from_ymd_res(2001, 2, 5)?,
                "Dia do Início da Luta Armada (ponte)",
            ),
            (NaiveDate::from_ymd_res(2001, 2, 27)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2001, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2001, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2001, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 12)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2002, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2002, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2002, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2002, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2003, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 4)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2003, 5, 26)?,
                "Dia da África (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 2)?,
                "Dia Internacional da Criança (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2003, 11, 3)?,
                "Dia dos Finados (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2004, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 5)?,
                "Dia dos Mártires da Repressão Colonial (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2004, 2, 24)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 5)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2004, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2004, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2005, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2005, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Dia Internacional do Trabalhador (ponte)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2005, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2005, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Dia do Natal"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "Dia do Natal (ponte)",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Dia do Ano Novo (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2006, 2, 28)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2006, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2006, 9, 18)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2006, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2007, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 5)?,
                "Dia do Início da Luta Armada (ponte)",
            ),
            (NaiveDate::from_ymd_res(2007, 2, 20)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2007, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2007, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2007, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 12)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2008, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2008, 2, 5)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2008, 5, 26)?,
                "Dia da África (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 2)?,
                "Dia Internacional da Criança (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2008, 11, 3)?,
                "Dia dos Finados (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2009, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 5)?,
                "Dia dos Mártires da Repressão Colonial (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2009, 2, 24)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "Dia Internacional da Mulher (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2009, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2009, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2010, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (NaiveDate::from_ymd_res(2010, 2, 16)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 25)?, "Dia da África"),
            (
                NaiveDate::from_ymd_res(2010, 6, 1)?,
                "Dia Internacional da Criança",
            ),
            (
                NaiveDate::from_ymd_res(2010, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2010, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Dia do Natal"),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2011, 1, 4)?,
                "Dia dos Mártires da Repressão Colonial",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 4)?,
                "Dia do Início da Luta Armada",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "Dia Internacional da Mulher; Dia do Carnaval",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Dia Internacional do Trabalhador (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2011, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2012, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2012, 2, 21)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2012, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2012, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 12)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2013, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2013, 2, 12)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2013, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2013, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2014, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 4)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2014, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2014, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2015, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2015, 2, 17)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 9)?,
                "Dia Internacional da Mulher (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2015, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2015, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2016, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2016, 2, 9)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2016, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2016, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Dia Internacional do Trabalhador (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2016, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2017, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2017, 2, 28)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2017, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 18)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2017, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Dia de Natal e da Família",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 23)?,
                "Dia de eleições gerais",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Dia do Ano Novo (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 5)?,
                "Dia do Início da Luta Armada de Libertação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2018, 2, 13)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2018, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2018, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Dia de Natal e da Família",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Dia de Natal e da Família (ponte)",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2019, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 5)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2019, 3, 4)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 5)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 16)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2019, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2020, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 2, 3)?,
                "Dia do Início da Luta Armada de Libertação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2020, 2, 25)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2020, 2, 24)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 9, 18)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2020, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2021, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 5)?,
                "Dia do Início da Luta Armada de Libertação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2021, 2, 16)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 22)?,
                "Dia da Libertação da África Austral (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2021, 11, 1)?,
                "Dia dos Finados (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 12)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2022, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 1)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2022, 2, 28)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 7)?,
                "Dia Internacional da Mulher (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2022, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2023, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2023, 2, 21)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2023, 2, 20)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 24)?,
                "Dia da Libertação da África Austral (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 3)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2023, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2023, 11, 3)?,
                "Dia dos Finados (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2024, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2024, 2, 13)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2024, 2, 12)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 5)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 16)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2024, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2025, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 3)?,
                "Dia do Início da Luta Armada de Libertação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 4)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2025, 3, 3)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 2)?,
                "Dia Internacional do Trabalhador (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2025, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 10)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 25)?,
                "Dia de Natal e da Família",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "Dia de Natal e da Família (ponte)",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2026, 1, 2)?,
                "Dia do Ano Novo (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2026, 2, 17)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 9, 18)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2026, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2027, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 5)?,
                "Dia do Início da Luta Armada de Libertação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2027, 2, 9)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 22)?,
                "Dia da Libertação da África Austral (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2027, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2027, 11, 1)?,
                "Dia dos Finados (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 12)?,
                "Dia da Independência Nacional (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2028, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2028, 2, 29)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2028, 3, 24)?,
                "Dia da Libertação da África Austral (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 3)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2028, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2028, 11, 3)?,
                "Dia dos Finados (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2029, 12, 31)?,
                "Dia do Ano Novo (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2029, 2, 13)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2029, 2, 12)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 9)?,
                "Dia Internacional da Mulher (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 30)?,
                "Dia Internacional do Trabalhador (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2029, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 25)?,
                "Dia de Natal e da Família",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 24)?,
                "Dia de Natal e da Família (ponte)",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Dia do Ano Novo"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Dia do Início da Luta Armada de Libertação Nacional",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 5)?, "Dia do Carnaval"),
            (
                NaiveDate::from_ymd_res(2030, 3, 4)?,
                "Dia do Carnaval (ponte)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 8)?,
                "Dia Internacional da Mulher",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 23)?,
                "Dia da Libertação da África Austral",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 4)?,
                "Dia da Paz e Reconciliação Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 5)?,
                "Dia da Paz e Reconciliação Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Sexta-Feira Santa"),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Dia Internacional do Trabalhador",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 17)?,
                "Dia do Fundador da Nação e do Herói Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 16)?,
                "Dia do Fundador da Nação e do Herói Nacional (ponte)",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 2)?, "Dia dos Finados"),
            (
                NaiveDate::from_ymd_res(2030, 11, 11)?,
                "Dia da Independência Nacional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 25)?,
                "Dia de Natal e da Família",
            ),
        ],
        &mut map,
        Country::AO,
        "Angola",
    );

    Ok(map)
}
