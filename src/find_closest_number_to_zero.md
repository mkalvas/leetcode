# Find Closest Number to Zero

Given an integer array `nums` of size `n`, return the _number with the value **closest** to `0` in `nums`_. If there are multiple answers, return _the number with the **largest** value_.

**Example 1:**

```txt
Input: nums = [-4,-2,1,4,8]
Output: 1
Explanation:
The distance from -4 to 0 is |-4| = 4.
The distance from -2 to 0 is |-2| = 2.
The distance from 1 to 0 is |1| = 1.
The distance from 4 to 0 is |4| = 4.
The distance from 8 to 0 is |8| = 8.
Thus, the closest number to 0 in the array is 1.
```

**Example 2:**

```txt
Input: nums = [2,-1,1]
Output: 1
Explanation: 1 and -1 are both the closest numbers to 0, so 1 being larger is returned.
```

**Constraints:**

- `1 <= n <= 1000`
- `-105 <= nums[i] <= 105`

## Solution

This one is straightforward as a problem, but the simplicity of the solution relies on some cool Rust features to be so concise.

```rust
pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .max_by_key(|&number| (Reverse(number.unsigned_abs()), number))
        .expect("nums is guaranteed to be non-empty")
}
```

First, we have a `max_by_key` function (there are similar functions like `sort_by_key`, `min_by_key`, and more) that will allow us to iterate through an iterator, provide some `key` for each element, and then only retain the `max`.

Second, we have a `std::cmp::Reverse` helper struct that will take any `T` (though most helpfully a `T` that is `PartialOrd` or `Ord`) and reverse its comparison operations. This really shows the power of the trait system in Rust that we can implement such a powerful, abstract, generic idea as "any order-able thing's ordering can be reversed".

Third, we have the language fact that tuples implement `Ord` and `PartialOrd` so that they can be compared in field order. This allows you to sort on multiple fields (for ties) in a key by creating a tuple of those key values.

Combining these facts, we create a tuple for every number and its absolute value, then look for the max of these. Obviously the max of numbers is the largest number and when that's reversed, it's the smallest. So we get the tuples ordered by smallest absolute value followed by largest regular value. In our specific case, the only ties we can have are between `-x` and `x` so this will pick the larger (positive) value as requested by the problem spec.

One other edge case worth calling out is that if the problem was defined differently and a number could be `i32::MIN`, it would overflow when taking the absolute value, so we use the `unsigned_abs` instead which works for all `i32`.
