//! Pakistan
use super::*;

/// Generate holiday map for Pakistan.
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
                NaiveDate::from_ymd_res(2000, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2000, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2000, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 10)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 29)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 17)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 18)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 14)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2000, 4, 15)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2001, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2001, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 18)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 6)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 7)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 3)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2001, 4, 4)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 23)?,
                "Ashura (estimated); Pakistan Day",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2002, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2002, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 6)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 7)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 23)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 24)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 24)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2003, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2003, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 27)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 12)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 13)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 12)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2003, 3, 13)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 23)?, "Pakistan Day"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Eid Milad-un-Nabi (estimated); Labour Day",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2004, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 16)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 2)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 3)?,
                "Eid-ul-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2004, 2, 29)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2004, 3, 1)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2005, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2005, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2005, 11, 4)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2005, 11, 5)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2005, 11, 6)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2005, 1, 22)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2005, 1, 23)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2005, 4, 22)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2005, 2, 17)?, "Ashura"),
            (NaiveDate::from_ymd_res(2005, 2, 18)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2006, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2006, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2006, 10, 25)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2006, 10, 26)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2006, 1, 11)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2006, 1, 12)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2006, 4, 11)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2006, 2, 7)?, "Ashura"),
            (NaiveDate::from_ymd_res(2006, 2, 8)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2007, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2007, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2007, 10, 14)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2007, 10, 15)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2007, 12, 21)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2007, 12, 22)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2007, 3, 31)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2007, 1, 27)?, "Ashura"),
            (NaiveDate::from_ymd_res(2007, 1, 28)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2008, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2008, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2008, 10, 2)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2008, 10, 3)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2008, 10, 4)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2008, 12, 9)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2008, 12, 10)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2008, 12, 11)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2008, 1, 17)?, "Ashura"),
            (NaiveDate::from_ymd_res(2008, 1, 18)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2009, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2009, 11, 9)?, "Iqbal Day"),
            (
                NaiveDate::from_ymd_res(2009, 12, 25)?,
                "Ashura; Quaid-e-Azam Day",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2009, 9, 22)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2009, 9, 23)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2009, 11, 28)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2009, 11, 29)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2009, 11, 30)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2009, 3, 9)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2009, 1, 5)?, "Ashura"),
            (NaiveDate::from_ymd_res(2009, 1, 6)?, "Ashura"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2010, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2010, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2010, 9, 11)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2010, 9, 12)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2010, 11, 18)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2010, 11, 19)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2010, 3, 1)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2010, 12, 15)?, "Ashura"),
            (NaiveDate::from_ymd_res(2010, 12, 16)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2011, 8, 14)?, "Independence Day"),
            (
                NaiveDate::from_ymd_res(2011, 11, 9)?,
                "Eid-ul-Adha; Iqbal Day",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2011, 9, 1)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2011, 9, 2)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2011, 11, 7)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2011, 11, 8)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2011, 2, 17)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2011, 12, 4)?, "Ashura"),
            (NaiveDate::from_ymd_res(2011, 12, 5)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 2, 5)?,
                "Eid Milad-un-Nabi; Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2012, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2012, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2012, 8, 20)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2012, 8, 21)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2012, 10, 27)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2012, 10, 28)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2012, 11, 22)?, "Ashura"),
            (NaiveDate::from_ymd_res(2012, 11, 23)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2013, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2013, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2013, 8, 9)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2013, 8, 10)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2013, 10, 16)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2013, 10, 17)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2013, 1, 24)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2013, 11, 12)?, "Ashura"),
            (NaiveDate::from_ymd_res(2013, 11, 13)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2014, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2014, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2014, 7, 29)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2014, 7, 30)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2014, 7, 31)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2014, 10, 6)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2014, 10, 7)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2014, 10, 8)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2014, 1, 14)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2014, 11, 2)?, "Ashura"),
            (NaiveDate::from_ymd_res(2014, 11, 3)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2015, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2015, 7, 18)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2015, 7, 19)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2015, 9, 25)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2015, 9, 26)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2015, 1, 4)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2015, 10, 22)?, "Ashura"),
            (NaiveDate::from_ymd_res(2015, 10, 23)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2016, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2016, 7, 7)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2016, 7, 8)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2016, 9, 13)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2016, 9, 14)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2016, 12, 12)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2016, 10, 10)?, "Ashura"),
            (NaiveDate::from_ymd_res(2016, 10, 11)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2017, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2017, 6, 26)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2017, 6, 27)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2017, 6, 28)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2017, 9, 3)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2017, 9, 4)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2017, 12, 1)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2017, 9, 29)?, "Ashura"),
            (NaiveDate::from_ymd_res(2017, 9, 30)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2018, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2018, 6, 16)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2018, 6, 17)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2018, 6, 18)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2018, 8, 23)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2018, 8, 24)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2018, 11, 21)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2018, 9, 20)?, "Ashura"),
            (NaiveDate::from_ymd_res(2018, 9, 21)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2019, 8, 14)?,
                "Eid-ul-Adha; Independence Day",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2019, 6, 6)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2019, 6, 7)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2019, 8, 12)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2019, 8, 13)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2019, 11, 10)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2019, 9, 8)?, "Ashura"),
            (NaiveDate::from_ymd_res(2019, 9, 9)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2020, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2020, 5, 26)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2020, 8, 2)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2020, 10, 30)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2020, 8, 28)?, "Ashura"),
            (NaiveDate::from_ymd_res(2020, 8, 29)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2021, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2021, 5, 14)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2021, 5, 15)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2021, 7, 21)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2021, 7, 22)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2021, 7, 23)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2021, 10, 19)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2021, 8, 17)?, "Ashura"),
            (NaiveDate::from_ymd_res(2021, 8, 18)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2022, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2022, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2022, 5, 4)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2022, 5, 5)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2022, 7, 11)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2022, 7, 12)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2022, 10, 9)?, "Eid Milad-un-Nabi"),
            (NaiveDate::from_ymd_res(2022, 8, 8)?, "Ashura"),
            (NaiveDate::from_ymd_res(2022, 8, 9)?, "Ashura"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2023, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2023, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2023, 4, 23)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2023, 4, 24)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2023, 6, 30)?, "Eid-ul-Adha"),
            (NaiveDate::from_ymd_res(2023, 7, 1)?, "Eid-ul-Adha"),
            (
                NaiveDate::from_ymd_res(2023, 9, 27)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2023, 7, 27)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2023, 7, 28)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2024, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2024, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Quaid-e-Azam Day"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2024, 4, 11)?, "Eid-ul-Fitr"),
            (NaiveDate::from_ymd_res(2024, 4, 12)?, "Eid-ul-Fitr"),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 18)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2024, 7, 15)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2024, 7, 16)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2025, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2025, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 8)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2025, 7, 4)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2025, 7, 5)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2026, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2026, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 22)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 29)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 24)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2026, 6, 25)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Labour Day"),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "Eid Milad-un-Nabi (estimated); Independence Day",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 11)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 18)?,
                "Eid-ul-Adha (estimated)",
            ),
            (NaiveDate::from_ymd_res(2027, 6, 14)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2027, 6, 15)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2028, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2028, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 7)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 2)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2028, 6, 3)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 2, 5)?,
                "Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2029, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2029, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 26)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 22)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2029, 5, 23)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Eid-ul-Fitr (estimated); Kashmir Solidarity Day",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 23)?, "Pakistan Day"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Labour Day"),
            (NaiveDate::from_ymd_res(2030, 8, 14)?, "Independence Day"),
            (NaiveDate::from_ymd_res(2030, 11, 9)?, "Iqbal Day"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Quaid-e-Azam Day"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "Eid-ul-Fitr (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 15)?,
                "Eid-ul-Adha (estimated)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "Eid Milad-un-Nabi (estimated)",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 11)?, "Ashura (estimated)"),
            (NaiveDate::from_ymd_res(2030, 5, 12)?, "Ashura (estimated)"),
        ],
        &mut map,
        Country::PK,
        "Pakistan",
    );

    Ok(map)
}
