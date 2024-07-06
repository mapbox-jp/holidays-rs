//! Serbia
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Serbia";
const COUNTY_CODE: Country = Country::RS;

/// Generate holiday map for Serbia.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2000, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2000, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2000, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2000, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 1)?,
                "Други дан Васкрса; Празник рада",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2000, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 28)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2000, 4, 29)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2000, 4, 30)?, "Васкрс"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2001, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2001, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2001, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2001, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2001, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (
                NaiveDate::from_ymd_res(2001, 11, 12)?,
                "Дан примирја у Првом светском рату (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2002, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2002, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2002, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2002, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2002, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 3)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2002, 5, 4)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2003, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2003, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2003, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2003, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2003, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 25)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2003, 4, 26)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2003, 4, 28)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2004, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2004, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2004, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2004, 5, 3)?,
                "Празник рада (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2004, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2005, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2005, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2005, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2005, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Васкрс; Празник рада"),
            (
                NaiveDate::from_ymd_res(2005, 5, 3)?,
                "Празник рада (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 2)?,
                "Други дан Васкрса; Празник рада",
            ),
            (
                NaiveDate::from_ymd_res(2005, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 29)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2005, 4, 30)?, "Велика субота"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2006, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2006, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2006, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2006, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2006, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2006, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 21)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2006, 4, 22)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2006, 4, 24)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2007, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2007, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2007, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2007, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2007, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 12)?,
                "Дан примирја у Првом светском рату (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2008, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2008, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2008, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2008, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 25)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2008, 4, 26)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2008, 4, 28)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2009, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2009, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2009, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2009, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2009, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 17)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2009, 4, 18)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2010, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2010, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2010, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2010, 5, 3)?,
                "Празник рада (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2010, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2011, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2011, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2011, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2011, 5, 3)?,
                "Празник рада (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2011, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2012, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2012, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2012, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2012, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2012, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 12)?,
                "Дан примирја у Првом светском рату (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 13)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2012, 4, 14)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2013, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2013, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2013, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2013, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 3)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2013, 5, 4)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2014, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2014, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2014, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2014, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2015, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2015, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2015, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 10)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2015, 4, 11)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2015, 4, 13)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2016, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2016, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Васкрс; Празник рада"),
            (
                NaiveDate::from_ymd_res(2016, 5, 3)?,
                "Празник рада (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 2)?,
                "Други дан Васкрса; Празник рада",
            ),
            (
                NaiveDate::from_ymd_res(2016, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 29)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2016, 4, 30)?, "Велика субота"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2017, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2017, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2017, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2017, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2017, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2018, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2018, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2018, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2018, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 12)?,
                "Дан примирја у Првом светском рату (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 6)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2018, 4, 7)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2018, 4, 8)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2018, 4, 9)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2019, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2019, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2019, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2019, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 26)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2019, 4, 27)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2019, 4, 28)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2019, 4, 29)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2020, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2020, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2020, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2020, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2020, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 17)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2020, 4, 18)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2020, 4, 19)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2020, 4, 20)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2021, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2021, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 1)?,
                "Велика субота; Празник рада",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "Васкрс; Празник рада"),
            (
                NaiveDate::from_ymd_res(2021, 5, 4)?,
                "Празник рада (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2021, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 30)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2022, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2022, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2022, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2022, 5, 3)?,
                "Празник рада (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2022, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 22)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2022, 4, 23)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2022, 4, 24)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2022, 4, 25)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2023, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2023, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2023, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2023, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2023, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 14)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2023, 4, 15)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2023, 4, 16)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2023, 4, 17)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2024, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2024, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2024, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2024, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 3)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2024, 5, 4)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2025, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2025, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2025, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2025, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2026, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 17)?,
                "Дан државности Србије (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2026, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2026, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2026, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 10)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2026, 4, 11)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2026, 4, 12)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2026, 4, 13)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2027, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2027, 2, 16)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 1)?,
                "Велика субота; Празник рада",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "Васкрс; Празник рада"),
            (
                NaiveDate::from_ymd_res(2027, 5, 4)?,
                "Празник рада (слободан дан)",
            ),
            (
                NaiveDate::from_ymd_res(2027, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 30)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "Нова година"),
            (
                NaiveDate::from_ymd_res(2028, 1, 3)?,
                "Нова година (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2028, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2028, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2028, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2028, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2029, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2029, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2029, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2029, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 12)?,
                "Дан примирја у Првом светском рату (слободан дан)",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 6)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2029, 4, 7)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2029, 4, 8)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2029, 4, 9)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Нова година"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "Нова година"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "Божић"),
            (
                NaiveDate::from_ymd_res(2030, 2, 15)?,
                "Дан државности Србије",
            ),
            (
                NaiveDate::from_ymd_res(2030, 2, 16)?,
                "Дан државности Србије",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Празник рада"),
            (NaiveDate::from_ymd_res(2030, 5, 2)?, "Празник рада"),
            (
                NaiveDate::from_ymd_res(2030, 11, 11)?,
                "Дан примирја у Првом светском рату",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 26)?, "Велики петак"),
            (NaiveDate::from_ymd_res(2030, 4, 27)?, "Велика субота"),
            (NaiveDate::from_ymd_res(2030, 4, 28)?, "Васкрс"),
            (NaiveDate::from_ymd_res(2030, 4, 29)?, "Други дан Васкрса"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
