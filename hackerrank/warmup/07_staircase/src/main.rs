// https://www.hackerrank.com/challenges/staircase/problem
use std::io;


fn staircase(stairs: i32) {
    for i in 1 .. stairs+1 {
        let empty = stairs - i;
        let mut line = String::new();

        for _ in 1 .. empty + 1 {
            line.push(' ');
        }

        let blocks = stairs - empty;
        for _ in 1 .. blocks + 1 {
            line.push('#');
        }
        println!("{}", line);
    }
}



fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number");

    let number: i32 = number.trim().parse::<i32>().expect("Failed parse number");

    staircase(number);
}
