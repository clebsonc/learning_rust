fn color_to_number(color: &str) -> i8 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(val: i64) -> i64 {
    if val == 1 {
        return 1;
    }
    return val * factorial(val - 1);
}

fn non_recursive_factorial(val: i64) -> i64 {
    let mut res = 1;
    let mut i = 1;
    while i <= val {
        res *= i;
        i += 1;
    }
    res
}

fn main() {
    println!("Color number {}", color_to_number("black"));
    println!("Color number {}", color_to_number("blue"));
    println!("Color number {}", color_to_number("red"));
    println!("Color number {}", color_to_number("green"));

    println!("Factorial 5: {}", factorial(5));
    println!("Factorial 4: {}", factorial(4));

    println!("Factorial non recursive 5: {}", non_recursive_factorial(5));
    println!("Factorial non recursive 4: {}", non_recursive_factorial(4));
}
