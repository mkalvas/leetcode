// id=1935 slug=maximum-number-of-words-you-can-type lang=rust

use std::collections::HashSet;

pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let broken_letters: HashSet<char> = HashSet::from_iter(broken_letters.chars());
    text.split_whitespace()
        .filter(|w| !w.chars().any(|c| broken_letters.contains(&c)))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(
            can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1
        );
    }

    #[test]
    fn base_test_3() {
        assert_eq!(
            can_be_typed_words("leet code".to_string(), "e".to_string()),
            0
        );
    }

    #[test]
    fn empty_broken_letters() {
        assert_eq!(
            can_be_typed_words("hello world".to_string(), String::new()),
            2
        );
    }

    #[test]
    fn single_word() {
        assert_eq!(can_be_typed_words("hello".to_string(), "x".to_string()), 1);
    }
}
