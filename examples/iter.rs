use chrono::NaiveDate;
use holidays::Country;

fn main() -> anyhow::Result<()> {
    holidays::Builder::new()
        .countries(&[Country::JP])
        .years(2022..2023)
        .init()?;

    let s = NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date");
    let u = NaiveDate::from_ymd_opt(2023, 1, 1).expect("Invalid date");

    for holiday in holidays::iter(Country::JP, s, u)?.map(|h| h.date) {
        println!("{holiday:?}",);
    }

    Ok(())
}
