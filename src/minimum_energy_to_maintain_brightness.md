# Minimum Energy to Maintain Brightness

You are given an integer `n`, representing `n` light bulbs arranged in a line and indexed from 0 to `n - 1`.

You are also given an integer `brightness` and a 2D integer array `intervals`, where `intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]` represents an **inclusive** time interval during which the lighting requirement **must** be satisfied.

At each time unit, every bulb can independently be either on or off. A bulb that is on **illuminates** its own position and its **adjacent** positions, if they exist.

The **total illumination** at a time unit is the number of **illuminated** positions. Each position is counted **at most once**.

For every integer time unit covered by **at least** one interval in `intervals`, the **total illumination** must be **at least** `brightness`. At time units not covered by any interval, all bulbs may remain off. Each bulb that is on consumes 1 unit of energy for that time unit.

Return an integer denoting the **minimum** total energy required.

**Example 1:**

> **Input:** n = 5, brightness = 5, intervals = [[6,12]]
>
> **Output:** 14
>
> **Explanation:**
>
> - Turn on the light bulbs at positions 1 and 4.
> - Current state of line: `0 1 0 0 1`.
> - All 5 positions are illuminated, so the required brightness is reached.
> - The active interval has length `12 - 6 + 1 = 7`, so the total energy is `2 * 7 = 14`.

**Example 2:**

> **Input:** n = 2, brightness = 1, intervals = [[0,0],[2,2]]
>
> **Output:** 2
>
> **Explanation:**
>
> - Turn on one light bulb during each active interval.
> - Each interval has length 1, so the total active time is `1 + 1 = 2`.
> - The total energy is `1 * 2 = 2`.

**Example 3:**

> **Input:** n = 4, brightness = 2, intervals = [[1,3],[2,4]]
>
> **Output:** 4
>
> **Explanation:**
>
> - Turn on one light bulb. It can illuminate at least 2 positions.
> - The active intervals overlap, so the total active time is the length of `[1,4]`, which is 4.
> - The total energy is `1 * 4 = 4`.

**Constraints:**

- `1 <= n <= 10<sup>6</sup>`
- `1 <= brightness <= n`
- `1 <= intervals.length <= 10<sup>5</sup>`
- `intervals[i] == [start<sub>i</sub>, end<sub>i</sub>]`
- `0 <= start<sub>i</sub> <= end<sub>i</sub> <= 10<sup>9</sup>`

## Other solutions

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

Correct but too slow for adversarial intervals (note that `brightness == 0` is incorrect but disallowed by the problem statement). So we need to do the intervals in a smarter way. Since all we need is the number of integers in the merged intervals we can just count them directly. This is a similar approach as something called a last merged interval. We need to pre-sort the array by the beginning of each interval. This guarantees that the starts of each interval will always be non-decreasing, and therefore we can look at the end of the previously merged interval to see if we need to merge this one too, or account for a gap.

```rust
pub fn min_energy(_n: i32, brightness: i32, mut intervals: Vec<Vec<i32>>) -> i64 {
    intervals.sort_unstable_by_key(|interval| interval[0]);

    let mut end_time = intervals[0][1];
    let mut on_times = i64::from(end_time - intervals[0][0] + 1);
    for interval in &intervals[1..] {
        let (start, end) = (interval[0], interval[1]);
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

There are a few other tweaks we can do to make it slightly faster but all within the same ballpark (see the final version in the code file).

We _can_ go further though. There's a branchless version of the core loop that looks like this

```rust
for &(start, end) in &intervals[1..] {
    let base = end_time.max(start - 1);
    on_times += i64::from((end - base).max(0));
    end_times = end_time.max(end);
}
```

Which the astute among you will realize still has branching in it since `max` desugars to `if a < b { b } else { a }` under the hood. The reason that this _becomes_ branchless is that the compiler can take that specific form of if statement and turn it into a `csel` instruction which is a **c**onditional-**sel**ect instruction. So this turns into a data flow, not a true control flow branch (which as it turns out are very different things to CPUs) meaning it can't trip on branch mispredictions (the thing you're really talking about when you talk about being "branchless").

But unfortunately this leetcode problem is actually not very well constructed. First of all we don't even need the `n` parameter in the function signature. Second, dropping the owned `intervals` `Vec` is the dominating performance bottleneck of the problem, followed distantly by the sort that's required for the intended solution. There aren't really ways around them. You could `mem::forget` the `Vec` but that's leaking memory and banking on the OS to clean up your intentional litter — not something you typically want to do. There's also an insane radix sort trick you can do (that I would never be able to come up with on my own) that I won't bother to explain here.
