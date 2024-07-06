//! Madagascar
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Madagascar";
const COUNTY_CODE: Country = Country::MG;

/// Generate holiday map for Madagascar.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2000, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2000, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2000, 6, 1)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2000, 6, 12)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 28)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2000, 6, 18)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2000, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2001, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2001, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2001, 5, 24)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2001, 6, 4)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 27)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2001, 6, 17)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2001, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2002, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2002, 5, 20)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 26)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2002, 6, 16)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2002, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2003, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2003, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2003, 5, 29)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2003, 6, 9)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 25)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2003, 6, 15)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2003, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2004, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2004, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2004, 5, 20)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2004, 5, 31)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2004, 6, 6)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2004, 6, 20)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2004, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2005, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2005, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2005, 5, 5)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2005, 5, 16)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 29)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2005, 6, 19)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2005, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2006, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2006, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2006, 5, 25)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2006, 6, 5)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 28)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2006, 6, 18)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2006, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2007, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2007, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2007, 5, 17)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2007, 5, 28)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2007, 6, 3)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2007, 6, 17)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2007, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2008, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2008, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Alatsinain'ny paska"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Fetin'ny asa; Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2008, 5, 12)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 25)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2008, 6, 15)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2008, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2009, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2009, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2009, 5, 21)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2009, 6, 1)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2009, 6, 7)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2009, 6, 21)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2009, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2010, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2010, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2010, 5, 13)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2010, 5, 24)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 30)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2010, 6, 20)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2010, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2011, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2011, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2011, 6, 2)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2011, 6, 13)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 29)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2011, 6, 19)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2011, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2011, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2012, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2012, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2012, 5, 17)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2012, 5, 28)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2012, 6, 3)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2012, 6, 17)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2012, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2012, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2013, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2013, 5, 20)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 26)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2013, 6, 16)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2013, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2013, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2014, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2014, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2014, 5, 29)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2014, 6, 9)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 25)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2014, 6, 15)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2014, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2014, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2015, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2015, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2015, 5, 14)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2015, 5, 25)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 31)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2015, 6, 21)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2015, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2015, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2016, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2016, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2016, 5, 5)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2016, 5, 16)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 29)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2016, 6, 19)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2016, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2016, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2017, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2017, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2017, 5, 25)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2017, 6, 5)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 28)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2017, 6, 18)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2017, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2017, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2018, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2018, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2018, 5, 10)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2018, 5, 21)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 27)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2018, 6, 17)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2018, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2018, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2019, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2019, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2019, 5, 30)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2019, 6, 10)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 26)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2019, 6, 16)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2019, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2019, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2020, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2020, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2020, 5, 21)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2020, 6, 1)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2020, 6, 7)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2020, 6, 21)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2020, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2020, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2021, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2021, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2021, 5, 13)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2021, 5, 24)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 30)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2021, 6, 20)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2021, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2021, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2022, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2022, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2022, 5, 26)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2022, 6, 6)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 29)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2022, 6, 19)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2022, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2022, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2023, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2023, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2023, 5, 18)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2023, 5, 29)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2023, 6, 4)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2023, 6, 18)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2023, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2023, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2023, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2024, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2024, 5, 20)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 26)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2024, 6, 16)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2024, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2024, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2025, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2025, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2025, 5, 29)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2025, 6, 9)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 25)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2025, 6, 15)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2025, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2025, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2026, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2026, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2026, 5, 14)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2026, 5, 25)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 31)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2026, 6, 21)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2026, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2026, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2027, 3, 8)?, "Fetin'ny vehivavy"),
            (
                NaiveDate::from_ymd_res(2027, 3, 29)?,
                "Alatsinain'ny paska; Fetin'ny mahery fo",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2027, 5, 6)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2027, 5, 17)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 30)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2027, 6, 20)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2027, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2027, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2028, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2028, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2028, 5, 25)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2028, 6, 5)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 28)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2028, 6, 18)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2028, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2028, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2029, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2029, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2029, 5, 10)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2029, 5, 21)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 27)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2029, 6, 17)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2029, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2029, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Taom-baovao"),
            (NaiveDate::from_ymd_res(2030, 3, 8)?, "Fetin'ny vehivavy"),
            (NaiveDate::from_ymd_res(2030, 3, 29)?, "Fetin'ny mahery fo"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Fetin'ny paska"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Alatsinain'ny paska"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Fetin'ny asa"),
            (
                NaiveDate::from_ymd_res(2030, 5, 30)?,
                "Fiakaran'ny Jesosy kristy tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Pentekosta"),
            (
                NaiveDate::from_ymd_res(2030, 6, 10)?,
                "Alatsinain'ny pentekosta",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 26)?, "Fetin'ny reny"),
            (NaiveDate::from_ymd_res(2030, 6, 16)?, "Fetin'ny ray"),
            (
                NaiveDate::from_ymd_res(2030, 6, 26)?,
                "Fetin'ny fahaleovantena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 8, 15)?,
                "Fiakaran'ny Masina Maria tany an-danitra",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Fetin'ny olo-masina"),
            (NaiveDate::from_ymd_res(2030, 12, 11)?, "Fetin'ny Repoblika"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Fetin'ny noely"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
