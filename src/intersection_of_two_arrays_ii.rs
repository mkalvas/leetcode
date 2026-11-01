// id=350 slug=intersection-of-two-arrays-ii lang=rust
use std::collections::HashMap;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut counts = count(nums1);
    let mut out = Vec::new();

    for n in nums2 {
        if let Some(remaining) = counts.get_mut(&n) {
            if *remaining > 0 {
                out.push(n);
                *remaining -= 1;
            }
        }
    }

    out
}

fn count(ns: Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for n in ns {
        *counts.entry(n).or_default() += 1;
    }
    counts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        let mut result = intersect(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort_unstable();
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn base_test_2() {
        let mut result = intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort_unstable();
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn empty_vecs() {
        assert_eq!(intersect(vec![], vec![1, 2, 3]), vec![]);
        assert_eq!(intersect(vec![1, 2, 3], vec![]), vec![]);
    }

    #[test]
    fn reverse_base_test_2() {
        let mut result = intersect(vec![9, 4, 9, 8, 4], vec![4, 9, 5]);
        result.sort_unstable();
        assert_eq!(result, vec![4, 9]);
    }
}
