use crate::query::selection::*;
use std::time::Duration;

/// Internal date representation.
///
/// This type is not part of public API and only serves as an intermediate
/// representation of a date for this crate.
///
/// It can change at any time without affecting the semver and shouldn't be
/// used outside the library.
///
/// Smallest representable date: -25252734927764585-06-07
/// Largest representable date:   25252734927766554-09-25
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Date(
    /// Number of days since 1st of January, 1970. (UNIX epoch).
    pub(crate) i64,
);

#[allow(missing_docs)]
impl Date {
    const YEAR_DAYS: u32 = 365;
    const ERA_YEARS: u32 = 400; // calendar repeats itself exactly every 400 years

    const DAYS_IN_4_YEARS: u32 = 4 * Self::YEAR_DAYS + 1; // 1461
    const DAYS_IN_100_YEARS: u32 = 25 * Self::DAYS_IN_4_YEARS - 1; // 36524
    const DAYS_IN_400_YEARS: u32 = 4 * Self::DAYS_IN_100_YEARS + 1; // 146097
    const ERA_DAYS: u32 = Self::DAYS_IN_400_YEARS;

    const UNIX_EPOCH_DAY: i64 = 719468;

    pub const fn from_ymd(year: i64, month: u8, day: u8) -> Self {
        // Source: https://howardhinnant.github.io/date_algorithms.html#days_from_civil

        debug_assert!(month >= 1 && month <= 12, "month not in range [1, 12]");
        debug_assert!(day >= 1 && day <= 31, "day not in range [1, 31]");
        #[cfg(debug_assertions)]
        match (year, month, day) {
            (25252734927766554, 9, d) if d > 25 => panic!("date too large"),
            (25252734927766554, m, _) if m > 9 => panic!("date too large"),
            (y, ..) if y > 25252734927766554 => panic!("date too large"),
            (-25252734927764585, 6, d) if d < 7 => panic!("date too small"),
            (-25252734927764585, m, _) if m < 6 => panic!("date too small"),
            (y, ..) if y < -25252734927764585 => panic!("date too small"),
            _ => {}
        }

        let mut y = year;
        let m = month as i64;
        let d = day as i64;

        if m <= 2 {
            y -= 1
        }

        let era = y.div_euclid(Self::ERA_YEARS as i64);
        let year_of_era = (y - era * Self::ERA_YEARS as i64) as u32;
        debug_assert!(year_of_era < Self::ERA_YEARS, "year_of_era >= ERA_YEARS");

        let day_of_year = ((153 * ((m + 9) % 12) + 2) / 5 + d - 1) as u32;
        debug_assert!(day_of_year <= Self::YEAR_DAYS, "day_of_year > YEAR_DAYS");

        let day_of_era =
            year_of_era * Self::YEAR_DAYS + year_of_era / 4 - year_of_era / 100 + day_of_year;
        debug_assert!(year_of_era < Self::ERA_YEARS, "year_of_era >= ERA_YEARS");

        let days = era * Self::ERA_DAYS as i64 + (day_of_era as i64) - Self::UNIX_EPOCH_DAY;

        Self(days)
    }

    #[inline]
    pub const fn from_year(year: i64) -> Self {
        Self::from_ymd(year, 1, 1)
    }

    pub const fn ymd(&self) -> (i64, u8, u8) {
        // Source: https://howardhinnant.github.io/date_algorithms.html#civil_from_days

        debug_assert!(self.0 < i64::MAX - Self::UNIX_EPOCH_DAY, "date too large");
        let julian_days = self.0 + Self::UNIX_EPOCH_DAY;

        let era = julian_days.div_euclid(Self::ERA_DAYS as i64);
        let day_of_era = julian_days.rem_euclid(Self::ERA_DAYS as i64) as u32;
        debug_assert!(day_of_era < Self::ERA_DAYS, "day_of_era >= ERA_DAYS");

        let year_of_era = {
            let leap_years = day_of_era / (Self::DAYS_IN_4_YEARS - 1); // adjustment for leap days according to julian calendar
            let centuries = day_of_era / (Self::DAYS_IN_100_YEARS - 1); // except century years not exactly divisible by 400
            let last_day = day_of_era / (Self::DAYS_IN_400_YEARS - 1); // but the first one should be included
            (day_of_era - leap_years + centuries - last_day) / Self::YEAR_DAYS
        };
        debug_assert!(year_of_era < Self::ERA_YEARS, "year_of_era >= ERA_YEARS");

        let mut year = year_of_era as i64 + era * Self::ERA_YEARS as i64;
        let day_of_year: u32 =
            day_of_era - (Self::YEAR_DAYS * year_of_era + year_of_era / 4 - year_of_era / 100);
        debug_assert!(day_of_year <= Self::YEAR_DAYS, "day_of_year > YEAR_DAYS");

        let month_shifted: u32 = (5 * day_of_year + 2) / 153;
        debug_assert!(month_shifted <= 11, "month_shifted > 11");

        let day: u32 = day_of_year - (153 * month_shifted + 2) / 5 + 1;
        debug_assert!(day >= 1 && day <= 31, "day not in range [1, 31]");

        let month: u32 = if month_shifted < 10 {
            month_shifted + 3
        } else {
            month_shifted - 9
        };
        debug_assert!(month >= 1 && month <= 12, "month not in range [1, 12]");

        if month <= 2 {
            year += 1;
        }

        (year, month as u8, day as u8)
    }

    /// Day of the month
    #[inline]
    pub const fn day(&self) -> u8 {
        self.ymd().2
    }

    /// Month of the year
    #[inline]
    pub const fn month(&self) -> u8 {
        self.ymd().1
    }

    /// Year
    #[inline]
    pub const fn year(&self) -> i64 {
        self.ymd().0
    }

    pub const fn days_since(&self, other: &Self) -> Result<usize, usize> {
        if self.0 > other.0 {
            Ok((self.0 - other.0) as usize)
        } else {
            Err((other.0 - self.0) as usize)
        }
    }

    pub const fn duration_since(&self, other: &Self) -> Result<Duration, Duration> {
        if self.0 > other.0 {
            Ok(Duration::from_secs(
                SECONDS_IN_DAY as u64 * (self.0 - other.0) as u64,
            ))
        } else {
            Err(Duration::from_secs(
                SECONDS_IN_DAY as u64 * (other.0 - self.0) as u64,
            ))
        }
    }
}

/// An `i64` value is treated like a year.
impl From<i64> for Date {
    fn from(value: i64) -> Self {
        Date::from_year(value)
    }
}

const SECONDS_IN_DAY: i64 = 86400;

impl TryFrom<Date> for std::time::SystemTime {
    type Error = DateConversionError;

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let mul_res = value
            .0
            .checked_mul(SECONDS_IN_DAY)
            .ok_or(DateConversionError)?;

        if value.0 >= 0 {
            Ok(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(mul_res as u64))
        } else {
            Ok(std::time::SystemTime::UNIX_EPOCH - std::time::Duration::from_secs(-mul_res as u64))
        }
    }
}
impl From<std::time::SystemTime> for Date {
    fn from(value: std::time::SystemTime) -> Self {
        let days = match value.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(duration) => (duration.as_secs() as i128 / SECONDS_IN_DAY as i128) as i64,
            Err(err) => (-(err.duration().as_secs() as i128 / SECONDS_IN_DAY as i128)) as i64,
        };

        Date(days)
    }
}

#[cfg(feature = "chrono")]
impl TryFrom<Date> for chrono::NaiveDate {
    type Error = DateConversionError;

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        if value.0 > i32::MAX as isize - 719163 {
            return Err(DateConversionError);
        }
        chrono::NaiveDate::from_num_days_from_ce_opt(value.0 as i32 + 719163)
            .ok_or(DateConversionError)
    }
}
#[cfg(feature = "chrono")]
impl From<chrono::NaiveDate> for Date {
    fn from(value: chrono::NaiveDate) -> Self {
        let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let days = value.signed_duration_since(epoch).num_days();
        Date(days as isize)
    }
}

#[cfg(feature = "time")]
impl TryFrom<Date> for time::Date {
    type Error = DateConversionError;

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        if value.0.abs() > i64::MAX as isize {
            return Err(DateConversionError);
        }

        // `Date` days since 1970-01-01; time::Date::from_julian_day starts from -4713-11-24
        let epoch = time::Date::from_calendar_date(1970, time::Month::January, 1)
            .map_err(|_| DateConversionError)?;
        Ok(epoch.saturating_add(time::Duration::days(value.0 as i64)))
    }
}
#[cfg(feature = "time")]
impl From<time::Date> for Date {
    fn from(value: time::Date) -> Self {
        let epoch = time::Date::from_calendar_date(1970, time::Month::January, 1).unwrap();
        let days = (value - epoch).whole_days();
        Date(days as isize)
    }
}

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (y, m, d) = self.ymd();
        write!(f, "Date({y:04}-{m:02}-{d:02})")
    }
}

/// Utility functions that extend all supported date types and provide methods
/// on them to directly query holiday information.
pub trait DateExt<DateLike, DateRange = std::ops::Range<DateLike>>:
    Into<DateSelection<DateLike, DateRange>> + Clone
where
    DateLike: Into<Date> + Clone,
    DateRange: std::ops::RangeBounds<DateLike>,
{
    // Most date(time) types are trivial to Clone

    /// Returns an iterator of holidays that are observed on this date (range)
    /// in specified `countries`.
    ///
    /// This is an alias for [`get_holidays`] method, see that method for more
    /// details.
    ///
    /// [`get_holidays`]: crate::get_holidays
    fn holidays<CountryIter>(
        &self,
        countries: impl Into<CountrySelection<CountryIter>>,
    ) -> crate::Iter
    where
        CountryIter: IntoIterator,
        CountryIter::Item: Into<crate::Country>,
    {
        crate::get_holidays(countries, self.clone())
    }

    /// Returns `true` if any holidays are observed on this date (range) in
    /// specified `countries`.
    ///
    /// This is an alias for [`is_holiday`] method, see that method for more
    /// details.
    ///
    /// [`is_holiday`]: crate::is_holiday
    fn is_holiday<CountryIter>(&self, countries: impl Into<CountrySelection<CountryIter>>) -> bool
    where
        CountryIter: IntoIterator,
        CountryIter::Item: Into<crate::Country>,
    {
        crate::is_holiday(countries, self.clone())
    }
}

macro_rules! impl_ext_for_t {
    (if $guard: literal $($param: tt)*) => {
        #[cfg(feature = $guard)]
        impl DateExt<$($param)*> for $($param)* {}
        #[cfg(feature = $guard)]
        impl<R> DateExt<$($param)*, R> for R where
            R: std::ops::RangeBounds<$($param)*> + Clone {}
    };
    ($($param: tt)*) => {
        impl DateExt<$($param)*> for $($param)* {}
        impl<R> DateExt<$($param)*, R> for R where
            R: std::ops::RangeBounds<$($param)*> + Clone {}
    };
}
impl_ext_for_t!(std::time::SystemTime);
impl_ext_for_t!(if "chrono" chrono::NaiveDate);
impl_ext_for_t!(if "chrono" chrono::DateTime<chrono::Utc>);
impl_ext_for_t!(if "chrono" chrono::DateTime<chrono::Local>);
impl_ext_for_t!(if "time" time::Date);
impl_ext_for_t!(if "time" time::OffsetDateTime);
impl_ext_for_t!(if "time" time::PrimitiveDateTime);

/// Error returned when conversion to/from another date format can't be
/// performed because one has larger span than the other and conversion would
/// cause an overflow.
///
/// In most practical use cases this won't happen because dates that are stored
/// in holidays table can be reasonably converted to most time libraries.
#[derive(Debug, PartialEq, Eq)]
pub struct DateConversionError;
crate::error::error_msg!(DateConversionError, "Date is too large for conversion");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Country;
    use std::{hint::black_box, time::SystemTime};

    fn round_trip(y: i64, m: u8, d: u8) {
        let date = black_box(Date::from_ymd(y, m, d));
        let (y_out, m_out, d_out) = date.ymd();
        assert_eq!(y, y_out);
        assert_eq!(m, m_out);
        assert_eq!(d, d_out);
    }

    #[test]
    fn ymd_to_days() {
        // test ymd is inverse of from_ymd
        round_trip(2025, 5, 20);
        round_trip(4423, 1, 18);
        round_trip(-2344, 2, 6);

        // days are computed using: https://aa.usno.navy.mil/data/JulianDate
        // with 2440588 offset from their

        println!("{:?}", Date(-2440588));

        // test from_ymd works:
        let date = Date::from_ymd(1970, 1, 1);
        assert_eq!(date.0, 0);

        let date = Date::from_ymd(1970, 2, 1);
        assert_eq!(date.0, 31);

        let date = Date::from_ymd(2025, 6, 12);
        assert_eq!(date.0, 20251);

        let date = Date::from_ymd(1602, 10, 12);
        assert_eq!(date.0, -134125);

        let date = Date::from_ymd(6453, 3, 15);
        assert_eq!(date.0, 1637456);
    }

    #[test]
    fn date_ext_type_interface() {
        // This test pins down type interface requirements of DateExt.
        // It's failing if it doesn't compile.

        let time = SystemTime::now();

        let country_opt: Option<Country> = None;

        let _ = time.holidays(Any);
        let _ = time.holidays(country_opt);
        let _ = time.holidays(Country::US);
        let _ = time.holidays(&[Country::US, Country::JP]);
        let _ = time.holidays([Country::US, Country::JP]);
        let _ = time.holidays(vec![Country::DE, Country::HR]);

        let time_ref = &time;

        let _ = time_ref.holidays(Any);
        let _ = time_ref.holidays(country_opt);
        let _ = time_ref.holidays(Country::US);
        let _ = time_ref.holidays(&[Country::US, Country::JP]);
        let _ = time_ref.holidays([Country::US, Country::JP]);
        let _ = time_ref.holidays(vec![Country::DE, Country::HR]);

        let time_range = time..time;

        let _ = time_range.holidays(Any);
        let _ = time_range.holidays(country_opt);
        let _ = time_range.holidays(Country::US);
        let _ = time_range.holidays(&[Country::US, Country::JP]);
        let _ = time_range.holidays([Country::US, Country::JP]);
        let _ = time_range.holidays(vec![Country::DE, Country::HR]);
    }
}
