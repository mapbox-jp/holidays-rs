use std::hash::Hash;

use crate::country::Country;
use crate::date::Date;
use crate::Holiday;

include!(concat!(env!("OUT_DIR"), "/holiday_data.rs"));

pub(crate) fn year_to_index(year: i64) -> Option<usize> {
    if year < DATA_MIN_YEAR {
        return None;
    }
    if year > DATA_MAX_YEAR {
        return None;
    }
    // SAFETY: Bounds explicitly handled by above returns. `year`s in
    // [DATA_MIN_YEAR, DATA_MAX_YEAR] range are ensured to be in YEAR_JUMP_TABLE
    // by build.rs
    unsafe {
        Some(*YEAR_JUMP_TABLE.get_unchecked((year - DATA_MIN_YEAR) as usize))
    }
}

pub(crate) fn date_to_index(date: Date) -> Option<usize> {
    let y = date.year();
    let start = year_to_index(y)?;
    let end = year_to_index(y + 1).unwrap_or(DATA.len());

    let index = DATA[start..end]
        .binary_search_by(|entry| entry.date.cmp(&date))
        .unwrap_or_else(|i| i);

    let absolute_index = start + index;

    if absolute_index >= DATA.len() {
        None
    } else {
        Some(absolute_index)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Point(Country, Date);
impl phf::PhfHash for Point {
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as u16).hash(state);
        (self.1 .0).hash(state);
    }
}
impl phf_shared::PhfBorrow<Point> for Point {
    fn borrow(&self) -> &Point {
        self
    }
}

pub(crate) fn country_date_to_holiday(country: Country, date: Date) -> Option<&'static Holiday> {
    DATA_MAP.get(&Point(country, date)).map(|i| &DATA[*i])
}
