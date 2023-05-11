// https://www.hackerrank.com/challenges/grading/problem?isFullScreen=true


pub fn grading_students(grades: &Vec<i32>) -> Vec<i32> {
    let mut rounded: Vec<i32> = Vec::new();

    for val in grades {
        let to_round =  5 - (*val % 5);
        if *val < 38 || to_round > 2 {
            rounded.push(*val);
        } else {
            rounded.push(*val + to_round);
        }
    }
    rounded
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = vec![73, 67, 38, 33];
        let result = grading_students(&input);
        let expected = vec![75, 67, 40, 33];
        assert_eq!(result, expected);
    }
}
