use std::ops::Not;

use chrono::NaiveDate;
use holidays::Country;

fn main() -> anyhow::Result<()> {
    // Building a subdivision will load the holidays of the main country and the subdivision.
    holidays::Builder::new()
        .countries(&[Country::US_WV])
        .years(2024..2025)
        .init()?;

    let d = NaiveDate::from_ymd_opt(2024, 6, 20).expect("Invalid date");
    assert!(holidays::contains(Country::US, d)?.not());
    println!("{d} is not a holiday in the US.");

    assert!(holidays::contains(Country::US_WV, d)?);
    println!("{d} is a holiday in West Virginia.");
    println!("{:?}", holidays::get(Country::US_WV, d)?.unwrap());

    Ok(())
}
