//! Bulgaria
use super::*;

/// Generate holiday map for Bulgaria.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![

        (NaiveDate::from_ymd_res(2000, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2000, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2000, 4, 28)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2000, 4, 29)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2000, 4, 30)?, "Великден"),
        (NaiveDate::from_ymd_res(2000, 5, 1)?, "Великден; Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2000, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2000, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2000, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2000, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2000, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2000, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2000, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2001,
        vec![

        (NaiveDate::from_ymd_res(2001, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2001, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2001, 4, 13)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2001, 4, 14)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2001, 4, 15)?, "Великден"),
        (NaiveDate::from_ymd_res(2001, 4, 16)?, "Великден"),
        (NaiveDate::from_ymd_res(2001, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2001, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2001, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2001, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2001, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2001, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2001, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2001, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2002,
        vec![

        (NaiveDate::from_ymd_res(2002, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2002, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2002, 5, 3)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2002, 5, 4)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2002, 5, 5)?, "Великден"),
        (NaiveDate::from_ymd_res(2002, 5, 6)?, "Великден; Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2002, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2002, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2002, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2002, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2002, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2002, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2002, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2003,
        vec![

        (NaiveDate::from_ymd_res(2003, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2003, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2003, 4, 25)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2003, 4, 26)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2003, 4, 27)?, "Великден"),
        (NaiveDate::from_ymd_res(2003, 4, 28)?, "Великден"),
        (NaiveDate::from_ymd_res(2003, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2003, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2003, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2003, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2003, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2003, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2003, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2003, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2004,
        vec![

        (NaiveDate::from_ymd_res(2004, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2004, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2004, 4, 9)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2004, 4, 10)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2004, 4, 11)?, "Великден"),
        (NaiveDate::from_ymd_res(2004, 4, 12)?, "Великден"),
        (NaiveDate::from_ymd_res(2004, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2004, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2004, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2004, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2004, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2004, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2004, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2004, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2005,
        vec![

        (NaiveDate::from_ymd_res(2005, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2005, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2005, 4, 29)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2005, 4, 30)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2005, 5, 1)?, "Великден; Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2005, 5, 2)?, "Великден"),
        (NaiveDate::from_ymd_res(2005, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2005, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2005, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2005, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2005, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2005, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2005, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2006,
        vec![

        (NaiveDate::from_ymd_res(2006, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2006, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2006, 4, 21)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2006, 4, 22)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2006, 4, 23)?, "Великден"),
        (NaiveDate::from_ymd_res(2006, 4, 24)?, "Великден"),
        (NaiveDate::from_ymd_res(2006, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2006, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2006, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2006, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2006, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2006, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2006, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2006, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2007,
        vec![

        (NaiveDate::from_ymd_res(2007, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2007, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2007, 4, 6)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2007, 4, 7)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2007, 4, 8)?, "Великден"),
        (NaiveDate::from_ymd_res(2007, 4, 9)?, "Великден"),
        (NaiveDate::from_ymd_res(2007, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2007, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2007, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2007, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2007, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2007, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2007, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2007, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2008,
        vec![

        (NaiveDate::from_ymd_res(2008, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2008, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2008, 4, 25)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2008, 4, 26)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2008, 4, 27)?, "Великден"),
        (NaiveDate::from_ymd_res(2008, 4, 28)?, "Великден"),
        (NaiveDate::from_ymd_res(2008, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2008, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2008, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2008, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2008, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2008, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2008, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2008, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2009,
        vec![

        (NaiveDate::from_ymd_res(2009, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2009, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2009, 4, 17)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2009, 4, 18)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2009, 4, 19)?, "Великден"),
        (NaiveDate::from_ymd_res(2009, 4, 20)?, "Великден"),
        (NaiveDate::from_ymd_res(2009, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2009, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2009, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2009, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2009, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2009, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2009, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2009, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2010,
        vec![

        (NaiveDate::from_ymd_res(2010, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2010, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2010, 4, 2)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2010, 4, 3)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2010, 4, 4)?, "Великден"),
        (NaiveDate::from_ymd_res(2010, 4, 5)?, "Великден"),
        (NaiveDate::from_ymd_res(2010, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2010, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2010, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2010, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2010, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2010, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2010, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2010, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2011,
        vec![

        (NaiveDate::from_ymd_res(2011, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2011, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2011, 4, 22)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2011, 4, 23)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2011, 4, 24)?, "Великден"),
        (NaiveDate::from_ymd_res(2011, 4, 25)?, "Великден"),
        (NaiveDate::from_ymd_res(2011, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2011, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2011, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2011, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2011, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2011, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2011, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2011, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2012,
        vec![

        (NaiveDate::from_ymd_res(2012, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2012, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2012, 4, 13)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2012, 4, 14)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2012, 4, 15)?, "Великден"),
        (NaiveDate::from_ymd_res(2012, 4, 16)?, "Великден"),
        (NaiveDate::from_ymd_res(2012, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2012, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2012, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2012, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2012, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2012, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2012, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2012, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2013,
        vec![

        (NaiveDate::from_ymd_res(2013, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2013, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2013, 5, 3)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2013, 5, 4)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2013, 5, 5)?, "Великден"),
        (NaiveDate::from_ymd_res(2013, 5, 6)?, "Великден; Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2013, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2013, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2013, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2013, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2013, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2013, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2013, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2014,
        vec![

        (NaiveDate::from_ymd_res(2014, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2014, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2014, 4, 18)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2014, 4, 19)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2014, 4, 20)?, "Великден"),
        (NaiveDate::from_ymd_res(2014, 4, 21)?, "Великден"),
        (NaiveDate::from_ymd_res(2014, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2014, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2014, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2014, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2014, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2014, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2014, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2014, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2015,
        vec![

        (NaiveDate::from_ymd_res(2015, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2015, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2015, 4, 10)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2015, 4, 11)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2015, 4, 12)?, "Великден"),
        (NaiveDate::from_ymd_res(2015, 4, 13)?, "Великден"),
        (NaiveDate::from_ymd_res(2015, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2015, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2015, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2015, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2015, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2015, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2015, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2015, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2016,
        vec![

        (NaiveDate::from_ymd_res(2016, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2016, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2016, 4, 29)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2016, 4, 30)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2016, 5, 1)?, "Великден; Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2016, 5, 2)?, "Великден"),
        (NaiveDate::from_ymd_res(2016, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2016, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2016, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2016, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2016, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2016, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2016, 12, 26)?, "Рождество Христово"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2017,
        vec![

        (NaiveDate::from_ymd_res(2017, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2017, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2017, 4, 14)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2017, 4, 15)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2017, 4, 16)?, "Великден"),
        (NaiveDate::from_ymd_res(2017, 4, 17)?, "Великден"),
        (NaiveDate::from_ymd_res(2017, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2017, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2017, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2017, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2017, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2017, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2017, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2017, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2017, 1, 2)?, "Нова година (почивен ден)"),
        (NaiveDate::from_ymd_res(2017, 5, 8)?, "Гергьовден, Ден на храбростта и Българската армия (почивен ден)"),
        (NaiveDate::from_ymd_res(2017, 12, 27)?, "Бъдни вечер (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2018,
        vec![

        (NaiveDate::from_ymd_res(2018, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2018, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2018, 4, 6)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2018, 4, 7)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2018, 4, 8)?, "Великден"),
        (NaiveDate::from_ymd_res(2018, 4, 9)?, "Великден"),
        (NaiveDate::from_ymd_res(2018, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2018, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2018, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2018, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2018, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2018, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2018, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2018, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2018, 3, 5)?, "Ден на Освобождението на България от османско иго (почивен ден)"),
        (NaiveDate::from_ymd_res(2018, 5, 7)?, "Гергьовден, Ден на храбростта и Българската армия (почивен ден)"),
        (NaiveDate::from_ymd_res(2018, 9, 24)?, "Ден на Независимостта на България (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2019,
        vec![

        (NaiveDate::from_ymd_res(2019, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2019, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2019, 4, 26)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2019, 4, 27)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2019, 4, 28)?, "Великден"),
        (NaiveDate::from_ymd_res(2019, 4, 29)?, "Великден"),
        (NaiveDate::from_ymd_res(2019, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2019, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2019, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2019, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2019, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2019, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2019, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2019, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2019, 3, 4)?, "Ден на Освобождението на България от османско иго (почивен ден)"),
        (NaiveDate::from_ymd_res(2019, 9, 23)?, "Ден на Независимостта на България (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2020,
        vec![

        (NaiveDate::from_ymd_res(2020, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2020, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2020, 4, 17)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2020, 4, 18)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2020, 4, 19)?, "Великден"),
        (NaiveDate::from_ymd_res(2020, 4, 20)?, "Великден"),
        (NaiveDate::from_ymd_res(2020, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2020, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2020, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2020, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2020, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2020, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2020, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2020, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2020, 5, 25)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност (почивен ден)"),
        (NaiveDate::from_ymd_res(2020, 9, 7)?, "Ден на Съединението (почивен ден)"),
        (NaiveDate::from_ymd_res(2020, 12, 28)?, "Рождество Христово (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2021,
        vec![

        (NaiveDate::from_ymd_res(2021, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2021, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2021, 4, 30)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2021, 5, 1)?, "Велика събота; Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2021, 5, 2)?, "Великден"),
        (NaiveDate::from_ymd_res(2021, 5, 3)?, "Великден"),
        (NaiveDate::from_ymd_res(2021, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2021, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2021, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2021, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2021, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2021, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2021, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2021, 5, 4)?, "Ден на труда и на международната работническа солидарност (почивен ден)"),
        (NaiveDate::from_ymd_res(2021, 12, 27)?, "Рождество Христово (почивен ден)"),
        (NaiveDate::from_ymd_res(2021, 12, 28)?, "Рождество Христово (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2022,
        vec![

        (NaiveDate::from_ymd_res(2022, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2022, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2022, 4, 22)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2022, 4, 23)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2022, 4, 24)?, "Великден"),
        (NaiveDate::from_ymd_res(2022, 4, 25)?, "Великден"),
        (NaiveDate::from_ymd_res(2022, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2022, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2022, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2022, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2022, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2022, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2022, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2022, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2022, 1, 3)?, "Нова година (почивен ден)"),
        (NaiveDate::from_ymd_res(2022, 5, 2)?, "Ден на труда и на международната работническа солидарност (почивен ден)"),
        (NaiveDate::from_ymd_res(2022, 12, 27)?, "Бъдни вечер (почивен ден)"),
        (NaiveDate::from_ymd_res(2022, 12, 28)?, "Рождество Христово (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2023,
        vec![

        (NaiveDate::from_ymd_res(2023, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2023, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2023, 4, 14)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2023, 4, 15)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2023, 4, 16)?, "Великден"),
        (NaiveDate::from_ymd_res(2023, 4, 17)?, "Великден"),
        (NaiveDate::from_ymd_res(2023, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2023, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2023, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2023, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2023, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2023, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2023, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2023, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2023, 1, 2)?, "Нова година (почивен ден)"),
        (NaiveDate::from_ymd_res(2023, 5, 8)?, "Гергьовден, Ден на храбростта и Българската армия (почивен ден)"),
        (NaiveDate::from_ymd_res(2023, 12, 27)?, "Бъдни вечер (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2024,
        vec![

        (NaiveDate::from_ymd_res(2024, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2024, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2024, 5, 3)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2024, 5, 4)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2024, 5, 5)?, "Великден"),
        (NaiveDate::from_ymd_res(2024, 5, 6)?, "Великден; Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2024, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2024, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2024, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2024, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2024, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2024, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2024, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2024, 3, 4)?, "Ден на Освобождението на България от османско иго (почивен ден)"),
        (NaiveDate::from_ymd_res(2024, 9, 23)?, "Ден на Независимостта на България (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2025,
        vec![

        (NaiveDate::from_ymd_res(2025, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2025, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2025, 4, 18)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2025, 4, 19)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2025, 4, 20)?, "Великден"),
        (NaiveDate::from_ymd_res(2025, 4, 21)?, "Великден"),
        (NaiveDate::from_ymd_res(2025, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2025, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2025, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2025, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2025, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2025, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2025, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2025, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2025, 5, 26)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност (почивен ден)"),
        (NaiveDate::from_ymd_res(2025, 9, 8)?, "Ден на Съединението (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2026,
        vec![

        (NaiveDate::from_ymd_res(2026, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2026, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2026, 4, 10)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2026, 4, 11)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2026, 4, 12)?, "Великден"),
        (NaiveDate::from_ymd_res(2026, 4, 13)?, "Великден"),
        (NaiveDate::from_ymd_res(2026, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2026, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2026, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2026, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2026, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2026, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2026, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2026, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2026, 5, 25)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност (почивен ден)"),
        (NaiveDate::from_ymd_res(2026, 9, 7)?, "Ден на Съединението (почивен ден)"),
        (NaiveDate::from_ymd_res(2026, 12, 28)?, "Рождество Христово (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2027,
        vec![

        (NaiveDate::from_ymd_res(2027, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2027, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2027, 4, 30)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2027, 5, 1)?, "Велика събота; Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2027, 5, 2)?, "Великден"),
        (NaiveDate::from_ymd_res(2027, 5, 3)?, "Великден"),
        (NaiveDate::from_ymd_res(2027, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2027, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2027, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2027, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2027, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2027, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2027, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2027, 5, 4)?, "Ден на труда и на международната работническа солидарност (почивен ден)"),
        (NaiveDate::from_ymd_res(2027, 12, 27)?, "Рождество Христово (почивен ден)"),
        (NaiveDate::from_ymd_res(2027, 12, 28)?, "Рождество Христово (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2028,
        vec![

        (NaiveDate::from_ymd_res(2028, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2028, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2028, 4, 14)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2028, 4, 15)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2028, 4, 16)?, "Великден"),
        (NaiveDate::from_ymd_res(2028, 4, 17)?, "Великден"),
        (NaiveDate::from_ymd_res(2028, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2028, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2028, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2028, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2028, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2028, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2028, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2028, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2028, 1, 3)?, "Нова година (почивен ден)"),
        (NaiveDate::from_ymd_res(2028, 5, 8)?, "Гергьовден, Ден на храбростта и Българската армия (почивен ден)"),
        (NaiveDate::from_ymd_res(2028, 12, 27)?, "Бъдни вечер (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2029,
        vec![

        (NaiveDate::from_ymd_res(2029, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2029, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2029, 4, 6)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2029, 4, 7)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2029, 4, 8)?, "Великден"),
        (NaiveDate::from_ymd_res(2029, 4, 9)?, "Великден"),
        (NaiveDate::from_ymd_res(2029, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2029, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2029, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2029, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2029, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2029, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2029, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2029, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2029, 3, 5)?, "Ден на Освобождението на България от османско иго (почивен ден)"),
        (NaiveDate::from_ymd_res(2029, 5, 7)?, "Гергьовден, Ден на храбростта и Българската армия (почивен ден)"),
        (NaiveDate::from_ymd_res(2029, 9, 24)?, "Ден на Независимостта на България (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    build_year(
        years,
        2030,
        vec![

        (NaiveDate::from_ymd_res(2030, 1, 1)?, "Нова година"),
        (NaiveDate::from_ymd_res(2030, 3, 3)?, "Ден на Освобождението на България от османско иго"),
        (NaiveDate::from_ymd_res(2030, 4, 26)?, "Велики петък"),
        (NaiveDate::from_ymd_res(2030, 4, 27)?, "Велика събота"),
        (NaiveDate::from_ymd_res(2030, 4, 28)?, "Великден"),
        (NaiveDate::from_ymd_res(2030, 4, 29)?, "Великден"),
        (NaiveDate::from_ymd_res(2030, 5, 1)?, "Ден на труда и на международната работническа солидарност"),
        (NaiveDate::from_ymd_res(2030, 5, 6)?, "Гергьовден, Ден на храбростта и Българската армия"),
        (NaiveDate::from_ymd_res(2030, 5, 24)?, "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност"),
        (NaiveDate::from_ymd_res(2030, 9, 6)?, "Ден на Съединението"),
        (NaiveDate::from_ymd_res(2030, 9, 22)?, "Ден на Независимостта на България"),
        (NaiveDate::from_ymd_res(2030, 12, 24)?, "Бъдни вечер"),
        (NaiveDate::from_ymd_res(2030, 12, 25)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2030, 12, 26)?, "Рождество Христово"),
        (NaiveDate::from_ymd_res(2030, 3, 4)?, "Ден на Освобождението на България от османско иго (почивен ден)"),
        (NaiveDate::from_ymd_res(2030, 9, 23)?, "Ден на Независимостта на България (почивен ден)"),
        ],
        &mut map,
        Country::BG,
        "Bulgaria",
    );

    Ok(map)
}
