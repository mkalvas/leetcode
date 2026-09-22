// id=30 slug=substring-with-concatenation-of-all-words lang=rust

use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let string = s.as_str();
    let word_count = words.len();
    let word_len = words[0].len();
    let mut indices: Vec<i32> = Vec::new();
    let mut frequencies: HashMap<&str, i16> = HashMap::new();
    for word in &words {
        *frequencies.entry(word).or_insert(0) += 1;
    }

    for i in 0..word_len {
        let mut j = i;
        let mut matched = 0;
        let mut slices = Vec::<&str>::with_capacity(word_count);
        let mut freqs = frequencies.clone();
        while j + word_len <= string.len() {
            let slice = &string[j..(j + word_len)];

            slices.push(slice);
            freqs.entry(slice).and_modify(|c| {
                if *c > 0 {
                    matched += 1;
                }
                *c -= 1;
            });

            if slices.len() > word_count {
                let removed = slices[0];
                slices.remove(0);
                freqs.entry(removed).and_modify(|c| {
                    *c += 1;
                    if *c > 0 {
                        matched -= 1;
                    }
                });
            }

            j += word_len;
            if slices.len() == word_count && matched == word_count {
                indices.push((j - word_count * word_len) as i32);
            }
        }
    }

    indices
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test_1() {
        assert_eq!(
            find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
    }

    #[test]
    fn base_test_2() {
        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn base_test_3() {
        assert_eq!(
            find_substring(
                "barfoofoobarthefoobarman".to_string(),
                vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
            ),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn base_test_4() {
        assert_eq!(
            find_substring(
                "aabbccbbaa".to_string(),
                vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]
            ),
            vec![0, 4]
        )
    }

    #[test]
    fn overlapping() {
        assert_eq!(
            find_substring("abababa".to_string(), vec!["ababa".to_string()]),
            vec![0, 2]
        );
    }

    #[test]
    fn extra_at_multiple() {
        assert_eq!(
            find_substring("aab".to_string(), vec!["a".to_string(), "b".to_string()]),
            vec![1]
        );
    }

    #[test]
    fn adversarial() {
        let s = "a".repeat(5000).to_string();
        let words: Vec<String> = s.chars().map(Into::into).collect();
        assert_eq!(find_substring(s, words), vec![0])
    }
}
