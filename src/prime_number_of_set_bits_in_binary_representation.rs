// id=762 slug=prime-number-of-set-bits-in-binary-representation lang=rust

// Bits for primes at positions 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_MASK: i32 = 0b10100010100010101100;

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    (left..=right)
        .map(|n| (PRIME_MASK >> n.count_ones()) & 1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(count_prime_set_bits(10, 15), 5);
    }

    #[test]
    fn one_is_not_prime() {
        assert_eq!(count_prime_set_bits(1, 3), 1);
    }

    #[test]
    fn includes_nineteen_ones_number() {
        assert_eq!(count_prime_set_bits(977581, 983119), 2036);
    }
}
