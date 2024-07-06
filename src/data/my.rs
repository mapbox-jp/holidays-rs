//! Malaysia
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Malaysia";
const COUNTY_CODE: Country = Country::MY;

/// Generate holiday map for Malaysia.
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
            (
                NaiveDate::from_ymd_res(2000, 2, 5)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 6)?,
                "Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 18)?,
                "Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2000, 6, 3)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2000, 4, 6)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 6, 14)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 8)?,
                "Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 27)?,
                "Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 9)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 12, 28)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 3, 16)?,
                "Hari Raya Qurban (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 1, 10)?,
                "Cuti Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 7)?,
                "Cuti Tahun Baharu Cina (Hari Kedua) (anggaran)",
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
            (NaiveDate::from_ymd_res(2001, 1, 24)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2001, 1, 25)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 7)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2001, 6, 2)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2001, 3, 26)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 17)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2001, 12, 18)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2001, 3, 6)?, "Hari Raya Qurban"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 2, 12)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2002, 2, 13)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 27)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2002, 6, 1)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2002, 3, 15)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2002, 5, 24)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 6)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2002, 12, 7)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2002, 2, 23)?, "Hari Raya Qurban"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 2, 1)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2003, 2, 2)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 15)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2003, 6, 7)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2003, 3, 5)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2003, 5, 14)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 26)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2003, 11, 27)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2003, 2, 12)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2003, 2, 3)?,
                "Cuti Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2003, 9, 1)?, "Cuti Hari Kebangsaan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 22)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2004, 1, 23)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 3)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2004, 6, 5)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2004, 2, 22)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2004, 5, 2)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 14)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2004, 2, 2)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2004, 5, 4)?,
                "Cuti Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 16)?,
                "Cuti Hari Raya Puasa",
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
            (NaiveDate::from_ymd_res(2005, 2, 9)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2005, 2, 10)?,
                "Awal Muharam; Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 22)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2005, 6, 4)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2005, 4, 21)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 3)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2005, 11, 4)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 21)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "Cuti Hari Pekerja"),
            (NaiveDate::from_ymd_res(2005, 5, 23)?, "Cuti Hari Wesak"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Cuti Hari Krismas"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 29)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2006, 1, 30)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 12)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2006, 6, 3)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2006, 1, 31)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2006, 4, 11)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 24)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2006, 10, 25)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 10)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2006, 2, 1)?,
                "Cuti Tahun Baharu Cina",
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
            (NaiveDate::from_ymd_res(2007, 2, 18)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2007, 2, 19)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 1)?,
                "Hari Pekerja; Hari Wesak",
            ),
            (
                NaiveDate::from_ymd_res(2007, 6, 2)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2007, 1, 20)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2007, 3, 31)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 13)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2007, 10, 14)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 20)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2007, 2, 20)?,
                "Cuti Tahun Baharu Cina",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 15)?,
                "Cuti Hari Raya Puasa (Hari Kedua)",
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
            (NaiveDate::from_ymd_res(2008, 2, 7)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2008, 2, 8)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 19)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2008, 6, 7)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2008, 1, 10)?, "Awal Muharam"),
            (NaiveDate::from_ymd_res(2008, 12, 29)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2008, 3, 20)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 1)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2008, 10, 2)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 9)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2008, 9, 1)?, "Cuti Hari Kebangsaan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 26)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2009, 1, 27)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 9)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2009, 6, 6)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2009, 12, 18)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2009, 3, 9)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2009, 9, 20)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2009, 9, 21)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 28)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2009, 9, 22)?,
                "Cuti Hari Raya Puasa",
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
            (NaiveDate::from_ymd_res(2010, 2, 14)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 28)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2010, 6, 5)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2010, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2010, 12, 8)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2010, 2, 26)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2010, 9, 10)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2010, 9, 11)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 17)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2010, 2, 16)?,
                "Cuti Tahun Baharu Cina",
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
            (NaiveDate::from_ymd_res(2011, 2, 3)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2011, 2, 4)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 17)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2011, 6, 4)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 31)?,
                "Hari Kebangsaan; Hari Raya Puasa",
            ),
            (NaiveDate::from_ymd_res(2011, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2011, 11, 27)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2011, 2, 16)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (
                NaiveDate::from_ymd_res(2011, 9, 1)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 7)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "Cuti Hari Pekerja"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Cuti Hari Krismas"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 23)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2012, 1, 24)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 5)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2012, 6, 2)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2012, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2012, 11, 15)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2012, 2, 5)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 19)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 26)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2012, 2, 6)?,
                "Cuti Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 21)?,
                "Cuti Hari Raya Puasa",
            ),
            (NaiveDate::from_ymd_res(2012, 9, 17)?, "Cuti Hari Malaysia"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 2, 10)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2013, 2, 11)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 24)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2013, 6, 1)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2013, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2013, 11, 5)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2013, 1, 24)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 8)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2013, 8, 9)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 15)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2013, 2, 12)?,
                "Cuti Tahun Baharu Cina",
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
            (NaiveDate::from_ymd_res(2014, 1, 31)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2014, 2, 1)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 13)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2014, 6, 7)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2014, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2014, 10, 25)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2014, 1, 14)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2014, 7, 28)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2014, 7, 29)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 5)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2014, 9, 1)?, "Cuti Hari Kebangsaan"),
            (
                NaiveDate::from_ymd_res(2014, 10, 6)?,
                "Cuti Hari Raya Qurban",
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
            (NaiveDate::from_ymd_res(2015, 2, 19)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2015, 2, 20)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 3)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2015, 6, 6)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2015, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2015, 10, 14)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2015, 1, 3)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 24)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2015, 7, 17)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2015, 7, 18)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2015, 9, 24)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2015, 5, 4)?, "Cuti Hari Wesak"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 2, 8)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2016, 2, 9)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 21)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2016, 6, 4)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2016, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2016, 10, 2)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2016, 12, 12)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2016, 7, 6)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2016, 7, 7)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2016, 9, 12)?, "Hari Raya Qurban"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "Cuti Hari Pekerja"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Cuti Hari Krismas"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 28)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2017, 1, 29)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 10)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2017, 9, 9)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2017, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2017, 9, 22)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2017, 12, 1)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2017, 9, 1)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2017, 4, 24)?,
                "Hari Pertabalan Yang di-Pertuan Agong ke-15",
            ),
            (
                NaiveDate::from_ymd_res(2017, 9, 4)?,
                "Cuti tambahan sempena memperingati SAT 2017",
            ),
            (
                NaiveDate::from_ymd_res(2017, 1, 30)?,
                "Cuti Tahun Baharu Cina (Hari Kedua)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 6, 27)?,
                "Cuti Hari Raya Puasa",
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
            (NaiveDate::from_ymd_res(2018, 2, 16)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2018, 2, 17)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 29)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2018, 9, 9)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2018, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2018, 9, 11)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2018, 11, 20)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2018, 6, 15)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2018, 6, 16)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 22)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2018, 5, 9)?,
                "Cuti Peristiwa (pilihan raya umum)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 10)?,
                "Cuti Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2018, 9, 17)?, "Cuti Hari Malaysia"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 2, 5)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2019, 2, 6)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 19)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2019, 9, 9)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2019, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2019, 9, 1)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2019, 11, 9)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 5)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2019, 6, 6)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 11)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2019, 7, 30)?,
                "Hari Pertabalan Yang di-Pertuan Agong ke-16",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 20)?, "Cuti Hari Wesak"),
            (
                NaiveDate::from_ymd_res(2019, 8, 12)?,
                "Cuti Hari Raya Qurban",
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
            (NaiveDate::from_ymd_res(2020, 1, 25)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2020, 1, 26)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 7)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2020, 6, 8)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2020, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2020, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2020, 8, 20)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2020, 10, 29)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 24)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2020, 5, 25)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2020, 7, 31)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2020, 1, 27)?,
                "Cuti Tahun Baharu Cina (Hari Kedua)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 26)?,
                "Cuti Hari Raya Puasa",
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
            (NaiveDate::from_ymd_res(2021, 2, 12)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2021, 2, 13)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 26)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2021, 6, 7)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2021, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2021, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2021, 8, 10)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2021, 10, 19)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2021, 5, 14)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2021, 7, 20)?, "Hari Raya Qurban"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 2, 1)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2022, 2, 2)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 15)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2022, 6, 6)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2022, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2022, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2022, 7, 30)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2022, 10, 10)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2022, 7, 10)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2022, 11, 18)?,
                "Cuti Peristiwa (pilihan raya umum)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 11, 19)?,
                "Cuti Peristiwa (pilihan raya umum)",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 28)?, "Cuti Peristiwa"),
            (NaiveDate::from_ymd_res(2022, 5, 4)?, "Cuti Hari Pekerja"),
            (NaiveDate::from_ymd_res(2022, 5, 16)?, "Cuti Hari Wesak"),
            (
                NaiveDate::from_ymd_res(2022, 7, 11)?,
                "Cuti Hari Raya Qurban",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Cuti Hari Krismas"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 22)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2023, 1, 23)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 4)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2023, 6, 5)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2023, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2023, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2023, 7, 19)?, "Awal Muharam"),
            (
                NaiveDate::from_ymd_res(2023, 9, 28)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 22)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2023, 4, 23)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 29)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2023, 4, 21)?,
                "Hari Raya Puasa (pergantian hari)",
            ),
            (
                NaiveDate::from_ymd_res(2023, 1, 24)?,
                "Cuti Tahun Baharu Cina",
            ),
            (
                NaiveDate::from_ymd_res(2023, 4, 24)?,
                "Cuti Hari Raya Puasa (Hari Kedua)",
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
            (NaiveDate::from_ymd_res(2024, 2, 10)?, "Tahun Baharu Cina"),
            (
                NaiveDate::from_ymd_res(2024, 2, 11)?,
                "Tahun Baharu Cina (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 22)?, "Hari Wesak"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2024, 6, 3)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2024, 8, 31)?, "Hari Kebangsaan"),
            (
                NaiveDate::from_ymd_res(2024, 9, 16)?,
                "Hari Keputeraan Nabi Muhammad S.A.W.; Hari Malaysia",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Hari Krismas"),
            (NaiveDate::from_ymd_res(2024, 7, 7)?, "Awal Muharam"),
            (NaiveDate::from_ymd_res(2024, 4, 10)?, "Hari Raya Puasa"),
            (
                NaiveDate::from_ymd_res(2024, 4, 11)?,
                "Hari Raya Puasa (Hari Kedua)",
            ),
            (NaiveDate::from_ymd_res(2024, 6, 17)?, "Hari Raya Qurban"),
            (
                NaiveDate::from_ymd_res(2024, 2, 12)?,
                "Cuti Tahun Baharu Cina (Hari Kedua)",
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
            (
                NaiveDate::from_ymd_res(2025, 1, 29)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 1, 30)?,
                "Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 11)?,
                "Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2025, 6, 2)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2025, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2025, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 9, 4)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 30)?,
                "Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 3, 31)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 6, 6)?,
                "Hari Raya Qurban (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 4, 1)?,
                "Cuti Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 12)?,
                "Cuti Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2025, 9, 1)?, "Cuti Hari Kebangsaan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (
                NaiveDate::from_ymd_res(2026, 2, 17)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 18)?,
                "Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 1)?,
                "Hari Pekerja; Hari Wesak (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 6, 1)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2026, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2026, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2026, 6, 16)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 25)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 20)?,
                "Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 3, 21)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 27)?,
                "Hari Raya Qurban (anggaran)",
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
            (
                NaiveDate::from_ymd_res(2027, 2, 6)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 7)?,
                "Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 20)?,
                "Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2027, 6, 7)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2027, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2027, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2027, 6, 6)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 14)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 9)?,
                "Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 3, 10)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 16)?,
                "Hari Raya Qurban (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 8)?,
                "Cuti Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Cuti Hari Raya Qurban (anggaran)",
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
            (
                NaiveDate::from_ymd_res(2028, 1, 26)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 1, 27)?,
                "Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2028, 6, 5)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2028, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2028, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 3)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 26)?,
                "Hari Raya Puasa (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 27)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 5)?,
                "Hari Raya Qurban (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 28)?,
                "Cuti Hari Raya Puasa (Hari Kedua) (anggaran)",
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
            (
                NaiveDate::from_ymd_res(2029, 2, 13)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 14)?,
                "Hari Raya Puasa (anggaran); Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 27)?,
                "Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2029, 6, 4)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2029, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2029, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2029, 5, 14)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 7, 24)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 4, 24)?,
                "Hari Raya Qurban (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 28)?,
                "Cuti Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2029, 9, 17)?, "Cuti Hari Malaysia"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (
                NaiveDate::from_ymd_res(2030, 2, 3)?,
                "Tahun Baharu Cina (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 4)?,
                "Hari Raya Puasa (anggaran); Tahun Baharu Cina (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 16)?,
                "Hari Wesak (anggaran)",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Hari Pekerja"),
            (
                NaiveDate::from_ymd_res(2030, 6, 3)?,
                "Hari Keputeraan Rasmi Seri Paduka Baginda Yang di-Pertuan Agong",
            ),
            (NaiveDate::from_ymd_res(2030, 8, 31)?, "Hari Kebangsaan"),
            (NaiveDate::from_ymd_res(2030, 9, 16)?, "Hari Malaysia"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Hari Krismas"),
            (
                NaiveDate::from_ymd_res(2030, 5, 3)?,
                "Awal Muharam (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 7, 13)?,
                "Hari Keputeraan Nabi Muhammad S.A.W. (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 5)?,
                "Hari Raya Puasa (Hari Kedua) (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 4, 13)?,
                "Hari Raya Qurban (anggaran)",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 6)?,
                "Cuti Tahun Baharu Cina (anggaran)",
            ),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
