// id=2335 slug=minimum-amount-of-time-to-fill-cups lang=rust

pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    amount.sort_unstable();
    amount[2].max((amount[0] + amount[1] + amount[2] + 1) / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(fill_cups(vec![1, 4, 2]), 4);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(fill_cups(vec![5, 4, 4]), 7);
    }

    #[test]
    fn base_test_3() {
        assert_eq!(fill_cups(vec![5, 0, 0]), 5);
    }

    #[test]
    fn sequential_data_cases() {
        assert_eq!(fill_cups(vec![1, 2, 1]), 2);

        assert_eq!(fill_cups(vec![2, 0, 0]), 2);
        assert_eq!(fill_cups(vec![2, 1, 0]), 2);
        assert_eq!(fill_cups(vec![2, 2, 0]), 2);
        assert_eq!(fill_cups(vec![2, 2, 1]), 3);
        assert_eq!(fill_cups(vec![2, 2, 2]), 3);
        assert_eq!(fill_cups(vec![2, 2, 3]), 4);

        assert_eq!(fill_cups(vec![3, 1, 1]), 3);
        assert_eq!(fill_cups(vec![3, 2, 1]), 3);

        assert_eq!(fill_cups(vec![3, 3, 2]), 4);
        assert_eq!(fill_cups(vec![3, 3, 3]), 5);
        assert_eq!(fill_cups(vec![3, 3, 4]), 5);
        assert_eq!(fill_cups(vec![3, 4, 4]), 6);

        assert_eq!(fill_cups(vec![4, 4, 2]), 5);
        assert_eq!(fill_cups(vec![4, 4, 3]), 6);
        assert_eq!(fill_cups(vec![4, 4, 4]), 6);
        assert_eq!(fill_cups(vec![4, 4, 5]), 7);

        assert_eq!(fill_cups(vec![6, 6, 6]), 9);
    }
}
