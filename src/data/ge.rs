//! Georgia
use super::*;

/// Generate holiday map for Georgia.
#[allow(unused_mut, unused_variables)]
pub fn build(
    years: Option<&std::ops::Range<Year>>,
) -> Result<HashMap<Year, BTreeMap<NaiveDate, Holiday>>> {
    let mut map = HashMap::new();

    build_year(
        years,
        2000,
        vec![
            (NaiveDate::from_ymd_res(2000, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2000, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2000, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2000, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2000, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2000, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2000, 4, 28)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2000, 4, 29)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2000, 4, 30)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2000, 5, 1)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2000, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2000, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2000, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2000, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2000, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2000, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2001,
        vec![
            (NaiveDate::from_ymd_res(2001, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2001, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2001, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2001, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2001, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2001, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2001, 4, 13)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2001, 4, 14)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2001, 4, 15)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2001, 4, 16)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2001, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2001, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2001, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2001, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2001, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2001, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2002,
        vec![
            (NaiveDate::from_ymd_res(2002, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2002, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2002, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2002, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2002, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2002, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 3)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2002, 5, 4)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2002, 5, 5)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2002, 5, 6)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2002, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2002, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2002, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2002, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2002, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2002, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2003,
        vec![
            (NaiveDate::from_ymd_res(2003, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2003, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2003, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2003, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2003, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2003, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2003, 4, 25)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2003, 4, 26)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2003, 4, 27)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2003, 4, 28)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2003, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2003, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2003, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2003, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2003, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2003, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2004,
        vec![
            (NaiveDate::from_ymd_res(2004, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2004, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2004, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2004, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2004, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2004, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (
                NaiveDate::from_ymd_res(2004, 4, 9)?,
                "ეროვნული ერთიანობის დღე; წითელი პარასკევი",
            ),
            (NaiveDate::from_ymd_res(2004, 4, 10)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2004, 4, 11)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2004, 4, 12)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2004, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2004, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2004, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2004, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2004, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2004, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2005,
        vec![
            (NaiveDate::from_ymd_res(2005, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2005, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2005, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2005, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2005, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2005, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2005, 4, 29)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2005, 4, 30)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2005, 5, 1)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2005, 5, 2)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2005, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2005, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2005, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2005, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2005, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2005, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2006,
        vec![
            (NaiveDate::from_ymd_res(2006, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2006, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2006, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2006, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2006, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2006, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2006, 4, 21)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2006, 4, 22)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2006, 4, 23)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2006, 4, 24)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2006, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2006, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2006, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2006, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2006, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2006, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2007,
        vec![
            (NaiveDate::from_ymd_res(2007, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2007, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2007, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2007, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2007, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2007, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2007, 4, 6)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2007, 4, 7)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2007, 4, 8)?, "აღდგომა"),
            (
                NaiveDate::from_ymd_res(2007, 4, 9)?,
                "ეროვნული ერთიანობის დღე; შავი ორშაბათი",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2007, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2007, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2007, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2007, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2007, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2008,
        vec![
            (NaiveDate::from_ymd_res(2008, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2008, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2008, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2008, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2008, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2008, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2008, 4, 25)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2008, 4, 26)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2008, 4, 27)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2008, 4, 28)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2008, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2008, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2008, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2008, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2008, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2008, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2009,
        vec![
            (NaiveDate::from_ymd_res(2009, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2009, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2009, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2009, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2009, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2009, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2009, 4, 17)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2009, 4, 18)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2009, 4, 19)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2009, 4, 20)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2009, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2009, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2009, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2009, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2009, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2009, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2010,
        vec![
            (NaiveDate::from_ymd_res(2010, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2010, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2010, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2010, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2010, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2010, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2010, 4, 2)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2010, 4, 3)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2010, 4, 4)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2010, 4, 5)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2010, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2010, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2010, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2010, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2010, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2010, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2011,
        vec![
            (NaiveDate::from_ymd_res(2011, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2011, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2011, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2011, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2011, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2011, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2011, 4, 22)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2011, 4, 23)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2011, 4, 24)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2011, 4, 25)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2011, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2011, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2011, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2011, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2011, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2011, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2012,
        vec![
            (NaiveDate::from_ymd_res(2012, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2012, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2012, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2012, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2012, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2012, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2012, 4, 13)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2012, 4, 14)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2012, 4, 15)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2012, 4, 16)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2012, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2012, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2012, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2012, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2012, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2012, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2013,
        vec![
            (NaiveDate::from_ymd_res(2013, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2013, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2013, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2013, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2013, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2013, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 3)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2013, 5, 4)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2013, 5, 5)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2013, 5, 6)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2013, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2013, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2013, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2013, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2013, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2013, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2014,
        vec![
            (NaiveDate::from_ymd_res(2014, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2014, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2014, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2014, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2014, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2014, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2014, 4, 18)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2014, 4, 19)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2014, 4, 20)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2014, 4, 21)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2014, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2014, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2014, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2014, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2014, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2014, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2015,
        vec![
            (NaiveDate::from_ymd_res(2015, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2015, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2015, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2015, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2015, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2015, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2015, 4, 10)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2015, 4, 11)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2015, 4, 12)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2015, 4, 13)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2015, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2015, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2015, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2015, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2015, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2015, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2016,
        vec![
            (NaiveDate::from_ymd_res(2016, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2016, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2016, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2016, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2016, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2016, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2016, 4, 29)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2016, 4, 30)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2016, 5, 1)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2016, 5, 2)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2016, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2016, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2016, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2016, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2016, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2016, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2017,
        vec![
            (NaiveDate::from_ymd_res(2017, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2017, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2017, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2017, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2017, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2017, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2017, 4, 14)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2017, 4, 15)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2017, 4, 16)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2017, 4, 17)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2017, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2017, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2017, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2017, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2017, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2017, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2018,
        vec![
            (NaiveDate::from_ymd_res(2018, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2018, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2018, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2018, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2018, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2018, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2018, 4, 6)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2018, 4, 7)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2018, 4, 8)?, "აღდგომა"),
            (
                NaiveDate::from_ymd_res(2018, 4, 9)?,
                "ეროვნული ერთიანობის დღე; შავი ორშაბათი",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2018, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2018, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2018, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2018, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2018, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2019,
        vec![
            (NaiveDate::from_ymd_res(2019, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2019, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2019, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2019, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2019, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2019, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2019, 4, 26)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2019, 4, 27)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2019, 4, 28)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2019, 4, 29)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2019, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2019, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2019, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2019, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2019, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2019, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2020,
        vec![
            (NaiveDate::from_ymd_res(2020, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2020, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2020, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2020, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2020, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2020, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2020, 4, 17)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2020, 4, 18)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2020, 4, 19)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2020, 4, 20)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2020, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2020, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2020, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2020, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2020, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2020, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2021,
        vec![
            (NaiveDate::from_ymd_res(2021, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2021, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2021, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2021, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2021, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2021, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2021, 4, 30)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2021, 5, 1)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2021, 5, 2)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2021, 5, 3)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2021, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2021, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2021, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2021, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2021, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2021, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2022,
        vec![
            (NaiveDate::from_ymd_res(2022, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2022, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2022, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2022, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2022, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2022, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2022, 4, 22)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2022, 4, 23)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2022, 4, 24)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2022, 4, 25)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2022, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2022, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2022, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2022, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2022, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2022, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2023,
        vec![
            (NaiveDate::from_ymd_res(2023, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2023, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2023, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2023, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2023, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2023, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2023, 4, 14)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2023, 4, 15)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2023, 4, 16)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2023, 4, 17)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2023, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2023, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2023, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2023, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2023, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2023, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2024,
        vec![
            (NaiveDate::from_ymd_res(2024, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2024, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2024, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2024, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2024, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2024, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 3)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2024, 5, 4)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2024, 5, 5)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2024, 5, 6)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2024, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2024, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2024, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2024, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2024, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2024, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2025,
        vec![
            (NaiveDate::from_ymd_res(2025, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2025, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2025, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2025, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2025, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2025, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2025, 4, 18)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2025, 4, 19)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2025, 4, 20)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2025, 4, 21)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2025, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2025, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2025, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2025, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2025, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2025, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2026,
        vec![
            (NaiveDate::from_ymd_res(2026, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2026, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2026, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2026, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2026, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2026, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2026, 4, 10)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2026, 4, 11)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2026, 4, 12)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2026, 4, 13)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2026, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2026, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2026, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2026, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2026, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2026, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2027,
        vec![
            (NaiveDate::from_ymd_res(2027, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2027, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2027, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2027, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2027, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2027, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2027, 4, 30)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2027, 5, 1)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2027, 5, 2)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2027, 5, 3)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2027, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2027, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2027, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2027, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2027, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2027, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2028,
        vec![
            (NaiveDate::from_ymd_res(2028, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2028, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2028, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2028, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2028, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2028, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2028, 4, 14)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2028, 4, 15)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2028, 4, 16)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2028, 4, 17)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2028, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2028, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2028, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2028, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2028, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2028, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2029,
        vec![
            (NaiveDate::from_ymd_res(2029, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2029, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2029, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2029, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2029, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2029, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2029, 4, 6)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2029, 4, 7)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2029, 4, 8)?, "აღდგომა"),
            (
                NaiveDate::from_ymd_res(2029, 4, 9)?,
                "ეროვნული ერთიანობის დღე; შავი ორშაბათი",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2029, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2029, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2029, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2029, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2029, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    build_year(
        years,
        2030,
        vec![
            (NaiveDate::from_ymd_res(2030, 1, 1)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2030, 1, 2)?, "ახალი წელი"),
            (NaiveDate::from_ymd_res(2030, 1, 7)?, "ქრისტეშობა"),
            (NaiveDate::from_ymd_res(2030, 1, 19)?, "ნათლისღება"),
            (NaiveDate::from_ymd_res(2030, 3, 3)?, "დედის დღე"),
            (
                NaiveDate::from_ymd_res(2030, 3, 8)?,
                "ქალთა საერთაშორისო დღე",
            ),
            (NaiveDate::from_ymd_res(2030, 4, 26)?, "წითელი პარასკევი"),
            (NaiveDate::from_ymd_res(2030, 4, 27)?, "დიდი შაბათი"),
            (NaiveDate::from_ymd_res(2030, 4, 28)?, "აღდგომა"),
            (NaiveDate::from_ymd_res(2030, 4, 29)?, "შავი ორშაბათი"),
            (
                NaiveDate::from_ymd_res(2030, 4, 9)?,
                "ეროვნული ერთიანობის დღე",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 9)?,
                "ფაშიზმზე გამარჯვების დღე",
            ),
            (
                NaiveDate::from_ymd_res(2030, 5, 12)?,
                "წმინდა ანდრია პირველწოდებულის დღე",
            ),
            (NaiveDate::from_ymd_res(2030, 5, 26)?, "დამოუკიდებლობის დღე"),
            (NaiveDate::from_ymd_res(2030, 8, 28)?, "მარიამობა"),
            (NaiveDate::from_ymd_res(2030, 10, 14)?, "მცხეთობის"),
            (NaiveDate::from_ymd_res(2030, 11, 23)?, "გიორგობა"),
        ],
        &mut map,
        Country::GE,
        "Georgia",
    );

    Ok(map)
}
