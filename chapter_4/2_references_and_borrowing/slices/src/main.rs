// slices allow to access a contiguos sequence of elements in a collection rather than the whole
// collection. A slice is just a reference, so it does not have ownership.

use std::str::from_utf8;

fn main() {
    let sentence = String::from("Spring is the most beautifull season of the year!");
    println!("\nUnderstanding characters and bytes:\n");
    understanding_strings(&sentence);

    println!("\n-------------------------------------------\n");
    println!("The first word finishes at index {}", first_word(&sentence));

    println!("\n-------------------------------------------\n");
    println!(
        "The first word finishes at index {}",
        fist_word_with_enumerator(&sentence)
    );

    println!("\n-------------------------------------------\n");
    understanding_string_slices();

    println!("\n-------------------------------------------\n");
    println!("Retrieving word as slice: {:?}", first_word_with_string_slices(&sentence));

    println!("\n-------------------------------------------\n");
    println!("Retrieving word passing a String: {:?}", first_word_that_accepts_strings_and_string_literals(&sentence));
    let sentence2: &str = "Summer is to hot!";
    println!("Retrieving word passing a string literal: {:?}", first_word_that_accepts_strings_and_string_literals(&sentence2));

    other_slices();
}

fn understanding_strings(text: &String) {
    println!("Sentence: `{}`", text);
    let bytes: &[u8] = text.as_bytes();  // the return value of function `as_bytes` is a reference
                                         // to a vector of unsigned integers of 8 bytes
    println!("Sentence as bytes: {:?}", bytes);

    // Getting the first character
    let character = from_utf8(&bytes[0..1]).unwrap();
    println!("Byte 0 as char: {:?}", character);
    println!("Letter `S` as bytes: {}", b'S');
    println!("Letter `S` from the array at index 0: {}", bytes[0]);
}

fn first_word(text: &String) -> usize {
    // find the end of the first word:
    let mut counter: usize = 0;
    for &i in text.as_bytes() {
        if i == b' ' {
            return counter;
        }
        counter += 1;
    }
    counter
}

fn fist_word_with_enumerator(text: &String) -> usize {
    for (index, &value) in text.as_bytes().iter().enumerate() {
        if value == b' ' {
            return index;
        }
    }
    text.len()
}


fn understanding_string_slices() {
    // string slices are just literal strings, this is why its type &str
    let sentence = String::from("Hello beautifull world!");
    let first_slice: &str = &sentence[0..4];
    let second_slice: &str = &sentence[4..7];
    println!("First Slice: {}", first_slice);
    println!("First Slice: {}", second_slice);

    let ommiting_starting: &str = &sentence[..5];
    let ommiting_end: &str = &sentence[5..];
    let ommiting_both_sides: &str = &sentence[..];
    println!("Ommiting start: {}", ommiting_starting);
    println!("Ommiting end: {}", ommiting_end);
    println!("Ommiting both sides: {}", ommiting_both_sides);
}


fn first_word_with_string_slices(sentence: &String) -> &str { // the return type is a string slice, therefore it is immutable.
    let bytes: &[u8] = sentence.as_bytes();
    for (i, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &sentence[..i];
        }
    }
    &sentence[..]
}


fn first_word_that_accepts_strings_and_string_literals(sentence: &str) -> &str {
    let bytes: &[u8] = sentence.as_bytes();
    for (index, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &sentence[..index];
        }
    }
    &sentence[..]
}


fn other_slices() {
    // others collections can also be sliced just as strings:
    let values: [u8; 5] = [10, 20, 30, 40, 50];
    let slice1: &[u8] = &values[..3];
    let slice2: &[u8] = &values[3..];
    println!("Slice 1: {:?}", slice1);
    println!("Slice 2: {:?}", slice2);
}
