// id=2239 slug=find-closest-number-to-zero lang=rust

use std::cmp::Reverse;

pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .max_by_key(|&number| (Reverse(number.unsigned_abs()), number))
        .expect("nums is guaranteed to be non-empty")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(find_closest_number(vec![-2, -1, 1]), 1);
    }

    #[test]
    fn larger_first() {
        assert_eq!(find_closest_number(vec![-2, 1, -1]), 1);
    }

    #[test]
    fn negative_best() {
        assert_eq!(find_closest_number(vec![-100000, -100000]), -100000);
    }
}
