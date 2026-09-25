# Split Array Into Maximum Number of Subarrays

You are given an array `nums` consisting of **non-negative** integers.

We define the score of subarray `nums[l..r]` such that `l <= r` as `nums[l] AND nums[l + 1] AND ... AND nums[r]` where **AND** is the bitwise `AND` operation.

Consider splitting the array into one or more subarrays such that the following conditions are satisfied:

- **Each** element of the array belongs to **exactly** one subarray.
- The sum of scores of the subarrays is the **minimum** possible.

Return _the **maximum** number of subarrays in a split that satisfies the conditions above._

A **subarray** is a contiguous part of an array.

**Example 1:**

```txt
Input: nums = [1,0,2,0,1,2]
Output: 3
Explanation: We can split the array into the following subarrays:
- [1,0]. The score of this subarray is 1 AND 0 = 0.
- [2,0]. The score of this subarray is 2 AND 0 = 0.
- [1,2]. The score of this subarray is 1 AND 2 = 0.
The sum of scores is 0 + 0 + 0 = 0, which is the minimum possible score that we can obtain.
It can be shown that we cannot split the array into more than 3 subarrays with a total score of 0. So we return 3.
```

**Example 2:**

```txt
Input: nums = [5,7,1,3]
Output: 1
Explanation: We can split the array into one subarray: [5,7,1,3] with a score of 1, which is the minimum possible score that we can obtain.
It can be shown that we cannot split the array into more than 1 subarray with a total score of 1. So we return 1.
```

**Constraints:**

- `1 <= nums.length <= 10^5`
- `0 <= nums[i] <= 10^6`

## Solution

Let's consider an example to see how to begin thinking about this problem

```txt
nums = [1, 2, 3] = [00000001, 00000010, 00000011]

score 0..2    = 00000001 & 00000010 & 00000011   = 0
score 0..1, 2 = (00000001 & 00000010) + 00000011 = 3
score 0, 1..2 = 00000001 + (00000010 & 00000011) = 3
score 0, 1, 2 = 00000001 + 00000010 + 00000011   = 3
```

This has already shown us something interesting about how the bitwise and operation requires bits to be present in **every** number for the minimum score to not be `0`. For instance

```txt
nums = [1, 3, 5, 7] = [00000001, 00000011, 00000101, 00000111]

score 0..3 = 00000001 & 00000011 & 00000101 & 00000111 = 1
```

No matter what arrangement of the numbers we make, we can't get rid of the "ones" bit (`00000001`). We _could_ find some combination of numbers that all the other digits `&` to `0`. Therefore the maximum subarrays for any array that cannot score `0` is guaranteed to be `1`. This is because any addition between numbers that are non-zero will include the un-zero-able bit more than once. So in our instance, if we split the array into 2, we'll have `2 * b00000001 = 2`. You can see how this extends to mean that the _minimum score_ is the exact number of digits that are un-zero-able and thus the bitwise and of the whole array.

Now for arrays that _can_ score `0`, we need to find how to "make the most zero scores". In that first example I gave, there's a maximum of `1` because we need all the numbers to zero all the digits. Let's look at another example

```txt
nums = [1, 2, 1, 2, 1, 2]

score 0..5 = 00000001 & 00000010 & 00000001 & 00000010 & 00000001 & 00000010 = 0
score 0..1, 2..3, 4..5 =
    (00000001 & 00000010) +
    (00000001 & 00000010) +
    (00000001 & 00000010) = 0
```

We can see that there's a way to "get to zero" 3 times. So how do we find the right number? Turns out there's no trick needed. We simply keep `&`-ing numbers until we get to zero and then count it and reset the running "sum".

Here's my first pass version that's a little hard to follow. The `sum: Option<i32>` is `None` when we have no running "sum" and turns to `Some(sum)` when we have a run in progress. We start with `0` and then do `max(1, count)` so that we don't count a tail that didn't sum to `0`.

```rust
pub fn max_subarrays(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut sum: Option<i32> = None;

    for i in 0..nums.len() {
        sum = match sum {
            None => {
                if nums[i] == 0 {
                    count += 1;
                    None
                } else {
                    Some(nums[i])
                }
            }
            Some(mut s) => {
                s &= nums[i];
                if s == 0 {
                    count += 1;
                    None
                } else {
                    Some(s)
                }
            }
        };
    }

    i32::max(1, count)
}
```

But we can clean this up considerably. We can use the `-1_i32` as a sentinel value for "empty" because it's all `1`s and therefore `n & -1_i32 = n`. This gets rid of all the `Option` bookkeeping and `match` statements.

```rust
pub fn max_subarrays(nums: Vec<i32>) -> i32 {
    const ALL_ONES: i32 = -1;
    let mut score = ALL_ONES;
    let mut count = 0;

    for n in nums {
        score &= n;
        if score == 0 {
            count += 1;
            score = ALL_ONES;
        }
    }

    count.max(1)
}
```

This version is really slick in my opinion. I'm happy how this problem turned out and it was fun to go through the bitwise examples with pen and paper to figure out how to solve it.
