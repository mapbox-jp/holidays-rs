use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    ops::Range,
    sync::RwLock,
};

/// Type alias for Holiday map.
pub type HolidayMap = HashMap<Country, HashMap<Year, BTreeMap<NaiveDate, Holiday>>>;

/// Type alias for Year.
pub type Year = i32;

/// Result type of this crate.
pub type Result<T> = std::result::Result<T, Error>;

static DATA: Lazy<RwLock<HolidayMap>> = Lazy::new(|| RwLock::new(HolidayMap::new()));

/// Initialize holiday database for all the supported countries and years.
/// Note that this will use quite lots of memory. Please consider using
/// `Builder` to specify countries and years to load for if you're concerned
/// about memory usage.
pub fn init() -> Result<()> {
    let map = Builder::new().build()?;
    init_holiday(map)
}

/// Get holiday by ISO 3166-1 alpha-2 country code and date. If the specified country or year is
/// not available, it will return `Err(Error)`. If the specified date is not a holiday, it
/// will return `Ok(None)`. Otherwise, it will return `Ok(Some(Holiday))`.
pub fn get(country: Country, date: NaiveDate) -> Result<Option<Holiday>> {
    let Some(data) = Lazy::get(&DATA) else { return Err(Error::Uninitialized); };

    let data = data.read().map_err(|e| Error::LockError(e.to_string()))?;
    let Some(map) = data.get(&country) else {
        return Err(Error::CountryNotAvailable);
    };

    let Some(map) = map.get(&date.year()) else {
        return Err(Error::YearNotAvailable);
    };

    Ok(map.get(&date).cloned())
}

/// Check if the specified date is a holiday. If the specified country or year is
/// not available, it will return `Err(Error)`. If the date is not a holiday, it
/// will return `Ok(false)`. Otherwise, it will return `Ok(true)`.
pub fn contains(country: Country, date: NaiveDate) -> Result<bool> {
    let Some(data) = Lazy::get(&DATA) else { return Err(Error::Uninitialized); };

    let data = data.read().map_err(|e| Error::LockError(e.to_string()))?;
    let Some(map) = data.get(&country) else {
        return Err(Error::CountryNotAvailable);
    };

    let Some(map) = map.get(&date.year()) else {
        return Err(Error::YearNotAvailable);
    };

    Ok(map.get(&date).is_some())
}

#[derive(Debug)]
pub struct Iter {
    since: NaiveDate,
    until: NaiveDate,
    buf: VecDeque<Holiday>,
}

impl std::iter::Iterator for Iter {
    type Item = Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.buf.pop_front()?;

        if next.date < self.since {
            return self.next();
        }

        if next.date < self.until {
            Some(next)
        } else {
            None
        }
    }
}

/// Iterate holidays by dates.
pub fn iter(country: Country, since: NaiveDate, until: NaiveDate) -> Result<Iter> {
    let Some(data) = Lazy::get(&DATA) else { return Err(Error::Uninitialized); };

    let mut buf = VecDeque::new();

    let mut y = since.year();
    while y <= until.year() {
        let data = data.read().map_err(|e| Error::LockError(e.to_string()))?;
        let Some(map) = data.get(&country) else {
            return Err(Error::CountryNotAvailable);
        };

        let Some(map) = map.get(&y) else {
            break;
        };

        buf.extend(map.values().cloned());

        y += 1;
    }

    Ok(Iter { since, until, buf })
}

/// Holiday database builder.
#[derive(Default)]
pub struct Builder {
    countries: Option<HashSet<Country>>,
    years: Option<std::ops::Range<Year>>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify ISO 3166-1 alpha-2 country codes to load.
    pub fn countries(mut self, countries: &[Country]) -> Self {
        self.countries = Some(countries.iter().copied().collect());
        self
    }

    /// Specify range of years to load.
    pub fn years(mut self, years: Range<Year>) -> Self {
        self.years = Some(years);
        self
    }

    /// Build and get holiday database.
    pub fn build(self) -> Result<HolidayMap> {
        let Builder { countries, years } = self;
        build(countries.as_ref(), years.as_ref())
    }

    /// Build and initialize holiday database.
    pub fn init(self) -> Result<()> {
        let Builder { countries, years } = self;
        let map = build(countries.as_ref(), years.as_ref())?;
        init_holiday(map)
    }
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
    ) -> Holiday {
        Holiday {
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
    /// Failed to get RwLock.
    #[error("Failed to get RwLock: {0}")]
    LockError(String),
    /// Unexpexted error occurred.
    #[error("Unexpexted error occurred: {0}")]
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

include!("data.rs");
