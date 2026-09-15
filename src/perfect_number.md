# Perfect Number

A [**perfect number**](https://en.wikipedia.org/wiki/Perfect_number) is a **positive integer** that is equal to the sum of its **positive divisors**, excluding the number itself. A **divisor** of an integer `x` is an integer that can divide `x` evenly.

Given an integer `n`, return `true` *if* `n` *is a perfect number, otherwise return* `false`.

**Example 1:**

```txt
Input: num = 28
Output: true
Explanation: 28 = 1 + 2 + 4 + 7 + 14
1, 2, 4, 7, and 14 are all divisors of 28.
```

**Example 2:**

```txt
Input: num = 7
Output: false
```

**Constraints:**

- `1 <= num <= 10<sup>8</sup>`

## Other solutions

I like this solution a bit more but it's less performant than the final one and has hacky `[0, 0]` values and an extra filter pass.

```rust
pub fn check_perfect_number(num: i32) -> bool {
    let upper = f64::sqrt(num as f64).ceil() as i32;
    (1..=upper)
        .flat_map(|d| if num % d == 0 { [d, num / d] } else { [0, 0] })
        .filter(|d| d != &num)
        .sum::<i32>()
        == num
}
```
