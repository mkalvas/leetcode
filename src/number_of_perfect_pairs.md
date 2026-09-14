# Number of Perfect Pairs

You are given an integer array `nums`.

A pair of indices `(i, j)` is called **perfect** if the following conditions are satisfied:

- `i < j`
- Let `a = nums[i]`, `b = nums[j]`. Then:
  - `min(|a - b|, |a + b|) <= min(|a|, |b|)`
  - `max(|a - b|, |a + b|) >= max(|a|, |b|)`

Return the number of **distinct** perfect pairs.

**Note:** The absolute value `|x|` refers to the **non-negative** value of `x`.

**Example 1:**

> **Input:** nums = [0,1,2,3]
>
> **Output:** 2
>
> **Explanation:**
>
> There are 2 perfect pairs:
>
> | `(i, j)` | `(a, b)` | `min(\|a − b\|, \|a + b\|)`     | `min(\|a\|, \|b\|)` | `max(\|a − b\|, \|a + b\|)`     | `max(\|a\|, \|b\|)` |
> | -------- | -------- | ------------------------------- | ------------------- | ------------------------------- | ------------------- |
> | (1, 2)   | (1, 2)   | `min(\|1 − 2\|, \|1 + 2\|) = 1` | 1                   | `max(\|1 − 2\|, \|1 + 2\|) = 3` | 2                   |
> | (2, 3)   | (2, 3)   | `min(\|2 − 3\|, \|2 + 3\|) = 1` | 2                   | `max(\|2 − 3\|, \|2 + 3\|) = 5` | 3                   |

**Example 2:**

> **Input:** nums = [-3,2,-1,4]
>
> **Output:** 4
>
> **Explanation:**
>
> There are 4 perfect pairs:
>
> | `(i, j)` | `(a, b)` | `min(\|a − b\|, \|a + b\|)`           | `min(\|a\|, \|b\|)` | `max(\|a − b\|, \|a + b\|)`           | `max(\|a\|, \|b\|)` |
> | -------- | -------- | ------------------------------------- | ------------------- | ------------------------------------- | ------------------- |
> | (0, 1)   | (-3, 2)  | `min(\|-3 - 2\|, \|-3 + 2\|) = 1`     | 2                   | `max(\|-3 - 2\|, \|-3 + 2\|) = 5`     | 3                   |
> | (0, 3)   | (-3, 4)  | `min(\|-3 - 4\|, \|-3 + 4\|) = 1`     | 3                   | `max(\|-3 - 4\|, \|-3 + 4\|) = 7`     | 4                   |
> | (1, 2)   | (2, -1)  | `min(\|2 - (-1)\|, \|2 + (-1)\|) = 1` | 1                   | `max(\|2 - (-1)\|, \|2 + (-1)\|) = 3` | 2                   |
> | (1, 3)   | (2, 4)   | `min(\|2 - 4\|, \|2 + 4\|) = 2`       | 2                   | `max(\|2 - 4\|, \|2 + 4\|) = 6`       | 4                   |

**Example 3:**

> **Input:** nums = [1,10,100,1000]
>
> **Output:** 0
>
> **Explanation:**
>
> There are no perfect pairs. Thus, the answer is 0.
>
> **Constraints:**
>
> - `2 <= nums.length <= 10<sup>5</sup>`
> - `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`

## Other solutions

Not performant enough

```rust
pub fn perfect_pairs(nums: Vec<i32>) -> i64 {
    let mut count = 0;
    for i in 0..(nums.len() - 1) {
        for j in (i + 1)..nums.len() {
            let a = &nums[i];
            let b = &nums[j];
            if min((a - b).abs(), (a + b).abs()) <= min(a.abs(), b.abs())
                && max((a - b).abs(), (a + b).abs()) >= max(a.abs(), b.abs())
            {
                count += 1;
            }
        }
    }
    count
}
```

The trick is looking at the values and their signs

| a   | b   | a - b | a + b | \|a - b\| | \|a + b\| | min() |
| --- | --- | ----- | ----- | --------- | --------- | ----- |
| -3  | -5  | 2     | -8    | 2         | 8         | 2     |
| 3   | 5   | -2    | 8     | 2         | 8         | 2     |
| -3  | 5   | -8    | 2     | 8         | 2         | 2     |
| 3   | -5  | 8     | -2    | 8         | 2         | 2     |

We see that if we order the array before searching it so that we can simplify all the comparisons down to `b - a <= a` because we know that `b >= a`.

However, this (and breaking early) is still not fast enough. We have a `O(n^2)` solution in the worst case still and given we might have `10^5` values in `nums`, that's not going to be good enough.

```rust
pub fn perfect_pairs(nums: Vec<i32>) -> i64 {
    let mut abs_vals: Vec<i32> = nums.iter().map(|n| n.abs()).collect();
    abs_vals.sort_unstable();
    
    let mut count = 0;
    for i in 0..abs_vals.len() {
        for j in (i + 1)..abs_vals.len() {
            if abs_vals[j] - abs_vals[i] <= abs_vals[i] {
                count += 1;
            } else {
                break;
            }
        }
    }
    
    count
}
```

Instead we need to find the way to linearize it like we do in the actual solution that's in the code file now.
