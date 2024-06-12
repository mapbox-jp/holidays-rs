//! Latvia
use super::*;

/// Generate holiday map for Latvia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2000, 4, 21)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Darba svētki"),
            (NaiveDate::from_ymd_res(2000, 5, 14)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2000, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2000, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2000, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2000, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Darba svētki"),
            (NaiveDate::from_ymd_res(2001, 5, 13)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2001, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2001, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2001, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2001, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2002, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 12)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2002, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2002, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2002, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2002, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2003, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 11)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2003, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2003, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2003, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2003, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2004, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 9)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2004, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2004, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2004, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2004, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2005, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 8)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2005, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2005, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2005, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2005, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2006, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 14)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2006, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2006, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2006, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2006, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2007, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 13)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2007, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2007, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2007, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2007, 11, 19)?,
                "Latvijas Republikas proklamēšanas diena (brīvdiena)",
            ),
            (
                NaiveDate::from_ymd_res(2007, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2007, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2008, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 5)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2008, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2008, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2008, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2008, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2008, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2009, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 10)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2009, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2009, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2009, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2009, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2009, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2010, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 9)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2010, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2010, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2010, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2010, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2010, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2011, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 8)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2011, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2011, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2011, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2011, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2011, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2012, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 13)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2012, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2012, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2012, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 19)?,
                "Latvijas Republikas proklamēšanas diena (brīvdiena)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2012, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2013, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 6)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 12)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2013, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2013, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2013, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2013, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2014, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 5)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 11)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2014, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2014, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2014, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2014, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2015, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 10)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2015, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2015, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2015, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2015, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2016, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 8)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2016, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2016, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2016, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2016, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2016, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2017, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 14)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2017, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2017, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2017, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2017, 11, 20)?,
                "Latvijas Republikas proklamēšanas diena (brīvdiena)",
            ),
            (
                NaiveDate::from_ymd_res(2017, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2017, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2018, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 13)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2018, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2018, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2018, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 19)?,
                "Latvijas Republikas proklamēšanas diena (brīvdiena)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2018, 12, 31)?, "Vecgada vakars"),
            (
                NaiveDate::from_ymd_res(2018, 7, 9)?,
                "Vispārējo latviešu Dziesmu un deju svētku noslēguma dienu",
            ),
            (
                NaiveDate::from_ymd_res(2018, 9, 24)?,
                "Viņa Svētības pāvesta Franciska pastorālās vizītes Latvijā diena",
            ),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2019, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 6)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 12)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2019, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2019, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2019, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2019, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2020, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 10)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2020, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2020, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2020, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2020, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2021, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 9)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2021, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2021, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2021, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2021, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2021, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2022, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 8)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2022, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2022, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2022, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2022, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2022, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2023,
        vec![

        (NaiveDate::from_ymd_res(2023, 1, 1)?, "Jaunais Gads"),
        (NaiveDate::from_ymd_res(2023, 4, 7)?, "Lielā Piektdiena"),
        (NaiveDate::from_ymd_res(2023, 4, 9)?, "Lieldienas"),
        (NaiveDate::from_ymd_res(2023, 4, 10)?, "Otrās Lieldienas"),
        (NaiveDate::from_ymd_res(2023, 5, 1)?, "Darba svētki"),
        (NaiveDate::from_ymd_res(2023, 5, 4)?, "Latvijas Republikas Neatkarības atjaunošanas diena"),
        (NaiveDate::from_ymd_res(2023, 5, 14)?, "Mātes diena"),
        (NaiveDate::from_ymd_res(2023, 6, 23)?, "Līgo diena"),
        (NaiveDate::from_ymd_res(2023, 6, 24)?, "Jāņu diena"),
        (NaiveDate::from_ymd_res(2023, 11, 18)?, "Latvijas Republikas proklamēšanas diena"),
        (NaiveDate::from_ymd_res(2023, 11, 20)?, "Latvijas Republikas proklamēšanas diena (brīvdiena)"),
        (NaiveDate::from_ymd_res(2023, 12, 24)?, "Ziemassvētku vakars"),
        (NaiveDate::from_ymd_res(2023, 12, 25)?, "Ziemassvētki"),
        (NaiveDate::from_ymd_res(2023, 12, 26)?, "Otrie Ziemassvētki"),
        (NaiveDate::from_ymd_res(2023, 12, 31)?, "Vecgada vakars"),
        (NaiveDate::from_ymd_res(2023, 5, 29)?, "Diena, kad Latvijas hokeja komanda ieguva bronzas medaļu 2023. gada Pasaules hokeja čempionātā"),
        (NaiveDate::from_ymd_res(2023, 7, 10)?, "Vispārējo latviešu Dziesmu un deju svētku noslēguma dienu"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2024, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 6)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 12)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2024, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2024, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2024, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2024, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2025, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 5)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 11)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2025, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2025, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2025, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2025, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2025, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2026, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 10)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2026, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2026, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2026, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2026, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2026, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2027, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 9)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2027, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2027, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2027, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2027, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2027, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2028, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 14)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2028, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2028, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2028, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2028, 11, 20)?,
                "Latvijas Republikas proklamēšanas diena (brīvdiena)",
            ),
            (
                NaiveDate::from_ymd_res(2028, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2028, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2029, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 13)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2029, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2029, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2029, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2029, 11, 19)?,
                "Latvijas Republikas proklamēšanas diena (brīvdiena)",
            ),
            (
                NaiveDate::from_ymd_res(2029, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2029, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Jaunais Gads"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Lielā Piektdiena"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Lieldienas"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Otrās Lieldienas"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Darba svētki"),
            (
                NaiveDate::from_ymd_res(2030, 5, 4)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 6)?,
                "Latvijas Republikas Neatkarības atjaunošanas diena (brīvdiena)",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 12)?, "Mātes diena"),
            (NaiveDate::from_ymd_res(2030, 6, 23)?, "Līgo diena"),
            (NaiveDate::from_ymd_res(2030, 6, 24)?, "Jāņu diena"),
            (
                NaiveDate::from_ymd_res(2030, 11, 18)?,
                "Latvijas Republikas proklamēšanas diena",
            ),
            (
                NaiveDate::from_ymd_res(2030, 12, 24)?,
                "Ziemassvētku vakars",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Ziemassvētki"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Otrie Ziemassvētki"),
            (NaiveDate::from_ymd_res(2030, 12, 31)?, "Vecgada vakars"),
        ],
        &mut map,
        Country::LV,
        "Latvia",
    );

    Ok(map)
}
