// https://www.hackerrank.com/challenges/mini-max-sum/problem
use std::io;

fn read_values() -> Vec<i64> {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error while reading line");

    let mut values: Vec<i64> = buffer
        .trim()
        .split(' ')
        .map(|x| x.parse::<i64>().expect("Error parsing values to i64."))
        .collect();
    values.sort();
    values
}


fn mini_max_sum(arr: &[i64]) -> (i64, i64) {
    let mut mini: i64 = 0;
    let mut max: i64 = 0;
    for i in 0 .. 4 {
        mini = mini + arr[i];
    }
    for i in arr.len()-4 .. arr.len() {
        max = max + arr[i];
    }
    (mini, max)
}

fn main() {
    let values: Vec<i64> = read_values();
    let result: (i64, i64) = mini_max_sum(&values);
    println!("{} {}", result.0, result.1);
}

