# Substring with Concatenation of All Words

You are given a string `s` and an array of strings `words`. All the strings of `words` are of **the same length**.

A **concatenated string** is a string that exactly contains all the strings of any permutation of `words` concatenated.

- For example, if `words = ["ab","cd","ef"]`, then `"abcdef"`, `"abefcd"`, `"cdabef"`, `"cdefab"`, `"efabcd"`, and `"efcdab"` are all concatenated strings. `"acdbef"` is not a concatenated string because it is not the concatenation of any permutation of `words`.

Return an array of _the starting indices_ of all the concatenated substrings in `s`. You can return the answer in **any order**.

**Example 1:**

```txt
Input: s = "barfoothefoobarman", words = ["foo","bar"]
Output: [0,9]
Explanation:
- The substring starting at 0 is `"barfoo"`. It is the concatenation of `["bar","foo"]` which is a permutation of `words`.  
- The substring starting at 9 is `"foobar"`. It is the concatenation of `["foo","bar"]` which is a permutation of `words`.
```

**Example 2:**

```txt
Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
Output: []
Explanation:
- There is no concatenated substring.
```

**Example 3:**

```txt
Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
Output: [6,9,12]
Explanation:
- The substring starting at 6 is `"foobarthe"`. It is the concatenation of `["foo","bar","the"]`.
- The substring starting at 9 is `"barthefoo"`. It is the concatenation of `["bar","the","foo"]`.
- The substring starting at 12 is `"thefoobar"`. It is the concatenation of `["the","foo","bar"]`.
```

**Constraints:**

- `1 <= s.length <= 10^4`
- `1 <= words.length <= 5000`
- `1 <= words[i].length <= 30`
- `s` and `words[i]` consist of lowercase English letters.

## Solution

<aside>I'm trying something new here and writing out my thought process as I solve the problem. I'm not sure if I'll stick with this or go back to a more polished after-action report</aside>

Initially, I thought this might be a dynamic programming problem but I wasn't sure how that would work so I just tried to start going at it in a naïve way. The first thought that popped into my head was to to make a frequency map of all the characters in all the words in `words`, but that wouldn't work because the individual words need to be whole, even though they can be in any order. For example, if I just did frequency on characters for `words = ["aa", "bb"]` we'd get a frequency of `2` for `a` and `b` but that would match on `abab` which is _not_ a concatenation of the words.

Instead let's try something else. Maybe a frequency map on the words themselves? I'm thinking ahead though and not sure how I'm going to handle the "peeking" aspect where we don't know how many characters to look at. Maybe this is actually a depth-first search. I'm just going to not overthink it and start coding.

First I want to solve the problem of "I have a string and list of words. Does this string start with any of the words?".

```rust
fn has_prefix_match(mut s: String, mut words: Vec<String>) -> Option<(String, Vec<String>)> {
    for i in 0..words.len() {
        if s.starts_with(words[i].as_str()) {
            s = s
                .strip_prefix(words[i].as_str())
                .expect("checked that string strarted with the prefix")
                .to_string();
            words.remove(i);
            return Some((s, words));
        }
    }
    None
}
```

Next, let's use that building block for finding a full match of all our words.

```rust
fn has_full_match(mut s: String, mut words: Vec<String>) -> bool {
    while !words.is_empty() {
        match has_prefix_match(s, words) {
            None => return false,
            Some((rs, rws)) => {
                s = rs;
                words = rws;
            }
        }
    }
    true
}
```

Last this gives us the outer loop where we check every character for a full match.

```rust
pub fn find_substring(mut s: String, words: Vec<String>) -> Vec<i32> {
    let mut indices: Vec<i32> = Vec::new();
    let mut i = 0;
    while !s.is_empty() {
        if has_full_match(s.clone(), words.clone()) {
            indices.push(i);
        }
        s = s.split_off(1);
        i += 1;
    }
    indices
}
```

Which surprisingly was correct! This actually does solve the problem, but I got a timeout on the adversarial input of `s = "a".repeat(5000)` and `words` is a `Vec` of `"a"` 5000 times (i.e., a long `s` and a `words` array which equals `s` split into characters with one match at index `0`).

One optimization that's immediately apparent is that we can stop checking characters when the length of concatenated `words` is greater than the remaining string. In this particular case, it means we'd only check one character in `s`.

Another optimization is that we can get all the unique starting chars for all the  `words` and if the character we're looking at in `s` isn't a word starting character, skip it.

Also, let's stop allocating a bunch of junk and move to slices and references.

```rust
pub fn find_substring(mut s: String, words: Vec<String>) -> Vec<i32> {
    let mut indices: Vec<i32> = Vec::new();
    let min_len = words.join("").len();
    let word_len = words[0].len();
    let mut starters: HashSet<char> = HashSet::new();
    for word in &words {
        starters.insert(word.chars().nth(0).expect("all words should be non-empty"));
    }

    let mut i = 0;
    while !s.is_empty() && s.len() >= min_len {
        if starters.contains(&s.chars().nth(0).expect("s should be non-empty"))
            && has_full_match(&s, words.clone(), word_len)
        {
            indices.push(i);
        }
        s = s.split_off(1);
        i += 1;
    }
    indices
}

fn has_prefix_match(s: &str, mut words: Vec<String>) -> Option<Vec<String>> {
    for i in 0..words.len() {
        if s.starts_with(words[i].as_str()) {
            words.remove(i);
            return Some(words);
        }
    }
    None
}

fn has_full_match(s: &str, mut words: Vec<String>, word_len: usize) -> bool {
    let mut j = 0;
    while !words.is_empty() {
        match has_prefix_match(&s[j..], words) {
            None => return false,
            Some(new_words) => {
                words = new_words;
                j += word_len;
            }
        }
    }
    true
}
```

Still not fast enough though. Let's go back to the idea of a frequency map for the whole words. Now that I'm thinking through that, it would be much better wouldn't it? You'd have an `O(1)` hashmap lookup with a decrement instead of a "for word in words" inner loop.

```rust
fn has_match(
    s: &str,
    word_len: usize,
    mut word_count: usize,
    mut frequencies: HashMap<&str, u16>,
) -> bool {
    let mut j = 0;
    while j < s.len() {
        match frequencies.get_mut(&s[j..(j + word_len)]) {
            Some(v) => {
                if v == &0 {
                    return false;
                }
                *v -= 1;
                j += word_len;
                word_count -= 1;
                if word_count == 0 {
                    return true;
                }
            }
            None => {
                return false;
            }
        }
    }
    false
}
```

Which passes the submission! Unfortunately, it's still very slow and looks like crap. So let's clean it up and make it faster.

First, let's cut out the `HashSet` matching. The inner loop is better/quicker now and does the same check basically.

Second, I didn't use a slice in the top level and was using `split_off` instead. Let's correct that.

This is a little better

```rust
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut string = s.as_str();
    let word_count = words.len();
    let word_len = words[0].len();
    let min_s_len = words.join("").len();
    let mut indices: Vec<usize> = Vec::new();
    let mut frequencies: HashMap<&str, u16> = HashMap::new();
    for word in &words {
        frequencies.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut i: usize = 0;
    while !string.is_empty() && string.len() >= min_s_len {
        if has_match(&string, word_len, word_count, frequencies.clone()) {
            indices.push(i);
        }
        i += 1;
        string = &string[1..];
    }
    indices.iter().map(|i| *i as i32).collect()
}
```

But now I'm wondering if this can be faster if we consider it as a sliding window kind of thing. We'd look at a window of `[s[0..word_len], s[word_len..word_len + word_len], ...]`. Then when we get around to looking at `s[word_len..word_len + word_len]` we could possibly reuse the results from the first time with the difference of `s[0..word_len]` and see if we have an extra one of those lying around to match on.

More coherently, We'd slide over the string in chunks of `word_len` and keep track as we go and see if the next set of windows has the thing it needed to be right. We'd then also have to start one of these slides per index in the `word_len`.

```txt
s = aabbccbbaa
words = ["aa" "bb", "cc"];

slide 1 = ["aa", "bb", "cc"] -> ["bb", "cc", "bb"] -> ["cc", "bb", "aa"]
slide 2 = ["ab", "bc", "cb"] -> ["bc", "cb", "ba"]
```

which we can see covers all

```txt
[aabbcc]bbaa
a[abbccb]baa
aa[bbccbb]aa
aab[bccbba]a
aabb[ccbbaa]
```

So now how do we actually do this though? For the same scenario

```txt
frequencies = { "aa": 1, "bb": 1, "cc": 1 };

slide 1 = ["aa", "bb", "cc"] with fs = { "aa": 0, "bb": 0, "cc": 0 } = match
                             then roll "aa" off window and re-add to fs if valid
                             now fs = { "aa": 1, "bb": 0, "cc": 0 }
       -> ["bb", "cc", "bb"] and roll "bb" on to window and subtract
                             now fs = { "aa": 1, "bb": -1, "cc": 0 }
                             then roll "bb" off and re-add
                             now fs = { "aa": 1, "bb": 0, "cc": 0 }
       -> ["cc", "bb", "aa"] and roll "aa" on and substract
                             now fs = { "aa": 0, "bb": 0, "cc": 0 } = match

slide 2 = ["ab", "bc", "cb"] with fs = { "aa": 1, "bb": 1, "cc": 1 }
                             then roll "ab" off and re-add
                             now fs = { "aa": 1, "bb": 1, "cc": 1 }
       -> ["bc", "cb", "ba"] and roll "ba" on and subtract
                             now fs = { "aa": 1, "bb": 1, "cc": 1 }
```

So an immediate question comes up. If you go very negative on the frequency of something `s = aaaaaaab` with `words = ["a", "b"]`, should you skip more ahead at once since you know you have to go at least that many? But that seems mostly irrelevant.

What about the bookkeeping? It feels like you'll need to do the dumb thing at least once for the initial window and then you can do the sliding thing more efficiently. Is that true? No, you could just build up the window and only check things when the window is the correct size.

But is the bookkeeping more costly than the speedup in this approach? How do we keep track of the "all zeros" in an efficient way? Maybe we just shove something in an array or something?

Ok we did it!

```rust
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let string = s.as_str();
    let word_count = words.len();
    let word_len = words[0].len();
    let mut indices: Vec<i32> = Vec::new();
    let mut frequencies: HashMap<&str, i16> = HashMap::new();
    for word in &words {
        frequencies.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }

    for i in 0..word_len {
        let mut j = i;
        let mut slices = Vec::<&str>::with_capacity(word_count);
        let mut freqs = frequencies.clone();
        while j + word_len <= string.len() {
            let slice = &string[j..(j + word_len)];

            slices.push(slice);
            freqs.entry(slice).and_modify(|c| *c -= 1);

            if slices.len() > word_count {
                let removed = slices[0];
                slices.remove(0);
                freqs.entry(removed).and_modify(|c| *c += 1);
            }

            j += word_len;
            if slices.len() == word_count && freqs.iter().all(|(_, c)| *c == 0) {
                indices.push((j - word_count * word_len) as i32);
            }
        }
    }

    indices
}
```

And we're in the mid-to-top of the runtime and memory consumption. This is a good solution in my book, but I'm going to try to look even deeper. Let's read through and see what I can find.

- The `freqs.iter().all(|(_, c)| *c == 0)` is the big one that sticks out right away. How can we better measure if it's a match without going to simple `+/-` that wouldn't take into account a situation like `{ "a": 1, "b": -1 }`?
- `slices.remove(0)` might be slow? It shifts the array which feels avoidable maybe with just indexing correctly.
- Is there some big-brained algorithm for something like this that I'm totally missing?

Ok for the `freqs.iter().all()`, I did discover that it's short-circuiting which is good and means it's not that big of a win in average case. But the fix is to keep a counter of `matched` words and we can avoid the `+/-` problem by only adjusting `matched` if the  frequency count is still `c > 0`. The reason this works is that it removes the ability of the negative "surplus" words from cancelling the positive ones. It must return to positive correctly before it can be a match anyway, so we just clamp the match count at zero.

I decided not to do the `slices.remove(0)` thing when I realized it was possible to do that indexing and also possible to convert the `HashMap`s to `Vec`s. It just felt like a lot of tedium for basically the same answer. The `freqs.iter().all()` mostly just made things more confusing too. I also did some research on this problem and I don't think there are any good big-brain algorithms that apply. There are two: _Rabin-Karp rolling-hash_ and _Aho-Corasick string searching_  that seem potentially applicable but I'm not sure how much and they were not obvious or trivial to implement in my opinion. So I'm squarely in diminishing returns now and calling it here. Here's the final version of the code, let me know if I missed anything.

```rust
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
```
