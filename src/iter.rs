use chrono::{Datelike, NaiveDate};
use once_cell::sync::Lazy;

use crate::{prelude::*, Error, Holiday, Result, DATA};
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

    let mut buf = VecDeque::new();

    let mut y = since.year();
    while y <= until.year() {
        let data = data.read().map_err(|e| Error::LockError(e.to_string()))?;
        let map = data.get(&country).ok_or(Error::CountryNotAvailable)?;

        let Some(map) = map.get(&y) else {
            break;
        };

        buf.extend(map.values().cloned());

        y += 1;
    }

    Ok(Iter { since, until, buf })
}
