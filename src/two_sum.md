# Two Sum

You are given an array of integers `nums` and an integer `target`, return _indices of the two numbers such that they add up to `target`_.

You may assume that each input would have **_exactly_ one solution**, and you may not use the _same_ element twice.

You can return the answer in any order.

**Example 1:**

```txt
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

**Example 2:**

```txt
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

**Example 3:**

```txt
Input: nums = [3,3], target = 6
Output: [0,1]
```

**Constraints:**

- `2 <= nums.length <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
- `-10^9 <= target <= 10^9`
- **Only one valid answer exists.**

**Follow-up:** Can you come up with an algorithm that is less than `O(n^2)` time complexity?

## Solution

The trick to get a better than <math><mi>O</mi><mo>(</mo><msup><mi>n</mi><mn>2</mn></msup><mo>)</mo></math> complexity is to understand that as we go through the list of integers, we can keep track of the numbers we see. Then in the future, we check if that number was in the list.

For instance, if we have the array `[1, 7, 8, 4, 9]` and the target is `5`. When we read the first number `1`, we store that we've seen it. Then when we see `4` we check if `5 - 4 = 1` is in the list of seen numbers, find it, and get our match.

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut record: HashMap<i32, i32> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        if let Some(j) = record.get(&(target - n)) {
            return vec![*j, i as i32];
        }
        record.insert(*n, i as i32);
    }

    unreachable!("problem guarantees a solution");
}
```
