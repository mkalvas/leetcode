// id=1376 slug=time-needed-to-inform-all-employees lang=rust

pub fn num_of_minutes(
    _n: i32,
    _head_id: i32,
    mut manager: Vec<i32>,
    mut inform_time: Vec<i32>,
) -> i32 {
    let mut max = 0;
    let mut path = Vec::new();

    for i in 0..manager.len() {
        let mut j = i;
        while manager[j] != -1 {
            path.push(j);
            j = manager[j] as usize;
        }

        while let Some(m) = path.pop() {
            inform_time[m] += inform_time[manager[m] as usize];
            manager[m] = -1;
        }

        max = max.max(inform_time[i]);
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(num_of_minutes(1, 0, vec![-1], vec![0]), 0);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(
            num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
            1
        );
    }

    #[test]
    fn simple_split() {
        assert_eq!(num_of_minutes(3, 2, vec![2, 2, -1], vec![0, 0, 1]), 1)
    }

    #[test]
    fn unequal_time_split() {
        assert_eq!(
            num_of_minutes(5, 4, vec![2, 3, 4, 4, -1], vec![0, 0, 3, 1, 1]),
            4
        )
    }

    #[test]
    fn unequal_depth_split() {
        assert_eq!(num_of_minutes(4, 3, vec![1, 3, 3, -1], vec![0, 1, 0, 1]), 2)
    }

    #[test]
    fn deep_but_not_same() {
        assert_eq!(
            // 0 - 1 - 2 - 3 - 4 - 5 - 6
            // 7 - 8 --┘
            num_of_minutes(
                9,
                6,
                vec![1, 2, 3, 4, 5, 6, -1, 8, 2],
                vec![0, 1, 1, 1, 1, 1, 1, 0, 2]
            ),
            7
        )
    }
}
