//! Croatia
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "Croatia";
const COUNTY_CODE: Country = Country::HR;

/// Generate holiday map for Croatia.
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
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2000, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2000, 5, 30)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2000, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2000, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2001, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2001, 5, 30)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2001, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2001, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2001, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2002, 5, 30)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2002, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2002, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2002, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2002, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2002, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2003, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2003, 6, 19)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2003, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2003, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2003, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2003, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2004, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2004, 6, 10)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2004, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2004, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2004, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2004, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2004, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2005, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2005, 5, 26)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2005, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2005, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2005, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2005, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2006, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2006, 6, 15)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2006, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2006, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2006, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2006, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2007, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2007, 6, 7)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2007, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2007, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2007, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti",
            ),
            (NaiveDate::from_ymd_res(2007, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2007, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2008, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2008, 5, 22)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2008, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2008, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2008, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2008, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2009, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2009, 6, 11)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2009, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2009, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2009, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2009, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2009, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2010, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2010, 6, 3)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2010, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2010, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2010, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2010, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2010, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2011, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2011, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2011, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2011, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2011, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2012, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2012, 6, 7)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2012, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2012, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2012, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2012, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2012, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2013, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2013, 5, 30)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2013, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2013, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2013, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2013, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2013, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2014, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2014, 6, 19)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2014, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2014, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2014, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2014, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2015, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2015, 6, 4)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2015, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2015, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2015, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2015, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2016, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2016, 5, 26)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2016, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2016, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2016, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2016, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2017, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2017, 6, 15)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2017, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2017, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2017, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2017, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2018, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2018, 5, 31)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2018, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2018, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2018, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2018, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2018, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Nova godina"),
            (
                NaiveDate::from_ymd_res(2019, 1, 6)?,
                "Bogojavljenje ili Sveta tri kralja",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2019, 6, 20)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2019, 6, 25)?, "Dan državnosti"),
            (
                NaiveDate::from_ymd_res(2019, 6, 22)?,
                "Dan antifašističke borbe",
            ),
            (
                NaiveDate::from_ymd_res(2019, 8, 5)?,
                "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja",
            ),
            (NaiveDate::from_ymd_res(2019, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2019, 10, 8)?, "Dan neovisnosti"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2020, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2020, 6, 11)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2020, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2020, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2020, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2020, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2020, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2021, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2021, 6, 3)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2021, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2021, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2021, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2021, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2021, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2022, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2022, 6, 16)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2022, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2022, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2022, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2022, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2023, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2023, 6, 8)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2023, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2023, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2023, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2023, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2024, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2024, 5, 30)?, "Dan državnosti; Tijelovo"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2024, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2024, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2024, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2024, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2025, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2025, 6, 19)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2025, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2025, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2025, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2025, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2026, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2026, 6, 4)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2026, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2026, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2026, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2026, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2026, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2027, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2027, 5, 27)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2027, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2027, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2027, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2027, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2027, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2028, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2028, 6, 15)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2028, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2028, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2028, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2028, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2029, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2029, 5, 31)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2029, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2029, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2029, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2029, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2029, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Nova godina"),
            (NaiveDate::from_ymd_res(2030, 1, 6)?, "Bogojavljenje ili Sveta tri kralja"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Uskrs"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Uskrsni ponedjeljak"),
            (NaiveDate::from_ymd_res(2030, 6, 20)?, "Tijelovo"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Praznik rada"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Dan državnosti"),
            (NaiveDate::from_ymd_res(2030, 6, 22)?, "Dan antifašističke borbe"),
            (NaiveDate::from_ymd_res(2030, 8, 5)?, "Dan pobjede i domovinske zahvalnosti i Dan hrvatskih branitelja"),
            (NaiveDate::from_ymd_res(2030, 8, 15)?, "Velika Gospa"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Svi sveti"),
            (NaiveDate::from_ymd_res(2030, 11, 18)?, "Dan sjećanja na žrtve Domovinskog rata i Dan sjećanja na žrtvu Vukovara i Škabrnje"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Božić"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Sveti Stjepan"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
