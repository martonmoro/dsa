# Google L4 — Rust Interview Program (14-Day Sprint)

**Target:** Software Engineer III (L4), AI-Powered Rust, Munich
**Immediate goal:** pass the technical phone screen (1 medium-hard problem, ~45 min, plain Google Doc, no autocomplete, no compiler)
**Then:** onsite loop = ~3 coding rounds + 1 Googleyness round (system design is light/often absent at L4)

> Companion file: `rust_interview_harness.rs` — retype sections from it every morning before problems. The borrow checker, not the algorithm, is what costs Rust candidates time.

---

## Legend

- ⭐ **Core** — do these no matter what
- 📄 In Google's own published prep doc — treat as required
- 🎯 Extra-likely given this Rust / compiler / code-translation role
- Difficulty: **E** easy · **M** medium · **H** hard

---

## Rules of Engagement (read once, follow every session)

1. **No autocomplete, no compiler, no run button.** Use a plain text editor or paper. The real phone screen is a bare Google Doc. If you only ever practice in an IDE, the borrow checker will surprise you live.
2. **Timebox to 35–45 min per problem.** If stuck at 20 min, peek at the _idea_ only, then finish coding yourself. Note it for re-do.
3. **Talk out loud the whole time.** Google weights communication heavily; "clarity beats a rushed finish."
4. **Run the protocol every time** (see below). Don't jump to code.
5. **Write 2–3 test cases by hand** including edge cases before declaring done.
6. **Re-do, don't move on.** Any problem that took >45 min or needed a hint goes in the re-do column and gets repeated 2–3 days later.

### The 8-step protocol (rehearse until automatic)

1. Restate the problem in your own words.
2. Ask clarifying questions (input size, ranges, duplicates, empty/null, encoding).
3. Give 1–2 worked examples, including an edge case.
4. State a brute-force approach + its Big-O.
5. State the optimal approach + its Big-O. **Get the interviewer's buy-in before coding.**
6. Code it cleanly.
7. Walk one concrete test case through your code line by line.
8. Discuss edge cases, then follow-ups (scale, concurrency, what changes if…).

---

## The Full Problem Set (by pattern)

### 1. Graphs & Grids — highest frequency at Google

- [x] ⭐ Number of Islands (M)
- [ ] Flood Fill (E)
- [ ] Max Area of Island (M)
- [ ] ⭐ Rotting Oranges (M) — multi-source BFS
- [ ] 01 Matrix / Walls and Gates (M)
- [ ] Clone Graph (M)
- [ ] ⭐🎯 Course Schedule (M) — cycle detection / topo sort = dependency resolution
- [ ] 🎯 Course Schedule II (M) — emit a topological order
- [ ] 🎯 Alien Dictionary (H) — topo sort + parsing (very on-theme)
- [ ] Pacific Atlantic Water Flow (M)
- [ ] ⭐ Word Ladder (H) — BFS shortest path
- [ ] Number of Provinces (M) — Union-Find
- [ ] Redundant Connection (M) — Union-Find
- [ ] Graph Valid Tree (M)
- [ ] Network Delay Time (M) — Dijkstra
- [ ] Cheapest Flights Within K Stops (M)

### 2. Strings, Parsing & Stacks — Google loves these; on-theme for this role

- [ ] ⭐📄🎯 Decode String (M) — `3[abc]` → `abcabcabc`; **the first example in Google's doc**
- [ ] 🎯 Basic Calculator II (M) — expression evaluation
- [ ] 🎯 Basic Calculator (H) — with parentheses
- [ ] Valid Parentheses (E)
- [ ] Minimum Remove to Make Valid Parentheses (M)
- [ ] ⭐ Longest Substring Without Repeating Characters (M) — sliding window
- [ ] 📄 Longest Word in Dictionary through Deleting (M) — subsequence; **in Google's doc**
- [ ] Group Anagrams (M)
- [ ] Valid Anagram (E)
- [ ] 🎯 String to Integer / atoi (M) — careful parsing
- [ ] 🎯 Evaluate Reverse Polish Notation (M) — stack

### 3. Arrays / Two Pointers / Sliding Window / Intervals

- [ ] Two Sum (E)
- [ ] ⭐ 3Sum (M)
- [ ] Container With Most Water (M)
- [ ] Trapping Rain Water (H)
- [ ] ⭐ Product of Array Except Self (M)
- [ ] Maximum Subarray (M) — Kadane
- [ ] ⭐ Subarray Sum Equals K (M) — prefix-sum + hashmap
- [ ] ⭐ Merge Intervals (M)
- [ ] Insert Interval (M)
- [ ] Non-overlapping Intervals (M) — greedy
- [ ] ⭐ Meeting Rooms II (M) — heap of end times
- [ ] Minimum Window Substring (H)
- [ ] Sliding Window Maximum (H) — monotonic deque

### 4. Trees

- [ ] Maximum Depth of Binary Tree (E)
- [ ] Diameter of Binary Tree (E/M)
- [ ] Invert Binary Tree (E)
- [ ] Binary Tree Level Order Traversal (M)
- [ ] Binary Tree Right Side View (M)
- [ ] ⭐ Lowest Common Ancestor (BST then Binary Tree) (M)
- [ ] ⭐ Validate Binary Search Tree (M)
- [ ] Kth Smallest Element in a BST (M)
- [ ] Construct Binary Tree from Preorder and Inorder (M)
- [ ] ⭐ Binary Tree Maximum Path Sum (H)
- [ ] ⭐ Serialize and Deserialize Binary Tree (H)

### 5. Heaps / Top-K

- [ ] ⭐ Kth Largest Element in an Array (M)
- [ ] ⭐ Top K Frequent Elements (M)
- [ ] ⭐ Merge k Sorted Lists (H)
- [ ] Find Median from Data Stream (H) — two heaps
- [ ] K Closest Points to Origin (M)
- [ ] Task Scheduler (M)

### 6. Binary Search

- [ ] Binary Search (E)
- [ ] ⭐ Search in Rotated Sorted Array (M)
- [ ] Find First and Last Position of Element (M)
- [ ] Find Minimum in Rotated Sorted Array (M)
- [ ] ⭐ Koko Eating Bananas (M) — binary search on the answer
- [ ] Split Array Largest Sum (H) — binary search on the answer
- [ ] Median of Two Sorted Arrays (H) — _stretch_

### 7. Dynamic Programming — the round that fails L4 candidates; drill to fluency

- [ ] Climbing Stairs (E)
- [ ] House Robber (M)
- [ ] ⭐ Coin Change (M)
- [ ] Coin Change II (M)
- [ ] ⭐ Word Break (M) 🎯 — DP + parsing
- [ ] Decode Ways (M)
- [ ] Unique Paths (M)
- [ ] Maximum Product Subarray (M)
- [ ] Partition Equal Subset Sum (M)
- [ ] ⭐ Longest Increasing Subsequence (M)
- [ ] Russian Doll Envelopes (H) — box-nesting variant **reported as a real L4 question**
- [ ] Longest Common Subsequence (M)
- [ ] ⭐🎯 Edit Distance (H) — string-to-string transformation, almost literally this job

### 8. Backtracking

- [ ] Subsets (M)
- [ ] Permutations (M)
- [ ] Combination Sum (M)
- [ ] Generate Parentheses (M)
- [ ] ⭐ Word Search (M) — grid backtracking
- [ ] Palindrome Partitioning (M)
- [ ] Letter Combinations of a Phone Number (M)

### 9. Trie (role-adjacent)

- [ ] Implement Trie (M)
- [ ] Design Add and Search Words Data Structure (M)
- [ ] Word Search II (H) — _stretch_

### 10. Design (sometimes appears)

- [ ] ⭐ LRU Cache (M) — **brutal in safe Rust; practice it specifically** (`HashMap` + `Rc<RefCell>` or a `Vec` arena of indices)
- [ ] Min Stack (E/M)

---

## If You're Short on Time: the Core 20

If life eats a week, these are the highest-yield, do-them-or-die set:

Number of Islands · Rotting Oranges · Course Schedule II · Word Ladder · **Decode String** · Longest Substring Without Repeating · 3Sum · Product of Array Except Self · Subarray Sum Equals K · Merge Intervals · Validate BST · LCA · Binary Tree Maximum Path Sum · Serialize/Deserialize Tree · Top K Frequent · Merge k Sorted Lists · Search in Rotated Sorted Array · Koko Eating Bananas · Coin Change · **Edit Distance**

---

## The 14-Day Schedule

Each day: **retype 2 harness sections from memory** → solve the day's problems under timer → log re-dos.

| Day    | Focus                               | Problems                                                                                                                                                               |
| ------ | ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1**  | Rust fluency warm-up                | Set up harness project + retype it once. Then: Two Sum, Valid Parentheses, Valid Anagram, Climbing Stairs, Maximum Depth. _Goal: write Rust without a compiler._       |
| **2**  | Arrays / two pointers               | 3Sum, Container With Most Water, Product of Array Except Self, Maximum Subarray, Longest Substring Without Repeating                                                   |
| **3**  | Sliding window / intervals          | Subarray Sum Equals K, Merge Intervals, Insert Interval, Meeting Rooms II, Minimum Window Substring                                                                    |
| **4**  | Graphs/grids I (BFS/DFS)            | Number of Islands, Flood Fill, Max Area of Island, Rotting Oranges, 01 Matrix                                                                                          |
| **5**  | Graphs II (topo + BFS path)         | Clone Graph, Course Schedule, Course Schedule II, Pacific Atlantic, Word Ladder                                                                                        |
| **6**  | Graphs III (DSU + Dijkstra)         | Number of Provinces, Redundant Connection, Graph Valid Tree, Network Delay Time, Alien Dictionary                                                                      |
| **7**  | Strings / parsing / stack 🎯        | **Decode String**, Basic Calculator II, Minimum Remove to Make Valid Parens, Eval RPN, Longest Word through Deleting                                                   |
| **8**  | Trees I                             | Diameter, Invert, Level Order, Right Side View, Lowest Common Ancestor                                                                                                 |
| **9**  | Trees II                            | Validate BST, Kth Smallest in BST, Construct from Preorder/Inorder, Binary Tree Max Path Sum, Serialize/Deserialize                                                    |
| **10** | DP I                                | House Robber, Coin Change, Word Break, Decode Ways, Unique Paths                                                                                                       |
| **11** | DP II 🎯                            | Longest Increasing Subsequence, Longest Common Subsequence, **Edit Distance**, Partition Equal Subset Sum, Maximum Product Subarray (+ Russian Doll Envelopes if time) |
| **12** | Heaps + binary search               | Kth Largest, Top K Frequent, Merge k Sorted Lists, Search in Rotated Sorted Array, Koko Eating Bananas                                                                 |
| **13** | Backtracking + design + **Mock #1** | Subsets, Permutations, Word Search, Generate Parentheses, LRU Cache → then **1 full timed mock** (45 min, plain doc, talk aloud)                                       |
| **14** | **Mock #2** + review + behavioral   | 1 full timed mock → re-do 2–3 problems that hurt → light review of harness → draft 4 STAR stories → **rest**                                                           |

Weekend days, if you have more hours, pull extra problems from the unsolved pool or from the categories you scored worst on.

---

## Rust Drills to Bake In (from the harness)

Do these blind, no IDE, until automatic — these are the syntax traps that eat clock time:

- [ ] String → `Vec<char>` or `as_bytes()` (because `s[i]` doesn't compile)
- [ ] `HashMap::entry(k).or_insert(0)` / `.or_default()` counting
- [ ] `VecDeque` BFS for both grid and `Vec<Vec<usize>>` graph
- [ ] **Index-based graph** (`Vec<Vec<usize>>`) — never node objects
- [ ] `BinaryHeap` max-heap + `Reverse` min-heap + heap of `(priority, payload)`
- [ ] Tree as `Option<Box<TreeNode>>` with `&self` recursion (+ the `Rc<RefCell>` variant for LeetCode signatures)
- [ ] Linked list reversal with `.take()`
- [ ] `lower_bound` by hand + binary-search-on-answer template
- [ ] Backtracking push/recurse/pop
- [ ] 2D DP init `vec![vec![0; m]; n]` + Edit Distance
- [ ] Union-Find struct with path compression
- [ ] Overflow-safe `mid = lo + (hi - lo) / 2`; widen to `i64` before multiplying

**Live tactic:** when the borrow checker fights you, `clone()` to keep moving and say aloud "I'd remove this allocation in production." For _this_ role, clean idiomatic Rust (iterators, sensible ownership, `Option`/`Result`) is itself part of the signal — but correctness and communication score first.

---

## Progress Tracker

| Date | Day | Problems attempted | Solved unaided | Needed hint / >45 min (→ re-do) | Notes |
| ---- | --- | ------------------ | -------------- | ------------------------------- | ----- |
|      | 1   |                    |                |                                 |       |
|      | 2   |                    |                |                                 |       |
|      | 3   |                    |                |                                 |       |
|      | 4   |                    |                |                                 |       |
|      | 5   |                    |                |                                 |       |
|      | 6   |                    |                |                                 |       |
|      | 7   |                    |                |                                 |       |
|      | 8   |                    |                |                                 |       |
|      | 9   |                    |                |                                 |       |
|      | 10  |                    |                |                                 |       |
|      | 11  |                    |                |                                 |       |
|      | 12  |                    |                |                                 |       |
|      | 13  |                    |                |                                 |       |
|      | 14  |                    |                |                                 |       |

**Readiness checkpoints**

- [ ] Can recognize the pattern within ~5 minutes of reading a new problem
- [ ] Can write a clean BFS, DFS, and binary search in Rust from memory, no compiler
- [ ] Edit Distance and one topo-sort problem solved cold
- [ ] Two full mocks done under real conditions (plain doc, timed, talking aloud)

---

## After the Phone Screen — Onsite Loop (prep in parallel, lightly)

Same coding patterns, harder problems and tougher follow-ups. Plus a **Googleyness & Leadership** round that's behavioral, not technical — prepare ~4 STAR stories now so you're not scrambling:

- [ ] A time you collaborated across teams / handled a disagreement
- [ ] A time you pushed through an ambiguous, under-specified problem
- [ ] A meaningful failure and what you changed afterward
- [ ] A project where you drove impact or took initiative

System design at L4 is light or absent — don't spend sprint time on it now. If your recruiter confirms a design round, prep LRU Cache, a rate limiter, and one "design a service" walkthrough afterward.

---

_When you want worked solutions, name a category or a specific problem and you'll get idiomatic Rust with the ownership reasoning spelled out, plus the follow-ups an L4 interviewer pushes on._
