use std::cmp::min;

pub fn fix_playlist_window<'a>(songs: &'a [&'a str], start: usize, count: usize) -> &'a [&'a str] {
    songs[start..min(start + count, songs.len())].into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            fix_playlist_window(&["Intro", "Storm", "Sunrise", "Finale"], 1, 2),
            vec!["Storm", "Sunrise"]
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(
            fix_playlist_window(&["Blue", "Green", "Gold", "Red", "Violet"], 2, 3),
            vec!["Gold", "Red", "Violet"]
        );
    }

    #[test]
    fn base_test_3() {
        assert_eq!(
            fix_playlist_window(&["Moon", "Stars", "Dawn"], 1, 5),
            vec!["Stars", "Dawn"]
        );
    }

    #[test]
    fn start_at_end_returns_empty() {
        assert_eq!(fix_playlist_window(&["Solo"], 1, 3), Vec::<String>::new());
    }

    #[test]
    fn count_less_than_songs_returns_head() {
        assert_eq!(
            fix_playlist_window(
                &[
                    "Wake Up",
                    "Road Trip",
                    "Ocean Air",
                    "City Lights",
                    "Home Again"
                ],
                0,
                4
            ),
            vec!["Wake Up", "Road Trip", "Ocean Air", "City Lights"]
        );
    }
}
