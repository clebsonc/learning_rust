// https://www.hackerrank.com/challenges/plus-minus/problem

use std::io;


fn plus_minus(arr: &Vec<i32>) -> (f32, f32, f32){
    let mut negative: i32 = 0;
    let mut positive: i32 = 0;
    let mut zeros: i32 = 0;
    for &value in arr {
        if value < 0 {
            negative += 1;
        }
        else if value > 0 {
            positive += 1;
        }
        else {
            zeros += 1;
        }
    }
    (positive as f32 / arr.len() as f32, negative as f32 / arr.len() as f32, zeros as f32 / arr.len() as f32)
}

fn round_f32(x: f32, decimal_places: u32) -> f32 {
    let multiplier = 10.0_f32.powi(decimal_places as i32);
    (x * multiplier).round() / multiplier
}

fn main() {
    let mut total_numbers: String = String::new();
    io::stdin().read_line(&mut total_numbers).expect("failed to read total_numbers");
    let mut numbers: String = String:: new();
    io::stdin().read_line(&mut numbers).expect("failed to read numbers");
    let word_vec: Vec<i32> = numbers.trim().split(" ").map(|x| x.to_string().parse::<i32>().expect("Error parsing values to i32.")).collect();
    let (positive, negative, zeros): (f32, f32, f32) = plus_minus(&word_vec);
    println!("{}", round_f32(positive, 6));
    println!("{}", round_f32(negative, 6));
    println!("{}", round_f32(zeros, 6));


}

