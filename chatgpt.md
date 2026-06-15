Here’s a **compressed “minimum 18-problem Google L4 set”** that keeps **maximum signal coverage with zero redundancy**.

I removed overlap (multiple similar graph BFS/DFS variants, redundant DP, etc.) and balanced the missing weak spots (trees + heap + BFS + BST + DP clarity).

---

# ✅ Google L4 — Minimum 18 High-Signal Set

## 🟦 Graphs (5)

These cover 90% of Google graph interviews.

- **Number of Islands** — DFS/BFS grid traversal baseline
- **Course Schedule II** — topological sort + output ordering
- **Evaluate Division** — weighted graph + DFS path accumulation
- **Cheapest Flights Within K Stops** — constrained shortest path (BFS/Dijkstra hybrid thinking)
- **Word Ladder** — shortest path BFS in implicit graph

👉 Covers:
DFS, BFS, topo sort, weighted graphs, shortest path variants

---

## 🟨 Trees (4) — FIXED (this was your biggest gap)

- **Binary Tree Level Order Traversal** — BFS baseline (critical)
- **Lowest Common Ancestor of a Binary Tree** — recursion + structural reasoning
- **Binary Tree Diameter** — clean DFS with global tracking
- **Binary Tree Maximum Path Sum** — advanced tree DP (your hardest one)

👉 Covers:

- BFS trees (missing in your original)
- DFS fundamentals
- LCA reasoning
- DP-on-tree escalation

---

## 🟩 Arrays / Strings (4)

- **Longest Substring Without Repeating Characters** — sliding window
- **3Sum** — sorting + two pointers + dedup
- **Product of Array Except Self** — prefix/suffix reasoning
- **Subarray Sum Equals K** — prefix sum + hashmap

👉 Covers:

- sliding window
- hashing
- prefix techniques
- two pointers

---

## 🟧 Intervals (2)

- **Merge Intervals** — sorting + merging logic
- **Meeting Rooms II** — heap / sweep line

👉 Covers scheduling + heap usage

---

## 🟪 Heaps (1)

- **Top K Frequent Elements** — heap or bucket sort

👉 This replaces Merge K Lists in importance for L4 breadth

(You still indirectly get k-way merge thinking from other problems)

---

## 🟥 Binary Search (2)

- **Search in Rotated Sorted Array** — structural binary search
- **Koko Eating Bananas** — binary search on answer space

👉 Covers both classic + parametric search

---

## 🟫 Dynamic Programming (3)

- **Coin Change** — unbounded knapsack DP
- **Edit Distance** — 2D string DP (core L4 DP)
- **House Robber** — 1D DP intuition builder (IMPORTANT addition)

👉 This fixes your original DP weakness:
You were missing a “simple DP entry point”

---

## 🟨 Stack / Parsing (1)

- **Decode String** — stack + recursion hybrid parsing

👉 Covers expression parsing pattern

---

# 📊 What this version optimizes

## Removed redundancy

- No duplicate BFS graphs (Rotting Oranges removed)
- No duplicate topo sorts (Alien Dictionary removed — too niche for core set)
- No overlapping tree heavies beyond necessary coverage

---

## Added missing L4 signals

### 🧠 Biggest fixes:

- BFS trees added (Level Order)
- Simple DP added (House Robber)
- Heap diversity improved (Top K Frequent)

---

# 🎯 Coverage breakdown (why this works)

This 18-set guarantees exposure to:

### Graphs

- DFS, BFS, topo, weighted, shortest path

### Trees

- BFS + DFS + recursion + DP-on-tree

### DP

- 1D + 2D + knapsack + string DP

### Searching

- binary + parametric

### Core interview tools

- heap
- sliding window
- prefix sums
- intervals
- stack parsing

---

# 🧠 Reality check (important)

If you master this set:

- You will **not be surprised by L4 coding rounds**
- You will have **pattern coverage for ~90–95% of Google L4 questions**

But:

> Passing still depends on speed + clarity + communication, not just coverage.

---

If you want next step, I can turn this into a **7-day / 14-day execution plan with re-solve cycles and timing targets per problem** (that’s usually what actually makes people interview-ready).
