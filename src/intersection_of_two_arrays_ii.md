# Intersection of Two Arrays II

Given two integer arrays `nums1` and `nums2`, return an array containing their intersection. Include each value as many times as it occurs in both arrays, using the smaller of its two occurrence counts. The result may be in any order.

**Example 1:**

```txt
Input: nums1 = [1,2,2,1], nums2 = [2,2]
Output: [2,2]
```

**Example 2:**

```txt
Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
Output: [4,9]
Explanation: [9,4] is also accepted.
```

**Constraints:**

- `1 <= nums1.length, nums2.length <= 1000`
- `0 <= nums1[i], nums2[i] <= 1000`

**Follow-up:**

- How could you optimize the algorithm if the arrays were already sorted?
- Which algorithm would you choose if `nums1` were much smaller than `nums2`?
- How would you handle `nums2` stored on disk when memory cannot hold all of its elements at once?

## Solution

The interesting wrinkle here is that we need to return the same number of copies of the intersecting value as there are in the array with fewer copies of that value. This takes away the simple path of converting to sets and taking the intersection.

We can get an <math><mi>O</mi><mo>(</mo><mi>n</mi><mo>⋅</mo><mi>m</mi><mo>)</mo></math> solution (in the worst case, where the arrays share no values at all) by simply walking the first array, checking the second for the value, and removing it from the second if we find it so we don't count it again.

```rust
pub fn intersect(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    for n1 in nums1.iter() {
        if let Some(i) = nums2.iter().position(|n2| n2 == n1) {
            out.push(n1.clone());
            nums2.remove(i);
        }
    }

    out
}
```

But surely we can do better and find an <math><mi>O</mi><mo>(</mo><mi>n</mi><mo>)</mo></math> solution or something.

My initial thought is to build counts of each and then intersect them and add the smaller of the two counts. This works in <math><mi>O</mi><mo>(</mo><mi>n</mi><mo>+</mo><mi>m</mi><mo>)</mo></math> time.

```rust
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let counts1 = count(nums1);
    let counts2 = count(nums2);

    let mut out = Vec::new();
    for (&n, &c1) in &counts1 {
        let c2 = counts2.get(&n).copied().unwrap_or(0);
        out.extend(repeat_n(n, c1.min(c2)));
    }

    out
}

fn count(ns: Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for n in ns {
        *counts.entry(n).or_default() += 1;
    }
    counts
}
```

This is probably the intended time complexity for this problem, but there's another improvement we can make by only using one count map. In this case it simplifies the complexity and bookkeeping as well — win win!

```rust
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut counts = count(nums1);
    let mut out = Vec::new();

    for n in nums2 {
        if let Some(remaining) = counts.get_mut(&n) {
            if *remaining > 0 {
                out.push(n);
                *remaining -= 1;
            }
        }
    }

    out
}
```

We just decrement and guard on the initial count map to guarantee that we only grab the fewest number of copies.

Finally, given the problem constraint on input array lengths, we could use a sparse array with indexes for "keys" instead of a `HashMap` which would avoid the overhead of the hashing algorithm. But that feels like overkill and not as clear or obvious.
