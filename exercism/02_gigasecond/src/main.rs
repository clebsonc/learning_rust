use time::PrimitiveDateTime as DateTime;
use time_macros::{date, datetime, time};

fn after(start: DateTime) -> () {
    println!("{:#?}", start);
}

fn main() {
    let start = DateTime::new(date!(2012 - 11 - 03));
    after(DateTime::from("2023-12-03"))
}
