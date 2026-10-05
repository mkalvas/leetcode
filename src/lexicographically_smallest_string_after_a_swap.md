# Lexicographically Smallest String After a Swap

Given a string `s` containing only digits, return the lexicographically smallest string that can be obtained after swapping **adjacent** digits in `s` with the same **parity** at most **once**.

Digits have the same parity if both are odd or both are even. For example, 5 and 9, as well as 2 and 4, have the same parity, while 6 and 9 do not.

**Example 1:**

```txt
Input: s = "45320"
Output: "43520"
Explanation:
`s[1] == '5'` and `s[2] == '3'` both have the same parity, and swapping them results in the lexicographically smallest string.
```

**Example 2:**

```txt
Input: s = "001"
Output: "001"
Explanation:
There is no need to perform a swap because `s` is already the lexicographically smallest.
```

**Constraints:**

- `2 <= s.length <= 100`
- `s` consists only of digits.

## Solution

I first read the sentence

> after swapping **adjacent** digits in `s` with the same **parity** at most **once**.

as allowing us to do multiple swaps in the string but each digit could only participate in 1 swap. For the first example, this would mean that we would swap the `5` and `3` but then _also_ swap the `2` and `0`. I'm pretty sure this is in fact what this sentence means grammatically and the "correct" version is a classic example of a misplaced modifier. A clearer definition would be something like

> We can only perform one total swap operation to produce then resulting string.

But I digress. The confused version is actually slightly harder because we had to keep track of skipping over swapped items but not skipping non-swapped items. It's not considerably more complicated, but it did add a bit more to the following solution. I mention all this merely to explain why I have this version with manual indexing into the string and the possibilities for off-by-one errors (notice the final push in the non-early-return path). I started doing the other problem and manually tracking all that, then simplified it to actually solve the problem correctly, but left the structure the same for now.

```rust
pub fn get_smallest_string(s: String) -> String {
    let mut output = Vec::with_capacity(s.len());
    let bytes = s.as_bytes();

    let mut i = 1;
    while i < bytes.len() {
        if bytes[i] < bytes[i - 1] && bytes[i] % 2 == bytes[i - 1] % 2 {
            output.push(bytes[i]);
            output.push(bytes[i - 1]);
            output.extend_from_slice(&bytes[(i + 1)..]);
            return String::from_utf8(output).expect("problem guarantees ascii")
        }

        output.push(bytes[i - 1]);
        i += 1;
    }

    output.push(bytes[i - 1]);
    String::from_utf8(output).expect("problem guarantees ascii")
}
```

But given we _do_ only need to perform at most 1 total swap, we can simplify this drastically. We only need to find and swap the first (most significant) digit we can find in order to make the smallest lexicographic string. Taking advantage of the excellent iterator tools available to us in Rust. The `windows` method gives us consecutive windows. For example, `vec![1,2,3].windows(2)` produces an iterator with windows `[1, 2]` and `[2, 3]`. The `position` method is a simple "find index" operation that gives us the index at the first match of the predicate.

```rust
pub fn get_smallest_string(s: String) -> String {
    let mut bytes = s.into_bytes();
    if let Some(i) = bytes
        .windows(2)
        .position(|w| w[1] < w[0] && w[0] % 2 == w[1] % 2)
    {
        bytes.swap(i, i + 1);
    }

    String::from_utf8(bytes).expect("problem guarantees ascii")
}
```

Notice also that this performs no allocations! The `.into_bytes` _moves_ the underlying `String`'s `Vec` out of the string, consuming the string, then the `String::from_utf8` moves the vec back in, wrapping it with the `String` metadata container. Also the `bytes.swap()` modifies it in place (hand-wavingly basically a `mem::swap`). So not only is this considerably easier to read, maintain, understand, and is more idiomatic Rust, it's also faster! Another win for Rust as a language, making the simple way the fast way.
