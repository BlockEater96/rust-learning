#![allow(unused)]
#[derive(Debug)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Weekday {
    fn as_str(&self) -> &str {
        match self {
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
            Weekday::Sunday => "Sunday",
        }
    }
}
fn main() {
    println!("=====Enums in Rust=====");
    let day = Weekday::Wednesday;
    println!("Today is {:?}", day);
    println!("Today is {}", day.as_str());
    // Using the Debug trait to print the  enum
    println!("Today is {:#?}", day);
    println!("=====End of Enums in Rust=====");
}
