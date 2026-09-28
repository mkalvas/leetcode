# Minimum Energy to Maintain Brightness

You are given an integer `n`, representing `n` light bulbs arranged in a line and indexed from 0 to `n - 1`.

You are also given an integer `brightness` and a 2D integer array `intervals`, where `intervals[i] = [start_i, end_i]` represents an **inclusive** time interval during which the lighting requirement **must** be satisfied.

At each time unit, every bulb can independently be either on or off. A bulb that is on **illuminates** its own position and its **adjacent** positions, if they exist.

The **total illumination** at a time unit is the number of **illuminated** positions. Each position is counted **at most once**.

For every integer time unit covered by **at least** one interval in `intervals`, the **total illumination** must be **at least** `brightness`. At time units not covered by any interval, all bulbs may remain off. Each bulb that is on consumes 1 unit of energy for that time unit.

Return an integer denoting the **minimum** total energy required.

**Example 1:**

```txt
Input: n = 5, brightness = 5, intervals = [[6,12]]
Output: 14
Explanation:
- Turn on the light bulbs at positions 1 and 4.
- Current state of line: `0 1 0 0 1`.
- All 5 positions are illuminated, so the required brightness is reached.
- The active interval has length `12 - 6 + 1 = 7`, so the total energy is `2 * 7 = 14`.
```

**Example 2:**

```txt
Input: n = 2, brightness = 1, intervals = [[0,0],[2,2]]
Output: 2
Explanation:
- Turn on one light bulb during each active interval.
- Each interval has length 1, so the total active time is `1 + 1 = 2`.
- The total energy is `1 * 2 = 2`.
```

**Example 3:**

```txt
Input: n = 4, brightness = 2, intervals = [[1,3],[2,4]]
Output: 4
Explanation:
- Turn on one light bulb. It can illuminate at least 2 positions.
- The active intervals overlap, so the total active time is the length of `[1,4]`, which is 4.
- The total energy is `1 * 4 = 4`.
```

**Constraints:**

- `1 <= n <= 10^6`
- `1 <= brightness <= n`
- `1 <= intervals.length <= 10^5`
- `intervals[i] == [start_i, end_i]`
- `0 <= start_i <= end_i <= 10^9`

## Solution

I went for the notebook and pen for this one because it seemed immediately obvious to me that there would be a mathematical formula for the correct energy requirement. Given each lightbulb can illuminate itself and its two neighbors, it felt like we were looking for something to do with integer division of 3. So I picked the length of 7 to find a pattern since it would likely cover 2+ "cycles" of the pattern.

```txt
0 0 0 0 0 0 0   brightness = 0  min_energy = 0  <- disallowed by problem statement
1 0 0 0 0 0 0   brightness = 1  min_energy = 1
1 0 0 0 0 0 0   brightness = 2  min_energy = 1
0 1 0 0 0 0 0   brightness = 3  min_energy = 1
0 1 0 0 0 0 1   brightness = 4  min_energy = 2
0 1 0 0 0 0 1   brightness = 5  min_energy = 2
0 1 0 0 1 0 0   brightness = 6  min_energy = 2
0 1 0 0 1 0 1   brightness = 7  min_energy = 3
```

I played around with other arrangements just a bit (e.g., the more precise `1 0 0 1 0 0 1` for `brightness = 7`) before I was convinced that we simply needed `(brightness - 1) / 3 + 1` (note the integer division or use a `floor` if your language doesn't have that) for the minimum energy.

The tricky part of this problem was going to be the interval flattening though. So I just started with the naïve approach.

```rust
pub fn min_energy(n: i32, brightness: i32, intervals: Vec<Vec<i32>>) -> i64 {
    let mut times = HashSet::new();
    for interval in intervals {
        for i in interval[0]..=interval[1] {
            times.insert(i);
        }
    }

    let multiple = (brightness as i64 - 1) / 3 + 1;
    multiple * times.len() as i64
}
```

This is correct and very easy to understand but too slow for the submission, especially for adversarial intervals. How can we do the intervals in a smarter way?

Since all we need is the number of integers in the merged intervals we can just count them directly. This is a similar approach as something called a "last merged interval". We need to pre-sort the array by the _beginning_ of each interval. This guarantees that the starts of each interval will always be non-decreasing, and therefore we can look at the _end_ of the previously merged interval to see if we need to merge this one too, or account for a gap. For instance,

```txt
[[1, 3], [2, 4]]
  start at 1,
  set end to 3,
  compare 2 < 3 and 3 < 4,
  set new end to 4

[[1, 6], [2, 4]]
  start at 1,
  set end to 6,
  compare 2 < 6 but 6 > 4,
  don't set new end

[[1, 3], [5, 7]]
  start at 1,
  set end to 3,
  compare 3 < 5
  start new interval with [5, 7]
```

This gives us a straightforward way to count the interval once complete with `end - start + 1` since it's inclusive on both ends. Then we simply multiply the number of on lights from the brightness formula with the number of on times to get the total energy.

```rust
pub fn min_energy(_n: i32, brightness: i32, intervals: Vec<Vec<i32>>) -> i64 {
    let mut intervals: Vec<(i32, i32)> = intervals.iter().map(|iv| (iv[0], iv[1])).collect();
    intervals.sort_unstable_by_key(|i| i.0);

    let (first_start, first_end) = intervals[0];
    let mut end_time = first_end;
    let mut on_times = i64::from(first_end - first_start + 1);
    for &(start, end) in &intervals[1..] {
        if start <= end_time && end_time < end {
            on_times += i64::from(end - end_time);
            end_time = end;
        } else if end_time < start {
            on_times += i64::from(end - start + 1);
            end_time = end;
        }
    }

    let multiple = i64::from((brightness - 1) / 3 + 1);
    multiple * on_times
}
```

This is the final version I stuck with since it's fast and understandable. We _can_ go further though. There's a branchless version of the core loop that looks like this

```rust
for &(start, end) in &intervals[1..] {
    let base = end_time.max(start - 1);
    on_times += i64::from((end - base).max(0));
    end_times = end_time.max(end);
}
```

Which the astute among you will realize still has branching in it since `max` de-sugars to `if a < b { b } else { a }` under the hood. The reason that this _becomes_ branchless is that the compiler can take that specific form of `if` statement and turn it into a `csel` instruction which is a **c**onditional-**sel**ect instruction. So this turns into _data_ flow, not a true _control_ flow branch (which are very different things to CPUs) meaning it can't trip on branch mis-predictions (the thing you're really talking about when you talk about being "branchless").

But unfortunately this Leetcode problem is actually not very well constructed. First of all we don't even need the `n` parameter in the function signature. Second, dropping the owned `intervals` `Vec` is the dominating performance bottleneck of the problem, followed distantly by the sort that's required for the intended solution. There aren't really ways around them. You could `mem::forget` the `Vec` but that's leaking memory and banking on the OS to clean up your intentional litter — not something you typically want to do. There's also an insane radix sort trick you can do (that I would never be able to come up with on my own) that I won't bother to explain here. So it's unfortunate but the problem as stated and presented, is really just bounded by the input size, the memory allocator, and a sort routine, not the actual meat of the problem.
