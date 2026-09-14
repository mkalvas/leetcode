// id=3285 slug=find-indices-of-stable-mountains lang=rust

pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    (1..height.len())
        .filter(|&i| height[i - 1] > threshold)
        .map(|i| i as i32) // guaranteed to be safe by problem construction
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(stable_mountains(vec![1, 2, 3, 4, 5], 2), vec![3, 4]);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(stable_mountains(vec![10, 1, 10, 1, 10], 3), vec![1, 3]);
    }

    #[test]
    fn base_test_3() {
        assert!(stable_mountains(vec![10, 1, 10, 1, 10], 10).is_empty(),);
    }

    #[test]
    fn min_len_array() {
        assert!(stable_mountains(vec![1, 2], 3).is_empty());
    }

    #[test]
    fn all_stable() {
        assert_eq!(stable_mountains(vec![5, 5, 5], 1), vec![1, 2]);
    }
}
