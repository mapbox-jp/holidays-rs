//! Hungary
use super::*;

/// Generate holiday map for Hungary.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2000, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2000, 4, 23)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2000, 6, 11)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2000, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2000, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2000, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2001, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2001, 6, 3)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2001, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2001, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2001, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2002, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2002, 3, 31)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2002, 5, 19)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2002, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2002, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2002, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2003, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2003, 4, 20)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2003, 6, 8)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2003, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2003, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2003, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2004, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2004, 5, 30)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2004, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2004, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2004, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2005, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2005, 3, 27)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2005, 5, 15)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2005, 5, 16)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2005, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2005, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2005, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2006, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2006, 4, 16)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2006, 6, 4)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2006, 6, 5)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2006, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2006, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2006, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2007, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2007, 5, 27)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2007, 5, 28)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2007, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2007, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2007, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2008, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2008, 3, 23)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2008, 5, 11)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2008, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2008, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2008, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2009, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2009, 4, 12)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2009, 5, 31)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2009, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2009, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2009, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2010, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2010, 5, 23)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2010, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2010, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2010, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2010, 12, 24)?,
                "Pihenőnap (2010. 12. 11.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2011, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2011, 6, 12)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2011, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2011, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2011, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2011, 3, 14)?,
                "Pihenőnap (2011. 03. 19.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 31)?,
                "Pihenőnap (2011. 11. 05.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2012, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2012, 4, 8)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2012, 5, 27)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2012, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2012, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2012, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2012, 3, 16)?,
                "Pihenőnap (2012. 03. 24.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 4, 30)?,
                "Pihenőnap (2012. 04. 21.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 22)?,
                "Pihenőnap (2012. 10. 27.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 11, 2)?,
                "Pihenőnap (2012. 11. 10.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 24)?,
                "Pihenőnap (2012. 12. 15.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2012, 12, 31)?,
                "Pihenőnap (2012. 12. 01.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2013, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2013, 3, 31)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2013, 5, 19)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2013, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2013, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2013, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2013, 8, 19)?,
                "Pihenőnap (2013. 08. 24.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 24)?,
                "Pihenőnap (2013. 12. 07.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2013, 12, 27)?,
                "Pihenőnap (2013. 12. 21.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2014, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2014, 6, 8)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2014, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2014, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2014, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2014, 5, 2)?,
                "Pihenőnap (2014. 05. 10.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 24)?,
                "Pihenőnap (2014. 10. 18.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2014, 12, 24)?,
                "Pihenőnap (2014. 12. 13.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2015, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2015, 4, 5)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2015, 5, 24)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2015, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2015, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2015, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2015, 1, 2)?,
                "Pihenőnap (2015. 01. 10.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 8, 21)?,
                "Pihenőnap (2015. 08. 08.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2015, 12, 24)?,
                "Pihenőnap (2015. 12. 12.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2016, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2016, 3, 27)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2016, 5, 15)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2016, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2016, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2016, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2016, 3, 14)?,
                "Pihenőnap (2016. 03. 05.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 31)?,
                "Pihenőnap (2016. 10. 15.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2017, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2017, 6, 4)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2017, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2017, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2017, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2018, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2018, 4, 1)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2018, 5, 20)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2018, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2018, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2018, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2018, 3, 16)?,
                "Pihenőnap (2018. 03. 10.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 4, 30)?,
                "Pihenőnap (2018. 04. 21.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 22)?,
                "Pihenőnap (2018. 10. 13.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 11, 2)?,
                "Pihenőnap (2018. 11. 10.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 24)?,
                "Pihenőnap (2018. 12. 01.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2018, 12, 31)?,
                "Pihenőnap (2018. 12. 15.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2019, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2019, 6, 9)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2019, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2019, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2019, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2019, 8, 19)?,
                "Pihenőnap (2019. 08. 10.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 24)?,
                "Pihenőnap (2019. 12. 07.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2019, 12, 27)?,
                "Pihenőnap (2019. 12. 14.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2020, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2020, 4, 12)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2020, 5, 31)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2020, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2020, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2020, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2020, 8, 21)?,
                "Pihenőnap (2020. 08. 29.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2020, 12, 24)?,
                "Pihenőnap (2020. 12. 12.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2021, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2021, 4, 4)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2021, 5, 23)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2021, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2021, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2021, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2021, 12, 24)?,
                "Pihenőnap (2021. 12. 11.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2022, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2022, 4, 17)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2022, 6, 5)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2022, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2022, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2022, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2022, 3, 14)?,
                "Pihenőnap (2022. 03. 26.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 31)?,
                "Pihenőnap (2022. 10. 15.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2023, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2023, 4, 9)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2023, 5, 28)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2023, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2023, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2023, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2024, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2024, 3, 31)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2024, 5, 19)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2024, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2024, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2024, 12, 26)?, "Karácsony másnapja"),
            (
                NaiveDate::from_ymd_res(2024, 8, 19)?,
                "Pihenőnap (2024. 08. 03.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 24)?,
                "Pihenőnap (2024. 12. 07.-től helyettesítve)",
            ),
            (
                NaiveDate::from_ymd_res(2024, 12, 27)?,
                "Pihenőnap (2024. 12. 14.-től helyettesítve)",
            ),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2025, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2025, 6, 8)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2025, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2025, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2025, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2026, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2026, 4, 5)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2026, 5, 24)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2026, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2026, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2026, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2027, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2027, 3, 28)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2027, 5, 16)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2027, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2027, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2027, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2028, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2028, 6, 4)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2028, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2028, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2028, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2029, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2029, 4, 1)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2029, 5, 20)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2029, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2029, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2029, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Újév"),
            (NaiveDate::from_ymd_res(2030, 3, 15)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Nagypéntek"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Húsvét"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Húsvét Hétfő"),
            (NaiveDate::from_ymd_res(2030, 6, 9)?, "Pünkösd"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Pünkösdhétfő"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "A Munka ünnepe"),
            (
                NaiveDate::from_ymd_res(2030, 8, 20)?,
                "Az államalapítás ünnepe",
            ),
            (NaiveDate::from_ymd_res(2030, 10, 23)?, "Nemzeti ünnep"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Mindenszentek"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Karácsony"),
            (NaiveDate::from_ymd_res(2030, 12, 26)?, "Karácsony másnapja"),
        ],
        &mut map,
        Country::HU,
        "Hungary",
    );

    Ok(map)
}
