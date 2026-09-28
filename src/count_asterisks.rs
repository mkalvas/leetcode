// id=2315 slug=count-asterisks lang=rust

pub fn count_asterisks(s: String) -> i32 {
    let mut in_pair = false;
    let mut count = 0;
    for c in s.chars() {
        match c {
            '|' => in_pair = !in_pair,
            '*' if !in_pair => count += 1,
            _ => {}
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(count_asterisks("l|*e*et|c**o|*de|".to_string()), 2);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(count_asterisks("iamprogrammer".to_string()), 0);
    }

    #[test]
    fn base_test_3() {
        assert_eq!(count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_string()), 5);
    }

    #[test]
    fn empty_pair() {
        assert_eq!(count_asterisks("||**".to_string()), 2);
    }

    #[test]
    fn all_excluded() {
        assert_eq!(count_asterisks("|**|".to_string()), 0);
    }

    #[test]
    fn single_asterisk() {
        assert_eq!(count_asterisks("*".to_string()), 1);
    }

    #[test]
    fn asterisks_at_both_ends() {
        assert_eq!(count_asterisks("*|*|*".to_string()), 2);
    }
}
