// id=48 slug=rotate-image lang=rust

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        for j in (i + 1)..matrix.len() {
            let [row_i, row_j] = matrix
                .get_disjoint_mut([i, j])
                .expect("index in range by loop construction");
            std::mem::swap(&mut row_i[j], &mut row_j[i]);
        }
    }

    for row in matrix {
        row.reverse();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn base_test_2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ]
        );
    }

    fn numbered(n: usize) -> Vec<Vec<i32>> {
        (0..n)
            .map(|i| (0..n).map(|j| (i * n + j) as i32).collect())
            .collect()
    }

    #[test]
    fn single_element() {
        let mut matrix = vec![vec![1]];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }

    #[test]
    fn two_by_two() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
    }

    #[test]
    fn negative_values() {
        let mut matrix = vec![vec![-1000, 0], vec![1000, -1]];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1000, -1000], vec![-1, 0]]);
    }

    #[test]
    fn matches_out_of_place_rotation() {
        for n in 1..=20 {
            let original = numbered(n);
            let mut expected = vec![vec![0; n]; n];
            for (i, row) in original.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    expected[j][n - 1 - i] = value;
                }
            }

            let mut matrix = original;
            rotate(&mut matrix);
            assert_eq!(matrix, expected, "n = {n}");
        }
    }

    #[test]
    fn four_rotations_is_identity() {
        for n in 1..=20 {
            let original = numbered(n);
            let mut matrix = original.clone();
            for _ in 0..4 {
                rotate(&mut matrix);
            }
            assert_eq!(matrix, original, "n = {n}");
        }
    }
}
