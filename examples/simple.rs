use chrono::NaiveDate;
use holidays::Country;

fn main() -> anyhow::Result<()> {
    // NOTE: This will load the holidays of all the supported countries and years into memory.
    // Consider using `holidays::Builder` to limit by countries and years.
    holidays::init()?;

    let d = NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date");
    println!(
        "Is {d} a holiday in Japan? Answer is {}",
        holidays::contains(Country::JP, d)?
    );

    println!("{:?}", holidays::get(Country::JP, d)?.unwrap());

    Ok(())
}
