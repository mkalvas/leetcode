# Mike's `leetcode` solutions

This is a repo for working on and saving all my leetcode problems and solutions outside of the leetcode walled garden.

## How to do a problem

I've been using the [`leetrs`](https://github.com/shadowmkj/leetrs) cli to orchestrate this and make it easy. Here are the steps for solving a problem:

1. `leetrs auth` with a valid login toke in your browser for leetcode.
2. `leetrs` for tui mode to browse files in TUI mode or use the browser to find a problem you want to do. Note down the problem's `SLUG`.
3. `cd src`
4. `leetrs pick <SLUG>`
5. Add `mod <SLUG_SNAKE_CASE>;` to `lib.rs`
6. `leetrs test <SLUG>.<LANG_EXT>`
7. `leetrs submit <SLUG>.<LANG_EXT>`
