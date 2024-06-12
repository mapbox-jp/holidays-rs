//! Slovenia
use super::*;

/// Generate holiday map for Slovenia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2000, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2000, 4, 24)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2000, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2000, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2000, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2000, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2001, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2001, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2001, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2001, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2001, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2002, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2002, 4, 1)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2002, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2002, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2002, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2002, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2002, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2003, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2003, 4, 21)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2003, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2003, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2003, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2003, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2004, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2004, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2004, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2004, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2004, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2005, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2005, 3, 28)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2005, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2005, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2005, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2005, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2006, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2006, 4, 17)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2006, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2006, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2006, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2006, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2007, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2007, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2007, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2007, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2007, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2008, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2008, 3, 24)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2008, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2008, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2008, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2009, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2009, 4, 13)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2009, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2009, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2009, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2009, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2010, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2010, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2010, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2010, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2010, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2011, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2011, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2011, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2011, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2011, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2012, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2012, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2012, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2012, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2012, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2013, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2013, 4, 1)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2013, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2013, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2013, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2013, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2013, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2014, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2014, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2014, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2014, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2015, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2015, 4, 6)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2015, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2015, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2015, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2015, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2016, 3, 28)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2016, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2016, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2016, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2016, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2017, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2017, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2017, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2017, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2018, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2018, 4, 2)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2018, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2018, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2018, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2018, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2019, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2019, 4, 22)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2019, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2019, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2019, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2019, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2020, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2020, 4, 13)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2020, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2020, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2020, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2020, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2021, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2021, 4, 5)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2021, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2021, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2021, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2021, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2022, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2022, 4, 18)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2022, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2022, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2022, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2022, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2023, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2023, 4, 10)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2023, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2023, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2023, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2023, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 14)?, "dan solidarnosti"),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2024, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2024, 4, 1)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2024, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2024, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2024, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2024, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2025, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2025, 4, 21)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2025, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2025, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2025, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2025, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2026, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2026, 4, 6)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2026, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2026, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2026, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2026, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2026, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2027, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2027, 3, 29)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2027, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2027, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2027, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2027, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2028, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2028, 4, 17)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2028, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2028, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2028, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2028, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2028, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2029, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2029, 4, 2)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2029, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2029, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2029, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2029, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "novo leto"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "novo leto"),
            (NaiveDate::from_ymd_res(2030, 2, 8)?, "Prešernov dan"),
            (
                NaiveDate::from_ymd_res(2030, 4, 22)?,
                "Velikonočni ponedeljek",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 27)?,
                "dan upora proti okupatorju",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2030, 5, 2)?, "praznik dela"),
            (NaiveDate::from_ymd_res(2030, 6, 25)?, "dan državnosti"),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Marijino vnebovzetje",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 31)?, "dan reformacije"),
            (
                NaiveDate::from_ymd_res(2030, 11, 1)?,
                "dan spomina na mrtve",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Božič"),
            (
                NaiveDate::from_ymd_res(2030, 12, 26)?,
                "dan samostojnosti in enotnosti",
            ),
        ],
        &mut map,
        Country::SI,
        "Slovenia",
    );

    Ok(map)
}
