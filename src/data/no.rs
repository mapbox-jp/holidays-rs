//! Norway
use super::*;

/// Generate holiday map for Norway.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2000, 4, 20)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2000, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2000, 6, 1)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2001, 4, 12)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2001, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2001, 5, 24)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2002, 3, 28)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2002, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2003, 4, 17)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2003, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2003, 5, 29)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2004, 4, 8)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2004, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2004, 5, 20)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2005, 3, 24)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2005, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2005, 5, 16)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2006, 4, 13)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2006, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2007, 4, 5)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Arbeidernes dag"),
            (
                NaiveDate::from_ymd_res(2007, 5, 17)?,
                "Grunnlovsdag; Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Andre påskedag"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Arbeidernes dag; Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 17)?, "Grunnlovsdag"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2009, 4, 9)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2009, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2009, 5, 21)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2010, 4, 1)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2010, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2010, 5, 13)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2011, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2011, 6, 2)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2012, 4, 5)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Arbeidernes dag"),
            (
                NaiveDate::from_ymd_res(2012, 5, 17)?,
                "Grunnlovsdag; Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2013, 3, 28)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2013, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2014, 4, 17)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2014, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2014, 5, 29)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2015, 4, 2)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2015, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2015, 5, 14)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2016, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2016, 5, 5)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2017, 4, 13)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2017, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2018, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2018, 5, 10)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2019, 4, 18)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2019, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2019, 5, 30)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2020, 4, 9)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2020, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2020, 5, 21)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2021, 4, 1)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2021, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2022, 4, 14)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2022, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2022, 5, 26)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2023, 4, 6)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2023, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2023, 5, 18)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2024, 3, 28)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2024, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2025, 4, 17)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2025, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2025, 5, 29)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2026, 4, 2)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2026, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2026, 5, 14)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2027, 3, 25)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Arbeidernes dag"),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Andre pinsedag; Grunnlovsdag",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 6)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2028, 4, 13)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2028, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2029, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2029, 5, 10)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Første nyttårsdag"),
            (NaiveDate::from_ymd_res(2030, 4, 18)?, "Skjærtorsdag"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Langfredag"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Første påskedag"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Andre påskedag"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Arbeidernes dag"),
            (NaiveDate::from_ymd_res(2030, 5, 17)?, "Grunnlovsdag"),
            (
                NaiveDate::from_ymd_res(2030, 5, 30)?,
                "Kristi himmelfartsdag",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Første pinsedag"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Andre pinsedag"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Første juledag"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Andre juledag"),
        ],
        &mut map,
        Country::NO,
        "Norway",
    );

    Ok(map)
}
