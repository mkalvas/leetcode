// id=2881 slug=split-strings-by-separator lang=rust

pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words
        .iter()
        .flat_map(|s| s.split(separator))
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            split_words_by_separator(
                vec![
                    "one.two.three".to_string(),
                    "four.five".to_string(),
                    "six".to_string()
                ],
                '.'
            ),
            vec!["one", "two", "three", "four", "five", "six"]
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(
            split_words_by_separator(vec!["$easy$".to_string(), "$problem$".to_string()], '$'),
            vec!["easy", "problem"]
        );
    }

    #[test]
    fn base_test_3() {
        assert_eq!(
            split_words_by_separator(vec!["|||".to_string()], '|'),
            Vec::<String>::new()
        );
    }
}
