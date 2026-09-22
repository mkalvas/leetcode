// id=2000 slug=reverse-prefix-of-word lang=rust

pub fn reverse_prefix(word: String, ch: char) -> String {
    let Some(i) = word.find(ch) else { return word };
    let mut buf = word.into_bytes();
    buf[..i + ch.len_utf8()].reverse();
    String::from_utf8(buf).expect("problem guarantees `word` is ascii")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(reverse_prefix("abcdefd".to_string(), 'd'), "dcbaefd");
    }

    #[test]
    fn base_test_2() {
        assert_eq!(reverse_prefix("xyxzxe".to_string(), 'z'), "zxyxxe");
    }

    #[test]
    fn base_test_3() {
        assert_eq!(reverse_prefix("abcd".to_string(), 'z'), "abcd");
    }

    #[test]
    fn single_letter() {
        assert_eq!(reverse_prefix("a".to_string(), 'a'), "a");
    }
}
