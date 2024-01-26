#[derive(Debug)]
pub struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn new(start: u32, end: u32) -> Self {
        Range {
            start, end
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range {
            start: 0,
            end: 10,
        }
    }
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.start;
        if self.start < self.end {
            self.start += 1;
        }
        else {
            return None;
        }
        Some(current)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_value() {

        let val = Range::default();
        assert_eq!(val.start, 0);
        assert_eq!(val.end, 10);
    }

    #[test]
    fn test_iterator() {
        let mut val = Range {start: 0, end:3};
        assert_eq!(val.next(), Some(0));
        assert_eq!(val.next(), Some(1));
        assert_eq!(val.next(), Some(2));
        assert_eq!(val.next(), None);
    }
}
