# Count Asterisks

You are given a string `s`, where every **two** consecutive vertical bars `'|'` are grouped into a **pair**. In other words, the 1st and 2nd `'|'` make a pair, the 3rd and 4th `'|'` make a pair, and so forth.

Return _the number of_ `'*'` _in_ `s`, **excluding** the* `'*'` _between each pair of_ `'|'`.

**Note** that each `'|'` will belong to **exactly** one pair.

**Example 1:**

```txt
Input: s = "l|*e*et|c**o|*de|"
Output: 2
Explanation: The considered characters are underlined: "l|*e*et|c**o|*de|".
The characters between the first and second '|' are excluded from the answer.
Also, the characters between the third and fourth '|' are excluded from the answer.
There are 2 asterisks considered. Therefore, we return 2.
```

**Example 2:**

```txt
Input: s = "iamprogrammer"
Output: 0
Explanation: In this example, there are no asterisks in s. Therefore, we return 0.
```

**Example 3:**

```txt
Input: s = "yo|uar|e**|b|e***au|tifu|l"
Output: 5
Explanation: All 5 asterisks are outside of pairs
```

**Constraints:**

- `1 <= s.length <= 1000`
- `s` consists of lowercase English letters, vertical bars `'|'`, and asterisks `'*'`.
- `s` contains an **even** number of vertical bars `'|'`.

## Solution

We simply walk the characters, keeping an `in_pair` boolean and a `count` number. When we come across a `'|'` character, we flip the boolean and if we come across a `'*'` we count it only if we are _outside_ a pair (as stated in the problem).

```rust
pub fn count_asterisks(s: String) -> i32 {
    let mut in_pair = false;
    let mut count = 0;
    for c in s.chars() {
        match c {
            '|' => in_pair = !in_pair,
            '*' if !in_pair => count += 1,
            _ => {},
        }
    }
    count
}
```

Rust's [match guards](https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards) are a pretty cool way to handle this in an elegant way.

We could use `bytes()` instead of `chars()` and match on `b'|'` and `b'*'` because we're guaranteed valid ascii strings by the problem statement, but that's an unnecessary optimization for this small problem.

Another approach entirely would be to split the string into sections of `'|'` and `step_by(2)`. This is slightly less performant but may read better for some people. I think the even number skipping is a bit opaque, so I'll just leave my solution as is.

```rust
pub fn count_asterisks(s: String) -> i32 {
    s.split('|')
        .step_by(2)
        .map(|seg| seg.matches('*').count())
        .sum::<usize>() as i32
}
```
