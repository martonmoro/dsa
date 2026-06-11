# Google L4 — Rust Interview Program (14-Day Sprint)

**Target:** Software Engineer III (L4), AI-Powered Systems / Compilers / Code-Translation, Munich Hub
**Immediate Goal:** Pass the technical screen (1 medium-hard problem, ~45 min, plain Google Doc, no autocomplete, no compiler).
**Follow-up:** Onsite Loop (3 Coding Rounds + 1 Googliness Round).

---

## 🚨 The Rust-Specific Rules of Engagement

1. **No autocomplete, no compiler, no run button.** Use a bare text editor (e.g., Notepad, TextEdit) or a blank Google Doc. The borrow checker is your hidden enemy when writing raw text; you must compile the code mentally.
2. **Timebox to 35–45 minutes per problem.** If stuck at 20 minutes, check the structural _concept_ or algorithmic pattern only. Then close the solution and code it yourself.
3. **Talk out loud continuously.** Google values structured communication. Explain _why_ you are choosing a specific data structure or why you are borrowing instead of moving ownership.
4. **Run the 8-Step Protocol every single time.** Do not write a single line of code until Step 6.

### The 8-Step Protocol

1. **Restate** the problem in your own words.
2. **Clarify** constraints (input sizes, values, duplicates, empty bounds, memory/time limits).
3. **Draft** 1-2 concrete test cases, including a subtle edge case.
4. **State** a brute-force approach first + its Big-O complexity.
5. **State** the optimal approach + its Big-O. **Get the interviewer's explicit buy-in before typing.**
6. **Code** cleanly (modular helpers, idiomatic variable names).
7. **Dry-run** one concrete test case through your code line by line, writing variable states on screen.
8. **Discuss** scale up, system follow-ups, and concurrency bottlenecks.

---

## 🛠️ Companion Sheet: The Morning Warm-up Harness

_Retype these 4 templates from memory every morning before starting your problem sets. They bypass 90% of live interview compile bottlenecks._

### A. The Index-Based Graph Arena (No `Rc<RefCell>`)

_Never use pointers or lifetimed references for graphs at Google. Use index vectors._

```rust
struct Node {
    val: i32,
    neighbors: Vec<usize>, // Indirection via index vectors
}

struct Graph {
    nodes: Vec<Node>,
}
```

### B. O(1) String Tokenizer / Indexer

_Rust strings are UTF-8 and do not support `s[i]`. Collect characters instantly._

```rust
let chars: Vec<char> = s.chars().collect();
// Safe O(1) random access: chars[i]
// Safe slicing: &chars[start..end]
```

### C. Standard Min-Heap / Custom Ordering

_Rust's `BinaryHeap` is a Max-Heap by default. Use `Reverse` for Min-Heaps (Intervals/Dijkstra)._

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(current_weight));
// Pop retrieves the lowest element: let Reverse(val) = min_heap.pop().unwrap();
```

### D. Streaming Parser State Machine

_Clean character parsing for AST/Calculators without variable ownership mutation issues._

```rust
let mut chars = s.chars().peekable();
while let Some(&ch) = chars.peek() {
    match ch {
        '0'..='9' => {
            let mut num = 0;
            while let Some(&c) = chars.peek() {
                if !c.is_ascii_digit() { break; }
                num = num * 10 + (chars.next().unwrap() as i32 - '0' as i32);
            }
        }
        _ => { chars.next(); }
    }
}
```

---

## 🎯 Updated Problem Set (Categorized by Pattern)

### 1. Graphs & Grids (Highest Google Frequency)

- [x] ⭐ **Number of Islands** (M)
- [ ] Flood Fill (E)
- [ ] Max Area of Island (M)
- [x] ⭐ **Rotting Oranges** (M) — Multi-source BFS
- [ ] 01 Matrix / Walls and Gates (M)
- [ ] ⭐ **Clone Graph** (M)
- [x] ⭐ **Course Schedule** (M) — Cycle detection
- [ ] ⭐ **Course Schedule II** (M) — Emitting topological sorting (compilation sequence)
- [ ] ⭐ **Alien Dictionary** (H) — Topological sort + parser constraints (Highly Critical)
- [ ] ⭐ **Evaluate Division** (M) — Directed weighted graphs via hash maps
- [ ] Pacific Atlantic Water Flow (M)
- [x] ⭐ **Word Ladder** (H) — Shortest path BFS
- [ ] Number of Provinces (M) — Union-Find
- [ ] Redundant Connection (M) — Union-Find
- [ ] Graph Valid Tree (M)
- [ ] Network Delay Time (M) — Dijkstra
- [ ] Cheapest Flights Within K Stops (M)

### 2. Strings, Parsing & Stacks (On-Theme for Compiler/Translation Role)

- [x] ⭐📄🎯 **Decode String** (M) — `3[abc]` -> `abcabcabc` (_First example in Google's prep document_)
- [ ] ⭐🎯 **Basic Calculator II** (M) — Operator precedence token string parsing
- [ ] 🎯 **Basic Calculator** (H) — Parsing nested parentheses with stacks
- [ ] ⭐🎯 **Design Expression Tree With Evaluate Function** (M) — Postfix AST building in Rust
- [ ] 🎯 **Asteroid Collision** (M) — Safe vector loop mutation pattern via `.last()` checks
- [ ] Valid Parentheses (E)
- [ ] Minimum Remove to Make Valid Parentheses (M)
- [x] ⭐ **Longest Substring Without Repeating Characters** (M) — Sliding window tracking
- [ ] 📄 Longest Word in Dictionary through Deleting (M) — Subsequence checking (_In Google doc_)
- [ ] Group Anagrams (M)
- [ ] Valid Anagram (E)
- [ ] 🎯 String to Integer / atoi (M) — Overflow parsing logic
- [ ] 🎯 Evaluate Reverse Polish Notation (M) — Stack token evaluation

### 3. Arrays, Two Pointers, Sliding Window & Intervals

- [ ] Two Sum (E)
- [x] ⭐ **3Sum** (M)
- [ ] Container With Most Water (M)
- [ ] ⭐ **Trapping Rain Water** (H)
- [x] ⭐ **Product of Array Except Self** (M)
- [ ] Maximum Subarray (M) — Kadane's algorithm
- [x] ⭐ **Subarray Sum Equals K** (M) — Prefix-sum + hashmap matching
- [x] ⭐ **Merge Intervals** (M)
- [ ] Insert Interval (M)
- [ ] Non-overlapping Intervals (M) — Greedy choices
- [x] ⭐ **Meeting Rooms II** (M) — Interval sweep/heap tracking
- [ ] Minimum Window Substring (H)
- [ ] ⭐ **Sliding Window Maximum** (H) — Monotonic deque execution

### 4. Trees

- [ ] Maximum Depth of Binary Tree (E)
- [ ] Diameter of Binary Tree (E/M)
- [ ] Invert Binary Tree (E)
- [ ] Binary Tree Level Order Traversal (M)
- [ ] Binary Tree Right Side View (M)
- [ ] ⭐ **Lowest Common Ancestor (BST then Binary Tree)** (M)
- [ ] ⭐ **Validate Binary Search Tree** (M)
- [ ] Kth Smallest Element in a BST (M)
- [ ] Construct Binary Tree from Preorder and Inorder (M)
- [ ] ⭐ **Binary Tree Maximum Path Sum** (H)
- [ ] ⭐ **Serialize and Deserialize Binary Tree** (H) — Text serialization of graphs/trees

### 5. Heaps / Top-K Tracking

- [ ] ⭐ **Kth Largest Element in an Array** (M)
- [ ] ⭐ **Top K Frequent Elements** (M)
- [ ] ⭐ **Merge k Sorted Lists** (H) — Requires custom `Reverse` wrapper sorting in Rust
- [ ] Find Median from Data Stream (H) — Dynamic multi-heap strategy
- [ ] K Closest Points to Origin (M)
- [ ] Task Scheduler (M)

### 6. Binary Search

- [ ] Binary Search (E)
- [ ] ⭐ **Search in Rotated Sorted Array** (M)
- [ ] Find First and Last Position of Element (M)
- [ ] Find Minimum in Rotated Sorted Array (M)
- [ ] ⭐ **Koko Eating Bananas** (M) — Optimization monotonic space search
- [ ] Split Array Largest Sum (H) — Binary search on answer range
- [ ] Median of Two Sorted Arrays (H)

### 7. Dynamic Programming (Crucial L4 Bar-Raiser)

- [ ] Climbing Stairs (E)
- [ ] House Robber (M)
- [ ] ⭐ **Coin Change** (M)
- [ ] Coin Change II (M)
- [ ] ⭐ **Word Break** (M) 🎯 — Matrix parsing + DP optimization
- [ ] Decode Ways (M)
- [ ] Unique Paths (M)
- [ ] Maximum Product Subarray (M)
- [ ] Partition Equal Subset Sum (M)
- [ ] ⭐ **Longest Increasing Subsequence** (M)
- [ ] ⭐ **Russian Doll Envelopes** (H) — _Highly frequent Google Munich L4 problem_
- [ ] Longest Common Subsequence (M)
- [ ] ⭐🎯 **Edit Distance** (H) — Direct pattern transformation metrics

### 8. Backtracking

- [ ] Subsets (M)
- [ ] Permutations (M)
- [ ] Combination Sum (M)
- [ ] Generate Parentheses (M)
- [ ] ⭐ **Word Search** (M) — Grid tracking backtracker
- [ ] Palindrome Partitioning (M)
- [ ] Letter Combinations of a Phone Number (M)

### 9. Trie (Highly Adjacency Relevant)

- [ ] ⭐ **Implement Trie** (M) — Standard dynamic character allocation arrays
- [ ] Design Add and Search Words Data Structure (M)
- [ ] Word Search II (H)

### 10. Memory & Data Structure Design

- [ ] ⭐ **LRU Cache** (M) — _Highly brutal to write in safe Rust. Implement using custom vector arenas or linked indices, not traditional pointer links._
- [ ] Min Stack (E/M)

---

## ⚡ The High-Yield Core 24 (If you have 1 week left)

_If you are severely compressed for time, filter the list down strictly to these 24 problems:_

Number of Islands · Rotting Oranges · Course Schedule II · Alien Dictionary · Evaluate Division · **Decode String** · Basic Calculator II · Design Expression Tree With Evaluate Function · Longest Substring Without Repeating · 3Sum · Product of Array Except Self · Subarray Sum Equals K · Merge Intervals · Meeting Rooms II · Validate BST · LCA · Binary Tree Maximum Path Sum · Serialize/Deserialize Tree · Merge k Sorted Lists · Search in Rotated Sorted Array · Koko Eating Bananas · Coin Change · Russian Doll Envelopes · **Edit Distance**

---

## 🗓️ The Compressed Schedule

| Day       | Focus Area            | Morning Warm-up Template Target    | Problems to Conquer                                                                           |
| :-------- | :-------------------- | :--------------------------------- | :-------------------------------------------------------------------------------------------- |
| **Day 1** | **Rust Raw Fluency**  | Retype template **B** + **D** once | Two Sum, Valid Parentheses, Decode String, Rotting Oranges                                    |
| **Day 2** | **Arrays & Windows**  | Retype template **B**              | 3Sum, Product of Array Except Self, Longest Substring Without Repeating                       |
| **Day 3** | **Intervals & Heaps** | Retype template **C**              | Merge Intervals, Meeting Rooms II, Merge k Sorted Lists                                       |
| **Day 4** | **Graphs & TopSort**  | Retype template **A**              | Course Schedule II, Alien Dictionary, Evaluate Division                                       |
| **Day 5** | **Compiler Parsing**  | Retype template **D**              | Basic Calculator II, Design Expression Tree, Asteroid Collision                               |
| **Day 6** | **The DP Bar-Raiser** | Retype template **B + C**          | Coin Change, Russian Doll Envelopes, Edit Distance                                            |
| **Day 7** | **Googliness & Mock** | Dry-run code blocks                | Design 3 behavioral stories (Ambiguity, Technical Disagreement, Scale) using the STAR system. |
