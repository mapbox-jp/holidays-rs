use crate::country::{Country, CountrySet, CountrySetHolidayIter};
use crate::{date::Date, Holiday};

#[derive(Clone, Copy)]
pub(crate) struct Query {
    countries: CountrySet,
    date_filter: Option<DateQuery>,
}

impl Query {
    pub const EMPTY: Query = Query {
        countries: CountrySet::all(),
        date_filter: None,
    };

    pub const fn country(value: Country) -> Self {
        Query {
            countries: {
                let mut countries = CountrySet::new();
                countries.insert(value);
                countries
            },
            date_filter: None,
        }
    }

    pub fn countries<I>(value: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Country>,
    {
        Query {
            countries: {
                let mut countries = CountrySet::new();
                countries.extend(value);
                countries
            },
            date_filter: None,
        }
    }

    #[allow(dead_code)]
    pub const fn year(value: i64) -> Self {
        Query {
            countries: CountrySet::new(),
            date_filter: Some(DateQuery::year(value)),
        }
    }

    #[allow(dead_code)]
    pub fn year_range<R: std::ops::RangeBounds<i64>>(value: R) -> Self {
        Query {
            countries: CountrySet::new(),
            date_filter: DateQuery::year_range(value),
        }
    }

    pub fn date(value: impl Into<Date>) -> Self {
        Query {
            countries: CountrySet::new(),
            date_filter: Some(DateQuery::date(value)),
        }
    }

    pub fn date_range<D, R>(value: R) -> Self
    where
        D: Into<Date> + Clone,
        R: std::ops::RangeBounds<D>,
    {
        Query {
            countries: CountrySet::new(),
            date_filter: DateQuery::date_range(value),
        }
    }

    pub fn and(mut self, other: Self) -> Self {
        self.countries |= other.countries;
        self.date_filter = match (self.date_filter, other.date_filter) {
            (None, Some(it)) => Some(it),
            (Some(it), None) => Some(it),
            (Some(a), Some(b)) => Some(a & b),
            (None, None) => None,
        };
        self
    }
}

impl std::ops::BitAnd for Query {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl std::ops::BitAndAssign for Query {
    fn bitand_assign(&mut self, rhs: Self) {
        self.countries &= rhs.countries;
        self.date_filter = match (self.date_filter, rhs.date_filter) {
            (None, Some(it)) => Some(it),
            (Some(it), None) => Some(it),
            (Some(a), Some(b)) => Some(a & b),
            (None, None) => None,
        };
    }
}

impl IntoIterator for Query {
    type Item = <Iter as Iterator>::Item;
    type IntoIter = Iter;
    
    fn into_iter(self) -> Self::IntoIter {
        Iter(match self.date_filter {
            Some(empty) if empty.is_empty() => IterImpl::Empty,
            Some(DateQuery::Exact(date)) => IterImpl::Exact {
                inner: self.countries.iter(),
                date,
            },
            Some(date_query) => IterImpl::DateRange {
                range: date_query.as_data_range(),
                countries: self.countries,
            },
            None => IterImpl::NoDate(self.countries.holidays()),
        })
    }
}

#[derive(Clone, Copy)]
enum DateQuery {
    Exact(Date),
    FromDate(Date),
    ToDate(Date),
    DateRange(Date, Date),
}

impl DateQuery {
    const EMPTY: DateQuery = DateQuery::DateRange(Date(0), Date(0));

    #[allow(dead_code)]
    #[inline(always)]
    const fn year(value: i64) -> Self {
        DateQuery::DateRange(Date::from_year(value), Date::from_year(value + 1))
    }

    #[allow(dead_code)]
    #[inline(always)]
    fn year_range<R>(value: R) -> Option<Self>
    where
        R: std::ops::RangeBounds<i64>,
    {
        let start = match value.start_bound() {
            std::ops::Bound::Included(it) => Date::from_year(*it),
            std::ops::Bound::Excluded(it) => Date::from_year(it + 1),
            std::ops::Bound::Unbounded => match value.end_bound() {
                std::ops::Bound::Included(it) => {
                    return Some(DateQuery::ToDate(Date::from_year(it + 1)))
                }
                std::ops::Bound::Excluded(it) => {
                    return Some(DateQuery::ToDate(Date::from_year(*it)))
                }
                std::ops::Bound::Unbounded => return None,
            },
        };

        let end = match value.end_bound() {
            std::ops::Bound::Included(it) => Date::from_year(it + 1),
            std::ops::Bound::Excluded(it) => Date::from_year(*it),
            std::ops::Bound::Unbounded => match value.start_bound() {
                std::ops::Bound::Included(it) => {
                    return Some(DateQuery::FromDate(Date::from_year(*it)))
                }
                std::ops::Bound::Excluded(it) => {
                    return Some(DateQuery::FromDate(Date::from_year(it + 1)))
                }
                std::ops::Bound::Unbounded => return None, // unreachable
            },
        };

        Some(DateQuery::DateRange(start, end))
    }

    #[inline(always)]
    fn date(value: impl Into<Date>) -> Self {
        DateQuery::Exact(value.into())
    }

    #[inline(always)]
    fn date_range<D, R>(value: R) -> Option<Self>
    where
        D: Into<Date> + Clone,
        R: std::ops::RangeBounds<D>,
    {
        let start = match value.start_bound() {
            std::ops::Bound::Included(it) => it.clone().into(),
            std::ops::Bound::Excluded(it) => {
                let mut it = it.clone().into();
                it.0 += 1;
                it
            }
            std::ops::Bound::Unbounded => match value.end_bound() {
                std::ops::Bound::Included(it) => {
                    return Some(DateQuery::ToDate({
                        let mut it = it.clone().into();
                        it.0 += 1;
                        it
                    }))
                }
                std::ops::Bound::Excluded(it) => return Some(DateQuery::ToDate(it.clone().into())),
                std::ops::Bound::Unbounded => return None,
            },
        };

        let end = match value.end_bound() {
            std::ops::Bound::Included(it) => {
                let mut it = it.clone().into();
                it.0 += 1;
                it
            }
            std::ops::Bound::Excluded(it) => it.clone().into(),
            std::ops::Bound::Unbounded => match value.start_bound() {
                std::ops::Bound::Included(it) => {
                    return Some(DateQuery::FromDate(it.clone().into()))
                }
                std::ops::Bound::Excluded(it) => {
                    return Some(DateQuery::FromDate({
                        let mut it = it.clone().into();
                        it.0 += 1;
                        it
                    }))
                }
                std::ops::Bound::Unbounded => return None, // unreachable
            },
        };

        Some(DateQuery::DateRange(start, end))
    }

    fn is_empty(&self) -> bool {
        match self {
            DateQuery::DateRange(a, b) => a >= b,
            _ => false,
        }
    }

    fn as_data_range(&self) -> std::ops::Range<usize> {
        const DATA_LEN: usize = crate::data::DATA.len();
        match self {
            DateQuery::DateRange(from, to) => {
                match (
                    crate::data::date_to_index(*from),
                    crate::data::date_to_index(*to),
                ) {
                    (Some(a), Some(b)) => a..b,
                    (Some(a), None) => a..DATA_LEN,
                    (None, Some(b)) => 0..b,
                    (None, None) => 0..0,
                }
            }
            DateQuery::FromDate(date) => match crate::data::date_to_index(*date) {
                Some(it) => it..DATA_LEN,
                None => DATA_LEN..DATA_LEN,
            },
            DateQuery::ToDate(date) => match crate::data::date_to_index(*date) {
                Some(it) => 0..it,
                None => 0..0,
            },
            DateQuery::Exact(date) => {
                let from = match crate::data::date_to_index(*date) {
                    Some(it) => it,
                    None => return DATA_LEN..DATA_LEN,
                };
                match crate::data::date_to_index(Date(date.0 + 1)) {
                    Some(to) => from..to,
                    None => from..DATA_LEN,
                }
            }
        }
    }
}

impl std::ops::BitAnd for DateQuery {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DateQuery::FromDate(a), DateQuery::FromDate(b)) => DateQuery::FromDate(a.max(b)),
            (DateQuery::ToDate(a), DateQuery::ToDate(b)) => DateQuery::ToDate(a.min(b)),
            (DateQuery::Exact(a), DateQuery::Exact(b)) => {
                if a != b {
                    DateQuery::EMPTY
                } else {
                    DateQuery::Exact(a)
                }
            }

            (DateQuery::FromDate(a), DateQuery::Exact(b))
            | (DateQuery::Exact(b), DateQuery::FromDate(a)) => {
                if a > b {
                    DateQuery::EMPTY
                } else {
                    DateQuery::Exact(b)
                }
            }
            (DateQuery::ToDate(a), DateQuery::Exact(b))
            | (DateQuery::Exact(b), DateQuery::ToDate(a)) => {
                if b >= a {
                    DateQuery::EMPTY
                } else {
                    DateQuery::Exact(b)
                }
            }
            (DateQuery::Exact(a), DateQuery::DateRange(b_from, b_to))
            | (DateQuery::DateRange(b_from, b_to), DateQuery::Exact(a)) => {
                if b_from > a || b_to <= a {
                    DateQuery::EMPTY
                } else {
                    DateQuery::Exact(a)
                }
            }
            (DateQuery::FromDate(a), DateQuery::ToDate(b))
            | (DateQuery::ToDate(b), DateQuery::FromDate(a)) => {
                if a >= b {
                    DateQuery::EMPTY
                } else {
                    DateQuery::DateRange(a, b)
                }
            }
            (DateQuery::FromDate(a), DateQuery::DateRange(b_from, b_to))
            | (DateQuery::DateRange(b_from, b_to), DateQuery::FromDate(a)) => {
                let from = a.max(b_from);
                if from >= b_to {
                    DateQuery::EMPTY
                } else {
                    DateQuery::DateRange(from, b_to)
                }
            }
            (DateQuery::ToDate(a), DateQuery::DateRange(b_from, b_to))
            | (DateQuery::DateRange(b_from, b_to), DateQuery::ToDate(a)) => {
                let to = a.max(b_to);
                if to >= b_from {
                    DateQuery::EMPTY
                } else {
                    DateQuery::DateRange(b_from, to)
                }
            }
            (DateQuery::DateRange(a_from, a_to), DateQuery::DateRange(b_from, b_to)) => {
                let from = a_from.max(b_from);
                let to = a_to.min(b_to);
                if to >= from {
                    DateQuery::EMPTY
                } else {
                    DateQuery::DateRange(from, to)
                }
            }
        }
    }
}

#[derive(Clone)]
enum IterImpl {
    Empty,
    Exact {
        inner: crate::country::CountrySetIter,
        date: Date,
    },
    DateRange {
        range: std::ops::Range<usize>,
        countries: CountrySet,
    },
    NoDate(CountrySetHolidayIter),
}

/// Iterator over holiday query results.
#[derive(Clone)]
pub struct Iter(IterImpl);

impl Iterator for Iter {
    type Item = &'static Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterImpl::Empty => None,
            IterImpl::Exact { inner, date } => loop {
                let next = inner.next()?;
                if let Some(it) = crate::data::country_date_to_holiday(next, *date) {
                    return Some(it);
                }
            },
            IterImpl::DateRange { range, countries } => loop {
                let i = range.next()?;
                let result = &crate::data::DATA[i];
                if countries.contains(result.code) {
                    return Some(result);
                }
            },
            IterImpl::NoDate(inner) => inner.next(),
        }
    }
}

#[derive(Clone)]
enum BoundsResultImpl<I>
where
    I: Iterator,
    I::Item: Into<Country>,
{
    Empty,
    One(Country),
    Many(I),
}
#[derive(Clone)]
pub(crate) struct BoundsResult<I>(BoundsResultImpl<I>)
where
    I: Iterator,
    I::Item: Into<Country>;

impl<I> Iterator for BoundsResult<I>
where
    I: Iterator,
    I::Item: Into<Country>,
{
    type Item = (Country, Option<(&'static Holiday, &'static Holiday)>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = match &mut self.0 {
            BoundsResultImpl::Empty => return None,
            BoundsResultImpl::One(country) => {
                let value = *country;
                self.0 = BoundsResultImpl::Empty;
                value
            }
            BoundsResultImpl::Many(inner) => inner.next().map(|it| it.into())?,
        };

        let indices = crate::data::COUNTRY_JUMP_TABLE[next as usize];
        let bounds = indices.first().map(|it| {
            let min = it;
            // SAFETY: If first value exists, as guaranteed by `map`, then last
            // value must exist as well and be either the same value or some
            // later one in the slice.
            let max = unsafe { indices.last().unwrap_unchecked() };

            // SAFETY: Every index stored in `COUNTRY_JUMP_TABLE` is a valid index into `DATA`
            let (min, max) = unsafe {
                (
                    crate::data::DATA.get_unchecked(*min),
                    crate::data::DATA.get_unchecked(*max),
                )
            };

            (min, max)
        });

        Some((next, bounds))
    }
}

pub mod selection {
    use super::*;

    /// Selection qualifier that makes the query ignore a certain axis.
    pub struct Any;

    pub enum CountrySelection<I>
    where
        I: IntoIterator,
        I::Item: Into<Country>,
    {
        All,
        One(Country),
        Many(I),
    }

    impl<I> CountrySelection<I>
    where
        I: IntoIterator,
        I::Item: Into<Country>,
    {
        pub(crate) fn into_query(self) -> Query {
            match self {
                CountrySelection::All => Query::EMPTY,
                CountrySelection::One(one) => Query::country(one),
                CountrySelection::Many(many) => Query::countries(many),
            }
        }

        pub(crate) fn bounds(self) -> BoundsResult<I::IntoIter> {
            BoundsResult(match self {
                CountrySelection::All => BoundsResultImpl::Empty,
                CountrySelection::One(country) => BoundsResultImpl::One(country),
                CountrySelection::Many(countries) => BoundsResultImpl::Many(countries.into_iter()),
            })
        }
    }

    impl From<Any> for CountrySelection<std::iter::Empty<Country>> {
        fn from(_: Any) -> Self {
            CountrySelection::All
        }
    }

    impl From<Country> for CountrySelection<std::iter::Empty<Country>> {
        fn from(value: Country) -> Self {
            CountrySelection::One(value)
        }
    }

    // `Option` is an iterator as well
    impl<I> From<I> for CountrySelection<I>
    where
        I: IntoIterator,
        I::Item: Into<Country>,
    {
        fn from(value: I) -> Self {
            CountrySelection::Many(value)
        }
    }

    pub enum DateSelection<D, R>
    where
        D: Into<Date>,
        R: std::ops::RangeBounds<D>,
    {
        None,
        One(D),
        Range(R),
    }

    impl<D, R> DateSelection<D, R>
    where
        D: Into<Date> + Clone,
        R: std::ops::RangeBounds<D>,
    {
        pub(crate) fn into_query(self) -> Query {
            match self {
                DateSelection::None => Query::EMPTY,
                DateSelection::One(one) => Query::date(one),
                DateSelection::Range(range) => Query::date_range(range),
            }
        }
    }

    impl<D> From<Any> for DateSelection<D, std::ops::Range<D>>
    where
        D: Into<Date>,
    {
        fn from(_: Any) -> Self {
            DateSelection::None
        }
    }

    impl<D> From<Option<D>> for DateSelection<D, std::ops::Range<D>>
    where
        D: Into<Date>,
    {
        fn from(value: Option<D>) -> Self {
            match value {
                Some(it) => DateSelection::One(it),
                None => DateSelection::None,
            }
        }
    }

    impl<D> From<D> for DateSelection<D, std::ops::Range<D>>
    where
        D: Into<Date>,
    {
        fn from(value: D) -> Self {
            DateSelection::One(value)
        }
    }

    impl<D, R> From<R> for DateSelection<D, R>
    where
        D: Into<Date> + Clone,
        R: std::ops::RangeBounds<D>,
    {
        fn from(value: R) -> Self {
            DateSelection::Range(value)
        }
    }
}
