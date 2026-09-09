// id=389 slug=find-the-difference lang=rust

use std::collections::HashMap;

pub fn find_the_difference(s: String, t: String) -> char {
    let mut counts: HashMap<char, u16> = HashMap::with_capacity(26);
    for c in s.chars() {
        *counts.entry(c).or_default() += 1;
    }

    for c in t.chars() {
        match counts.get_mut(&c) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return c,
        }
    }

    unreachable!("problem guarantees one char difference");
}

/// Every letter in `s + t` has an even count except the extra one
pub fn find_the_difference_xor(s: String, t: String) -> char {
    char::from(s.bytes().chain(t.bytes()).fold(0, |acc, c| acc ^ c))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(find_the_difference(String::new(), "y".to_string()), 'y');
    }

    #[test]
    fn my_test_1() {
        assert_eq!(
            find_the_difference(
                "abcdklqweruickjuie".to_string(),
                "abcdklqweruickjuies".to_string()
            ),
            's'
        );
    }

    #[test]
    fn duplicate_char_case() {
        assert_eq!(find_the_difference("a".to_string(), "aa".to_string()), 'a');
    }

    #[test]
    fn shuffled_with_duplicate_extra() {
        assert_eq!(
            find_the_difference("aabbcc".to_string(), "bcabcab".to_string()),
            'b'
        );
    }

    #[test]
    fn xor_version_base_test_1() {
        assert_eq!(
            find_the_difference_xor("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }

    #[test]
    fn xor_version_base_test_2() {
        assert_eq!(find_the_difference_xor(String::new(), "y".to_string()), 'y');
    }

    #[test]
    fn xor_version_my_test_1() {
        assert_eq!(
            find_the_difference_xor(
                "abcdklqweruickjuie".to_string(),
                "abcdklqweruickjuies".to_string()
            ),
            's'
        );
    }

    #[test]
    fn xor_version_duplicate_char_case() {
        assert_eq!(
            find_the_difference_xor("a".to_string(), "aa".to_string()),
            'a'
        );
    }

    #[test]
    fn xor_version_shuffled_with_duplicate_extra() {
        assert_eq!(
            find_the_difference_xor("aabbcc".to_string(), "bcabcab".to_string()),
            'b'
        );
    }
}
