use holidays::Country;
use holidays::internal::Date; // use chrono or time types instead

fn main() {
    let date = Date::from_ymd(2022, 1, 1);
    
    let holidays = holidays::get_holidays(Country::JP, date).map(|h| h.date::<Date>().unwrap());
    for holiday in holidays {
        println!("{holiday:?}",);
    }
}
