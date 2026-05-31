use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;

fn apply_to_jobs(number: i32, title: &str) {
    println!("Applied to {} {} jobs", number, title);
}

fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}

fn main() {
    let title = String::from("Firefighter");
    apply_to_jobs(10, &title);
    apply_to_jobs(5, "Taxi driver");
    println!("Is even {}", is_even(10));
    println!("Is even {}", is_even(11));
    println!("Alphabet: {:?}", alphabets("banana"));
    println!("Alphabet: {:?}", alphabets("zoology"));
    println!("Alphabet: {:?}", alphabets("zebra"));
}
