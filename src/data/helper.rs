use std::collections::HashSet;

use chrono::NaiveDate;

use crate::{prelude::*, Holiday, HolidayPerYearMap, Year};

pub fn should_build(countries: Option<&HashSet<Country>>, country: Country) -> bool {
    match countries {
        Some(c) => c.contains(&country),
        None => true,
    }
}

pub fn add_main_country_from_subdivisions(c: &mut HashSet<Country>) {
    c.extend(
        c.iter()
            .copied()
            .filter_map(Country::country_from_subdivision)
            .collect::<Vec<_>>(),
    );
}

pub fn should_build_year(years: Option<&std::ops::Range<Year>>, year: Year) -> bool {
    years.map_or(true, |r| r.contains(&year))
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_year(
    years: Option<&std::ops::Range<Year>>,
    year: Year,
    holidays: impl IntoIterator<Item = (NaiveDate, &'static str)>,
    map: &mut HolidayPerYearMap,
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
