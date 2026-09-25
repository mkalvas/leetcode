# Maximum Number of Words You Can Type

There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

Given a string `text` of words separated by a single space (no leading or trailing spaces) and a string `brokenLetters` of all **distinct** letter keys that are broken, return _the **number of words** in `text` you can fully type using this keyboard_.

**Example 1:**

```txt
Input: text = "hello world", brokenLetters = "ad"
Output: 1
Explanation: We cannot type "world" because the 'd' key is broken.
```

**Example 2:**

```txt
Input: text = "leet code", brokenLetters = "lt"
Output: 1
Explanation: We cannot type "leet" because the 'l' and 't' keys are broken.
```

**Example 3:**

```txt
Input: text = "leet code", brokenLetters = "e"
Output: 0
Explanation: We cannot type either word because the 'e' key is broken.
```

**Constraints:**

- `1 <= text.length <= 104`
- `0 <= brokenLetters.length <= 26`
- `text` consists of words separated by a single space without any leading or trailing spaces.
- Each word only consists of lowercase English letters.
- `brokenLetters` consists of **distinct** lowercase English letters.

## Solution

For this problem we just need to look at every word and check if any of our broken letters are in them.

```rust
pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let broken_letters: HashSet<char> = HashSet::from_iter(broken_letters.chars());
    text.split_whitespace()
        .filter(|w| !w.chars().any(|c| broken_letters.contains(&c)))
        .count() as i32
}
```

We use a `HashSet` for simplicity of deduplication and checking the words against. I'm pretty sure there isn't a faster algorithm (because we have to check every character no matter what) but there may be a faster implementation than the `HashSet`. This is immediately clear and easy code though, so it wins over clever.
