// id=2937 slug=make-three-strings-equal lang=rust

pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let common = s1
        .bytes()
        .zip(s2.bytes())
        .zip(s3.bytes())
        .take_while(|((a, b), c)| a == b && b == c)
        .count();

    if common == 0 {
        -1
    } else {
        (s1.len() + s2.len() + s3.len() - 3 * common) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            find_minimum_operations("abc".to_string(), "abb".to_string(), "ab".to_string()),
            2
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(
            find_minimum_operations("dac".to_string(), "bac".to_string(), "cac".to_string()),
            -1
        );
    }

    #[test]
    fn base_test_3() {
        assert_eq!(
            find_minimum_operations("cc".to_string(), "cccb".to_string(), "c".to_string()),
            4
        );
    }
}
