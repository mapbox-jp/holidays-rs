//! Moldova
use super::*;

/// Generate holiday map for Moldova.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2000, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 30)?, "Paștele"),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Paștele; Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 8)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2000, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2001, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Paștele"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2001, 4, 23)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2002, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2002, 5, 13)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2003, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "Paștele"),
            (NaiveDate::from_ymd_res(2003, 4, 28)?, "Paștele"),
            (NaiveDate::from_ymd_res(2003, 5, 5)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2004, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Paștele"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Paștele"),
            (NaiveDate::from_ymd_res(2004, 4, 19)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2005,
        vec![

        (NaiveDate::from_ymd_res(2005, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2005, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul)"),
        (NaiveDate::from_ymd_res(2005, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul)"),
        (NaiveDate::from_ymd_res(2005, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2005, 5, 1)?, "Paștele; Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2005, 5, 2)?, "Paștele"),
        (NaiveDate::from_ymd_res(2005, 5, 9)?, "Paștele blajinilor; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2005, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2005, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2006, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "Paștele"),
            (NaiveDate::from_ymd_res(2006, 4, 24)?, "Paștele"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "Paștele blajinilor; Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2007, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Paștele"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Paștele"),
            (NaiveDate::from_ymd_res(2007, 4, 16)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2008, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "Paștele"),
            (NaiveDate::from_ymd_res(2008, 4, 28)?, "Paștele"),
            (NaiveDate::from_ymd_res(2008, 5, 5)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2009, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "Paștele"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "Paștele"),
            (NaiveDate::from_ymd_res(2009, 4, 27)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2010, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Paștele"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2010, 4, 12)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2011, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Paștele"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Paștele"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2012, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "Paștele"),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "Paștele"),
            (NaiveDate::from_ymd_res(2012, 4, 23)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 31)?, "Limba noastră"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2013, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "Paștele"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "Paștele"),
            (NaiveDate::from_ymd_res(2013, 5, 13)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 31)?, "Limba noastră"),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)",
            ),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2014, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Paștele"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Paștele"),
            (NaiveDate::from_ymd_res(2014, 4, 28)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 31)?, "Limba noastră"),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)",
            ),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Anul Nou"),
            (
                NaiveDate::from_ymd_res(2015, 1, 7)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 8)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "Ziua internatională a femeii",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "Paștele"),
            (NaiveDate::from_ymd_res(2015, 4, 13)?, "Paștele"),
            (NaiveDate::from_ymd_res(2015, 4, 20)?, "Paștele blajinilor"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Ziua internaţională a solidarităţii oamenilor muncii",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 9)?,
                "Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 27)?,
                "Ziua independenţei Republicii Moldova",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 31)?, "Limba noastră"),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)",
            ),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2016,
        vec![

        (NaiveDate::from_ymd_res(2016, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2016, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2016, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2016, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2016, 5, 1)?, "Paștele; Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2016, 5, 2)?, "Paștele"),
        (NaiveDate::from_ymd_res(2016, 5, 9)?, "Paștele blajinilor; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2016, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2016, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2016, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2016, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2017,
        vec![

        (NaiveDate::from_ymd_res(2017, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2017, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2017, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2017, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2017, 4, 16)?, "Paștele"),
        (NaiveDate::from_ymd_res(2017, 4, 17)?, "Paștele"),
        (NaiveDate::from_ymd_res(2017, 4, 24)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2017, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2017, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2017, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2017, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2017, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2017, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2018,
        vec![

        (NaiveDate::from_ymd_res(2018, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2018, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2018, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2018, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2018, 4, 8)?, "Paștele"),
        (NaiveDate::from_ymd_res(2018, 4, 9)?, "Paștele"),
        (NaiveDate::from_ymd_res(2018, 4, 16)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2018, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2018, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2018, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2018, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2018, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2018, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2019,
        vec![

        (NaiveDate::from_ymd_res(2019, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2019, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2019, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2019, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2019, 4, 28)?, "Paștele"),
        (NaiveDate::from_ymd_res(2019, 4, 29)?, "Paștele"),
        (NaiveDate::from_ymd_res(2019, 5, 6)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2019, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2019, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2019, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2019, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2019, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2019, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2020,
        vec![

        (NaiveDate::from_ymd_res(2020, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2020, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2020, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2020, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2020, 4, 19)?, "Paștele"),
        (NaiveDate::from_ymd_res(2020, 4, 20)?, "Paștele"),
        (NaiveDate::from_ymd_res(2020, 4, 27)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2020, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2020, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2020, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2020, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2020, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2020, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2021,
        vec![

        (NaiveDate::from_ymd_res(2021, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2021, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2021, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2021, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2021, 5, 2)?, "Paștele"),
        (NaiveDate::from_ymd_res(2021, 5, 3)?, "Paștele"),
        (NaiveDate::from_ymd_res(2021, 5, 10)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2021, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2021, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2021, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2021, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2021, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2021, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2022,
        vec![

        (NaiveDate::from_ymd_res(2022, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2022, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2022, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2022, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2022, 4, 24)?, "Paștele"),
        (NaiveDate::from_ymd_res(2022, 4, 25)?, "Paștele"),
        (NaiveDate::from_ymd_res(2022, 5, 2)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2022, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2022, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2022, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2022, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2022, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2022, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2023,
        vec![

        (NaiveDate::from_ymd_res(2023, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2023, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2023, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2023, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2023, 4, 16)?, "Paștele"),
        (NaiveDate::from_ymd_res(2023, 4, 17)?, "Paștele"),
        (NaiveDate::from_ymd_res(2023, 4, 24)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2023, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2023, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2023, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2023, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2023, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2023, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2024,
        vec![

        (NaiveDate::from_ymd_res(2024, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2024, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2024, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2024, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2024, 5, 5)?, "Paștele"),
        (NaiveDate::from_ymd_res(2024, 5, 6)?, "Paștele"),
        (NaiveDate::from_ymd_res(2024, 5, 13)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2024, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2024, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2024, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2024, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2024, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2024, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2025,
        vec![

        (NaiveDate::from_ymd_res(2025, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2025, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2025, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2025, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2025, 4, 20)?, "Paștele"),
        (NaiveDate::from_ymd_res(2025, 4, 21)?, "Paștele"),
        (NaiveDate::from_ymd_res(2025, 4, 28)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2025, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2025, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2025, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2025, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2025, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2025, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2026,
        vec![

        (NaiveDate::from_ymd_res(2026, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2026, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2026, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2026, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2026, 4, 12)?, "Paștele"),
        (NaiveDate::from_ymd_res(2026, 4, 13)?, "Paștele"),
        (NaiveDate::from_ymd_res(2026, 4, 20)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2026, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2026, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2026, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2026, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2026, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2026, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2027,
        vec![

        (NaiveDate::from_ymd_res(2027, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2027, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2027, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2027, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2027, 5, 2)?, "Paștele"),
        (NaiveDate::from_ymd_res(2027, 5, 3)?, "Paștele"),
        (NaiveDate::from_ymd_res(2027, 5, 10)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2027, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2027, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2027, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2027, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2027, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2027, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2028,
        vec![

        (NaiveDate::from_ymd_res(2028, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2028, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2028, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2028, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2028, 4, 16)?, "Paștele"),
        (NaiveDate::from_ymd_res(2028, 4, 17)?, "Paștele"),
        (NaiveDate::from_ymd_res(2028, 4, 24)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2028, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2028, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2028, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2028, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2028, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2028, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2029,
        vec![

        (NaiveDate::from_ymd_res(2029, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2029, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2029, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2029, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2029, 4, 8)?, "Paștele"),
        (NaiveDate::from_ymd_res(2029, 4, 9)?, "Paștele"),
        (NaiveDate::from_ymd_res(2029, 4, 16)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2029, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2029, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2029, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2029, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2029, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2029, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    build_year(
        years,
        2030,
        vec![

        (NaiveDate::from_ymd_res(2030, 1, 1)?, "Anul Nou"),
        (NaiveDate::from_ymd_res(2030, 1, 7)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2030, 1, 8)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil vechi)"),
        (NaiveDate::from_ymd_res(2030, 3, 8)?, "Ziua internatională a femeii"),
        (NaiveDate::from_ymd_res(2030, 4, 28)?, "Paștele"),
        (NaiveDate::from_ymd_res(2030, 4, 29)?, "Paștele"),
        (NaiveDate::from_ymd_res(2030, 5, 6)?, "Paștele blajinilor"),
        (NaiveDate::from_ymd_res(2030, 5, 1)?, "Ziua internaţională a solidarităţii oamenilor muncii"),
        (NaiveDate::from_ymd_res(2030, 5, 9)?, "Ziua Europei; Ziua Victoriei și a comemorării eroilor căzuţi pentru Independenţa Patriei"),
        (NaiveDate::from_ymd_res(2030, 6, 1)?, "Ziua Ocrotirii Copilului"),
        (NaiveDate::from_ymd_res(2030, 8, 27)?, "Ziua independenţei Republicii Moldova"),
        (NaiveDate::from_ymd_res(2030, 8, 31)?, "Limba noastră"),
        (NaiveDate::from_ymd_res(2030, 12, 25)?, "Nașterea lui Iisus Hristos (Crăciunul pe stil nou)"),
        ],
        &mut map,
        Country::MD,
        "Moldova",
    );

    Ok(map)
}
