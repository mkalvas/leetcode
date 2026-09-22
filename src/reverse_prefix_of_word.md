# Reverse Prefix of Word

Given a **0-indexed** string `word` and a character `ch`, **reverse** the segment of `word` that starts at index `0` and ends at the index of the **first occurrence** of `ch` (**inclusive**). If the character `ch` does not exist in `word`, do nothing.

- For example, if `word = "abcdefd"` and `ch = "d"`, then you should **reverse** the segment that starts at `0` and ends at `3` (**inclusive**). The resulting string will be `"dcbaefd"`.

Return _the resulting string_.

**Example 1:**

```txt
Input: word = "abcdefd", ch = "d"
Output: "dcbaefd"
Explanation: The first occurrence of "d" is at index 3.
Reverse the part of word from 0 to 3 (inclusive), the resulting string is "dcbaefd".
```

**Example 2:**

```txt
Input: word = "xyxzxe", ch = "z"
Output: "zxyxxe"
Explanation: The first and only occurrence of "z" is at index 3.
Reverse the part of word from 0 to 3 (inclusive), the resulting string is "zxyxxe".
```

**Example 3:**

```txt
Input: word = "abcd", ch = "z"
Output: "abcd"
Explanation: "z" does not exist in word.
You should not do any reverse operation, the resulting string is "abcd".
```

**Constraints:**

- `1 <= word.length <= 250`
- `word` consists of lowercase English letters.
- `ch` is a lowercase English letter.

## Other Solutions

First pass

```rust
pub fn reverse_prefix(word: String, ch: char) -> String {
    let mut should_reverse = word.contains(ch);
    word.split_inclusive(ch)
        .fold(String::with_capacity(word.len()), |mut out, p| {
            if should_reverse {
                should_reverse = false;
                out.push_str(p.chars().rev().collect::<String>().as_str());
            } else {
                out.push_str(p);
            }
            out
        })
}
```

Much more idiomatic. Forgot about the `chain` method and we're not splitting on every delimiter `ch` (think about one letter 250 times). But this one is actually slower than the previous version.

```rust
pub fn reverse_prefix(word: String, ch: char) -> String {
    let Some(i) = word.find(ch) else { return word };
    let (prefix, suffix) = word.split_at(i + ch.len_utf8());
    prefix.chars().rev().chain(suffix.chars()).collect()
}
```

You could get a faster "somewhat idiomatic" version using a single `with_capacity` and building the string via slices of the input. This one is a single allocation instead of 3 in the previous version and 4 in the original `fold` version.

```rust
pub fn reverse_prefix(word: String, ch: char) -> String {
    let Some(i) = word.find(ch) else { return word };
    let end = i + ch.len_utf8();
    let mut out = String::with_capacity(word.len());
    out.extend(word[..end].chars().rev());
    out.push_str(&word[end..]);
    out
}
```

But ultimately in this instance, I think the in-place modification of the string bytes is actually the closest to being the problem statement. Yes it's mutations and indexing into a byte buffer, and yes it would look a bit more complicated with full utf8 support (probably including magic byte comparisons for utf8 groups!), but I actually think it's the nicest of the bunch overall.

Looking at this deeper, I think the full utf8 in-place will lose out on both the readabilty _and performance_ to the single allocation example with `out.extend`. Doing it in place requires regrouping the utf8 byte sequences and revalidating the entire buffer which more than eats up all the previous gains.

And even deeper, I think the in-place is only faster on short strings where allocation is a significant portion of the runtime. So probably the "best" general version of this is actually the one right above this with a single exact `with_capacity` combined with `extend`, then `push_str` (`memcpy` in a trench coat). It supports full utf8 and is fastest at all sizes. However, if you can guarantee the prefix is valid ascii only (e.g., `prefix.is_ascii()`) then you'd want to use the in-place version always. You could even write a version that dispatches on the runtime `is_ascii` check which would only add a little overhead.
