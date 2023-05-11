// https://www.hackerrank.com/challenges/diagonal-difference/problem
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut main_diagonal: i32 = 0;
    let mut inverse_diagonal: i32 = 0;
    let arr_length: usize = arr.len() - 1;
    for i in 0 .. arr.len() {
        main_diagonal = main_diagonal + arr[i][i];
        let j: usize = arr_length - i;
        inverse_diagonal = inverse_diagonal + arr[i][j];
    }
    (main_diagonal - inverse_diagonal).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

