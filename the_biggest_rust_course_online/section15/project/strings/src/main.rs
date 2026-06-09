use std::io;

/// concatenate $$$ to the end of the String.
fn make_money(word: &mut String) {
    word.push_str("$$$");
}

/// remove whitespaces and make all characters uppercase
fn trim_and_capitalize(word: &str) -> String {
    let new_word = String::from(word);
    new_word.trim().to_uppercase()
}

fn find_elements<'a>(word: &'a str, pattern: &str) -> Vec<&'a str> {
    let result = word.split(pattern).collect();
    result
}

fn get_identity() {
    let first_name = loop {
        let mut name = String::default();
        let res = io::stdin().read_line(&mut name);
        match res {
            Ok(val) => {
                println!("Read {} bytes of data", val);
                break name;
            }
            Err(e) => eprintln!("Error reading first name: {}", e),
        }
    };
    println!("Your first name is: {first_name}");
}

fn main() {
    let mut word: String = String::from("340");
    make_money(&mut word);
    println!("word: {}", word);

    let lowercase = "\n  hello world";
    println!("{}", trim_and_capitalize(lowercase));

    let elements = "gold!silver!platinum";
    println!("{:#?}", find_elements(elements, "!"));

    get_identity();
}
