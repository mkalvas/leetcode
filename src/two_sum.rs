// id=1 slug=two-sum lang=rust

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut record: HashMap<i32, i32> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = record.get(&(target - n)) {
                return vec![*j, i as i32];
            }
            record.insert(*n, i as i32);
        }

        unreachable!("problem guarantees a solution");
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn base_test_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
