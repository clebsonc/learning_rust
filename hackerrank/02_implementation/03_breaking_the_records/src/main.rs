use std::io;

fn breaking_records(input: &[i32]) -> Vec<i32> {
    let mut result = vec![0, 0];
    let mut min = input[0];
    let mut max = input[0];

    for value in input.iter().skip(1) {
        if value > &max {
            println!("Broke max record: {} {}", max, value);
            result[0] += 1;
            max = *value;
        } else if value < &min {
            println!("Broke min record: {} {}", min, value);
            result[1] += 1;
            min = *value;
        }
    }

    result
}

fn main() {
    let mut buffer = String::new();
    let s = io::stdin().read_line(&mut buffer);
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_check() {
        println!("works");
    }

    #[test]
    fn test_input_0() {
        let input = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        let output = vec![2, 4];
        let result = breaking_records(&input);
        assert_eq!(result, output);
    }

    #[test]
    fn test_input_1() {
        let input = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        let output = vec![4, 0];
        let result = breaking_records(&input);
        assert_eq!(result, output);
    }
}
