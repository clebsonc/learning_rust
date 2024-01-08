fn question1() {
    let val1: u8 = 5;
    let val2: u8 = 2;
    let ans: u8 = val1 % val2;

    println!("{}", ans);
}

fn question2() {
    let mut v = vec![2, 4, 6, 8, 10];
    println!("{:?}", v);
    let value_to_remove = 10;
    let index = v.iter().position(|&x| x == value_to_remove).unwrap();
    v.swap_remove(index);
    println!("{:?}", v);
    v.push(12);
    println!("{:?}", v);
}

fn concat_string(name: &str) -> String {
    let suffix = String::from(" World");
    let mut prefix = String::from(name);

    prefix.push_str(&suffix);
    prefix
}

fn question3() {
    println!("{:?}", concat_string("Hello"));
}

fn control_flow(value: i32) {
    if value == 1 {
        println!("The value is one");
    } else if value > 50 {
        println!("The value is greater than 50");
    } else if value < 25 {
        println!("The value is less than 25");
    } else if value > 25 {
        println!("The value is greater than 25 but less than 50")
    }
}

fn question4() {
    control_flow(25);
    control_flow(1);
    control_flow(100);
    control_flow(23);
    control_flow(40);
}

fn main() {
    question1();
    question2();
    question3();
    question4();
}
