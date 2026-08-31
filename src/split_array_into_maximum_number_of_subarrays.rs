// id=2871 slug=split-array-into-maximum-number-of-subarrays lang=rust

impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        const ALL_ONES: i32 = -1;
        let mut score = ALL_ONES;
        let mut count = 0;

        for n in nums {
            score &= n;
            if score == 0 {
                count += 1;
                score = ALL_ONES;
            }
        }

        count.max(1)
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(Solution::max_subarrays(vec![1, 0, 2, 0, 1, 2]), 3);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(Solution::max_subarrays(vec![5, 7, 1, 3]), 1);
    }

    #[test]
    fn array_of_1_1() {
        assert_eq!(Solution::max_subarrays(vec![1]), 1);
    }

    #[test]
    fn array_of_1_0() {
        assert_eq!(Solution::max_subarrays(vec![0]), 1);
    }

    #[test]
    fn array_of_2_0() {
        assert_eq!(Solution::max_subarrays(vec![0, 0]), 2);
    }

    #[test]
    fn array_of_zero_still_max_one() {
        assert_eq!(Solution::max_subarrays(vec![0, 1, 3, 5, 7]), 1);
    }

    #[test]
    fn array_of_zero_still_max_one_palindrome() {
        assert_eq!(Solution::max_subarrays(vec![7, 5, 3, 1, 0, 1, 3, 5, 7]), 1);
    }

    #[test]
    fn array_of_zero_palindrome_split() {
        assert_eq!(
            Solution::max_subarrays(vec![0, 7, 5, 3, 1, 0, 1, 3, 5, 7, 0]),
            3
        );
    }

    #[test]
    fn array_of_1_0_3() {
        assert_eq!(Solution::max_subarrays(vec![1, 0, 3]), 1);
    }

    #[test]
    fn array_absorbs_tail_when_needed() {
        assert_eq!(Solution::max_subarrays(vec![0, 0, 5]), 2);
    }
}
