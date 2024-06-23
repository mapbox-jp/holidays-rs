//! France
#[allow(clippy::wildcard_imports)]
use super::*;

const COUNTY_NAME: &str = "France";
const COUNTY_CODE: Country = Country::FR;

/// Generate holiday map for France.
#[allow(
    unused_mut,
    unused_variables,
    clippy::too_many_lines,
    clippy::missing_errors_doc
)]
pub fn build(years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerCountryMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        [
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2000, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2000, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2000, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2000, 4, 24)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2000, 6, 12)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2000, 6, 1)?, "Ascension"),
            (NaiveDate::from_ymd_res(2000, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2000, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2001,
        [
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2001, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2001, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2001, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2001, 6, 4)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2001, 5, 24)?, "Ascension"),
            (NaiveDate::from_ymd_res(2001, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2001, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2002,
        [
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2002, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2002, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2002, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2002, 4, 1)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2002, 5, 20)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2002, 5, 9)?, "Ascension"),
            (NaiveDate::from_ymd_res(2002, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2002, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2003,
        [
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2003, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2003, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2003, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2003, 6, 9)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2003, 5, 29)?, "Ascension"),
            (NaiveDate::from_ymd_res(2003, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2003, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2004,
        [
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2004, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2004, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2004, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2004, 5, 31)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2004, 5, 20)?, "Ascension"),
            (NaiveDate::from_ymd_res(2004, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2004, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2005,
        [
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2005, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2005, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2005, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2005, 3, 28)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2005, 5, 5)?, "Ascension"),
            (NaiveDate::from_ymd_res(2005, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2005, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2006,
        [
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2006, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2006, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2006, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2006, 4, 17)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2006, 5, 25)?, "Ascension"),
            (NaiveDate::from_ymd_res(2006, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2006, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2007,
        [
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2007, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2007, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2007, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2007, 4, 9)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2007, 5, 17)?, "Ascension"),
            (NaiveDate::from_ymd_res(2007, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2007, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2008,
        [
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "Jour de l'an"),
            (
                NaiveDate::from_ymd_res(2008, 5, 1)?,
                "Ascension; Fête du Travail",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2008, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2008, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2008, 3, 24)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2008, 5, 12)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2008, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2008, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2009,
        [
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2009, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2009, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2009, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2009, 4, 13)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2009, 6, 1)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2009, 5, 21)?, "Ascension"),
            (NaiveDate::from_ymd_res(2009, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2009, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2010,
        [
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2010, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2010, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2010, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2010, 5, 24)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2010, 5, 13)?, "Ascension"),
            (NaiveDate::from_ymd_res(2010, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2010, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2011,
        [
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2011, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2011, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2011, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2011, 6, 13)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2011, 6, 2)?, "Ascension"),
            (NaiveDate::from_ymd_res(2011, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2011, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2012,
        [
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2012, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2012, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2012, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2012, 4, 9)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2012, 5, 28)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2012, 5, 17)?, "Ascension"),
            (NaiveDate::from_ymd_res(2012, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2012, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2013,
        [
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2013, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2013, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2013, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2013, 4, 1)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2013, 5, 20)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2013, 5, 9)?, "Ascension"),
            (NaiveDate::from_ymd_res(2013, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2013, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2014,
        [
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2014, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2014, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2014, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2014, 6, 9)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2014, 5, 29)?, "Ascension"),
            (NaiveDate::from_ymd_res(2014, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2014, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2015,
        [
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2015, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2015, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2015, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2015, 4, 6)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2015, 5, 25)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2015, 5, 14)?, "Ascension"),
            (NaiveDate::from_ymd_res(2015, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2015, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2016,
        [
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2016, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2016, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2016, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2016, 3, 28)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2016, 5, 16)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2016, 5, 5)?, "Ascension"),
            (NaiveDate::from_ymd_res(2016, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2016, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2017,
        [
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2017, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2017, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2017, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2017, 6, 5)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2017, 5, 25)?, "Ascension"),
            (NaiveDate::from_ymd_res(2017, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2017, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2018,
        [
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2018, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2018, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2018, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2018, 4, 2)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2018, 5, 21)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2018, 5, 10)?, "Ascension"),
            (NaiveDate::from_ymd_res(2018, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2018, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2019,
        [
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2019, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2019, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2019, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2019, 4, 22)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2019, 6, 10)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2019, 5, 30)?, "Ascension"),
            (NaiveDate::from_ymd_res(2019, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2019, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2020,
        [
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2020, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2020, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2020, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2020, 4, 13)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2020, 6, 1)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2020, 5, 21)?, "Ascension"),
            (NaiveDate::from_ymd_res(2020, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2020, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2021,
        [
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2021, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2021, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2021, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2021, 4, 5)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2021, 5, 24)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2021, 5, 13)?, "Ascension"),
            (NaiveDate::from_ymd_res(2021, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2021, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2022,
        [
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2022, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2022, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2022, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2022, 4, 18)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2022, 6, 6)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "Ascension"),
            (NaiveDate::from_ymd_res(2022, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2022, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2023,
        [
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2023, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2023, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2023, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2023, 4, 10)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2023, 5, 29)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2023, 5, 18)?, "Ascension"),
            (NaiveDate::from_ymd_res(2023, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2023, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2024,
        [
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2024, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2024, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2024, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2024, 4, 1)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2024, 5, 20)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2024, 5, 9)?, "Ascension"),
            (NaiveDate::from_ymd_res(2024, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2024, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2025,
        [
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2025, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2025, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2025, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2025, 6, 9)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2025, 5, 29)?, "Ascension"),
            (NaiveDate::from_ymd_res(2025, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2025, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2026,
        [
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2026, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2026, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2026, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2026, 4, 6)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2026, 5, 25)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2026, 5, 14)?, "Ascension"),
            (NaiveDate::from_ymd_res(2026, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2026, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2027,
        [
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2027, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2027, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2027, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2027, 3, 29)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2027, 5, 17)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2027, 5, 6)?, "Ascension"),
            (NaiveDate::from_ymd_res(2027, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2027, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2028,
        [
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2028, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2028, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2028, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2028, 6, 5)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2028, 5, 25)?, "Ascension"),
            (NaiveDate::from_ymd_res(2028, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2028, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2029,
        [
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2029, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2029, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2029, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2029, 4, 2)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2029, 5, 21)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2029, 5, 10)?, "Ascension"),
            (NaiveDate::from_ymd_res(2029, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2029, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    build_year(
        years,
        2030,
        [
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "Jour de l'an"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Fête du Travail"),
            (NaiveDate::from_ymd_res(2030, 5, 8)?, "Fête de la Victoire"),
            (NaiveDate::from_ymd_res(2030, 7, 14)?, "Fête nationale"),
            (NaiveDate::from_ymd_res(2030, 11, 11)?, "Armistice"),
            (NaiveDate::from_ymd_res(2030, 4, 22)?, "Lundi de Pâques"),
            (NaiveDate::from_ymd_res(2030, 6, 10)?, "Lundi de Pentecôte"),
            (NaiveDate::from_ymd_res(2030, 5, 30)?, "Ascension"),
            (NaiveDate::from_ymd_res(2030, 8, 15)?, "Assomption"),
            (NaiveDate::from_ymd_res(2030, 11, 1)?, "Toussaint"),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Noël"),
        ],
        &mut map,
        COUNTY_CODE,
        COUNTY_NAME,
    );

    Ok(map)
}
