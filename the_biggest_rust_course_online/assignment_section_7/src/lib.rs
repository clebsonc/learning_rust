pub mod simple_sum {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::simple_sum::add;

    #[test]
    fn add_two_plus_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
