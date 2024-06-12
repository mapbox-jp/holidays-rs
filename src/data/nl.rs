//! Netherlands
use super::*;

/// Generate holiday map for Netherlands.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2000, 4, 29)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2000, 5, 5)?, "Bevrijdingsdag"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2001, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2002, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2003, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2004, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2005, 4, 30)?, "Koninginnedag"),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "Bevrijdingsdag; Hemelvaartsdag",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2005, 5, 16)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2006, 4, 29)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2007, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2008, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2009, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2010, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2010, 5, 5)?, "Bevrijdingsdag"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2011, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2012, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2013, 4, 30)?, "Koninginnedag"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2014, 4, 26)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2015, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2015, 5, 5)?, "Bevrijdingsdag"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2016, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2017, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2018, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2019, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2020, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2020, 5, 5)?, "Bevrijdingsdag"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2021, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2022, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2023, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2024, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2025, 4, 26)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2025, 5, 5)?, "Bevrijdingsdag"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2026, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2027, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2028, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2029, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nieuwjaarsdag"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Eerste paasdag"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Tweede paasdag"),
            (NaiveDate::from_ymd_res(2030, 4, 27)?, "Koningsdag"),
            (NaiveDate::from_ymd_res(2030, 5, 5)?, "Bevrijdingsdag"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Hemelvaartsdag"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Eerste Pinksterdag"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Tweede Pinksterdag"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Eerste Kerstdag"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Tweede Kerstdag"),
        ],
        &mut map,
        Country::NL,
        "Netherlands",
    );

    Ok(map)
}
