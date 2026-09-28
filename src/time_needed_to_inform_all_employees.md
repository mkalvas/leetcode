# Time Needed to Inform All Employees

A company has `n` employees with a unique ID for each employee from `0` to `n - 1`. The head of the company is the one with `headID`.

Each employee has one direct manager given in the `manager` array where `manager[i]` is the direct manager of the `i-th` employee, `manager[headID] = -1`. Also, it is guaranteed that the subordination relationships have a tree structure.

The head of the company wants to inform all the company employees of an urgent piece of news. He will inform his direct subordinates, and they will inform their subordinates, and so on until all employees know about the urgent news.

The `i-th` employee needs `informTime[i]` minutes to inform all of his direct subordinates (i.e., After informTime[i] minutes, all his direct subordinates can start spreading the news).

Return _the number of minutes_ needed to inform all the employees about the urgent news.

**Example 1:**

```txt
Input: n = 1, headID = 0, manager = [-1], informTime = [0]
Output: 0
Explanation: The head of the company is the only employee in the company.
```

**Example 2:**

![graph representation of example 2](https://assets.leetcode.com/uploads/2020/02/27/graph.png)

```txt
Input: n = 6, headID = 2, manager = [2,2,-1,2,2,2], informTime = [0,0,1,0,0,0]
Output: 1
Explanation: The head of the company with id = 2 is the direct manager of all the employees in the company and needs 1 minute to inform them all.
The tree structure of the employees in the company is shown.
```

**Constraints:**

- `1 <= n <= 10^5`
- `0 <= headID < n`
- `manager.length == n`
- `0 <= manager[i] < n`
- `manager[headID] == -1`
- `informTime.length == n`
- `0 <= informTime[i] <= 1000`
- `informTime[i] == 0` if employee `i` has no subordinates.
- It is **guaranteed** that all the employees can be informed.

## Solution

Let's solve this in the most direct way possible first. We walk up the tree from every employee and compute their personal `inform_time` and then take the max of all of them.

```rust
pub fn num_of_minutes(_n: i32, _head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..manager.len() {
        let mut m = manager[i];
        let mut path_time = 0;
        while m != -1 {
            path_time += inform_time[m as usize];
            m = manager[m as usize];
        }
        if path_time > max {
            max = path_time;
        }
    }

    max
}
```

Somewhat to my surprise, this was accepted and didn't fall victim to any adversarial time complexity inputs. I did only beat 9% of other submissions though so we're on the very slow end. Let's see what we can do about that.

Immediately, I'm thinking that we can memoize or store subtree values. But I'm also thinking about how a depth-first search approach seems valid here too.

I went with the DFS first was because I realized that if we were to  memoize bottom-up, we'd have to go up the tree and then back down to do the memoization in the right direction. This was after I threw in a naïve implementation of memoization and realized I was memoizing the path _up_ to a certain manager instead of the path _down_ from the head to that manager. So it felt like it was going to be a bit of a faff to get it all sorted out and maybe not actually better. It turns out it's not that bad if we just store our path, but we'll get to that after the DFS version.

```rust
pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut max_time = 0;
    let mut stack: Vec<(usize, i32)> = vec![(head_id as usize, 0)];

    let mut children = vec![vec![]; n as usize];
    for (i, &id) in manager.iter().enumerate() {
        if id >= 0 {
            children[id as usize].push(i);
        }
    }

    while let Some((id, path_time)) = stack.pop() {
        let time = path_time + inform_time[id];
        max_time = max_time.max(time);
        stack.extend(children[id].iter().map(|i| (*i, time)));
    }

    max_time
}
```

The interesting piece here is that we pre-compute the `children` array, swapping the relationship from bottom-up to top-down so that we don't have to compute that on the fly in our hot-loop of the search.

This solution was faster, but not by a lot. It seems like we can do better so I'm going to check if the memoized version of our first, bottom-up solution is actually better.

```rust
pub fn num_of_minutes(
    _n: i32,
    _head_id: i32,
    mut manager: Vec<i32>,
    mut inform_time: Vec<i32>,
) -> i32 {
    let mut max = 0;
    let mut path = Vec::new();

    for i in 0..manager.len() {
        let mut j = i;
        while manager[j] != -1 {
            path.push(j);
            j = manager[j] as usize;
        }

        while let Some(m) = path.pop() {
            inform_time[m] += inform_time[manager[m] as usize];
            manager[m] = -1;
        }

        max = max.max(inform_time[i]);
    }

    max
}
```

Let's take it apart a bit since it took me a minute to figure it out.

First, we have our basic loop over employees in the `manager` array. Then we walk up the tree from the employee until the head, pushing each index into the `path` array as we go so we can reverse our course. Then, once we're at the head, we reverse and pop the child off the path of the array. Now comes the most important part of the algorithm — we _mutate_ the `inform_time` for the employee we're on with the top-down value of their path _and_ set their manager to `-1` so that we'll stop there if we come across it again in another path. This is the memoization step so that we do in fact short-circuit as we find repeated paths.

This was an interesting one today. I had a hard time finding the best solution, but as usual there was an obvious way to get to a correct answer even if it wasn't the fastest. It's a little surprising to me that we left the initial approach and circled back. Normally, I find that I need to do more capital-A "Algorithms" in order to get the fastest marks. This time it was the obvious one plus optimizations that beat out the "Algorithmic" one.

P.S. thinking about working at a company where there are ~50k employees in a nightmare tree of reporting chains with thousands of people in them and a minimum communication time of `274,189` minutes is the stuff of nightmares.
