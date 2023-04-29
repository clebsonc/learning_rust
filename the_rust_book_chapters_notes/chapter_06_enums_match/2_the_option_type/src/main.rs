// Rust does not have the representation of abasence such as the NULL
// representation in other languages. Instead of NULL, rust has the enum
// `Option<t>`. It is autommatically imported in the prelude, so we can use
// without bringing it into scope. This enum has the variants Some and None, and
// there is no need to import them.


fn testing_the_enum_option() {
    let some_number = Some(5);
    let some_string = Some("this is a string slice");

    let absent_number: Option<i32> = Some(32);

    println!("\nNumber: {:#?} \n string: {:#?} \n absent: {:#?}", some_number, some_string, absent_number);
    if absent_number.is_some() {
        let numerator: i32 = absent_number.unwrap_or_default();
        println!("absent_number divided  by 2 equals {:#?}", numerator/2);
    }
}



fn main() {
    testing_the_enum_option();
}

