# Sum of Floored Pairs

Given an integer array `nums`, return the sum of `floor(nums[i] / nums[j])` for all pairs of indices `0 <= i, j < nums.length` in the array. Since the answer may be too large, return it **modulo** `10<sup>9</sup> + 7`.

The `floor()` function returns the integer part of the division.

**Example 1:**

```txt
Input: nums = [2,5,9]
Output: 10
Explanation:
floor(2 / 5) = floor(2 / 9) = floor(5 / 9) = 0
floor(2 / 2) = floor(5 / 5) = floor(9 / 9) = 1
floor(5 / 2) = 2
floor(9 / 2) = 4
floor(9 / 5) = 1
We calculate the floor of the division for every pair of indices in the array then sum them up.
```

**Example 2:**

```txt
Input: nums = [7,7,7,7,7,7,7]
Output: 49
```

**Constraints:**

- `1 <= nums.length <= 10<sup>5</sup>`
- `1 <= nums[i] <= 10<sup>5</sup>`

## Other solutions

Obviously this works but it's too slow.

```rust
const MODULO: i64 = 1_000_000_007;

pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
    let mut sum: i64 = 0;
    for i in &nums {
        for j in &nums {
            sum += i / j;
        }
    }
    (sum % MODULO) as i32
}
```

So what's the trick? When we look at one denominator `j` we know that the `floor`-ed value will be constant for a range of numerators. For instance, with `nums = [1, 2, 3, 10, 11, 12, 13, 20]` and `j = 10` we have `[1, 2, 3]`, `[10, 11, 12, 13]`, and `[20]` as the buckets corresponding to `0`, `1`, and `2`. So the sum (for only `j = 10`) is `0*3 + 1*4 + 2*1`. This also shows us that we can sort and ignore all numbers `i < j`.

I started with a frequency map, knowing that it probably wouldn't be fast enough (e.g., does it help when all numbers are unique?).

```rust
pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
    let mut freqs = HashMap::new();
    let mut max = 1;
    for n in &nums {
        freqs.entry(*n).and_modify(|f| *f += 1).or_insert(1);
        if n > &max {
            max = *n;
        }
    }

    let mut sum: i64 = 0;
    for n in &nums {
        for block in 1..=(max / n) {
            let left: i32 = block * n;
            let right: i32 = (block + 1) * n - 1;
            for (key, value) in freqs.iter() {
                if &left <= key && key <= &right {
                    // println!("n `{n}` cmp key `{key}` in [{left}, {right}] block number {block}");
                    sum += (block * *value) as i64;
                }
            }
        }
    }

    (sum % MODULO) as i32
}
```

The next thing we need to do is to take out the inner hot loop. If we ask ourselves, is there a faster way to look up the sum of an array from `i..j`, we recall the pattern we're searching for — a prefix sum array.

We can build that for `1..maxNum` with

```rust
let mut prefixes = vec![0];
for i in 1..=max {
    prefixes.push(prefixes[(i - 1) as usize] + freqs.get(&i).unwrap_or(&0));
}
```

and then use that like usual to find the sum of our range `[bi, bj - 1]`

```rust
sum += block * (prefixes[right as usize] - prefixes[(left - 1) as usize]);
```

and putting it all together, we get our final answer.

```rust
pub fn sum_of_floored_pairs_hashmap(nums: Vec<i32>) -> i32 {
    let nums: Vec<i64> = nums.iter().map(|n| *n as i64).collect();

    let mut freqs = HashMap::new();
    let mut max = 1;
    for n in &nums {
        freqs.entry(n).and_modify(|f| *f += 1).or_insert(1);
        if n > &max {
            max = *n;
        }
    }

    let mut prefixes = vec![0];
    for i in 1..=max {
        prefixes.push(prefixes[(i - 1) as usize] + freqs.get(&i).unwrap_or(&0));
    }

    let mut sum: i64 = 0;
    for (n, copies) in freqs.iter() {
        let mut sum_for_n = 0;
        for block in 1..=(max / *n) {
            let left = block * *n;
            let right = i64::min((block + 1) * *n - 1, max);
            sum_for_n += block * (prefixes[right as usize] - prefixes[(left - 1) as usize]);
        }
        sum += sum_for_n * copies;
    }

    (sum % MODULO) as i32
}
```

This version landed us in the middle of the performance curve on submissions so I went digging for improvements.

- The `nums` into `i64` is wasteful and I just wanted that to not have to do as many casts as I was working on the problem. Removed that.
- The `HashMap` is unnecessary. We can just use an array for frequencies with the index as the "hash key".
- We have two data structures (`freqs` and `prefixes`) when we can compute the prefix sums in place after computing the frequencies. Importantly, we can still recover the number of `copies` for a number by `prefixes[i] - prefixes[i - 1]` for later.
- At the cost of a little readability, we can get some cache efficiencies by checking sequentially over all `1..=max` and skipping out if there are no copies. This _checks_ more iterations but the sequential cache efficiencies are worth it and the early `if copy == 0` bailout is highly predictable to the CPU cache.
- We can also rewrite our `left` and `right` `block` bounds iterations to use addition instead of multipllication which is a small but worthwhile win at our current depths of optimization.

The best solution I could come up with without going crazy on things is in the code file.
