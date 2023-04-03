use std::io;

fn read_input() -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failure to reading line");
    buffer.clear();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failure to reading line");

    let values: Vec<i32> = buffer
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().expect("Failure parsing Values"))
        .collect();

    values
}

fn count_tallest(arr: &[i32]) -> i32 {
    let mut tallest = arr[0];
    let mut count = 1;

    for i in 1..arr.len() {
        if arr[i] == tallest {
            count += 1;
        } else if arr[i] > tallest {
            tallest = arr[i];
            count = 1;
        }
    }

    count
}

fn main() {
    let input: Vec<i32> = read_input();
    let qt_tallest: i32 = count_tallest(&input);
    println!("{}", qt_tallest);
}
