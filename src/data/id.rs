//! Indonesia
use super::*;

/// Generate holiday map for Indonesia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2000, 5, 18)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 1)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 6)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 24)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2001, 5, 7)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 24)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Hari Raya Idulfitri",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 17)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 6)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2001, 3, 26)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 15)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2002, 5, 26)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2002, 12, 6)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2002, 12, 7)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 23)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2002, 3, 15)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 4)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2003, 5, 15)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 29)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Hari Raya Idulfitri",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 26)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2003, 3, 5)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2003, 5, 13)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 9, 24)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2004, 1, 22)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2004, 6, 2)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 20)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Hari Raya Idulfitri",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2004, 2, 2)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2004, 2, 22)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 9, 12)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2005, 5, 22)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2005, 2, 10)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 9, 1)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2006, 1, 30)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2006, 5, 12)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2006, 10, 24)?,
                "Hari Raya Idulfitri",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 25)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2006, 1, 31)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2006, 4, 10)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 22)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2007, 2, 19)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2007, 6, 1)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2007, 5, 17)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2007, 10, 13)?,
                "Hari Raya Idulfitri",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2007, 1, 20)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 11)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2008, 5, 20)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2008, 1, 10)?, "Tahun Baru Islam"),
            (NaiveDate::from_ymd_res(2008, 12, 29)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 31)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2009, 3, 26)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2009, 5, 9)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2009, 5, 21)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2009, 12, 18)?, "Tahun Baru Islam"),
            (NaiveDate::from_ymd_res(2009, 3, 9)?, "Maulid Nabi Muhammad"),
            (
                NaiveDate::from_ymd_res(2009, 7, 20)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2010, 2, 15)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2010, 3, 16)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2010, 5, 28)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2010, 5, 13)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2010, 12, 7)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2010, 7, 9)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2011, 3, 5)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2011, 5, 17)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2011, 6, 2)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 6)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2011, 11, 27)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 29)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2012, 1, 23)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2012, 3, 23)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2012, 5, 6)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2012, 5, 17)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2012, 11, 15)?, "Tahun Baru Islam"),
            (NaiveDate::from_ymd_res(2012, 2, 5)?, "Maulid Nabi Muhammad"),
            (
                NaiveDate::from_ymd_res(2012, 6, 17)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2013, 2, 11)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2013, 3, 12)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2013, 5, 25)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2013, 11, 5)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 6)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2014, 1, 31)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2014, 3, 31)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2014, 5, 15)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 29)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 5)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2014, 10, 25)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2014, 1, 14)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 27)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2015, 3, 21)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2015, 6, 2)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 14)?,
                "Kenaikan Yesus Kristus",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2015, 10, 14)?, "Tahun Baru Islam"),
            (NaiveDate::from_ymd_res(2015, 1, 3)?, "Maulid Nabi Muhammad"),
            (
                NaiveDate::from_ymd_res(2015, 12, 24)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 16)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2016, 3, 9)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2016, 5, 22)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 5)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2016, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2016, 10, 2)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2016, 12, 12)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 6)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2017, 3, 28)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2017, 5, 11)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2017, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2017, 9, 21)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2017, 12, 1)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 24)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2018, 2, 16)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2018, 3, 17)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2018, 5, 29)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2018, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 10)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2018, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2018, 9, 11)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2018, 11, 20)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 14)?,
                "Isra Mikraj Nabi Muhammad",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 27)?, "Hari Pemilihan"),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2019, 3, 7)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2019, 5, 19)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2019, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 30)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2019, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2019, 6, 6)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2019, 9, 1)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 3)?,
                "Isra Mikraj Nabi Muhammad",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 17)?, "Hari Pemilihan"),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2020, 3, 25)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2020, 5, 7)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2020, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 21)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2020, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2020, 8, 20)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2020, 10, 29)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 22)?,
                "Isra Mikraj Nabi Muhammad",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 9)?, "Hari Pemilihan"),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2021, 3, 14)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2021, 5, 26)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Hari Raya Idulfitri; Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2021, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2021, 8, 11)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2021, 10, 19)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 11)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2022, 3, 3)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2022, 5, 16)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2022, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 26)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2022, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2022, 7, 30)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2022, 10, 8)?,
                "Maulid Nabi Muhammad",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 28)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Tahun Baru Masehi"),
            (NaiveDate::from_ymd_res(2023, 1, 22)?, "Tahun Baru Imlek"),
            (NaiveDate::from_ymd_res(2023, 3, 22)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Wafat Yesus Kristus"),
            (NaiveDate::from_ymd_res(2023, 6, 4)?, "Hari Raya Waisak"),
            (
                NaiveDate::from_ymd_res(2023, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 18)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2023, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2023, 4, 23)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "Hari Raya Iduladha"),
            (NaiveDate::from_ymd_res(2023, 7, 19)?, "Tahun Baru Islam"),
            (
                NaiveDate::from_ymd_res(2023, 9, 27)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 18)?,
                "Isra Mikraj Nabi Muhammad",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2024, 2, 10)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 11)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2024, 5, 22)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2024, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Hari Raya Natal"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Hari Raya Idulfitri"),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "Hari kedua dari Hari Raya Idulfitri",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 7, 7)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 15)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 8)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2025, 1, 29)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 29)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2025, 5, 11)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 29)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2025, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 27)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2026, 2, 17)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 19)?, "Hari Suci Nyepi"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2026, 5, 31)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 14)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2026, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2026, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 16)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 1, 16)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2027, 2, 6)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2027, 5, 20)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 6)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2027, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2027, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Hari Raya Natal; Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 6)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 5)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2028, 1, 26)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Kenaikan Yesus Kristus; Tahun Baru Islam (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2028, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 14)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2029, 2, 13)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2029, 5, 27)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 10)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2029, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2029, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 3)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Tahun Baru Masehi"),
            (
                NaiveDate::from_ymd_res(2030, 2, 3)?,
                "Tahun Baru Imlek (perkiraan)",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Wafat Yesus Kristus"),
            (
                NaiveDate::from_ymd_res(2030, 5, 16)?,
                "Hari Raya Waisak (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 1)?,
                "Hari Buruh Internasional",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 30)?,
                "Kenaikan Yesus Kristus",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 1)?, "Hari Lahir Pancasila"),
            (
                NaiveDate::from_ymd_res(2030, 8, 17)?,
                "Hari Kemerdekaan Republik Indonesia",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Hari Raya Natal"),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Hari kedua dari Hari Raya Idulfitri (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Hari Raya Iduladha (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 3)?,
                "Tahun Baru Islam (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "Maulid Nabi Muhammad (perkiraan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 23)?,
                "Isra Mikraj Nabi Muhammad (perkiraan)",
            ),
        ],
        &mut map,
        Country::ID,
        "Indonesia",
    );

    Ok(map)
}
