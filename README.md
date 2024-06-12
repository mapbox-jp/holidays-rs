<h1 align="center">holidays-rs</h1>
<p align="center">
<img src="logo.png" width="30%"/><br />
Rust library to provide accurate and up-to-date holiday dates based on Python holidays package</p>
<p align="center">

  <a href="https://crates.io/crates/holidays">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/holidays.svg">
  </a>
  <a href="https://docs.rs/holidays/latest/holidays/">
    <img alt="Docs.rs" src="https://img.shields.io/badge/docs.rs-holidays-blue">
  </a>
  <a href="https://github.com/mapbox-jp/holidays-rs/actions/workflows/test.yml">
    <img alt="GithubActions" src="https://github.com/mapbox-jp/holidays-rs/actions/workflows/test.yml/badge.svg">
  </a>
</p>

## Design
[python-holidays](https://github.com/dr-prodigy/python-holidays) is a well maintained package to generate holidays dynamically. This crate is on the other hand, rather than porting `python-holidays` into Rust, statically generates the Rust code of holidays by the script using `python-holidays`. Since the holiday database of this crate boils down to a thread-safe `HashMap` (There is an option for single thread use case) in Rust, it's ultra fast and flexible to use in most of the cases.

To keep freshness of the holiday database, we're planning to have Github Actions job to automate
1. Generating code by the script using the latest `python-holidays`
2. Build and run tests
3. If there is any change, it will bump the version and publish to crates.io

## Usage

The simplest usage is to call `holidays::init` to initialize holiday database in thread-safe `HashMap`. After holiday database is initialized, you can call `holidays::contains` to check if the specified date is a holiday or not. Calls `holidays::get` to gen an object which contains country code, country name, date and name of the holiday.
```rust
use chrono::NaiveDate;
use holidays::Country;

fn main() -> anyhow::Result<()> {
    holidays::init()?;

    let d = NaiveDate::from_ymd_opt(2022, 1, 1).expect("Invalid date");
    println!("Is {d} a holiday in Japan? Answer is {}", holidays::contains(Country::JP, d)?);
    println!("{:?}", holidays::get(Country::JP, d)?.unwrap());

    Ok(())
}
```

**`Note that holidays::init will load the holidays of all the supported countries and years into memory, so it's quite heavy.`** If you need holidays of certain countries and years, please consider using `holidays::Builder` to limit them.

```rust
holidays::Builder::new()
        .countries(&[Country::JP])
        .years(2022..2023)
        .init()?;
```

If you know what countries are needed at compile-time, you can specify the country code in `Cargo.toml`. This can improve the build performance significantly.
```toml
holidays = { version = "*", default-features = false, features = ["JP"] }
```

## Available countries and years

holidays-rs supports countries listed in the below link from 2000 to 2023.

https://github.com/dr-prodigy/python-holidays#available-countries

## FAQ

* Q: How can I use this crate in a single threaded environment?
* A: Don't call `init` nor `holidays::Builder`s `init` methods. Instead, you can get the internal HashMap by calling `holidays::Builder`'s `build` method.

## Acknowledgement

Thank you so much [python-holidays contributors](https://github.com/vacanza/python-holidays/graphs/contributors) for maintaining such a great package! 🙏

## License

This project is licensed under the [MIT license](https://github.com/mapbox-japan/blob/main/LICENSE).
