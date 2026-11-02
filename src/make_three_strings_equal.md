# Make Three Strings Equal

You are given three strings: `s1`, `s2`, and `s3`. In one operation you can choose one of these strings and delete its rightmost character. Note that you cannot completely empty a string.

Return the _minimum number of operations_ required to make the strings equal. If it is impossible to make them equal, return `-1`.

**Example 1:**

```txt
Input: s1 = "abc", s2 = "abb", s3 = "ab"
Output: 2
Explanation:
Remove the final character from s1 and s2. All three strings then contain "ab".
```

**Example 2:**

```txt
Input: s1 = "dac", s2 = "bac", s3 = "cac"
Output: -1
Explanation:
The strings start with different letters, so removing characters from their ends cannot make them equal.
```

**Constraints:**

- Each string has a length between `1` and `100`, inclusive.
- All characters in `s1`, `s2`, and `s3` are lowercase English letters.

## Solution

This is a super simple problem. Just read from the left until a character at the index isn't equal in all three strings. Then take the remainder of all the string lengths and add them together.

My first pass was ugly as hell.

```rust
    let sb1 = s1.as_bytes();
    let sb2 = s2.as_bytes();
    let sb3 = s3.as_bytes();

    let mut i = 0;
    while i < s1.len() {
        if sb1.get(i) != sb2.get(i) || sb2.get(i) != sb3.get(i) {
            break;
        }
        i += 1;
    }

    if i == 0 {
        return -1;
    }

    let l = i as i32;
    let sl1 = s1.len() as i32;
    let sl2 = s2.len() as i32;
    let sl3 = s3.len() as i32;
    0.max(sl1 - l) + 0.max(sl2 - l) + 0.max(sl3 - l)
```

And then going back and cleaning it up we can use `zip` to handle the counting (since it will exit on the smallest iterator length) and simplifying the remainder math (the common chars can't be larger than the string lengths so remove the `max` and then simplify terms).

```rust
let common = s1
    .bytes()
    .zip(s2.bytes())
    .zip(s3.bytes())
    .take_while(|((a, b), c)| a == b && b == c)
    .count();

if common == 0 {
    -1
} else {
    (s1.len() + s2.len() + s3.len() - 3 * common) as i32
}
```
