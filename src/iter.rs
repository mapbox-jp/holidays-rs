use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;

use crate::{get_map_for_country_and_year, prelude::*, Error, Holiday, Result, DATA};
use std::collections::VecDeque;

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
#[allow(dead_code, clippy::missing_errors_doc)]
pub fn iter(country: Country, since: NaiveDate, until: NaiveDate) -> Result<Iter> {
    let data = Lazy::get(&DATA).ok_or(Error::Uninitialized)?;

    let mut buf = Vec::new();

    for y in since.year()..=until.year() {
        let data = data.read().map_err(|e| Error::LockError(e.to_string()))?;

        let country_map = match get_map_for_country_and_year(&data, country, y) {
            Err(Error::CountryNotAvailable) => return Err(Error::CountryNotAvailable),
            Err(_) => break,
            Ok(map) => map,
        };

        buf.extend(
            country_map
                .country
                .values()
                .chain(
                    country_map
                        .subdivision
                        .map(|s| s.values())
                        .unwrap_or_default(),
                )
                .cloned(),
        );
    }

    buf.sort_by_key(|h| h.date);

    Ok(Iter {
        since,
        until,
        buf: buf.into_iter().collect(),
    })
}
