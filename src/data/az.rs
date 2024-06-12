//! Azerbaijan
use super::*;

/// Generate holiday map for Azerbaijan.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2000, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2000, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2000, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2000, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2000, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2000, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 18)?,
                "Milli Müstəqillik Günü",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Qurban bayrami (təxmini)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2001, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2001, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2001, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2001, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2001, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2001, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 18)?,
                "Milli Müstəqillik Günü",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "Qurban bayrami (təxmini)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2002, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2002, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2002, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2002, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2002, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2002, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 18)?,
                "Milli Müstəqillik Günü",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 4)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2002, 2, 21)?, "Qurban bayrami"),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2003, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2003, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2003, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2003, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2003, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2003, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2003, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 18)?,
                "Milli Müstəqillik Günü",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 25)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2003, 2, 11)?, "Qurban bayrami"),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2004, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2004, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2004, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2004, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2004, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2004, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2004, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 18)?,
                "Milli Müstəqillik Günü",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2004, 2, 1)?, "Qurban bayrami"),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2005, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2005, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2005, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2005, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2005, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2005, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 18)?,
                "Milli Müstəqillik Günü",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2005, 1, 22)?, "Qurban bayrami"),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2006, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2006, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2006, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2006, 3, 21)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2006, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2006, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2006, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü; Qurban bayrami",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2006, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 29)?,
                "Respublika Günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2007,
        vec![

        (NaiveDate::from_ymd_res(2007, 1, 1)?, "Qurban bayrami; Yeni il bayramı"),
        (NaiveDate::from_ymd_res(2007, 1, 2)?, "Yeni il bayramı"),
        (NaiveDate::from_ymd_res(2007, 1, 20)?, "Ümumxalq hüzn günü"),
        (NaiveDate::from_ymd_res(2007, 3, 8)?, "Qadınlar günü"),
        (NaiveDate::from_ymd_res(2007, 3, 20)?, "Novruz bayramı"),
        (NaiveDate::from_ymd_res(2007, 3, 21)?, "Novruz bayramı"),
        (NaiveDate::from_ymd_res(2007, 3, 22)?, "Novruz bayramı"),
        (NaiveDate::from_ymd_res(2007, 3, 23)?, "Novruz bayramı"),
        (NaiveDate::from_ymd_res(2007, 3, 24)?, "Novruz bayramı"),
        (NaiveDate::from_ymd_res(2007, 5, 9)?, "Faşizm üzərində qələbə günü"),
        (NaiveDate::from_ymd_res(2007, 5, 28)?, "Respublika Günü"),
        (NaiveDate::from_ymd_res(2007, 6, 15)?, "Azərbaycan xalqının milli qurtuluş günü"),
        (NaiveDate::from_ymd_res(2007, 6, 26)?, "Azərbaycan Respublikasının Silahlı Qüvvələri günü"),
        (NaiveDate::from_ymd_res(2007, 12, 31)?, "Dünya azərbaycanlılarının həmrəyliyi günü"),
        (NaiveDate::from_ymd_res(2007, 1, 3)?, "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur); Qurban bayrami (müşahidə olunur)"),
        (NaiveDate::from_ymd_res(2007, 10, 12)?, "Ramazan bayrami"),
        (NaiveDate::from_ymd_res(2007, 10, 13)?, "Ramazan bayrami"),
        (NaiveDate::from_ymd_res(2007, 12, 20)?, "Qurban bayrami"),
        (NaiveDate::from_ymd_res(2007, 12, 21)?, "Qurban bayrami"),
        (NaiveDate::from_ymd_res(2007, 3, 26)?, "Novruz bayramı (müşahidə olunur)"),
        (NaiveDate::from_ymd_res(2007, 10, 15)?, "Ramazan bayrami (müşahidə olunur)"),
        (NaiveDate::from_ymd_res(2007, 1, 4)?, "Qurban bayrami (müşahidə olunur)"),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2008, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2008, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2008, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2008, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2008, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2008, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 30)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2008, 12, 8)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2008, 12, 9)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2008, 3, 10)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 6, 16)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2009, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2009, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2009, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2009, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2009, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2009, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2009, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2009, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2009, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2009, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2009, 11, 28)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 11)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 9, 22)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 30)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2010, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2010, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2010, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2010, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2010, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2010, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2010, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2010, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2010, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 9)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2010, 11, 16)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2010, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 10)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 6, 28)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2011, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2011, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2011, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2011, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2011, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2011, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2011, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2011, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 30)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2011, 11, 6)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2011, 11, 7)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 30)?,
                "Respublika Günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 27)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 8)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 29)?,
                "İstirahət günü (27.08.2011 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2012, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2012, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2012, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2012, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2012, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2012, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2012, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2012, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2012, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2012, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2012, 8, 20)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2012, 10, 25)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2012, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 21)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2013, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2013, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2013, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2013, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2013, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2013, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2013, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2013, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2013, 8, 9)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2013, 10, 16)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2013, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 6, 17)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 11)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 3)?,
                "İstirahət günü (29.12.2012 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 4)?,
                "İstirahət günü (30.12.2012 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2014, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2014, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2014, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2014, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2014, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2014, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2014, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2014, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2014, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2014, 7, 29)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2014, 10, 5)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2014, 3, 10)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 6, 16)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 6)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 7)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 10)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 3)?,
                "İstirahət günü (28.12.2013 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "İstirahət günü (29.12.2013 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2015, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2015, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2015, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2015, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2015, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2015, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2015, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2015, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2015, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2015, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2015, 7, 18)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2015, 9, 25)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2015, 3, 9)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 11)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 7, 20)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2016, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2016, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2016, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2016, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2016, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2016, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2016, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2016, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2016, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2016, 7, 7)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2016, 9, 13)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2016, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 30)?,
                "Respublika Günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 27)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2017, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2017, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2017, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2017, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2017, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2017, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2017, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2017, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2017, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü; Ramazan bayrami",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 27)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2017, 9, 2)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2017, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 29)?,
                "Respublika Günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 4)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 28)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2018, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2018, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2018, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2018, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2018, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2018, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2018, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2018, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2018, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü; Ramazan bayrami",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2018, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 16)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2018, 8, 23)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2018, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 18)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 19)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 11)?,
                "Prezidenti seçkiləri",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2019, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2019, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2019, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2019, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2019, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2019, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2019, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2019, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2019, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2019, 6, 6)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2019, 8, 12)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2019, 8, 13)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2019, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 17)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 11)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 27)?,
                "Bələdiyyə seçkiləri",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2020, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2020, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2020, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2020, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2020, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2020, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2020, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2020, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 28)?, "Respublika Günü"),
            (
                NaiveDate::from_ymd_res(2020, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2020, 5, 25)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2020, 8, 1)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2020, 3, 9)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 11)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 26)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 3)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 27)?,
                "İstirahət günü (29.03.2020 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 27)?,
                "İstirahət günü (30.05.2020 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 3)?,
                "İstirahət günü (28.12.2019 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "İstirahət günü (29.12.2019 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2021, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2021, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2021, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2021, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2021, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2021, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2021, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2021, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2021, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2021, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2021, 5, 14)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2021, 7, 21)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2021, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 10)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 28)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 11)?,
                "İstirahət günü (08.05.2021 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 12)?,
                "İstirahət günü (16.05.2021 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 19)?,
                "İstirahət günü (17.07.2021 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2022, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2022, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2022, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2022, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2022, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2022, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2022, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2022, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2022, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2022, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 30)?,
                "Müstəqillik Günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 6, 27)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 11)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 12)?,
                "Qurban bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 7)?,
                "İstirahət günü (05.03.2022 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 7)?,
                "İstirahət günü (05.11.2022 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2023, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2023, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2023, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2023, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2023, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2023, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2023, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2023, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2023, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2023, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "Qurban bayrami"),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "Qurban bayrami"),
            (
                NaiveDate::from_ymd_res(2023, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Ramazan bayrami (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 29)?,
                "Müstəqillik Günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 27)?,
                "İstirahət günü (24.06.2023 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 30)?,
                "İstirahət günü (25.06.2023 ilə əvəz edilmişdir)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 10)?,
                "İstirahət günü (04.11.2023 ilə əvəz edilmişdir)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2024, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2024, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2024, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2024, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2024, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2024, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2024, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2024, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2024, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2024, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Ramazan bayrami"),
            (NaiveDate::from_ymd_res(2024, 4, 11)?, "Ramazan bayrami"),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 18)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 19)?,
                "Qurban bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 11)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2025, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2025, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2025, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2025, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2025, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2025, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2025, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2025, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2025, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2025, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 7)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 10)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "Ramazan bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 9)?,
                "Qurban bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 16)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 10)?,
                "Zəfər Günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 11)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2026, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2026, 3, 8)?, "Qadınlar günü"),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Novruz bayramı; Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Novruz bayramı; Ramazan bayrami (təxmini)",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2026, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2026, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2026, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 28)?,
                "Müstəqillik Günü; Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2026, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 9)?,
                "Qadınlar günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 25)?,
                "Novruz bayramı (müşahidə olunur); Ramazan bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 11)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 10)?,
                "Zəfər Günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2027, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2027, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2027, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2027, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2027, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2027, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2027, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 10)?,
                "Faşizm üzərində qələbə günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 18)?,
                "Qurban bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 6, 28)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2028, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2028, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2028, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2028, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2028, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2028, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2028, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2028, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2028, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2028, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 6)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 4)?,
                "Yeni il bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Ramazan bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 29)?,
                "Ramazan bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 8)?,
                "Qurban bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 29)?,
                "Müstəqillik Günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2029, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2029, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2029, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2029, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2029, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2029, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2029, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2029, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2029, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2029, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2029, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2029, 1, 3)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 25)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Yeni il bayramı"),
            (NaiveDate::from_ymd_res(2030, 1, 20)?, "Ümumxalq hüzn günü"),
            (NaiveDate::from_ymd_res(2030, 3, 8)?, "Qadınlar günü"),
            (NaiveDate::from_ymd_res(2030, 3, 20)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2030, 3, 21)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2030, 3, 22)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2030, 3, 23)?, "Novruz bayramı"),
            (NaiveDate::from_ymd_res(2030, 3, 24)?, "Novruz bayramı"),
            (
                NaiveDate::from_ymd_res(2030, 5, 9)?,
                "Faşizm üzərində qələbə günü",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 28)?, "Müstəqillik Günü"),
            (
                NaiveDate::from_ymd_res(2030, 6, 15)?,
                "Azərbaycan xalqının milli qurtuluş günü",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 26)?,
                "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 8)?, "Zəfər Günü"),
            (
                NaiveDate::from_ymd_res(2030, 11, 9)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 31)?,
                "Dünya azərbaycanlılarının həmrəyliyi günü",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Ramazan bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 14)?,
                "Qurban bayrami (təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 25)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 3, 26)?,
                "Novruz bayramı (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 15)?,
                "Qurban bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 16)?,
                "Qurban bayrami (müşahidə olunur, təxmini)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 6, 17)?,
                "Azərbaycan xalqının milli qurtuluş günü (müşahidə olunur)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 11)?,
                "Azərbaycan Respublikasının Dövlət bayrağı günü (müşahidə olunur)",
            ),
        ],
        &mut map,
        Country::AZ,
        "Azerbaijan",
    );

    Ok(map)
}
