use crate::{build::build, init_holiday, prelude::*, HolidayMap, Result, Year};
use std::{collections::HashSet, ops::Range};

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
