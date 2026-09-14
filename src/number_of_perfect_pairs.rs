// id=3963 slug=number-of-perfect-pairs lang=rust

pub fn perfect_pairs(nums: Vec<i32>) -> i64 {
    let mut abs_vals: Vec<i32> = nums.iter().map(|n| n.abs()).collect();
    abs_vals.sort_unstable();

    let mut j = 0;
    let mut count: i64 = 0;
    for i in 0..abs_vals.len() {
        j = j.max(i + 1);
        while j < abs_vals.len() && abs_vals[j] - abs_vals[i] <= abs_vals[i] {
            j += 1;
        }
        count += (j - i - 1) as i64;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(perfect_pairs(vec![0, 1, 2, 3]), 2);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(perfect_pairs(vec![-3, 2, -1, 4]), 4);
    }

    #[test]
    fn base_test_3() {
        assert_eq!(perfect_pairs(vec![1, 10, 100, 1000]), 0);
    }

    #[test]
    fn all_fives() {
        assert_eq!(perfect_pairs(vec![5, 5, 5, 5, 5]), 10);
    }
}
