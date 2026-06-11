# Google L4 — Merged Drill (12 done / 15 to go)

Your checked progress folded into the core list. `[x]` = you've already cleared it. Re-solve each at least once more before the loop — re-solving beats first-solving.

## Graphs (9)

- [x] **Number of Islands** — grid DFS/BFS foundation
- [x] **Rotting Oranges** — multi-source BFS
- [x] **Course Schedule** — cycle detection
- [x] **Course Schedule II** — topological order; the "emit the sequence" extension of the one above (nearly free now)
- [x] **Alien Dictionary** — topo sort, harder variant with parsing + cycle detection
- [x] **Evaluate Division** — weighted-edge graph, DFS with a running product
- [ ] **Accounts Merge** — Union-Find
- [ ] **Cheapest Flights Within K Stops** — Dijkstra / bounded Bellman-Ford
- [x] **Word Ladder** — BFS as shortest transformation

## Parsing / Stack (2)

- [x] **Decode String** — recursive/stack evaluation
- [ ] **Basic Calculator II** — operator precedence with a stack

## Arrays & Strings (4)

- [x] **Longest Substring Without Repeating** — sliding window
- [x] **3Sum** — sort + two pointers + dedup
- [x] **Product of Array Except Self** — prefix products
- [x] **Subarray Sum Equals K** — prefix sum + hashmap

## Intervals (2)

- [x] **Merge Intervals** — sort by start, fold
- [x] **Meeting Rooms II** — sweep line / min-heap

## Sliding Window / Monotonic (1)

- [x] **Sliding Window Maximum** — monotonic deque

## Trees (3)

- [ ] **Lowest Common Ancestor of a Binary Tree**
- [ ] **Binary Tree Maximum Path Sum** — tree DP with an accumulator
- [ ] **Serialize and Deserialize Binary Tree**

## Heap / Linked List (1)

- [ ] **Merge k Sorted Lists** — k-way merge with a heap (custom `Reverse` wrapper)

## Binary Search (2)

- [ ] **Search in Rotated Sorted Array** — binary search on a pivoted array
- [ ] **Koko Eating Bananas** — binary search on the answer

## Dynamic Programming (3)

- [ ] **Coin Change** — unbounded knapsack
- [ ] **Russian Doll Envelopes** — LIS in 2D
- [ ] **Edit Distance** — 2D string DP

---

## Where the remaining 15 sit

You've cleared the array/string/interval core and the graph-traversal basics. What's left clusters into exactly the harder-to-fake areas: **topological output + weighted graphs + Union-Find** (5 problems), **all of trees** (3), **binary search** (2), and **all of DP** (3). Spend the rest of the week there.

## Get these four patterns _cold_

- [ ] Union-Find (path compression + union by size) — your only fully-unchecked core graph pattern
- [ ] Topological sort (Kahn's + cycle detection) — you've done the boolean; drill emitting the order
- [ ] Dijkstra (with stale-entry skip)
- [ ] Binary-search-on-the-answer

## Daily habits the "cleared" reports all mention

- [ ] Clarify ambiguity **before** writing code
- [ ] State time/space complexity at the end, unprompted
- [ ] Dry-run line by line before declaring done
- [ ] Prep 5–6 STAR stories (collaboration, failure/learning, ambiguity, respectful disagreement)
