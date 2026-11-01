# Minimum Amount of Time to Fill Cups

You have a water dispenser that can dispense cold, warm, and hot water. Every second, you can either fill up `2` cups with different types of water, or `1` cup of any type of water.

You are given a **0-indexed** integer array `amount` of length `3` where `amount[0]`, `amount[1]`, and `amount[2]` denote the number of cold, warm, and hot water cups you need to fill respectively. Return the _**minimum** number of seconds needed to fill up all the cups_.

**Example 1:**

```txt
Input: amount = [1,4,2]
Output: 4
Explanation: One way to fill up the cups is:
Second 1: Fill up a cold cup and a warm cup.
Second 2: Fill up a warm cup and a hot cup.
Second 3: Fill up a warm cup and a hot cup.
Second 4: Fill up a warm cup.
It can be proven that 4 is the minimum number of seconds needed.
```

**Example 2:**

```txt
Input: amount = [5,4,4]
Output: 7
Explanation: One way to fill up the cups is:
Second 1: Fill up a cold cup, and a hot cup.
Second 2: Fill up a cold cup, and a warm cup.
Second 3: Fill up a cold cup, and a warm cup.
Second 4: Fill up a warm cup, and a hot cup.
Second 5: Fill up a cold cup, and a hot cup.
Second 6: Fill up a cold cup, and a warm cup.
Second 7: Fill up a hot cup.
```

**Example 3:**

```txt
Input: amount = [5,0,0]
Output: 5
Explanation: Every second, we fill up a cold cup.
```

**Constraints:**

- `amount.length == 3`
- `0 <= amount[i] <= 100`

## Solution

In order to get the fewest seconds needed to fill the cups, we want to fill the most demanded temperature(s) at every time step. This is important in the `[5,4,4]` example because if we naïvely fill the first two, we'll take an extra second to fill all the cups than we would if we always balance the most in-demand temperatures. This is moderately annoying and leads to a naïve solution that sorts the amounts every loop.

```rust
pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    let mut time = 0;
    while amount[0] + amount[1] + amount[2] > 0 {
        time += 1;
        let mut capacity = 2;
        amount.sort_unstable();
        if amount[0] > 0 && capacity > 0 {
            amount[0] -= 1;
            capacity -= 1;
        }
        if amount[1] > 0 && capacity > 0 {
            amount[1] -= 1;
            capacity -= 1;
        }
        if amount[2] > 0 && capacity > 0 {
            amount[2] -= 1;
        }
    }
    time
}
```

But surely we can do better, right? Based on the shape of the data, I'm wondering if there's a way we can get a general answer analytically instead of procedurally. Let's look at the data more by making a bunch of test cases and then recording some info about them.

| Input       | Output | y + z | (y + z) / 2 |
| ----------- | ------ | ----- | ----------- |
| `[3, 0, 0]` | 3      | 0     | 0           |
| `[3, 1, 0]` | 3      | 1     | 0           |
| `[3, 1, 1]` | 3      | 2     | 1           |
| `[3, 2, 1]` | 3      | 3     | 1           |
| `[3, 2, 2]` | 4      | 4     | 2           |
| `[3, 3, 2]` | 4      | 5     | 2           |
| `[3, 3, 3]` | 5      | 6     | 3           |
| `[3, 3, 4]` | 5      | 7     | 3           |
| `[4, 4, 3]` | 6      | 7     | 3           |
| `[5, 4, 4]` | 7      | 8     | 4           |

We're starting to see a pattern where we always have at least the max amount of seconds and then optionally more based on the other two values up to one of those values eclipsing the max. It seems to have to do with the remainder of a division by 2 because of the ability to fill up to 2 cups at once.

This helped me see that there are two bounds for the problem:

1. You can fill at most 1 cup of each temperature per second, so you need at least the largest individual count
2. You can fill at most 2 cups per second, so you need at least the `ceil(total / 2)` (treating `/` as integer division)

These two bounds show that we should take the `max` of these two conditions because they're "at least" bounds. Let's add the `total / 2`  to the table to ensure this meets the pattern we're already seeing. (Note the small difference using `+ 1` to do the `ceil` operation.)

| Input       | Output | y + z | (y + z) / 2 | (x + y + z + 1) / 2 |
| ----------- | ------ | ----- | ----------- | ------------------- |
| `[3, 0, 0]` | 3      | 0     | 0           | 2                   |
| `[3, 1, 0]` | 3      | 1     | 0           | 2                   |
| `[3, 1, 1]` | 3      | 2     | 1           | 3                   |
| `[3, 2, 1]` | 3      | 3     | 1           | 3                   |
| `[3, 2, 2]` | 4      | 4     | 2           | 4                   |
| `[3, 3, 2]` | 4      | 5     | 2           | 4                   |
| `[3, 3, 3]` | 5      | 6     | 3           | 5                   |
| `[3, 3, 4]` | 5      | 7     | 3           | 5                   |
| `[4, 4, 3]` | 6      | 7     | 3           | 6                   |
| `[5, 4, 4]` | 7      | 8     | 4           | 7                   |

That last column lines up with the answer in certain situations. Combining the two bounds, we can formulate a clear general analytic solution.

Say `z` is the max of `[x, y, z]`. This means `fill_cups([x, y, z]) = z` until `x + y > z` at which point it becomes `fill_cups([x, y, z]) = ceil((x + y + z) / 2)`. Expressed more rigorously,

<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mtable columnalign="left" rowspacing="1em">
    <mtr>
      <mtd>
        <mtext>For a given </mtext>
        <mo>[</mo><mi>x</mi><mo>, </mo><mi>y</mi><mo>, </mo><mi>z</mi><mo>]</mo>
        <mtext> sorted such that </mtext>
        <mi>x</mi><mo>≤</mo><mi>y</mi><mo>≤</mo><mi>z</mi><mo>:</mo>
      </mtd>
    </mtr>
    <mtr>
      <mtd>
        <mi>f</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>,</mo><mi>z</mi><mo>)</mo>
        <mo>=</mo>
        <mrow>
          <mo stretchy="true">{</mo>
          <mtable columnalign="left left" columnspacing="2em" rowspacing="0.5em">
            <mtr>
              <mtd><mi>z</mi></mtd>
              <mtd>
                <mtext>if </mtext>
                <mi>z</mi><mo>≥</mo><mi>x</mi><mo>+</mo><mi>y</mi>
              </mtd>
            </mtr>
            <mtr>
              <mtd>
                <mrow>
                  <mo>⌈</mo>
                  <mfrac>
                    <mrow><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>z</mi></mrow>
                    <mn>2</mn>
                  </mfrac>
                  <mo>⌉</mo>
                </mrow>
              </mtd>
              <mtd>
                <mtext>if </mtext>
                <mi>z</mi><mo>&lt;</mo><mi>x</mi><mo>+</mo><mi>y</mi>
              </mtd>
            </mtr>
          </mtable>
        </mrow>
      </mtd>
    </mtr>
  </mtable>
</math>

Or in the code

```rust
pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    amount.sort_unstable();
    amount[2].max((amount[0] + amount[1] + amount[2] + 1) / 2)
}
```

Which is pretty slick if I do say so myself.
