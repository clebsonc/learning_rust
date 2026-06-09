fn reverse_string(input: &str) -> String {
    let res: String = input.chars().rev().collect();
    res
}

fn main() {
    println!("stressed: {}", reverse_string("stressed"));
}
