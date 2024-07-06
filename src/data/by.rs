//! Belarus
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Belarus";
const COUNTY_CODE: Country = Country::BY;

/// Generate holiday map for Belarus.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2000, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2000, 3, 8)?, "Дзень жанчын"),
            (
                NaiveDate::from_ymd_res(2000, 5, 9)?,
                "Дзень Перамогі; Радаўніца",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Свята працы"),
            (
                NaiveDate::from_ymd_res(2000, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 8)?,
                "Выходны (перанесены з 13.05.2000)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 11, 6)?,
                "Выходны (перанесены з 11.11.2000)",
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
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2001, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2001, 4, 24)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2001, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2001, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 2)?,
                "Выходны (перанесены з 20.01.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 9)?,
                "Выходны (перанесены з 03.03.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 23)?,
                "Выходны (перанесены з 21.04.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 30)?,
                "Выходны (перанесены з 28.04.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 7, 2)?,
                "Выходны (перанесены з 07.07.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 24)?,
                "Выходны (перанесены з 22.12.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 31)?,
                "Выходны (перанесены з 29.12.2001)",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2002, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2002, 5, 14)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2002, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 1, 2)?,
                "Выходны (перанесены з 05.01.2002)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 10)?,
                "Выходны (перанесены з 18.05.2002)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 11, 8)?,
                "Выходны (перанесены з 16.11.2002)",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2003, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2003, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2003, 5, 6)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2003, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2003, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2003, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Выходны (перанесены з 04.01.2003)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 5)?,
                "Выходны (перанесены з 03.05.2003)",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2004, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2004, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2004, 4, 20)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2004, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2004, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2004, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 2)?,
                "Выходны (перанесены з 10.01.2004)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 5)?,
                "Выходны (перанесены з 17.01.2004)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 6)?,
                "Выходны (перанесены з 31.01.2004)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 19)?,
                "Выходны (перанесены з 17.04.2004)",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2005, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2005, 5, 10)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2005, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2005, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2005, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 7)?,
                "Выходны (перанесены з 12.03.2005)",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2006, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2006, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2006, 5, 2)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2006, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2006, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2006, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 2)?,
                "Выходны (перанесены з 21.01.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 8)?,
                "Выходны (перанесены з 06.05.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 11, 6)?,
                "Выходны (перанесены з 04.11.2006)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2007, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2007, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2007, 4, 17)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2007, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2007, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 2)?,
                "Выходны (перанесены з 30.12.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 9)?,
                "Выходны (перанесены з 17.03.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 16)?,
                "Выходны (перанесены з 14.04.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 30)?,
                "Выходны (перанесены з 05.05.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 7, 2)?,
                "Выходны (перанесены з 07.07.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 24)?,
                "Выходны (перанесены з 22.12.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 31)?,
                "Выходны (перанесены з 29.12.2007)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2008, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2008, 5, 6)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2008, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2008, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 2)?,
                "Выходны (перанесены з 12.01.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 5)?,
                "Выходны (перанесены з 03.05.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 7, 4)?,
                "Выходны (перанесены з 28.06.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 26)?,
                "Выходны (перанесены з 20.12.2008)",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2009, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2009, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2009, 4, 28)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2009, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2009, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 2)?,
                "Выходны (перанесены з 10.01.2009)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 27)?,
                "Выходны (перанесены з 25.04.2009)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2010, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2010, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2010, 4, 13)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2010, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2010, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 8)?,
                "Выходны (перанесены з 23.01.2010)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 12)?,
                "Выходны (перанесены з 17.04.2010)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 10)?,
                "Выходны (перанесены з 15.05.2010)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2011, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2011, 5, 3)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2011, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2011, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 7)?,
                "Выходны (перанесены з 12.03.2011)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "Выходны (перанесены з 14.05.2011)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2012, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2012, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2012, 4, 24)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2012, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2012, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 9)?,
                "Выходны (перанесены з 11.03.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 23)?,
                "Выходны (перанесены з 28.04.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 7, 2)?,
                "Выходны (перанесены з 30.06.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 24)?,
                "Выходны (перанесены з 22.12.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "Выходны (перанесены з 29.12.2012)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2013, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2013, 5, 14)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2013, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 1, 2)?,
                "Выходны (перанесены з 05.01.2013)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 10)?,
                "Выходны (перанесены з 18.05.2013)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2014, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2014, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2014, 4, 29)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2014, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2014, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 2)?,
                "Выходны (перанесены з 04.01.2014)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Выходны (перанесены з 11.01.2014)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 30)?,
                "Выходны (перанесены з 03.05.2014)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 7, 4)?,
                "Выходны (перанесены з 12.07.2014)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 26)?,
                "Выходны (перанесены з 20.12.2014)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2015, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2015, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2015, 4, 21)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2015, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2015, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "Выходны (перанесены з 10.01.2015)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 20)?,
                "Выходны (перанесены з 25.04.2015)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2016, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2016, 5, 10)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2016, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2016, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 1, 8)?,
                "Выходны (перанесены з 16.01.2016)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 7)?,
                "Выходны (перанесены з 05.03.2016)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2017, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2017, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2017, 4, 25)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2017, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2017, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 2)?,
                "Выходны (перанесены з 21.01.2017)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 24)?,
                "Выходны (перанесены з 29.04.2017)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 8)?,
                "Выходны (перанесены з 06.05.2017)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 6)?,
                "Выходны (перанесены з 04.11.2017)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2018, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2018, 4, 17)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2018, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2018, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 1, 2)?,
                "Выходны (перанесены з 20.01.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 9)?,
                "Выходны (перанесены з 03.03.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 16)?,
                "Выходны (перанесены з 14.04.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "Выходны (перанесены з 28.04.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 7, 2)?,
                "Выходны (перанесены з 07.07.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Выходны (перанесены з 22.12.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Выходны (перанесены з 29.12.2018)",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2019, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2019, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2019, 5, 7)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2019, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2019, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 6)?,
                "Выходны (перанесены з 04.05.2019)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 8)?,
                "Выходны (перанесены з 11.05.2019)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 11, 8)?,
                "Выходны (перанесены з 16.11.2019)",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2020, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2020, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2020, 4, 28)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2020, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2020, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "Выходны (перанесены з 04.01.2020)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 27)?,
                "Выходны (перанесены з 04.04.2020)",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2021, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2021, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2021, 5, 11)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2021, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2021, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 1, 8)?,
                "Выходны (перанесены з 16.01.2021)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 10)?,
                "Выходны (перанесены з 15.05.2021)",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2022, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2022, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2022, 5, 3)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2022, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2022, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 7)?,
                "Выходны (перанесены з 12.03.2022)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 2)?,
                "Выходны (перанесены з 14.05.2022)",
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
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2023, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2023, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2023, 4, 25)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2023, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2023, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2023, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Выходны (перанесены з 29.04.2023)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 8)?,
                "Выходны (перанесены з 13.05.2023)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 11, 6)?,
                "Выходны (перанесены з 11.11.2023)",
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
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2024, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2024, 5, 14)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2024, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
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
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2025, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2025, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2025, 4, 29)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2025, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2025, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
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
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2026, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2026, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2026, 4, 21)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2026, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2026, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
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
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2027, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2027, 5, 11)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2027, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2027, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
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
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2028, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2028, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2028, 4, 25)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2028, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2028, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
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
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2029, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2029, 4, 17)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2029, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2029, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
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
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Новы год"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Новы год"),
            (
                NaiveDate::from_ymd_res(2030, 1, 7)?,
                "Нараджэнне Хрыстова (праваслаўнае Раство)",
            ),
            (NaiveDate::from_ymd_res(2030, 3, 8)?, "Дзень жанчын"),
            (NaiveDate::from_ymd_res(2030, 5, 7)?, "Радаўніца"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Свята працы"),
            (NaiveDate::from_ymd_res(2030, 5, 9)?, "Дзень Перамогі"),
            (
                NaiveDate::from_ymd_res(2030, 7, 3)?,
                "Дзень Незалежнасці Рэспублікі Беларусь (Дзень Рэспублікі)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 11, 7)?,
                "Дзень Кастрычніцкай рэвалюцыі",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 25)?,
                "Нараджэнне Хрыстова (каталіцкае Раство)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
