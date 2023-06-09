pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(2);
    for i in 0 .. (nums.len()-1) {
        for j in i + 1 .. nums.len() {
            let val = nums.get(i).unwrap() + nums.get(j).unwrap();
            if val == target {
                result.push(i as i32);
                result.push(j as i32);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_nums: Vec<i32> = vec![2, 7, 11, 15];
        let input_target: i32 = 9;
        let expected: Vec<i32> = vec![0, 1];
        let result: Vec<i32> = two_sum(input_nums, input_target);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let input_nums: Vec<i32> = vec![3, 2, 4];
        let input_target: i32 = 6;
        let expected: Vec<i32> = vec![1, 2];
        let result: Vec<i32> = two_sum(input_nums, input_target);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_3() {
        let input_nums: Vec<i32> = vec![3, 3];
        let input_target: i32 = 6;
        let expected: Vec<i32> = vec![0, 1];
        let result: Vec<i32> = two_sum(input_nums, input_target);
        assert_eq!(result, expected);
    }
}
