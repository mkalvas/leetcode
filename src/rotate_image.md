# Rotate Image

You are given an `n x n` 2D `matrix` representing an image, rotate the image by **90** degrees (clockwise).

You have to rotate the image [**in-place**](https://en.wikipedia.org/wiki/In-place_algorithm), which means you have to modify the input 2D matrix directly. **DO NOT** allocate another 2D matrix and do the rotation.

**Example 1:**

![example 1 before and after image rotation](https://assets.leetcode.com/uploads/2020/08/28/mat1.jpg)

```txt
Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [[7,4,1],[8,5,2],[9,6,3]]
```

**Example 2:**

![example 2 before and after image rotation](https://assets.leetcode.com/uploads/2020/08/28/mat2.jpg)

```txt
Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
```

**Constraints:**

- `n == matrix.length == matrix[i].length`
- `1 <= n <= 20`
- `-1000 <= matrix[i][j] <= 1000`

## Solution

For a given <math><mo>(</mo><mi>i</mi><mo>,</mo><mi>j</mi><mo>)</mo></math> pair, where <math><mi>i</mi></math> is the row and <math><mi>j</mi></math> is the column, we want to transform the value at that coordinate. Let's write down a `3x3` case and see if we can't glean the general formula from old position to new position.

| i   | j   | new i | new j |
| --- | --- | ----- | ----- |
| 0   | 0   | 0     | 2     |
| 0   | 1   | 1     | 2     |
| 0   | 2   | 2     | 2     |
| 1   | 0   | 0     | 1     |
| 1   | 1   | 1     | 1     |
| 1   | 2   | 2     | 1     |
| 2   | 0   | 0     | 0     |
| 2   | 1   | 1     | 0     |
| 2   | 2   | 2     | 0     |

Which brings us to the general formula <math><mi>f</mi><mo>(</mo><mi>i</mi><mo>,</mo><mi>j</mi><mo>)</mo><mo> = </mo><mo>(</mo><mi>j</mi><mo>,</mo><mo>max</mo> <mo>&#x2061;</mo><mo>(</mo><mi>j</mi><mo>)</mo><mo>-</mo><mi>i</mi><mo>)</mo></math>. If we imagine we _could_ allocate a new vector for this problem, the solution would look something like this.

```rust
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let max_j = matrix.len() - 1;
    let mut new = matrix.clone();
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            new[j][max_j - i] = matrix[i][j];
        }
    }
    new
}
```

But since we can't allocate, we need to do something a little smarter. At first, I wondered if we could just swap the current <math><mo>(</mo><mi>i</mi><mo>,</mo><mi>j</mi><mo>)</mo></math> with its destination, but this is obviously incorrect.

So after looking at it for a little bit, I visually realized that if we swap rows for columns and then reverse each row, we end up with the right thing. The only remaining gotcha is that we have to not run over the indices for positions we already swapped. We only need to do the "upper half triangle" of the matrix, which we can do by using the range `for j in i..matrix.len()`.

```txt
[1] [2] [3] -> 1 swaps with 1  2 swaps with 4  3 swaps with 7
 4  [5] [6] -> don't re-swap   5 swaps with 5  6 swaps with 8
 7   8  [9] -> don't re-swap   don't re-swap   9 swaps with 9
```

But this shows us a further optimization we can make by restricting the range to `for j in (i+1)..matrix.len()`

```txt
 1  [2] [3] -> skip diagonal  2 swaps with 4  3 swaps with 7
 4   5  [6] -> don't re-swap  skip diagonal   6 swaps with 8
 7   8   9  -> don't re-swap  don't re-swap   skip diagonal
```

This gives us a working solution.

```rust
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        for j in (i + 1)..matrix.len() {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }

    for row in matrix {
        row.reverse();
    }
}
```

I'm aware of the various "swap two variables without a temporary variable" tricks that are available to us here, but... no thanks.

Unfortunately, `clippy` is letting us down on this solution. It triggers the [`clippy::needless_range_loop`](https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#needless_range_loop) lint even though we do in fact need the `j` index specifically to do the swap. There's a way around this by splitting the rows and zipping the pairs.

```rust
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    for j in 1..matrix.len() {
        let (above, below) = matrix.split_at_mut(j);
        for (row, cell) in above.iter_mut().zip(&mut below[0]) {
            std::mem::swap(&mut row[j], cell);
        }
    }

    for row in matrix {
        row.reverse();
    }
}
```

In my opinion, this reads terribly compared to the previous one which clearly shows the iteration and swap.

So we could include a `#[allow(clippy::needless_range_loop)]` right above the `for j` loop. But that feels like a failure to me for some reason even though it's clearly a false positive.

Our last option is to do the looping the same, but then get the two rows in a `get_disjoint_mut` which also then allows us to do a true `std::mem::swap` instead of the temp variable dance.

```rust
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        for j in (i + 1)..matrix.len() {
            let [row_i, row_j] = matrix
                .get_disjoint_mut([i, j])
                .expect("index in range by loop construction");
            std::mem::swap(&mut row_i[j], &mut row_j[i]);
        }
    }

    for row in matrix {
        row.reverse();
    }
}
```

I'll let you be the judge of which of these versions you'd prefer to read.
