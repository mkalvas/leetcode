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

```txt
Input: nums = [0,1,2,3]
Output: 2
Explanation:
There are 2 perfect pairs
```

| `(i, j)` | `(a, b)` | `min(abs(a − b), abs(a + b))`     | `min(abs(a), abs(b))` | `max(abs(a − b), abs(a + b))`     | `max(abs(a), abs(b))` |
| -------- | -------- | --------------------------------- | --------------------- | --------------------------------- | --------------------- |
| (1, 2)   | (1, 2)   | `min(abs(1 − 2), abs(1 + 2)) = 1` | 1                     | `max(abs(1 − 2), abs(1 + 2)) = 3` | 2                     |
| (2, 3)   | (2, 3)   | `min(abs(2 − 3), abs(2 + 3)) = 1` | 2                     | `max(abs(2 − 3), abs(2 + 3)) = 5` | 3                     |

**Example 2:**

```txt
Input: nums = [-3,2,-1,4]
Output: 4
Explanation:
There are 4 perfect pairs
```

| `(i, j)` | `(a, b)` | `min(abs(a − b), abs(a + b))`           | `min(abs(a), abs(b))` | `max(abs(a − b), abs(a + b))`           | `max(abs(a), abs(b))` |
| -------- | -------- | --------------------------------------- | --------------------- | --------------------------------------- | --------------------- |
| (0, 1)   | (-3, 2)  | `min(abs(-3 - 2), abs(-3 + 2)) = 1`     | 2                     | `max(abs(-3 - 2), abs(-3 + 2)) = 5`     | 3                     |
| (0, 3)   | (-3, 4)  | `min(abs(-3 - 4), abs(-3 + 4)) = 1`     | 3                     | `max(abs(-3 - 4), abs(-3 + 4)) = 7`     | 4                     |
| (1, 2)   | (2, -1)  | `min(abs(2 - (-1)), abs(2 + (-1))) = 1` | 1                     | `max(abs(2 - (-1)), abs(2 + (-1))) = 3` | 2                     |
| (1, 3)   | (2, 4)   | `min(abs(2 - 4), abs(2 + 4)) = 2`       | 2                     | `max(abs(2 - 4), abs(2 + 4)) = 6`       | 4                     |

**Example 3:**

```txt
**Input:** nums = [1,10,100,1000]
**Output:** 0
**Explanation:**
There are no perfect pairs. Thus, the answer is 0.
```

**Constraints:**

- `2 <= nums.length <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

## Solution

There's an obvious <math><mi>O</mi><mo>(</mo><msup><mi>n</mi><mn>2</mn></msup><mo>)</mo></math> solution that is unfortunately not fast enough for the submission tests.

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

The trick is to write down some scenarios and look at the values and their signs.

| a   | b   | a - b | a + b | abs(a - b) | abs(a + b) | min() |
| --- | --- | ----- | ----- | ---------- | ---------- | ----- |
| -3  | -5  | 2     | -8    | 2          | 8          | 2     |
| 3   | 5   | -2    | 8     | 2          | 8          | 2     |
| -3  | 5   | -8    | 2     | 8          | 2          | 2     |
| 3   | -5  | 8     | -2    | 8          | 2          | 2     |

This shows us that all of our scenarios simplify to one of two situations depending on which number is greater. Therefore, if we order the array before searching it, we can simplify all the comparisons down to `b - a <= a` because we know that `b >= a`. This simplifies a lot of arithmetic work to a very quick operation.

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

However, this (and breaking early) is still not fast enough. We have an <math><mi>O</mi><mo>(</mo><msup><mi>n</mi><mn>2</mn></msup><mo>)</mo></math> solution in the worst case still and given we might have `10^5` values in `nums`, that's not going to be good enough.

Instead we need to find the way to get to <math><mi>O</mi><mo>(</mo><mi>n</mi><mo>&#x22C5;</mo><mi>log</mi><mo>&#x2061;</mo><mi>n</mi><mo>)</mo></math> or <math><mi>O</mi><mo>(</mo><mi>n</mi><mo>)</mo></math> algorithm itself. The answer is the common "sliding window" approach with two pointers. We fix a given `i` in the sorted, absolute values of the numbers for one of the pointers. Then we make sure that we're at least looking at the next number (`j = j.max(i + 1)`) in case `i` "catches up" to `j` as we move them. Then we do the core loop of value checking (which is now fast) and advancing `j`. All of those values will be valid so then we add the full range of valid values as a single arithmetic count. Then we advance `i` and go from there.

```rust
pub fn perfect_pairs(nums: Vec<i32>) -> i64 {
    let mut abs_vals: Vec<i32> = nums.iter().map(|n| n.abs()).collect();
    abs_vals.sort_unstable();

    let mut j = 0;
    let mut count: i64 = 0;
    for i in 0..abs_vals.len() {
        j = j.max(i + 1);
        while j < abs_vals.len() && abs_vals[j] - abs_vals[i] <= abs_vals[i] {
            j += 1;
        }
        count += (j - i - 1) as i64;
    }

    count
}
```
