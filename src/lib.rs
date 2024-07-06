mod build;
mod builder;
mod country;
mod data;
mod iter;
mod prelude;

pub use prelude::*;

use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, HashMap},
    sync::{RwLock, RwLockReadGuard},
};

/// Type alias for Holiday map.
pub type HolidayPerDayMap = BTreeMap<NaiveDate, Holiday>;
pub type HolidayPerYearMap = HashMap<Year, HolidayPerDayMap>;
pub type HolidayMap = HashMap<Country, HolidayPerYearMap>;

/// Type alias for Year.
pub type Year = i32;

/// Result type of this crate.
pub type Result<T> = std::result::Result<T, Error>;

static DATA: Lazy<RwLock<HolidayMap>> = Lazy::new(|| RwLock::new(HolidayMap::new()));

/// Initialize holiday database for all the supported countries and years.
/// Note that this will use quite lots of memory. Please consider using
/// `Builder` to specify countries and years to load for if you're concerned
/// about memory usage.
#[allow(clippy::missing_errors_doc)]
pub fn init() -> Result<()> {
    let map = Builder::new().build()?;
    init_holiday(map)
}

/// Get holiday by ISO 3166-1 alpha-2 country code and date. If the specified country or year is
/// not available, it will return `Err(Error)`. If the specified date is not a holiday, it
/// will return `Ok(None)`. Otherwise, it will return `Ok(Some(Holiday))`.
#[allow(clippy::missing_errors_doc)]
pub fn get(country: Country, date: NaiveDate) -> Result<Option<Holiday>> {
    let holiday_map = get_holiday_map()?;

    let map = get_map_for_country_and_year(&holiday_map, country, date.year())?;

    Ok(map.get(date).cloned())
}

/// Check if the specified date is a holiday. If the specified country or year is
/// not available, it will return `Err(Error)`. If the date is not a holiday, it
/// will return `Ok(false)`. Otherwise, it will return `Ok(true)`.
#[allow(clippy::missing_errors_doc)]
pub fn contains(country: Country, date: NaiveDate) -> Result<bool> {
    let holiday_map = get_holiday_map()?;

    let map = get_map_for_country_and_year(&holiday_map, country, date.year())?;

    Ok(map.get(date).is_some())
}

fn get_holiday_map() -> Result<RwLockReadGuard<'static, HolidayMap>> {
    let data = Lazy::get(&DATA).ok_or(Error::Uninitialized)?;
    data.read().map_err(|e| Error::LockError(e.to_string()))
}

struct CountryWithSubdivision<'a> {
    country: &'a HolidayPerDayMap,
    subdivision: Option<&'a HolidayPerDayMap>,
}

impl<'a> CountryWithSubdivision<'a> {
    pub fn get(&self, date: NaiveDate) -> Option<&Holiday> {
        self.country
            .get(&date)
            .or_else(|| self.subdivision.and_then(|s| s.get(&date)))
    }
}

fn get_map_for_country_and_year<'a>(
    holiday_map: &'a RwLockReadGuard<'a, HolidayMap>,
    country: Country,
    year: Year,
) -> Result<CountryWithSubdivision<'a>> {
    let (country, subdivision) = Country::country_from_subdivision(country)
        .map_or_else(|| (country, None), |c| (c, Some(country)));

    let map = holiday_map
        .get(&country)
        .ok_or(Error::CountryNotAvailable)?;

    let map = map.get(&year).ok_or(Error::YearNotAvailable)?;

    Ok(CountryWithSubdivision {
        country: map,
        subdivision: subdivision
            .and_then(|subdiv| holiday_map.get(&subdiv))
            .and_then(|m| m.get(&year)),
    })
}

/// Represents a holiday.
#[derive(Debug, Clone)]
pub struct Holiday {
    /// two-letter country code defined in ISO 3166-1 alpha-2.
    pub code: Country,
    /// Country name.
    pub country: String,
    /// Date of holiday.
    pub date: chrono::NaiveDate,
    /// Name of holiday.
    pub name: String,
}

impl Holiday {
    fn new(
        code: Country,
        country: impl Into<String>,
        date: NaiveDate,
        name: impl Into<String>,
    ) -> Self {
        Self {
            code,
            country: country.into(),
            date,
            name: name.into(),
        }
    }
}

/// Error type of this crate.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Holiday is not available for this country.
    #[error("Holiday is not available for this country")]
    CountryNotAvailable,
    /// Holiday is not available for this year.
    #[error("Holiday is not available for this year")]
    YearNotAvailable,
    /// Holiday database is not initialized yet.
    #[error("Holiday database is not initialized yet")]
    Uninitialized,
    /// Failed to get `RwLock`.
    #[error("Failed to get RwLock: {0}")]
    LockError(String),
    /// Unexpected error occurred.
    #[error("Unexpected error occurred: {0}")]
    Unexpected(String),
}

fn init_holiday(map: HolidayMap) -> Result<()> {
    match DATA.write() {
        Ok(mut data) => {
            *data = map;
            Ok(())
        }
        Err(e) => Err(Error::LockError(e.to_string())),
    }
}

/// Helper trait used inside the crate and tests.
#[doc(hidden)]
pub trait NaiveDateExt {
    fn from_ymd_res(year: i32, month: u32, day: u32) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| Error::Unexpected(format!("invalid date {year}-{month}-{day}")))
    }
}

impl NaiveDateExt for NaiveDate {}
