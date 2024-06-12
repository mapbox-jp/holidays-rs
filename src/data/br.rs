//! Brazil
use super::*;

/// Generate holiday map for Brazil.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (
                NaiveDate::from_ymd_res(2000, 1, 1)?,
                "Confraternização Universal",
            ),
            (
                NaiveDate::from_ymd_res(2000, 4, 21)?,
                "Sexta-feira Santa; Tiradentes",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2000, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2000, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2000, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2000, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2000, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2001,
        vec![
            (
                NaiveDate::from_ymd_res(2001, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2001, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2001, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2001, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2001, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2001, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2001, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2001, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2002,
        vec![
            (
                NaiveDate::from_ymd_res(2002, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2002, 3, 29)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2002, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2002, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2002, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2002, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2002, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2002, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2002, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2003,
        vec![
            (
                NaiveDate::from_ymd_res(2003, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 18)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2003, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2003, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2003, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2003, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2003, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2003, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2003, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2004,
        vec![
            (
                NaiveDate::from_ymd_res(2004, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 9)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2004, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2004, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2004, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2004, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2004, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2004, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2004, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2005,
        vec![
            (
                NaiveDate::from_ymd_res(2005, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2005, 3, 25)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2005, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2005, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2005, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2005, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2005, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2005, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2006,
        vec![
            (
                NaiveDate::from_ymd_res(2006, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 14)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2006, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2006, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2006, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2006, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2006, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2006, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2006, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2007,
        vec![
            (
                NaiveDate::from_ymd_res(2007, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2007, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2007, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2007, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2007, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2007, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2007, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2007, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2008,
        vec![
            (
                NaiveDate::from_ymd_res(2008, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2008, 3, 21)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2008, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2008, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2008, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2008, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2008, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2008, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2008, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2009,
        vec![
            (
                NaiveDate::from_ymd_res(2009, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 10)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2009, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2009, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2009, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2009, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2009, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2009, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2009, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2010,
        vec![
            (
                NaiveDate::from_ymd_res(2010, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2010, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2010, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2010, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2010, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2010, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2010, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2010, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2011,
        vec![
            (
                NaiveDate::from_ymd_res(2011, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2011, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2011, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2011, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2011, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2011, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2011, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2011, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2012,
        vec![
            (
                NaiveDate::from_ymd_res(2012, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 6)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2012, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2012, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2012, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2012, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2012, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2012, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2012, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2013,
        vec![
            (
                NaiveDate::from_ymd_res(2013, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2013, 3, 29)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2013, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2013, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2013, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2013, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2013, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2013, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2013, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2014,
        vec![
            (
                NaiveDate::from_ymd_res(2014, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2014, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2014, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2014, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2014, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2014, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2014, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2015,
        vec![
            (
                NaiveDate::from_ymd_res(2015, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 3)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2015, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2015, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2015, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2015, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2015, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2015, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2015, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2016,
        vec![
            (
                NaiveDate::from_ymd_res(2016, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2016, 3, 25)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2016, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2016, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2016, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2016, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2016, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2016, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2017,
        vec![
            (
                NaiveDate::from_ymd_res(2017, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2017, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2017, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2017, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2017, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2017, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2017, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2017, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2018,
        vec![
            (
                NaiveDate::from_ymd_res(2018, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2018, 3, 30)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2018, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2018, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2018, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2018, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2018, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2018, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2018, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2019,
        vec![
            (
                NaiveDate::from_ymd_res(2019, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 19)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2019, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2019, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2019, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2019, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2019, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2019, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2019, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2020,
        vec![
            (
                NaiveDate::from_ymd_res(2020, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 10)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2020, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2020, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2020, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2020, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2020, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2020, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2020, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2021,
        vec![
            (
                NaiveDate::from_ymd_res(2021, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 2)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2021, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2021, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2021, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2021, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2021, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2021, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2022,
        vec![
            (
                NaiveDate::from_ymd_res(2022, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 15)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2022, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2022, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2022, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2022, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2022, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2022, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2022, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2023,
        vec![
            (
                NaiveDate::from_ymd_res(2023, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 7)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2023, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2023, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2023, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2023, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2023, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2023, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2023, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2024,
        vec![
            (
                NaiveDate::from_ymd_res(2024, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2024, 3, 29)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2024, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2024, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2024, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2024, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2024, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2024, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2024, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2025,
        vec![
            (
                NaiveDate::from_ymd_res(2025, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2025, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2025, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2025, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2025, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2025, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2025, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2026,
        vec![
            (
                NaiveDate::from_ymd_res(2026, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 3)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2026, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2026, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2026, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2026, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2026, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2026, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2026, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2027,
        vec![
            (
                NaiveDate::from_ymd_res(2027, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2027, 3, 26)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2027, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2027, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2027, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2027, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2027, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2027, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2028,
        vec![
            (
                NaiveDate::from_ymd_res(2028, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2028, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2028, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2028, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2028, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2028, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2028, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2028, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2029,
        vec![
            (
                NaiveDate::from_ymd_res(2029, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2029, 3, 30)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2029, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2029, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2029, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2029, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2029, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2029, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2029, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    build_year(
        years,
        2030,
        vec![
            (
                NaiveDate::from_ymd_res(2030, 1, 1)?,
                "Confraternização Universal",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 19)?, "Sexta-feira Santa"),
            (NaiveDate::from_ymd_res(2030, 4, 21)?, "Tiradentes"),
            (NaiveDate::from_ymd_res(2030, 5, 1)?, "Dia do Trabalhador"),
            (
                NaiveDate::from_ymd_res(2030, 9, 7)?,
                "Independência do Brasil",
            ),
            (
                NaiveDate::from_ymd_res(2030, 10, 12)?,
                "Nossa Senhora Aparecida",
            ),
            (NaiveDate::from_ymd_res(2030, 11, 2)?, "Finados"),
            (
                NaiveDate::from_ymd_res(2030, 11, 15)?,
                "Proclamação da República",
            ),
            (NaiveDate::from_ymd_res(2030, 12, 25)?, "Natal"),
        ],
        &mut map,
        Country::BR,
        "Brazil",
    );

    Ok(map)
}
