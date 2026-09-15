// id=507 slug=perfect-number lang=rust

pub fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    let mut sum = 1;
    let mut d = 2;
    while d * d <= num {
        if num % d == 0 {
            sum += d;
            let pair = num / d;
            if pair != d {
                sum += pair;
            }
            if sum > num {
                return false;
            }
        }
        d += 1;
    }

    sum == num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(check_perfect_number(28), true);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(check_perfect_number(7), false);
    }

    #[test]
    fn large_number() {
        assert_eq!(check_perfect_number(99999999), false);
    }

    #[test]
    fn one() {
        assert_eq!(check_perfect_number(1), false);
    }
}
