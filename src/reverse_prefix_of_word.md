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

## Solution

This had some surprising depth to it even though the actual problem is easily solved in a performant-enough way. I guess that's what you get for thinking about utf8.

Here's a first pass that just does an obvious thing by splitting on the delimiter and then builds up the result string by reversing the first split segment and concatenating the rest without reversing it.

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

The next step was to make this a little more idiomatic Rust. I forgot about the `chain` method and now we're not splitting on every delimiter `ch` (think about one letter 250 times). But this one is actually slower than the previous version.

```rust
pub fn reverse_prefix(word: String, ch: char) -> String {
    let Some(i) = word.find(ch) else { return word };
    let (prefix, suffix) = word.split_at(i + ch.len_utf8());
    prefix.chars().rev().chain(suffix.chars()).collect()
}
```

You could get a faster "somewhat idiomatic" version using a single `with_capacity` and building the string via slices of the input. Doing this is a single allocation instead of 3 in the previous version and 4 in the original `fold` version.

```rust
// "with_capacity" version
pub fn reverse_prefix(word: String, ch: char) -> String {
    let Some(i) = word.find(ch) else { return word };
    let end = i + ch.len_utf8();
    let mut out = String::with_capacity(word.len());
    out.extend(word[..end].chars().rev());
    out.push_str(&word[end..]);
    out
}
```

But ultimately in this instance, I think an in-place modification of the string bytes is actually the closest to being the problem statement. Yes it's mutations and indexing into a byte buffer, and yes it would look a bit more complicated with full utf8 support (probably including magic byte comparisons for utf8 groups!), but I actually think it's the nicest of the bunch overall. So that's what I went with for my final version.

```rust
// "in-place" version
pub fn reverse_prefix(word: String, ch: char) -> String {
    let Some(i) = word.find(ch) else { return word };
    let mut buf = word.into_bytes();
    buf[..i + ch.len_utf8()].reverse();
    String::from_utf8(buf).expect("problem guarantees `word` is ascii")
}
```

Looking at this deeper, I think the full utf8 in-place (not written out here because I only half got it working) will lose out on both the readability _and performance_ to the single allocation `with_capacity` version. Doing it in place requires regrouping the utf8 byte sequences and revalidating the entire buffer which more than eats up all the previous gains.

And even deeper, I think my final "in-place" version is only faster on short strings where allocation is a significant portion of the runtime. So probably the "best" general version of this is actually the `with_capacity` version because it gets the exact memory needed, uses `extend` to use it, and finishes with `push_str` (`memcpy` in a trench coat). I say "best **general** version" because it supports full utf8 and is fastest when over all sizes. **However**, if you can guarantee the _prefix_ is valid ascii only (e.g., `prefix.is_ascii()`) then you'd probably want to use the in-place version always. This constraint is true for our problem statement which is why I stuck with that version in the end. You could alternatively write a version that dispatches to either the "in-place" final version or the `with_capacity` full utf8 version at runtime via an `is_ascii` check which would only add a little overhead and might be worth it for all the utf8 cases.
