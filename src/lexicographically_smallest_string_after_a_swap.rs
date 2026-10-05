// id=3216 slug=lexicographically-smallest-string-after-a-swap lang=rust

pub fn get_smallest_string(s: String) -> String {
    let mut bytes = s.into_bytes();
    if let Some(i) = bytes
        .windows(2)
        .position(|w| w[1] < w[0] && w[0] % 2 == w[1] % 2)
    {
        bytes.swap(i, i + 1);
    }

    String::from_utf8(bytes).expect("problem guarantees ascii")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            get_smallest_string("45320".to_string()),
            "43520".to_string()
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(get_smallest_string("001".to_string()), "001".to_string());
    }

    #[test]
    fn two_odd_descending() {
        assert_eq!(get_smallest_string("31".to_string()), "13".to_string());
    }

    #[test]
    fn two_even_descending() {
        assert_eq!(get_smallest_string("20".to_string()), "02".to_string());
    }

    #[test]
    fn two_mixed_parity_descending() {
        assert_eq!(get_smallest_string("21".to_string()), "21".to_string());
        assert_eq!(get_smallest_string("10".to_string()), "10".to_string());
    }

    #[test]
    fn two_ascending() {
        assert_eq!(get_smallest_string("13".to_string()), "13".to_string());
        assert_eq!(get_smallest_string("24".to_string()), "24".to_string());
    }

    #[test]
    fn two_equal() {
        assert_eq!(get_smallest_string("00".to_string()), "00".to_string());
        assert_eq!(get_smallest_string("77".to_string()), "77".to_string());
    }

    #[test]
    fn only_one_swap_allowed() {
        assert_eq!(get_smallest_string("531".to_string()), "351".to_string());
        assert_eq!(
            get_smallest_string("97531".to_string()),
            "79531".to_string()
        );
        assert_eq!(
            get_smallest_string("86420".to_string()),
            "68420".to_string()
        );
    }

    #[test]
    fn earliest_swap_wins() {
        assert_eq!(get_smallest_string("2031".to_string()), "0231".to_string());
    }

    #[test]
    fn swap_at_end() {
        assert_eq!(get_smallest_string("1153".to_string()), "1135".to_string());
    }

    #[test]
    fn swap_after_mixed_parity_pairs() {
        assert_eq!(get_smallest_string("5286".to_string()), "5268".to_string());
    }

    #[test]
    fn equal_digits_skipped() {
        assert_eq!(get_smallest_string("3311".to_string()), "3131".to_string());
    }

    #[test]
    fn alternating_parity_unchanged() {
        assert_eq!(
            get_smallest_string("9876543210".to_string()),
            "9876543210".to_string()
        );
        assert_eq!(
            get_smallest_string("1234567890".to_string()),
            "1234567890".to_string()
        );
    }

    #[test]
    fn already_sorted_same_parity() {
        assert_eq!(
            get_smallest_string("13579".to_string()),
            "13579".to_string()
        );
        assert_eq!(
            get_smallest_string("02468".to_string()),
            "02468".to_string()
        );
    }

    #[test]
    fn max_length_swap_at_end() {
        let input = format!("{}42", "0".repeat(98));
        let expected = format!("{}24", "0".repeat(98));
        assert_eq!(get_smallest_string(input), expected);
    }

    #[test]
    fn max_length_no_swap() {
        let input = format!("{}0", "1".repeat(99));
        assert_eq!(get_smallest_string(input.clone()), input);
    }
}
