use holidays::Country;
use holidays::internal::Date; // use chrono or time types instead

fn main() {
    let d = Date::from_ymd(2022, 1, 1);
    println!(
        "Is {d:?} a holiday in Japan? Answer is {}",
        holidays::is_holiday(Country::JP, d)
    );

    println!("{:?}", holidays::get_holidays(Country::JP, d).next().unwrap());
}
