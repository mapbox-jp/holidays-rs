use chrono::NaiveDate;

use crate::{prelude::*, Holiday, HolidayPerCountryMap, Year};

pub fn should_build_year(years: Option<&std::ops::Range<Year>>, year: Year) -> bool {
    years.map_or(true, |r| r.contains(&year))
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_year(
    years: Option<&std::ops::Range<Year>>,
    year: Year,
    holidays: impl IntoIterator<Item = (NaiveDate, &'static str)>,
    map: &mut HolidayPerCountryMap,
    country: Country,
    county_name: &(impl ToString + ?Sized),
) {
    if !should_build_year(years, year) {
        return;
    }

    let m = holidays
        .into_iter()
        .map(|h| {
            (
                h.0,
                Holiday::new(country, county_name.to_string(), h.0, h.1),
            )
        })
        .collect();

    map.insert(year, m);
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_subdivision_year(
    years: Option<&std::ops::Range<Year>>,
    year: Year,
    national_holidays: &mut HolidayPerCountryMap,
    subdivision_holidays: impl IntoIterator<Item = (NaiveDate, &'static str)>,
    map: &mut HolidayPerCountryMap,
    country: Country,
    county_name: &(impl ToString + ?Sized),
) {
    if !should_build_year(years, year) {
        return;
    }

    let m = national_holidays
        .remove(&year)
        .unwrap_or_default()
        .into_iter()
        .chain(subdivision_holidays.into_iter().map(|h| {
            (
                h.0,
                Holiday::new(country, county_name.to_string(), h.0, h.1),
            )
        }))
        .collect();

    map.insert(year, m);
}
