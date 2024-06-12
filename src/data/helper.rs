use chrono::NaiveDate;

use crate::{prelude::*, Holiday, HolidayPerCountryMap, Year};

pub fn should_build_year(years: &Option<&std::ops::Range<Year>>, year: Year) -> bool {
    years.map_or(true, |r| r.contains(&year))
}

pub fn build_year(
    years: &Option<&std::ops::Range<Year>>,
    year: Year,
    holidays: impl IntoIterator<Item = (NaiveDate, &'static str)>,
    map: &mut HolidayPerCountryMap,
    country: Country,
    county_name: impl ToString,
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
