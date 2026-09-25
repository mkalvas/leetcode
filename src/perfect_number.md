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

- `1 <= num <= 10^8`

## Solution

The key insight to this one is that we only need to check up to the square root of a number since we can just add its pair when we find one. For instance, to get all the clean divisors of `28` we can check `1` through `6` since when we check `4` we also get `7` from the division check.

Other than that, there's some bookkeeping for adding `1` and also not adding the number `n` itself (though we could instead check against `2n` for a match). This definition also excludes `1` from being a perfect number so in the second version below we short circuit that.

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

I like this solution a bit more but it's less performant than the following one, has hacky `[0, 0]` values, and an extra filter pass.

```rust
pub fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    let mut sum = 1;
    let mut d = 2;
    while d * d <= num {
        if num % d == 0 {
            sum += d;
            let pair = num / d;
            if pair != d {
                sum += pair;
            }
            if sum > num {
                return false;
            }
        }
        d += 1;
    }

    sum == num
}
```

The core loop uses `d * d <= num` instead of `sqrt` because it's faster and it short-circuits on `sum > num` for many large numbers whose divisors will sum to greater than the number far before we check for all the divisors.
