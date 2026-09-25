# Find Indices of Stable Mountains

There are `n` mountains in a row, and each mountain has a height. You are given an integer array `height` where `height[i]` represents the height of mountain `i`, and an integer `threshold`.

A mountain is called **stable** if the mountain just before it (**if it exists**) has a height **strictly greater** than `threshold`. **Note** that mountain 0 is **not** stable.

Return an array containing the indices of *all* **stable** mountains in **any** order.

**Example 1:**

```txt
Input: height = [1,2,3,4,5], threshold = 2
Output: [3,4]
Explanation:
- Mountain 3 is stable because height[2] == 3 is greater than threshold == 2.
- Mountain 4 is stable because height[3] == 4 is greater than threshold == 2.
```

**Example 2:**

```txt
Input: height = [10,1,10,1,10], threshold = 3
Output: [1,3]
```

**Example 3:**

```txt
Input: height = [10,1,10,1,10], threshold = 10
Output: []
```

**Constraints:**

* `2 <= n == height.length <= 100`
* `1 <= height[i] <= 100`
* `1 <= threshold <= 100`

## Solution

This is as simple as it seems. Just figure out if the entry satisfies the predicate and add it to the list if it does.

First pass, relatively naïve and non-"rusty"

```rust
pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut stables: Vec<i32> = vec![];

    for i in 1..height.len() {
        match height.get(i - 1) {
            Some(h) if h > &threshold => stables.push(i as i32),
            _ => {}
        }
    }

    stables
}
```

Messing around with the differences between `filter().map()` and `filter_map()`.

```rust
(1..height.len())
    .filter_map(|i| {
        if height[i - 1] > threshold {
            Some(i as i32)
        } else {
            None
        }
    })
    .collect()
```

```rust
pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    (1..height.len())
        .filter(|&i| height[i - 1] > threshold)
        .map(|i| i as i32) // guaranteed to be safe by problem construction
        .collect()
}
```

But importantly, all of these versions have the same performance characteristics. With the problem size constraints, we don't need `with_capacity` or other things here for everything to be snappy anyway.
