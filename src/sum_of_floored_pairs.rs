// id=1862 slug=sum-of-floored-pairs lang=rust

const MODULO: i64 = 1_000_000_007;

pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
    let max = nums.iter().copied().max().unwrap_or(0) as usize;

    let mut prefixes = vec![0_i64; max + 1];
    // get frequencies first
    for n in &nums {
        prefixes[*n as usize] += 1;
    }

    // convert frequencies into prefix sum
    for value in 1..=max {
        prefixes[value] += prefixes[value - 1];
    }

    let mut sum: i64 = 0;
    for divisor in 1..=max {
        // recover frequency from prefix sum
        let copies = prefixes[divisor] - prefixes[divisor - 1];
        if copies == 0 {
            continue;
        }

        let mut block: i64 = 1;
        let mut sum_for_n: i64 = 0;
        let mut left = divisor;
        while left <= max {
            let right = (left + divisor - 1).min(max);
            sum_for_n += block * (prefixes[right] - prefixes[left - 1]);
            block += 1;
            left += divisor;
        }

        sum += sum_for_n * copies;
    }

    (sum % MODULO) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(sum_of_floored_pairs(vec![2, 5, 9]), 10);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(sum_of_floored_pairs(vec![7, 7, 7, 7, 7, 7, 7]), 49);
    }

    #[test]
    fn simple_extra_case() {
        assert_eq!(sum_of_floored_pairs(vec![1, 2, 3, 10, 11, 12, 13, 20]), 143);
        // 1 + 2 + 3 + 10 + 11 + 12 + 13 + 20 = 72
        // 0 + 1 + 1 +  5 +  5 +  6 +  6 + 10 = 34
        // 0 + 0 + 1 +  3 +  3 +  4 +  4 + 6  = 21
        // 0 + 0 + 0 +  1 +  1 +  1 +  1 + 2  = 6
        // 0 + 0 + 0 +  0 +  1 +  1 +  1 + 1  = 4
        // 0 + 0 + 0 +  0 +  0 +  1 +  1 + 1  = 3
        // 0 + 0 + 0 +  0 +  0 +  0 +  1 + 1  = 2
        // 0 + 0 + 0 +  0 +  0 +  0 +  0 + 1  = 1
        //                                    = 143
    }

    #[test]
    fn large_array() {
        assert_eq!(
            sum_of_floored_pairs(vec![1; 100_000]),
            (10_000_000_000_i64 % MODULO) as i32
        );
    }

    #[test]
    fn timeout_adversarial_large_array() {
        let mut arr = vec![1; 99_999];
        arr.push(100_000);
        assert_eq!(sum_of_floored_pairs(arr), 999699869);
    }
}
