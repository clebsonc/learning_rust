fn matches_number_one(val: &[i8]) -> bool {
    matches!(val.first(), Some(&1))
}

fn vector_adds_15(val: &mut Vec<i8>) {
    val.push(15);
}

fn question1() {
    let mut val = vec![2, 3, 5, 7];
    println!("{}", matches_number_one(&val));
    vector_adds_15(&mut val);
    println!("{:?}", val);
}

fn add_number_two(value: &mut i8) {
    *value += 2;
}

fn question2() {
    let mut value: i8 = 5;
    add_number_two(&mut value);
    println!("{}", value);
}

fn main() {
    question1();
    question2();
}
