// id=3951 slug=minimum-energy-to-maintain-brightness lang=rust

pub fn min_energy(_n: i32, brightness: i32, intervals: Vec<Vec<i32>>) -> i64 {
    let mut intervals: Vec<(i32, i32)> = intervals.iter().map(|iv| (iv[0], iv[1])).collect();
    intervals.sort_unstable_by_key(|i| i.0);

    let (first_start, first_end) = intervals[0];
    let mut end_time = first_end;
    let mut on_times = i64::from(first_end - first_start + 1);
    for &(start, end) in &intervals[1..] {
        if start <= end_time && end_time < end {
            on_times += i64::from(end - end_time);
            end_time = end;
        } else if end_time < start {
            on_times += i64::from(end - start + 1);
            end_time = end;
        }
    }

    let multiple = i64::from((brightness - 1) / 3 + 1);
    multiple * on_times
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(min_energy(5, 5, vec![vec![6, 12]]), 14);
    }

    #[test]
    fn base_test_2() {
        assert_eq!(min_energy(2, 1, vec![vec![0, 0], vec![2, 2]]), 2);
    }

    #[test]
    fn base_test_3() {
        assert_eq!(min_energy(4, 2, vec![vec![1, 3], vec![2, 4]]), 4);
    }

    #[test]
    fn base_test_4() {
        assert_eq!(min_energy(6, 3, vec![vec![8, 9]]), 2);
    }

    #[test]
    fn overlap_all() {
        assert_eq!(min_energy(6, 2, vec![vec![14, 16], vec![6, 18]]), 13);
    }

    #[test]
    fn something() {
        assert_eq!(
            min_energy(
                15,
                8,
                vec![vec![2, 4], vec![18, 20], vec![8, 20], vec![14, 14]]
            ),
            48
        );
    }

    #[test]
    fn overflow() {
        assert_eq!(
            min_energy(
                738235,
                635017,
                vec![
                    vec![880012, 962435],
                    vec![880012, 984965],
                    vec![880012, 966345],
                    vec![880012, 959020],
                    vec![880012, 954813],
                    vec![880012, 891751],
                    vec![880012, 924920],
                    vec![880012, 998728],
                    vec![880012, 943084],
                    vec![880012, 909394],
                    vec![880012, 949521],
                    vec![880012, 966713],
                    vec![880012, 883519],
                    vec![880012, 930005],
                    vec![880012, 939543],
                    vec![880012, 960371],
                    vec![880012, 955023],
                    vec![880012, 929936],
                    vec![880012, 940539]
                ]
            ),
            25129183541
        );
    }

    #[test]
    fn adversarial_intervals() {
        assert_eq!(
            min_energy(
                185,
                4,
                vec![
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094],
                    vec![257201, 997094]
                ]
            ),
            1479788
        )
    }
}
