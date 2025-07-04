//! A fast and flexible holiday query library.
//!
//! This crate provides a static, compile-time generated database of holidays,
//! optimized for flexible and high-performance querying by country and date.
//!
//! Holiday data is sourced from python
//! [`holidays`](https://pypi.org/project/holidays) library.
//!
//! Holiday data is embedded in the binary using [`phf`](https://docs.rs/phf),
//! enabling fast lookups without runtime data loading. Querying is designed to
//! be ergonomic yet powerful, supporting a wide range of inputs such as single
//! dates, ranges, individual countries, and sets of countries.
//!
//! Internally, the query engine dynamically specializes the iteration strategy
//! to the structure of the query. This includes:
//!
//! - Jump tables for precise date lookup
//! - K-way merges for multi-country scanning
//! - Range-based filtering with sorted input
//!
//! While the query layer may perform heap allocations (e.g., when combining
//! iterators or constructing intermediate query plans), it strives to minimize
//! them through static dispatch, specialized iterators, and zero-copy access
//! patterns.
//!
//! # Example
//!
//! ```
//! # use holidays::internal::Date;
//! use holidays::Country;
//!
//! let holidays: Vec<_> = holidays::get_holidays(
//!     [Country::US, Country::GB], // multiple countries
//!     2025..=2026,                // year range
//! ).collect();
//!
//! for holiday in holidays {
//!     // in real uses, a crate like chrono or time should be used
//!     let time: std::time::SystemTime = holiday.date().unwrap();
//! #   let time = holiday.date::<Date>().unwrap();
//!     println!("{} on {:?}", holiday.name, time); // pretending SystemTime implements Debug
//! }
//! ```
//!
//! # Features
//!
//! - Static, zero-config and zero-allocation holiday database
//! - Efficient date and country queries
//! - Minimal heap allocations, used only when necessary
//! - Extension trait for external types
//! - Optional support for external time libraries: `chrono`, `time`
//!
//! # Performance
//!
//! Although some heap allocations may occur all iteration paths are selected
//! and optimized for performance. Query execution uses tight loops, minimal
//! branching, and completely avoids dynamic dispatch.
//!
//! # When to Use
//!
//! This crate is ideal if you need:
//!
//! - A compile-time embedded holiday dataset
//! - Flexible and expressive querying
//! - High-performance holiday iteration
//!
//! # No Runtime I/O
//!
//! All holiday data is generated at compile time. There is no dependency on
//! runtime files, databases, or network access.

#![warn(missing_docs)]
#![warn(clippy::undocumented_unsafe_blocks)]

mod country;
mod data;
mod date;
mod query;

use date::{Date, DateConversionError};
use query::selection::*;

pub use country::Country;
pub use date::DateExt;
pub use query::selection::Any;
pub use query::Iter;

/// Represents a holiday with an associated country, date, and name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Holiday {
    /// The `Country` this holiday is observed in.
    pub code: Country,
    /// The date of the holiday.
    date: Date,
    /// The name of the holiday.
    pub name: &'static str,
}

impl Holiday {
    /// Returns the date of the holiday in specified format.
    pub fn date<D>(&self) -> Result<D, DateConversionError>
    where
        D: TryFrom<Date>,
    {
        // TryFrom<T> is implemented for all From<T> and error type is
        // Infallible; to unify both conversions error is mapped to
        // DateConversionError. Once specialization is supported, this can be
        // cleaned up.
        <D as TryFrom<Date>>::try_from(self.date).map_err(|_| DateConversionError)
    }
}

/// Queries holidays by countries and date selection and returns an iterator
/// over matching holiday records.
///
/// # Parameters
/// - `countries`: A value that represents a country selection. It can be:
///   - [`Any`] to query all countries,
///   - [`Option`] acts as [`Any`] if `None`,
///   - a single [`Country`], or
///   - any [iterable] container of [`Country`]s (an array, slice, [`Vec`],
///     etc.).
/// - `date`: A value that represents a date range. It can be:
///   - [`Any`] to query all available dates,
///   - [`Option`] acts as [`Any`] if `None`,
///   - a single date, or
///   - a [range] of dates.
///
/// # Type Parameters
/// - `CountryIter`: An [iterable] collection of `Country` values.
/// - `DateLike`: A type that can be converted into a date. Can be any of:
///   - an `isize` representing a Gregorian calendar year,
///   - a [`SystemTime`],
#[cfg_attr(
    feature = "chrono",
    doc = "  - a [`chrono::NaiveDate`] or [`chrono::DateTime<Tz>`],"
)]
#[cfg_attr(
    feature = "time",
    doc = "  - a [`time::Date`], [`time::OffsetDateTime`], or [`time::PrimitiveDateTime`],"
)]
/// - `DateRange`: A type implementing [`RangeBounds<DateLike>`], like
///   [`std::ops::Range`] or [`std::ops::RangeInclusive`].
///
/// In most cases these type parameters can be automatically inferred from
/// provided arguments and don't need to be explicitly specified.
/// 
/// # Examples
///
/// Query holidays for a single country and a single date:
/// ```
/// # use holidays::internal::Date;
/// use holidays::Country;
///
/// let mut holidays = holidays::get_holidays(
///   Country::US,
///   Date::from_ymd(2025, 7, 4)
/// );
/// let holiday = holidays.next().expect("missing data");
/// 
/// assert_eq!(holiday.name, "Independence Day");
/// ```
/// 
/// Query holidays for a single country across a range of years:
/// ```
/// use holidays::Country;
///
/// let mut holidays = holidays::get_holidays(
///   Country::JP,
///   2025..=2026
/// );
/// let observed_holidays = holidays.count();
/// 
/// assert_eq!(observed_holidays, 19);
/// ```
/// 
/// Query holidays over a specific range of dates:
/// ```
/// # use holidays::internal::Date;
/// use holidays::Country;
///
/// let start = Date::from_ymd(2025, 12, 20);
/// let end = Date::from_ymd(2026, 1, 5);
///
/// let mut holidays = holidays::get_holidays(
///   Country::FR,
///   start..=end
/// );
///
/// let names: Vec<_> = holidays.map(|h| h.name).collect();
///
/// assert!(names.contains(&"Christmas Day"));
/// assert!(names.contains(&"New Year's Day"));
/// ```
/// 
/// Query holidays for multiple countries on a specific date:
/// ```
/// # use holidays::internal::Date;
/// use holidays::Country;
///
/// let mut holidays = holidays::get_holidays(
///   &[Country::JP, Country::US], // can be any IntoIterator<Item = Into<Country>>
///   Date::from_ymd(2025, 9, 23)
/// );
/// let holiday = holidays.next().unwrap();
/// 
/// assert_eq!(holiday.name, "Autumnal Equinox");
/// // Autumnal Equinox wasn't observed in US.
/// assert_eq!(holidays.next(), None);
/// ```
/// 
/// Use [`Any`] to include all countries or all dates without filtering:
/// ```
/// # use holidays::internal::Date;
/// use holidays::Any;
///
/// let holidays = holidays::get_holidays(
///   Any,
///   Date::from_ymd(2025, 1, 1)
/// );
/// let n = holidays.count();
/// 
/// println!("{n} countries celebrated New Year's Day!");
/// # assert!(n >= 93); // It's 93 for now but data isn't complete
/// ```
/// 
/// [iterable]: std::iter::IntoIterator
/// [`SystemTime`]: std::time::SystemTime
/// [range]: std::ops::RangeBounds
/// [`RangeBounds<DateLike>`]: std::ops::RangeBounds
pub fn get_holidays<CountryIter, DateLike, DateRange>(
    countries: impl Into<CountrySelection<CountryIter>>,
    date: impl Into<DateSelection<DateLike, DateRange>>,
) -> query::Iter
where
    CountryIter: IntoIterator,
    CountryIter::Item: Into<crate::Country>,
    DateLike: Into<Date> + Clone,
    DateRange: std::ops::RangeBounds<DateLike>,
{
    let country_query = countries.into().into_query();
    let date_query = date.into().into_query();
    country_query.and(date_query).into_iter()
}

/// Returns `true` if any holidays are observed in the specified countries
/// and date selection.
///
/// This function accepts the same flexible input types as [`get_holidays`],
/// but instead of returning an iterator, it simply checks whether at least
/// one matching holiday exists.
///
/// # Parameters
/// - `countries`: A value that represents a country selection. It can be:
///   - [`Any`] to check across all countries,
///   - [`Option`] (treated as [`Any`] if `None`),
///   - a single [`Country`], or
///   - any [iterable] container of [`Country`]s (an array, slice, [`Vec`], etc.).
/// - `date`: A value that represents a date range. It can be:
///   - [`Any`] to check across all dates,
///   - [`Option`] (treated as [`Any`] if `None`),
///   - a single date, or
///   - a [range] of dates.
///
/// # Type Parameters
/// - `CountryIter`: An [iterable] collection of `Country` values.
/// - `DateLike`: A type that can be converted into a date. Can be:
///   - an `isize` representing a Gregorian calendar year,
///   - a [`SystemTime`],
#[cfg_attr(
    feature = "chrono",
    doc = "  - a [`chrono::NaiveDate`] or [`chrono::DateTime<Tz>`],"
)]
#[cfg_attr(
    feature = "time",
    doc = "  - a [`time::Date`], [`time::OffsetDateTime`], or [`time::PrimitiveDateTime`],"
)]
/// - `DateRange`: A type implementing [`RangeBounds<DateLike>`], such as
///   [`std::ops::Range`] or [`std::ops::RangeInclusive`].
///
/// # Examples
///
/// Check if a specific day is a holiday in a given country:
/// ```
/// # use holidays::internal::Date;
/// use holidays::{Country, is_holiday};
///
/// assert!(is_holiday(Country::US, Date::from_ymd(2025, 7, 4)));
/// assert!(!is_holiday(Country::US, Date::from_ymd(2025, 7, 5)));
/// ```
///
/// Check for any holidays in multiple countries:
/// ```
/// # use holidays::internal::Date;
/// use holidays::{Country, is_holiday};
///
/// let countries = &[Country::US, Country::JP];
/// let date = Date::from_ymd(2025, 9, 23);
///
/// assert!(is_holiday(countries, date)); // Autumnal Equinox in JP
/// ```
///
/// Check for holidays within a date range:
/// ```
/// # use holidays::internal::Date;
/// use holidays::{Country, is_holiday};
///
/// let range = Date::from_ymd(2025, 12, 24)..=Date::from_ymd(2025, 12, 26);
///
/// assert!(is_holiday(Country::DE, range)); // Christmas observed
/// ```
///
/// Use [`Any`] to check if *any* country observes a holiday on a given date:
/// ```
/// # use holidays::internal::Date;
/// # use holidays::{Any, is_holiday};
///
/// assert!(is_holiday(Any, Date::from_ymd(2025, 1, 1)));
/// ```
///
/// [iterable]: std::iter::IntoIterator
/// [`SystemTime`]: std::time::SystemTime
/// [range]: std::ops::RangeBounds
/// [`RangeBounds<DateLike>`]: std::ops::RangeBounds
#[inline]
pub fn is_holiday<CountryIter, DateLike, DateRange>(
    countries: impl Into<CountrySelection<CountryIter>>,
    date: impl Into<DateSelection<DateLike, DateRange>>,
) -> bool
where
    CountryIter: IntoIterator,
    CountryIter::Item: Into<crate::Country>,
    DateLike: Into<Date> + Clone,
    DateRange: std::ops::RangeBounds<DateLike>,
{
    get_holidays(countries, date).next().is_some()
}

/// Returns an iterator that provides dates of first and last event for all
/// given `countries` in requested `DateFormat`.
/// 
/// # Panics
/// 
/// Returned iterator will panic if requested `DateFormat` can't represent date
/// of first or last event for some country.
pub fn get_bounding_entries<DateFormat, CountryIter>(
    countries: impl Into<CountrySelection<CountryIter>>,
) -> impl Iterator<Item = (Country, Option<(DateFormat, DateFormat)>)>
where
    DateFormat: TryFrom<Date, Error = DateConversionError>,
    CountryIter: IntoIterator,
    CountryIter::Item: Into<crate::Country>,
{
    countries.into().bounds().map(|(country, bounds)| {
        (
            country,
            bounds.map(|(min, max)| (min.date().unwrap(), max.date().unwrap())),
        )
    })
}

/// Error types returned from the crate.
pub mod error {
    pub use crate::country::CountryParseError;
    pub use crate::date::DateConversionError;

    macro_rules! error_msg {
        ($err: ty, $message: literal $(, $($arg: tt),+)?) => {
            impl std::fmt::Display for $err {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, $message $(, $(self.$arg),+)?)
                }
            }
            impl core::error::Error for $err {}
        };
    }
    pub(crate) use error_msg;
}

/// This module provides direct access to internals that aren't part of public
/// API and can change at any time without affecting semver.
#[doc(hidden)]
pub mod internal {
    pub use crate::date::Date;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        use crate::internal::Date;
        use crate::Any;

        let holidays = crate::get_holidays(Any, Date::from_ymd(2025, 1, 1));
        let o = holidays.count();

        println!("{o} countries celebrated New Year!");
    }
}