//! Ukraine
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Ukraine";
const COUNTY_CODE: Country = Country::UA;

/// Generate holiday map for Ukraine.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2000, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 30)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2000, 6, 18)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2000, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 24)?,
                "День незалежності України",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 3)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2000, 5, 3)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 19)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2000, 5, 8)?,
                "Вихідний день (перенесено з 06.05.2000)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 25)?,
                "Вихідний день (перенесено з 27.08.2000)",
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
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2001, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2001, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2001, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2001, 1, 8)?,
                "Різдво Христове (за юліанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 16)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2001, 3, 9)?,
                "Вихідний день (перенесено з 11.03.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 4, 30)?,
                "Вихідний день (перенесено з 28.04.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 10)?,
                "Вихідний день (перенесено з 05.05.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 11)?,
                "Вихідний день (перенесено з 06.05.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 6, 29)?,
                "Вихідний день (перенесено з 23.06.2001)",
            ),
            (
                NaiveDate::from_ymd_res(2001, 12, 31)?,
                "Вихідний день (перенесено з 29.12.2001)",
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
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2002, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2002, 6, 23)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2002, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2002, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 6)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2002, 6, 24)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2002, 8, 26)?,
                "День незалежності України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 3)?,
                "Вихідний день (перенесено з 11.05.2002)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 30)?,
                "Вихідний день (перенесено з 28.12.2002)",
            ),
            (
                NaiveDate::from_ymd_res(2002, 12, 31)?,
                "Вихідний день (перенесено з 29.12.2002)",
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
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2003, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2003, 6, 15)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2003, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2003, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2003, 3, 10)?,
                "Міжнародний жіночий день (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 4, 28)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 16)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2003, 6, 30)?,
                "День Конституції України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 25)?,
                "День незалежності України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Вихідний день (перенесено з 04.01.2003)",
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
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2004, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2004, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2004, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 12)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 3)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 4)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 10)?,
                "День перемоги (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2004, 1, 2)?,
                "Вихідний день (перенесено з 10.01.2004)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 5)?,
                "Вихідний день (перенесено з 17.01.2004)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 1, 6)?,
                "Вихідний день (перенесено з 31.01.2004)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 23)?,
                "Вихідний день (перенесено з 21.08.2004)",
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
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2005, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 1)?,
                "Великдень (Пасха); День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 19)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2005, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 24)?,
                "День незалежності України",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 3)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2005, 5, 3)?,
                "Великдень (Пасха) (вихідний); День міжнародної солідарності трудящих (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2005, 6, 20)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2005, 3, 7)?,
                "Вихідний день (перенесено з 05.03.2005)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 10)?,
                "Вихідний день (перенесено з 14.05.2005)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 6, 27)?,
                "Вихідний день (перенесено з 25.06.2005)",
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
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2006, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2006, 6, 11)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2006, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2006, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 24)?,
                "День незалежності України",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2006, 1, 9)?,
                "Різдво Христове (за юліанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 4, 24)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 12)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "Вихідний день (перенесено з 21.01.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 4)?,
                "Вихідний день (перенесено з 04.02.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 5)?,
                "Вихідний день (перенесено з 18.02.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 1, 6)?,
                "Вихідний день (перенесено з 11.03.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 8)?,
                "Вихідний день (перенесено з 06.05.2006)",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 25)?,
                "Вихідний день (перенесено з 09.09.2006)",
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
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2007, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2007, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 8)?,
                "Різдво Христове (за юліанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2007, 1, 2)?,
                "Вихідний день (перенесено з 20.01.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 3)?,
                "Вихідний день (перенесено з 27.01.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 4)?,
                "Вихідний день (перенесено з 10.02.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 1, 5)?,
                "Вихідний день (перенесено з 24.02.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 3, 9)?,
                "Вихідний день (перенесено з 03.03.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 4, 30)?,
                "Вихідний день (перенесено з 28.04.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 29)?,
                "Вихідний день (перенесено з 16.06.2007)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 31)?,
                "Вихідний день (перенесено з 29.12.2007)",
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
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2008, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2008, 6, 15)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2008, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2008, 3, 10)?,
                "Міжнародний жіночий день (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 28)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2008, 6, 16)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2008, 6, 30)?,
                "День Конституції України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 25)?,
                "День незалежності України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 2)?,
                "Вихідний день (перенесено з 12.01.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 3)?,
                "Вихідний день (перенесено з 26.01.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 1, 4)?,
                "Вихідний день (перенесено з 09.02.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 29)?,
                "Вихідний день (перенесено з 17.05.2008)",
            ),
            (
                NaiveDate::from_ymd_res(2008, 4, 30)?,
                "Вихідний день (перенесено з 31.05.2008)",
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
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2009, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2009, 6, 7)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2009, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2009, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "Міжнародний жіночий день (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 4, 20)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 4)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 11)?,
                "День перемоги (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 8)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2009, 6, 29)?,
                "День Конституції України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 2)?,
                "Вихідний день (перенесено з 10.01.2009)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 5)?,
                "Вихідний день (перенесено з 24.01.2009)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 1, 6)?,
                "Вихідний день (перенесено з 07.02.2009)",
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
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2010, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2010, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2010, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2010, 4, 5)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 3)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 4)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 10)?,
                "День перемоги (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2010, 1, 4)?,
                "Вихідний день (перенесено з 30.01.2010)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 5)?,
                "Вихідний день (перенесено з 13.02.2010)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 6)?,
                "Вихідний день (перенесено з 27.02.2010)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 1, 8)?,
                "Вихідний день (перенесено з 13.03.2010)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 23)?,
                "Вихідний день (перенесено з 21.08.2010)",
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
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2011, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2011, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2011, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 24)?,
                "День незалежності України",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 3)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2011, 4, 25)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 3)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2011, 3, 7)?,
                "Вихідний день (перенесено з 12.03.2011)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 6, 27)?,
                "Вихідний день (перенесено з 25.06.2011)",
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
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2012, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2012, 6, 3)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2012, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2012, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 24)?,
                "День незалежності України",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2012, 1, 9)?,
                "Різдво Христове (за юліанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 16)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 4)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2012, 3, 9)?,
                "Вихідний день (перенесено з 03.03.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 20)?,
                "Вихідний день (перенесено з 28.04.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 6, 29)?,
                "Вихідний день (перенесено з 07.07.2012)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "Вихідний день (перенесено з 29.12.2012)",
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
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2013, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2013, 6, 23)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2013, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2013, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 6)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2013, 6, 24)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2013, 8, 26)?,
                "День незалежності України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 3)?,
                "Вихідний день (перенесено з 18.05.2013)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 10)?,
                "Вихідний день (перенесено з 01.06.2013)",
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
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2014, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2014, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2014, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2014, 3, 10)?,
                "Міжнародний жіночий день (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 4, 21)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2014, 6, 30)?,
                "День Конституції України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 25)?,
                "День незалежності України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 2)?,
                "Вихідний день (перенесено з 11.01.2014)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 3)?,
                "Вихідний день (перенесено з 25.01.2014)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Вихідний день (перенесено з 08.02.2014)",
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
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2015, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2015, 5, 31)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2015, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 9)?, "День перемоги"),
            (
                NaiveDate::from_ymd_res(2015, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 14)?,
                "День захисника України",
            ),
            (
                NaiveDate::from_ymd_res(2015, 3, 9)?,
                "Міжнародний жіночий день (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 4, 13)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 4)?,
                "День міжнародної солідарності трудящих (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 11)?,
                "День перемоги (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2015, 6, 1)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2015, 6, 29)?,
                "День Конституції України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "Вихідний день (перенесено з 17.01.2015)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 8)?,
                "Вихідний день (перенесено з 31.01.2015)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 1, 9)?,
                "Вихідний день (перенесено з 14.02.2015)",
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
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2016, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 1)?,
                "Великдень (Пасха); День міжнародної солідарності трудящих",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 19)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 9)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 14)?,
                "День захисника України",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 3)?,
                "Великдень (Пасха) (вихідний); День міжнародної солідарності трудящих (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2016, 6, 20)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2016, 1, 8)?,
                "Вихідний день (перенесено з 16.01.2016)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 3, 7)?,
                "Вихідний день (перенесено з 12.03.2016)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 6, 27)?,
                "Вихідний день (перенесено з 02.07.2016)",
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
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2017, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Трійця"),
            (
                NaiveDate::from_ymd_res(2017, 5, 1)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 2)?,
                "День міжнародної солідарності трудящих",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 9)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 14)?,
                "День захисника України",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 25)?,
                "Різдво Христове (за григоріанським календарем)",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2017, 1, 9)?,
                "Різдво Христове (за юліанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 4, 17)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2017, 10, 16)?,
                "День захисника України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 8)?,
                "Вихідний день (перенесено з 13.05.2017)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 25)?,
                "Вихідний день (перенесено з 19.08.2017)",
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
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2018, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 8)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2018, 5, 27)?, "Трійця"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "День праці"),
            (
                NaiveDate::from_ymd_res(2018, 5, 9)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 14)?,
                "День захисника України",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 25)?,
                "Різдво Христове (за григоріанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 1, 8)?,
                "Різдво Христове (за юліанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 9)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 28)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2018, 10, 15)?,
                "День захисника України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 3, 9)?,
                "Вихідний день (перенесено з 03.03.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "Вихідний день (перенесено з 05.05.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 6, 29)?,
                "Вихідний день (перенесено з 23.06.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Вихідний день (перенесено з 22.12.2018)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Вихідний день (перенесено з 29.12.2018)",
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
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2019, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 28)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2019, 6, 16)?, "Трійця"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "День праці"),
            (
                NaiveDate::from_ymd_res(2019, 5, 9)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 14)?,
                "День захисника України",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 25)?,
                "Різдво Христове (за григоріанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 29)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 17)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2019, 8, 26)?,
                "День незалежності України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 4, 30)?,
                "Вихідний день (перенесено з 11.05.2019)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 30)?,
                "Вихідний день (перенесено з 21.12.2019)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 31)?,
                "Вихідний день (перенесено з 28.12.2019)",
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
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2020, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 19)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2020, 6, 7)?, "Трійця"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "День праці"),
            (
                NaiveDate::from_ymd_res(2020, 5, 9)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 14)?,
                "День захисника України",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 25)?,
                "Різдво Христове (за григоріанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 3, 9)?,
                "Міжнародний жіночий день (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 4, 20)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 11)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 8)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2020, 6, 29)?,
                "День Конституції України (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 1, 6)?,
                "Вихідний день (перенесено з 11.01.2020)",
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
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2021, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "Великдень (Пасха)"),
            (NaiveDate::from_ymd_res(2021, 6, 20)?, "Трійця"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "День праці"),
            (
                NaiveDate::from_ymd_res(2021, 5, 9)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 6, 28)?,
                "День Конституції України",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 24)?,
                "День незалежності України",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 14)?,
                "День захисників і захисниць України",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 25)?,
                "Різдво Христове (за григоріанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 3)?,
                "День праці (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 4)?,
                "Великдень (Пасха) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 10)?,
                "День перемоги над нацизмом у Другій світовій війні (День перемоги) (вихідний)",
            ),
            (NaiveDate::from_ymd_res(2021, 6, 21)?, "Трійця (вихідний)"),
            (
                NaiveDate::from_ymd_res(2021, 12, 27)?,
                "Різдво Христове (за григоріанським календарем) (вихідний)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 1, 8)?,
                "Вихідний день (перенесено з 16.01.2021)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 23)?,
                "Вихідний день (перенесено з 28.08.2021)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 15)?,
                "Вихідний день (перенесено з 23.10.2021)",
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
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Новий рік"),
            (
                NaiveDate::from_ymd_res(2022, 1, 7)?,
                "Різдво Христове (за юліанським календарем)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 3, 8)?,
                "Міжнародний жіночий день",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 3)?, "Новий рік (вихідний)"),
            (
                NaiveDate::from_ymd_res(2022, 3, 7)?,
                "Вихідний день (перенесено з 12.03.2022)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
