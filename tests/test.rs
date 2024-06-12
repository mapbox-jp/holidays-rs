use chrono::NaiveDate;
use holidays::{contains, get, init, iter, Builder, Country, Error, NaiveDateExt};
use serial_test::serial;

#[test]
#[serial]
fn init_all() -> anyhow::Result<()> {
    init()?;

    let d = NaiveDate::from_ymd_res(2022, 1, 1)?;
    assert_eq!("元日", get(Country::JP, d)?.unwrap().name);

    Ok(())
}

#[test]
#[serial]
fn build_by_country() -> anyhow::Result<()> {
    Builder::new().countries(&[Country::JP]).init()?;

    let d = NaiveDate::from_ymd_res(2022, 1, 1)?;
    assert_eq!("元日", get(Country::JP, d)?.unwrap().name);
    assert!(contains(Country::JP, d)?);

    let d = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!("元日", get(Country::JP, d)?.unwrap().name);
    assert!(contains(Country::JP, d)?);

    // Get None for non public holiday.
    let d = NaiveDate::from_ymd_res(2022, 1, 2)?;
    assert!(get(Country::JP, d)?.is_none());
    assert!(!contains(Country::JP, d)?);

    // Get an error for unsupported country.
    assert!(get(Country::US, d).is_err());
    assert!(contains(Country::US, d).is_err());

    Ok(())
}

#[test]
#[serial]
fn build_by_country_with_subdivision() -> anyhow::Result<()> {
    Builder::new()
        .countries(&[Country::DE, Country::DE_NW])
        .years(2024..2025)
        .init()?;

    let d = NaiveDate::from_ymd_res(2024, 5, 30)?;

    assert!(get(Country::DE, d)?.is_none());
    assert_eq!("Fronleichnam", get(Country::DE_NW, d)?.unwrap().name);

    Ok(())
}

#[test]
#[serial]
fn build_by_year() -> anyhow::Result<()> {
    Builder::new().years(2022..2023).init()?;

    let d = NaiveDate::from_ymd_res(2022, 1, 1)?;
    assert_eq!("元日", get(Country::JP, d)?.unwrap().name);
    assert_eq!("New Year's Day", get(Country::US, d)?.unwrap().name);
    assert!(contains(Country::JP, d)?);
    assert!(contains(Country::US, d)?);

    // Get an error for unsupported year.
    let d = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!(Error::YearNotAvailable, get(Country::JP, d).unwrap_err());
    assert_eq!(Error::YearNotAvailable, get(Country::US, d).unwrap_err());
    assert_eq!(
        Error::YearNotAvailable,
        contains(Country::JP, d).unwrap_err()
    );
    assert_eq!(
        Error::YearNotAvailable,
        contains(Country::US, d).unwrap_err()
    );

    Ok(())
}

#[test]
#[serial]
fn iterate() -> anyhow::Result<()> {
    init()?;

    let s = NaiveDate::from_ymd_res(2022, 1, 1)?;
    let u = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!(
        vec![
            NaiveDate::from_ymd_res(2022, 1, 1)?,
            NaiveDate::from_ymd_res(2022, 1, 10)?,
            NaiveDate::from_ymd_res(2022, 2, 11)?,
            NaiveDate::from_ymd_res(2022, 2, 23)?,
            NaiveDate::from_ymd_res(2022, 3, 21)?,
            NaiveDate::from_ymd_res(2022, 4, 29)?,
            NaiveDate::from_ymd_res(2022, 5, 3)?,
            NaiveDate::from_ymd_res(2022, 5, 4)?,
            NaiveDate::from_ymd_res(2022, 5, 5)?,
            NaiveDate::from_ymd_res(2022, 7, 18)?,
            NaiveDate::from_ymd_res(2022, 8, 11)?,
            NaiveDate::from_ymd_res(2022, 9, 19)?,
            NaiveDate::from_ymd_res(2022, 9, 23)?,
            NaiveDate::from_ymd_res(2022, 10, 10)?,
            NaiveDate::from_ymd_res(2022, 11, 3)?,
            NaiveDate::from_ymd_res(2022, 11, 23)?,
        ],
        iter(Country::JP, s, u)?.map(|h| h.date).collect::<Vec<_>>()
    );

    Ok(())
}

#[test]
#[serial]
fn iterate_country_not_available() -> anyhow::Result<()> {
    Builder::new().countries(&[Country::JP]).init()?;
    let s = NaiveDate::from_ymd_res(2022, 1, 1)?;
    let u = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!(
        Error::CountryNotAvailable,
        iter(Country::US, s, u).unwrap_err(),
    );

    Ok(())
}

#[test]
#[serial]
fn iterate_year_not_available() -> anyhow::Result<()> {
    Builder::new()
        .countries(&[Country::JP])
        .years(2000..2020)
        .init()?;
    let s = NaiveDate::from_ymd_res(2022, 1, 1)?;
    let u = NaiveDate::from_ymd_res(2023, 1, 1)?;
    assert_eq!(
        Vec::<NaiveDate>::new(),
        iter(Country::JP, s, u)?.map(|h| h.date).collect::<Vec<_>>()
    );

    Ok(())
}
