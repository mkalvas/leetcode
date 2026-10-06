# Prime Number of Set Bits in Binary Representation

Given two integers `left` and `right`, return _the **count** of numbers in the **inclusive** range_ `[left, right]` _having a **prime number of set bits** in their binary representation_.

Recall that the **number of set bits** an integer has is the number of `1`'s present when written in binary.

- For example, `21` written in binary is `10101`, which has `3` set bits.

**Example 1:**

```txt
Input: left = 6, right = 10
Output: 4
Explanation:
6  -> 110 (2 set bits, 2 is prime)
7  -> 111 (3 set bits, 3 is prime)
8  -> 1000 (1 set bit, 1 is not prime)
9  -> 1001 (2 set bits, 2 is prime)
10 -> 1010 (2 set bits, 2 is prime)
4 numbers have a prime number of set bits.
```

**Example 2:**

```txt
Input: left = 10, right = 15
Output: 5
Explanation:
10 -> 1010 (2 set bits, 2 is prime)
11 -> 1011 (3 set bits, 3 is prime)
12 -> 1100 (2 set bits, 2 is prime)
13 -> 1101 (3 set bits, 3 is prime)
14 -> 1110 (3 set bits, 3 is prime)
15 -> 1111 (4 set bits, 4 is not prime)
5 numbers have a prime number of set bits.
```

**Constraints:**

- `1 <= left <= right <= 10^6`
- `0 <= right - left <= 10^4`

## Solution

This is a surprisingly deep problem even though it's very simple to get a satisfactory solution working.

First the obvious way. Convert each number to a binary string representation and count the number of `'1'` characters in it.

```rust
const PRIMES: [usize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut count = 0;
    for n in left..=right {
        let ones = format!("{n:b}").chars().filter(|c| *c == '1').count();
        if PRIMES.contains(&ones) {
            count += 1;
        }
    }
    count
}
```

Note that we only have to check if the number of `'1'`s in the string is in the set `{2, 3, 5, 7, 11, 13, 17, 19}` because `10^9 = 1_000_000 = 0b11110100001001000000` whose length is `20`. So given our constraints, we can't have more than `20` bits and therefore can't count higher than `20` `'1'`s.

But this is unsatisfying. Surely there's something more direct we can do using bitwise operations or something.

I wonder if it would be faster to bitwise `&` each digit from 1–20 and then compare against zero each time to get the count of ones.

```rust
const PRIMES: [usize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

const DIGITS: [i32; 20] = [
    0b1,
    0b10,
    0b100,
    0b1000,
    0b10000,
    0b100000,
    0b1000000,
    0b10000000,
    0b100000000,
    0b1000000000,
    0b10000000000,
    0b100000000000,
    0b1000000000000,
    0b10000000000000,
    0b100000000000000,
    0b1000000000000000,
    0b10000000000000000,
    0b100000000000000000,
    0b1000000000000000000,
    0b10000000000000000000,
];

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut count = 0;
    for n in left..=right {
        let mut ones = 0;
        for d in DIGITS {
            if d & n != 0 {
                ones += 1;
            }
        }

        if PRIMES.contains(&ones) {
            count += 1;
        }
    }
    count
}
```

This _is_ faster, but it's not that fast still and feels hacky for some reason. Let's see if there's even a better way.

---

I feel a bit stupid for missing this too but there's a built in method for counting ones in rust that we could use.

```rust
pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut count = 0;
    for n in left..=right {
        if PRIMES.contains(&n.count_ones()) {
            count += 1;
        }
    }
    count
}
```

Or more idiomatically

```rust
pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    (left..=right)
        .filter(|n| PRIMES.contains(&n.count_ones()))
        .count() as i32
}
```

But I'm still unsatisfied that I couldn't find a fast way to do this without the built-in method.

---

Rethinking the bit counting, we could also do a loop over the number shifting it by one each time and counting that way instead of pre-registering the `DIGITS` array. This would make the solution generalizable to any integer width, and possibly (I didn't measure) make it faster.

```rust
pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut count = 0;
    for mut n in left..=right {
        let mut ones = 0;
        while n > 0 {
            if n & 1 != 0 {
                ones += 1;
            }
            n = n >> 1;
        }

        if PRIMES.contains(&ones) {
            count += 1;
        }
    }
    count
}
```

---

After hunting around and looking at the source code for the `count_ones` method in rust it's actually lowering directly to an [`llvm.ctpop.*`](https://llvm.org/docs/LangRef.html#llvm-ctpop-intrinsic) intrinsic. The `ctpop` stands for "count population" which means "count the number of populated bits in an integer". This is an intrinsic even in llvm and it, in turn, lowers to an assembly `popcount` instruction depending on the integer types you're using and which architecture you're on. This means there's physical silicon computing the `popcount` at that point and there's no way we're going to beat that performance in code.

There is one more thing we could do here though to remove the `PRIMES` lookup. First we construct a number that has the bits corresponding to the **counts** we're looking for.

```txt
bit:  19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
       1  0  1  0  0  0  1  0  1  0  0  0  1  0  1  0  1  1  0  0

= 10100010100010101100
= 665772
```

Then we take this number and shift it by the number of ones in our integer and check if the number of shifts results in one of these prime bits being in the `1`s place. We can do that by `result & 1 == 1`. For example,

```txt
(0b10100010100010101100 >> ones) & 1 == 1
(0b10100010100010101100 >> 3) & 1 == 1
(0b10100010100010101) & 0b00000000000000001 == 1
1 == 1 -> true
```

This gets rid of the array scan lookup in the hot loop.

```rust
// Bits for primes at positions 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_MASK: u32 = 0b10100010100010101100;

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    (left..=right)
        .filter(|n| (PRIME_MASK >> n.count_ones()) & 1 == 1)
        .count() as i32
}
```

And for one last "machine sympathy speedup", we can simply add the result of the shift and bitwise `&` to the count because it will be `0` if it's not a prime and `1` if it is. This _should_ allow the CPU to pipeline faster, not do an integer conversion, and vectorize more aggressively.

```rust
// Bits for primes at positions 2, 3, 5, 7, 11, 13, 17, 19
const PRIME_MASK: i32 = 0b10100010100010101100;

pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    (left..=right)
        .map(|n| (PRIME_MASK >> n.count_ones()) & 1)
        .sum()
}
```

This is probably as good as we can get on this one. That was a pretty cool exploration of intrinsics, loop optimizations, and bitwise operations.
