use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().unwrap();
    example_one(input);
    example_two(input);
}

fn example_one(input: u32) {
    // In rust, the branches of an if expression is usally called arms.
    // All `if` expressions should be evaluate to boolean type, otherwise the code does not
    // compile.
    if input % 4 == 0 {
        println!("number divisible by 4.");
    }
    else if input % 3 == 0 {
        println!("number divisible by 3.");
    }
    else {
        println!("number not divisible by 4 or 3.")
    }
}

fn example_two(input: u32) {
    // ternary operator with if in a let statement.
    let odd_or_even: String = if input % 2 == 0 { "even".to_string() } else { "odd".to_string() };
    println!("The given number is: {}", odd_or_even);
}
