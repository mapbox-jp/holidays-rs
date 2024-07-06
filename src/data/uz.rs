//! Uzbekistan
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Uzbekistan";
const COUNTY_CODE: Country = Country::UZ;

/// Generate holiday map for Uzbekistan.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2000, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2000, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2000, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2000, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2000, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Qurbon hayit (taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2001, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2001, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2001, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2001, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2001, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 16)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 5)?,
                "Qurbon hayit (taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2002, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2002, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2002, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2002, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 5)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 22)?,
                "Qurbon hayit (taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2003, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2003, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2003, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2003, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 25)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 11)?,
                "Qurbon hayit (taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2004, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2004, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2004, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2004, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2004, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 14)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 1)?,
                "Qurbon hayit (taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2005, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2005, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2005, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2005, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2005, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 3)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 1, 21)?,
                "Qurbon hayit (taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2006, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2006, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2006, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2006, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2006, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Qurbon hayit"),
            (NaiveDate::from_ymd_res(2006, 12, 30)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2007, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2007, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2007, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2007, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2007, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2007, 12, 19)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2008, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2008, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2008, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2008, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni; Ro‘za hayit",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni; Qurbon hayit",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2009, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2009, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2009, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2009, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 21)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2009, 11, 27)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2010, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2010, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2010, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2010, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2010, 11, 16)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2011, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2011, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2011, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 31)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2011, 11, 6)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2012, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2012, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2012, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2012, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2013, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2013, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2013, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2013, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 9)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2014, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2014, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2014, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2014, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2014, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2014, 10, 4)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2015, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2015, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2015, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2015, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 18)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2016, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2016, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2016, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2016, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "Qurbon hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2017, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2017, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2017, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 1)?,
                "Mustaqillik kuni; Qurbon hayit",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 26)?, "Ro‘za hayit"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2018, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2018, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2018, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2018, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2018, 8, 21)?, "Qurbon hayit"),
            (
                NaiveDate::from_ymd_res(2018, 1, 2)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2018, 1, 3)?,
                "Dam olish kuni (06/01 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 19)?,
                "Dam olish kuni (17/03 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 22)?,
                "Dam olish kuni (24/03 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 30)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 23)?,
                "Dam olish kuni (25/08 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 24)?,
                "Dam olish kuni (26/08 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 31)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 3)?,
                "Dam olish kuni (08/09 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 4)?,
                "Dam olish kuni (15/09 2018 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Dam olish kuni (29/12 2018 dan ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2019, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2019, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2019, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2019, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2019, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "Qurbon hayit"),
            (
                NaiveDate::from_ymd_res(2019, 1, 2)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2019, 1, 3)?,
                "Dam olish kuni (05/01 2019 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 22)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 6)?,
                "Dam olish kuni (01/06 2019 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 2)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2019, 9, 3)?,
                "Dam olish kuni (07/09 2019 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 31)?,
                "Dam olish kuni (28/12 2019 dan ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2020, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2020, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2020, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2020, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2020, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Qurbon hayit"),
            (
                NaiveDate::from_ymd_res(2020, 1, 2)?,
                "Dam olish kuni (04/01 2020 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 23)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 31)?,
                "Dam olish kuni (29/08 2020 dan ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2021, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2021, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2021, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2021, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2021, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "Qurbon hayit"),
            (
                NaiveDate::from_ymd_res(2021, 3, 22)?,
                "Dam olish kuni (27/03 2021 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 21)?,
                "Dam olish kuni (17/07 2021 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 7, 22)?,
                "Dam olish kuni (24/07 2021 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 2)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2021, 9, 3)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 31)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2022, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2022, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2022, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2022, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2022, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2022, 7, 9)?, "Qurbon hayit"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2022, 1, 4)?,
                "Dam olish kuni (08/01 2022 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 22)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 23)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 4)?,
                "Dam olish kuni (07/05 2022 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 11)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2022, 7, 12)?,
                "Dam olish kuni (16/07 2022 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 9, 2)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2023, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2023, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2023, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2023, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2023, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "Ro‘za hayit"),
            (NaiveDate::from_ymd_res(2023, 6, 28)?, "Qurbon hayit"),
            (
                NaiveDate::from_ymd_res(2023, 10, 2)?,
                "O‘qituvchi va murabbiylar kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 2)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 3)?,
                "Dam olish kuni (07/01 2023 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 20)?,
                "Dam olish kuni (11/03 2023 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 3, 22)?,
                "Dam olish kuni (25/03 2023 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 29)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2023, 6, 30)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2024, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2024, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2024, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2024, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Ro‘za hayit"),
            (
                NaiveDate::from_ymd_res(2024, 6, 16)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 17)?,
                "Qurbon hayit (ko‘chirilgan, taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 2)?,
                "Mustaqillik kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 9)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 1, 2)?,
                "Dam olish kuni (06/01 2024 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 3, 22)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2024, 4, 12)?,
                "Dam olish kuni (13/04 2024 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 6, 18)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2024, 9, 3)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 30)?,
                "Dam olish kuni (14/12 2024 dan ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 31)?,
                "Prezidentining farmoni bilan qo‘shimcha dam olish kuni",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2025, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2025, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2025, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2025, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 10)?,
                "Xotin-qizlar kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Ro‘za hayit (ko‘chirilgan, taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2026, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2026, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2026, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2026, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2026, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 9)?,
                "Xotin-qizlar kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 23)?,
                "Navro‘z bayrami (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 11)?,
                "Xotira va qadrlash kuni (ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2027, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2027, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2027, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2027, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2027, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 22)?,
                "Navro‘z bayrami (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 10)?,
                "Xotira va qadrlash kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Qurbon hayit (ko‘chirilgan, taxminiy)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2028, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2028, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2028, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2028, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "Yangi yil (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Ro‘za hayit (ko‘chirilgan, taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 2)?,
                "O‘qituvchi va murabbiylar kuni (ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2029, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2029, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2029, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2029, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 9, 3)?,
                "Mustaqillik kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 10)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni (ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Yangi yil"),
            (NaiveDate::from_ymd_res(2030, 3, 8)?, "Xotin-qizlar kuni"),
            (NaiveDate::from_ymd_res(2030, 3, 21)?, "Navro‘z bayrami"),
            (
                NaiveDate::from_ymd_res(2030, 5, 9)?,
                "Xotira va qadrlash kuni",
            ),
            (NaiveDate::from_ymd_res(2030, 9, 1)?, "Mustaqillik kuni"),
            (
                NaiveDate::from_ymd_res(2030, 10, 1)?,
                "O‘qituvchi va murabbiylar kuni",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 8)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Ro‘za hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Qurbon hayit (taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 15)?,
                "Qurbon hayit (ko‘chirilgan, taxminiy)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 9, 2)?,
                "Mustaqillik kuni (ko‘chirilgan)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 9)?,
                "O‘zbekiston Respublikasi Konstitutsiyasi kuni (ko‘chirilgan)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
