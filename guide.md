# The Complete Coding Interview Reference — in Rust

A single-document crash course covering every pattern, data structure, algorithm, and complexity you need for a Google L4-style coding interview, with idiomatic, compiling Rust for each. Read it top to bottom once; then use the cheat sheets at the end as your day-of reference.

---

## How to read this document

Each technique section follows the same shape:

- **Recognize it** — the signal in the problem statement that should make you reach for this tool.
- **The idea** — the mechanism in one or two sentences.
- **Template** — copy-pasteable Rust you can adapt.
- **Complexity** — time and space.
- **Where it shows up** — canonical problems.

The patterns are the high-order bit. Interviewers do not reward memorized solutions; they reward _fast recognition_ ("this is a sliding window because we want the longest contiguous run satisfying a constraint") plus clean execution. So as you read, train the recognition, not the keystrokes.

---

# Part 1 — Complexity (Big-O)

Big-O describes how runtime or memory grows as input size `n` grows, ignoring constants and lower-order terms. You must be able to state the complexity of your solution _before_ you write it, and justify it.

## The hierarchy (best to worst)

| Notation   | Name         | Example                          |
| ---------- | ------------ | -------------------------------- |
| O(1)       | constant     | hash lookup, array index         |
| O(log n)   | logarithmic  | binary search, balanced-tree op  |
| O(n)       | linear       | single pass over an array        |
| O(n log n) | linearithmic | comparison sort, heap of n items |
| O(n²)      | quadratic    | nested loop over all pairs       |
| O(2ⁿ)      | exponential  | enumerate all subsets (naive)    |
| O(n!)      | factorial    | enumerate all permutations       |

## How to compute it fast

- **Sequential** statements add: O(a) + O(b) → O(a + b), keep the dominant term.
- **Nested** loops multiply: a loop of n inside a loop of m → O(n·m).
- **Halving** the search space each step → O(log n). Halving repeated over n elements → O(n log n).
- **Recursion**: solve the recurrence. T(n) = 2·T(n/2) + O(n) → O(n log n) (merge sort). T(n) = T(n−1) + O(1) → O(n). T(n) = 2·T(n−1) + O(1) → O(2ⁿ).
- **Amortized**: a `Vec` push is O(1) amortized even though occasional reallocation is O(n), because reallocation happens rarely enough to average out.

## Space complexity

Count extra memory beyond the input: auxiliary arrays, hash maps, and — critically — **the recursion call stack**. A recursive tree traversal is O(h) space where h is the tree height (O(n) worst case for a skewed tree, O(log n) for a balanced one). Interviewers frequently ask "what's the space?" and forgetting the call stack is the most common miss.

## The thing people get wrong

"It has two loops so it's O(n²)" is often false. Two _sequential_ loops are O(n). Two _nested_ loops over the same data are O(n²). And a `while` loop whose pointer only ever moves forward across n elements is O(n) total even if it's nested inside a `for`, because the _total_ work is bounded — this is the amortized argument behind two-pointer and sliding-window solutions. Always ask "how many times does this inner line execute _in total_?", not "is it inside another loop?".

---

# Part 2 — Rust interview survival kit

Google lets you interview in Rust, but the interviewer cares about your algorithm, not your borrow-checker gymnastics. The goal is to write Rust that compiles on the first or second try without fighting ownership. This section is the minimum you must have automatic.

## The collections you actually need

```rust
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap, BTreeMap};
use std::cmp::Reverse;
```

### Vec — your default array/stack

```rust
let mut v: Vec<i32> = Vec::new();
let mut v = vec![0; n];            // n zeros
v.push(5);                          // append (O(1) amortized)
let last = v.pop();                 // Option<i32>, removes from end — this is your STACK
let x = v[i];                       // index (panics if out of bounds)
let len = v.len();
v.sort();                           // in place, O(n log n)
v.sort_unstable();                  // faster, no stability guarantee — prefer when ties don't matter
v.sort_by_key(|x| x.abs());         // sort by a derived key
v.sort_by(|a, b| b.cmp(a));         // custom comparator (here: descending)
let s: i32 = v.iter().sum();
let mx = *v.iter().max().unwrap();  // Option, unwrap when non-empty
```

`Vec` IS your stack. `push` / `pop` / `last()` give you LIFO with zero ceremony.

### HashMap — frequency counts and lookups

```rust
let mut m: HashMap<i32, i32> = HashMap::new();
*m.entry(key).or_insert(0) += 1;        // the frequency-count idiom — memorize this
let count = *m.get(&key).unwrap_or(&0);
if let Some(&v) = m.get(&key) { /* ... */ }
if m.contains_key(&key) { /* ... */ }
m.insert(key, val);                      // overwrites
for (k, v) in &m { /* iterate */ }
```

The `*m.entry(k).or_insert(0) += 1` line appears in a huge fraction of problems. Burn it in.

### HashSet — membership and dedup

```rust
let mut set: HashSet<i32> = HashSet::new();
set.insert(x);            // returns false if already present
set.contains(&x);
set.remove(&x);
let set: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();  // dedup
```

### VecDeque — your queue (for BFS)

```rust
let mut q: VecDeque<i32> = VecDeque::new();
q.push_back(x);     // enqueue
let f = q.pop_front();   // dequeue → Option<i32>
q.push_front(x);    // also a deque from the front
while let Some(x) = q.pop_front() { /* ... */ }
```

### BinaryHeap — a MAX-heap by default

```rust
let mut heap: BinaryHeap<i32> = BinaryHeap::new();
heap.push(5);
let top = heap.pop();        // Option<i32>, returns the LARGEST
let peek = heap.peek();      // &i32, largest without removing
```

**For a MIN-heap, wrap in `Reverse`:**

```rust
let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
heap.push(Reverse(5));
if let Some(Reverse(min)) = heap.pop() { /* smallest */ }
```

This min-heap-via-Reverse trick is the single most common Rust heap gotcha. To order by one field of a tuple, push tuples — `BinaryHeap` orders tuples lexicographically: `heap.push((priority, item))`.

## Strings: work with bytes

For interview problems strings are almost always ASCII, and indexing `String` directly is illegal in Rust (UTF-8). Convert to bytes:

```rust
let bytes = s.as_bytes();         // &[u8], now you can index bytes[i]
let c = bytes[i];                 // u8
let idx = (bytes[i] - b'a') as usize;   // map 'a'..='z' to 0..=25
for b in s.bytes() { /* u8 */ }
let chars: Vec<char> = s.chars().collect();  // if you truly need chars
let back: String = chars.into_iter().collect();
```

Use `b'a'`, `b'0'` for byte literals. `(b - b'a') as usize` is the standard alphabet-indexing move.

## The two recursive shapes: ListNode and TreeNode

LeetCode-style problems hand you these exact definitions. Know them cold.

### Linked list

```rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

Key moves: `node.next.take()` swaps the `next` out and leaves `None` in its place (returns the owned `Option<Box<ListNode>>`) — this is how you rewire pointers without fighting the borrow checker.

### Binary tree

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
```

The type `Option<Rc<RefCell<TreeNode>>>` looks scary but you only need three operations:

- `node.borrow()` → immutable access to read `.val`, `.left`, `.right`.
- `node.borrow_mut()` → mutable access to write fields.
- `some_link.clone()` → cheaply clone the `Rc` pointer (bumps a refcount, does NOT deep-copy the tree). You clone the link to pass it into a recursive call.

The recursion pattern that always works: write a helper that takes `&Option<Rc<RefCell<TreeNode>>>`, match on it, and inside the `Some(n)` arm do `let n = n.borrow();` then recurse on `&n.left` and `&n.right`. You'll see this repeatedly in Part 4.

**Interview advice for Rust trees:** don't agonize over the "perfect" ownership. Cloning the `Rc` on every recursive call is idiomatic and cheap (it's a refcount bump, not a copy). Make it work; the interviewer is not grading you on avoiding `.clone()`.

## Iterators — sugar that saves time

```rust
v.iter().enumerate()                 // (index, &value)
v.iter().rev()                       // reverse
v.iter().map(|x| x * 2)
v.iter().filter(|&&x| x > 0)
v.iter().position(|&x| x == target)  // Option<usize>
(0..n).collect::<Vec<usize>>()
nums.windows(2)                      // sliding pairs of size 2
nums.chunks(3)                       // non-overlapping groups of 3
```

`.unwrap()` is fine in interviews when you've reasoned the value exists; mention briefly that production code would handle the `None`. Don't let error handling slow you down.

---

# Part 3 — Data structures

You should be able to state, for each: what it stores, the cost of its core operations, and when to pick it.

## Array / Vec

Contiguous, index in O(1). Push/pop at the **end** O(1) amortized; insert/remove in the **middle** O(n) (shifts everything). Use it as your default container and as a stack.

## Hash map / Hash set

Average O(1) insert, lookup, delete; O(n) worst case under adversarial hashing (rare in interviews). No ordering. Reach for it whenever you find yourself asking "have I seen this before?" or "how many times has this appeared?" — it converts an O(n²) scan-for-a-match into O(n).

## Stack (LIFO)

Last in, first out. A `Vec` with `push`/`pop`. Used for: matching brackets, monotonic-stack problems, iterative DFS, undo, expression parsing.

## Queue (FIFO) / Deque

First in, first out. `VecDeque` with `push_back`/`pop_front`. The engine of BFS. A _deque_ allows O(1) at both ends — needed for sliding-window-maximum (the monotonic deque).

## Heap / Priority queue

Get the min or max in O(1) (peek), pop it in O(log n), insert in O(log n). Backed by `BinaryHeap`. Pick it when you repeatedly need "the smallest/largest remaining" — top-K, Dijkstra, merging k sorted lists, scheduling by next event.

## Linked list

O(1) insert/delete given a node reference, O(n) to find by position. In interviews it mostly tests pointer manipulation (reverse, detect cycle, merge), not as a practical container — in real Rust you'd use a `Vec`.

## Binary tree / BST

A tree where each node has ≤ 2 children. A **binary search tree** keeps left < node < right, giving O(h) search/insert/delete — O(log n) if balanced, O(n) if degenerate (a "stick"). Traversals: preorder (node, left, right), inorder (left, node, right — yields sorted order in a BST), postorder (left, right, node), and level-order (BFS).

## Trie (prefix tree)

A tree keyed by characters; each path from the root spells a prefix. Insert/search a word of length L in O(L), independent of how many words are stored. Use it for prefix queries, autocomplete, word-search-on-a-grid, and dictionary problems.

## Graph

Nodes + edges. Two representations:

- **Adjacency list** `Vec<Vec<usize>>` (or `Vec<Vec<(usize, weight)>>`): O(V + E) space, the default for sparse graphs.
- **Adjacency matrix** `Vec<Vec<bool>>`: O(V²) space, O(1) edge lookup, good for dense graphs.
  A grid is an _implicit_ graph: each cell is a node, edges connect orthogonal neighbors.

## Union-Find (Disjoint Set Union)

Tracks a partition of elements into disjoint sets; `union` merges two sets, `find` returns a set's representative. With path compression + union by rank, operations are effectively O(α(n)) — practically constant. The right tool for connectivity, counting connected components incrementally, and cycle detection in an _undirected_ graph. Template:

```rust
struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    // returns false if a and b were already in the same set
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb { return false; }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
        true
    }
}
```

---

# Part 4 — Patterns and techniques

This is the core. Roughly 90% of L4 coding rounds reduce to the patterns below.

## 4.1 Two pointers

**Recognize it:** a sorted array (or one you can sort) where you're looking for a pair/triple meeting a sum or difference condition; or any "shrink from both ends" / "fast and slow scan" over a sequence.

**The idea:** maintain two indices that move toward each other (or in the same direction at different speeds), using the sorted order to decide which pointer to advance — turning an O(n²) pair search into O(n).

**Template — pair summing to target in a sorted array:**

```rust
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    while lo < hi {
        let sum = nums[lo] + nums[hi];
        if sum == target {
            return Some((lo, hi));
        } else if sum < target {
            lo += 1;          // need a bigger sum
        } else {
            hi -= 1;          // need a smaller sum
        }
    }
    None
}
```

**3Sum** extends this: sort, fix one element with an outer loop, two-pointer the rest, and skip duplicates to avoid repeated triples:

```rust
fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let n = nums.len();
    let mut res = Vec::new();
    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] { continue; }   // skip dup anchor
        let (mut lo, mut hi) = (i + 1, n - 1);
        while lo < hi {
            let sum = nums[i] + nums[lo] + nums[hi];
            match sum.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    res.push(vec![nums[i], nums[lo], nums[hi]]);
                    lo += 1; hi -= 1;
                    while lo < hi && nums[lo] == nums[lo - 1] { lo += 1; } // skip dups
                    while lo < hi && nums[hi] == nums[hi + 1] { hi -= 1; }
                }
                std::cmp::Ordering::Less => lo += 1,
                std::cmp::Ordering::Greater => hi -= 1,
            }
        }
    }
    res
}
```

**Complexity:** two-pointer pass O(n); 3Sum O(n²) (sort is O(n log n), dominated by the nested scan). Space O(1) beyond the output.

**Where it shows up:** Two Sum II, 3Sum, Container With Most Water, Move Zeroes, Valid Palindrome, removing duplicates in place.

## 4.2 Sliding window

**Recognize it:** "longest / shortest / count of **contiguous** subarray or substring" satisfying a constraint (sum ≤ k, at most k distinct, no repeats).

**The idea:** keep a window `[left, right]`. Expand `right` to include elements; when the window violates the constraint, shrink from `left`. Each index enters and leaves the window once, so the whole thing is O(n) despite the nested loop.

**Template — longest substring without repeating characters:**

```rust
fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut last_seen: HashMap<u8, usize> = HashMap::new();
    let mut start = 0usize;
    let mut best = 0i32;
    for (i, &c) in bytes.iter().enumerate() {
        if let Some(&j) = last_seen.get(&c) {
            if j >= start {
                start = j + 1;       // jump past the previous occurrence
            }
        }
        last_seen.insert(c, i);
        best = best.max((i - start + 1) as i32);
    }
    best
}
```

**General shrinkable-window shape** (e.g. minimum window / at-most-k-distinct): expand right, and use a `while` to shrink left until the window is valid again, recording the answer each step.

```rust
// Skeleton: longest window with at most `k` distinct values
fn longest_at_most_k_distinct(nums: &[i32], k: usize) -> usize {
    let mut count: HashMap<i32, i32> = HashMap::new();
    let (mut left, mut best) = (0usize, 0usize);
    for right in 0..nums.len() {
        *count.entry(nums[right]).or_insert(0) += 1;
        while count.len() > k {
            let v = nums[left];
            let c = count.get_mut(&v).unwrap();
            *c -= 1;
            if *c == 0 { count.remove(&v); }
            left += 1;
        }
        best = best.max(right - left + 1);
    }
    best
}
```

**Complexity:** O(n) time, O(k) or O(alphabet) space.

**Where it shows up:** Longest Substring Without Repeating Characters, Minimum Window Substring, Longest Repeating Character Replacement, Max Consecutive Ones, Fruit Into Baskets, Sliding Window Maximum (with a monotonic deque).

## 4.3 Prefix sums (and prefix + hashmap)

**Recognize it:** many range-sum queries; or "count/find subarrays whose sum equals k".

**The idea:** `prefix[i]` = sum of the first i elements. Then the sum of any range `[l, r)` is `prefix[r] − prefix[l]` in O(1). Combined with a hashmap of seen prefix sums, you can count subarrays with a target sum in one pass: a subarray ending at `i` sums to `k` exactly when some earlier prefix equals `current_prefix − k`.

**Template — subarray sum equals k:**

```rust
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    counts.insert(0, 1);          // empty prefix
    let mut running = 0;
    let mut result = 0;
    for n in nums {
        running += n;
        if let Some(&c) = counts.get(&(running - k)) {
            result += c;          // every earlier prefix == running-k closes a valid subarray
        }
        *counts.entry(running).or_insert(0) += 1;
    }
    result
}
```

**Variant — product of array except self** uses the same prefix/suffix idea without division: one left-to-right pass building prefix products, one right-to-left applying suffix products.

```rust
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![1; n];
    let mut prefix = 1;
    for i in 0..n { res[i] = prefix; prefix *= nums[i]; }
    let mut suffix = 1;
    for i in (0..n).rev() { res[i] *= suffix; suffix *= nums[i]; }
    res
}
```

**Complexity:** O(n) time; O(n) space for the prefix array / hashmap (O(1) extra for product-except-self if the output doesn't count).

**Where it shows up:** Subarray Sum Equals K, Range Sum Query, Product of Array Except Self, Contiguous Array, Pivot Index. (2D prefix sums extend this to matrix region sums.)

## 4.4 Binary search

Two distinct flavors. Both rely on a monotonic property: as you move right, some condition flips from false to true exactly once.

### Flavor A — search a sorted array

**Recognize it:** the data is sorted (or rotated-sorted) and you want a value or boundary in O(log n).

```rust
fn binary_search(nums: &[i32], target: i32) -> i32 {
    let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;      // avoids overflow
        let v = nums[mid as usize];
        if v == target { return mid as i32; }
        if v < target { lo = mid + 1; } else { hi = mid - 1; }
    }
    -1
}
```

**Rotated array:** at each step one half is sorted; check which, then decide whether the target lies in it:

```rust
fn search_rotated(nums: Vec<i32>, target: i32) -> i32 {
    let (mut lo, mut hi) = (0i64, nums.len() as i64 - 1);
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let m = nums[mid as usize];
        if m == target { return mid as i32; }
        if nums[lo as usize] <= m {                    // left half sorted
            if nums[lo as usize] <= target && target < m { hi = mid - 1; }
            else { lo = mid + 1; }
        } else {                                        // right half sorted
            if m < target && target <= nums[hi as usize] { lo = mid + 1; }
            else { hi = mid - 1; }
        }
    }
    -1
}
```

### Flavor B — binary search on the answer (parametric search)

**Recognize it:** "minimize/maximize some value X such that a feasibility check passes," where larger X makes the check monotonically easier (or harder). You're not searching an array — you're searching the _range of possible answers_.

**Template — Koko eating bananas (min speed to finish in h hours):**

```rust
fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let feasible = |speed: i64| -> bool {
        // hours needed = sum of ceil(pile / speed)
        let hours: i64 = piles.iter().map(|&p| (p as i64 + speed - 1) / speed).sum();
        hours <= h as i64
    };
    let (mut lo, mut hi) = (1i64, *piles.iter().max().unwrap() as i64);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if feasible(mid) { hi = mid; }     // mid works → try smaller
        else { lo = mid + 1; }             // too slow → go faster
    }
    lo as i32
}
```

The mental template: write a `feasible(x) -> bool` predicate, confirm it's monotonic, then binary-search the smallest (or largest) `x` for which it's true.

**Complexity:** O(log(range) · cost_of_check). For Koko, O(n log(max_pile)).

**Where it shows up:** Search in Rotated Sorted Array, Find Minimum in Rotated Array, Koko Eating Bananas, Capacity to Ship Packages, Split Array Largest Sum, Median of Two Sorted Arrays.

## 4.5 Monotonic stack

**Recognize it:** "next/previous greater (or smaller) element," or building something where you repeatedly pop while the new element beats the top — histogram areas, temperature spans, stock spans.

**The idea:** keep a stack of indices whose values are monotonic (e.g. decreasing). When a new element breaks the order, pop everything it dominates and resolve their answers; each index is pushed and popped once → O(n).

**Template — daily temperatures (days until a warmer day):**

```rust
fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
    let n = temps.len();
    let mut res = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();   // indices, temps decreasing down the stack
    for i in 0..n {
        while let Some(&j) = stack.last() {
            if temps[i] > temps[j] {
                stack.pop();
                res[j] = (i - j) as i32;      // i is the next warmer day for j
            } else {
                break;
            }
        }
        stack.push(i);
    }
    res
}
```

**Complexity:** O(n) time, O(n) stack.

**Where it shows up:** Daily Temperatures, Next Greater Element, Largest Rectangle in Histogram, Trapping Rain Water, Remove K Digits, Stock Span.

## 4.6 Intervals

**Recognize it:** the input is a list of `[start, end]` ranges and you must merge, count overlaps, or schedule.

**The idea:** sort by start. Then a single pass with a "current" interval handles merging; a min-heap of end-times handles "how many overlap at once."

**Template — merge intervals:**

```rust
fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by_key(|iv| iv[0]);
    let mut res: Vec<Vec<i32>> = Vec::new();
    for iv in intervals {
        if let Some(last) = res.last_mut() {
            if iv[0] <= last[1] {                 // overlaps the previous
                last[1] = last[1].max(iv[1]);
                continue;
            }
        }
        res.push(iv);
    }
    res
}
```

**Template — minimum meeting rooms (max concurrent intervals)** via a min-heap of end times:

```rust
fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|iv| iv[0]);
    let mut ends: BinaryHeap<Reverse<i32>> = BinaryHeap::new();  // min-heap of end times
    for iv in intervals {
        if let Some(&Reverse(earliest)) = ends.peek() {
            if iv[0] >= earliest {
                ends.pop();             // a room freed up; reuse it
            }
        }
        ends.push(Reverse(iv[1]));
    }
    ends.len() as i32
}
```

**Complexity:** O(n log n) (the sort dominates).

**Where it shows up:** Merge Intervals, Insert Interval, Non-overlapping Intervals, Meeting Rooms I/II, Interval List Intersections.

## 4.7 Linked list manipulation (fast/slow pointers)

**Recognize it:** the input is a linked list and you must reverse it, find its middle, detect a cycle, or reorder it.

**The two core sub-skills:**

**Reverse a list** — the iterative three-pointer dance, using `take()` to move ownership:

```rust
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next.take();   // detach the rest
        node.next = prev;          // point backward
        prev = Some(node);         // advance prev
    }
    prev
}
```

**Find the middle** — fast pointer moves twice per slow step; when fast reaches the end, slow is at the middle. The same fast/slow idea detects a cycle (if fast ever meets slow, there's a loop — Floyd's algorithm).

**Reorder List** combines all three: find the middle, reverse the second half, then interleave the two halves. In Rust, linked-list problems are fiddlier than in GC languages because of ownership; the reliable approach is often to collect node values into a `Vec`, manipulate indices, and rebuild — acceptable in an interview if you explain the O(n) extra space trade-off.

**Complexity:** O(n) time; reversal/cycle detection O(1) space, the Vec-rebuild approach O(n) space.

**Where it shows up:** Reverse Linked List, Linked List Cycle, Middle of the List, Reorder List, Merge Two Sorted Lists, Remove Nth From End, Palindrome Linked List.

## 4.8 Tree traversal — DFS and BFS

The two ways to visit every node. DFS goes deep (recursion or an explicit stack); BFS goes level by level (a queue).

**BFS — level-order traversal.** The `for _ in 0..q.len()` trick processes exactly one level per outer iteration:

```rust
fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    if let Some(r) = root { q.push_back(r); }
    while !q.is_empty() {
        let mut level = Vec::new();
        for _ in 0..q.len() {                  // snapshot this level's size
            let node = q.pop_front().unwrap();
            let n = node.borrow();
            level.push(n.val);
            if let Some(l) = n.left.clone()  { q.push_back(l); }
            if let Some(r) = n.right.clone() { q.push_back(r); }
        }
        res.push(level);
    }
    res
}
```

**DFS — recursive depth.** The shape for almost every tree problem: match on the link, recurse on children, combine:

```rust
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let n = node.borrow();
            1 + max_depth(n.left.clone()).max(max_depth(n.right.clone()))
        }
    }
}
```

**Validate BST** — pass down the allowed `(low, high)` bounds; use `i64` so `i32::MIN`/`MAX` node values don't break the comparison:

```rust
fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check(node: &Option<Rc<RefCell<TreeNode>>>, lo: i64, hi: i64) -> bool {
        match node {
            None => true,
            Some(n) => {
                let n = n.borrow();
                let v = n.val as i64;
                v > lo && v < hi
                    && check(&n.left, lo, v)
                    && check(&n.right, v, hi)
            }
        }
    }
    check(&root, i64::MIN, i64::MAX)
}
```

**Lowest Common Ancestor** — postorder: recurse both sides; if one side finds p and the other finds q, the current node is the LCA:

```rust
fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root?;
    let (pv, qv) = (p.as_ref()?.borrow().val, q.as_ref()?.borrow().val);
    let nv = node.borrow().val;
    if nv == pv || nv == qv { return Some(node); }
    let left  = lowest_common_ancestor(node.borrow().left.clone(),  p.clone(), q.clone());
    let right = lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());
    match (left, right) {
        (Some(_), Some(_)) => Some(node),   // p and q split here → this is the LCA
        (Some(l), None)    => Some(l),
        (None, Some(r))    => Some(r),
        (None, None)       => None,
    }
}
```

**Tree DP — Binary Tree Maximum Path Sum.** The escalation: each call returns the best _downward_ path through the node, while a mutable global tracks the best path that _bends_ at the node (left + node + right). Clamp negative contributions to 0:

```rust
fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn gain(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let n = n.borrow();
                let left  = gain(&n.left,  best).max(0);   // ignore negative branches
                let right = gain(&n.right, best).max(0);
                *best = (*best).max(n.val + left + right); // path bending here
                n.val + left.max(right)                    // best straight-through path
            }
        }
    }
    let mut best = i32::MIN;
    gain(&root, &mut best);
    best
}
```

**Complexity:** every traversal is O(n) time; space is O(h) for DFS recursion (call stack) and O(width) for BFS — both O(n) worst case.

**Where it shows up:** Level Order, Max Depth, Invert Tree, Validate BST, LCA, Diameter, Max Path Sum, Right Side View, Serialize/Deserialize.

## 4.9 Graph traversal — DFS, BFS, and grids

A grid is just a graph in disguise: each cell is a node, edges go to orthogonal neighbors.

**Grid DFS — number of islands (flood fill).** Sink each island by overwriting its cells so you don't revisit:

```rust
fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    fn sink(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
        if grid[r][c] != '1' { return; }
        grid[r][c] = '0';
        if r > 0            { sink(grid, r - 1, c); }
        if r + 1 < grid.len()    { sink(grid, r + 1, c); }
        if c > 0            { sink(grid, r, c - 1); }
        if c + 1 < grid[0].len() { sink(grid, r, c + 1); }
    }
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1;
                sink(&mut grid, r, c);
            }
        }
    }
    count
}
```

**Multi-source BFS — rotting oranges.** Seed the queue with _all_ starting cells at once, then expand level by level counting "minutes." This is just BFS with more than one initial node — recognize it whenever something spreads simultaneously from multiple origins.

```rust
fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut fresh = 0;
    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                2 => q.push_back((r, c)),
                1 => fresh += 1,
                _ => {}
            }
        }
    }
    let dirs = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    let mut minutes = 0;
    while !q.is_empty() && fresh > 0 {
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in dirs {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        q.push_back((nr, nc));
                    }
                }
            }
        }
        minutes += 1;
    }
    if fresh == 0 { minutes } else { -1 }
}
```

The `dirs` array + bounds-check pattern is the standard way to visit 4-directional neighbors; the cast to `i32` lets you check the `-1` edge without underflowing `usize`.

**Complexity:** O(V + E), which for an `m × n` grid is O(m·n).

**Where it shows up:** Number of Islands, Rotting Oranges, Flood Fill, Max Area of Island, Walls and Gates, Surrounded Regions, Word Search (grid + backtracking), Clone Graph (general-graph DFS with an old→new hashmap), Number of Connected Components (general-graph DFS over an adjacency list).

## 4.10 Topological sort

**Recognize it:** a directed graph of dependencies ("you must do A before B"); you need a valid ordering, or to detect whether a valid ordering exists (i.e. whether there's a cycle).

**The idea (Kahn's algorithm, BFS):** compute every node's in-degree, start from the zero-in-degree nodes, and repeatedly remove a node and decrement its neighbors' in-degrees, queueing any that hit zero. If you output all n nodes, the order is valid; if fewer, a cycle blocked the rest.

**Template — Course Schedule II (return an ordering, or empty if impossible):**

```rust
fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    for p in &prerequisites {
        let (course, pre) = (p[0] as usize, p[1] as usize);
        adj[pre].push(course);      // pre → course
        indeg[course] += 1;
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
    let mut order = Vec::new();
    while let Some(u) = q.pop_front() {
        order.push(u as i32);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 { q.push_back(v); }
        }
    }
    if order.len() == n { order } else { vec![] }   // empty ⇒ cycle
}
```

**Complexity:** O(V + E).

**Where it shows up:** Course Schedule I/II, Alien Dictionary, Task Scheduling with dependencies, build-order problems.

## 4.11 Backtracking

**Recognize it:** "generate all / find all / count all" combinations, permutations, subsets, or arrangements satisfying constraints — the search space is a tree of choices.

**The idea:** at each step, _choose_ an option, _recurse_ on the remaining problem, then _un-choose_ (restore state) so you can try the next option. Prune branches that can't lead to a solution.

**Template — subsets (the canonical skeleton):**

```rust
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(start: usize, nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(path.clone());                  // every node is a valid subset
        for i in start..nums.len() {
            path.push(nums[i]);                  // choose
            backtrack(i + 1, nums, path, res);   // explore
            path.pop();                          // un-choose
        }
    }
    let mut res = Vec::new();
    let mut path = Vec::new();
    backtrack(0, &nums, &mut path, &mut res);
    res
}
```

That choose / explore / un-choose trio is the entire pattern. Permutations track a `used` boolean array instead of a `start` index; combination-sum lets you reuse an element by recursing with `i` instead of `i + 1`.

**Template — Word Search (backtracking on a grid with a visited mark):**

```rust
fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    let (rows, cols) = (board.len(), board[0].len());
    fn dfs(board: &mut Vec<Vec<char>>, r: i32, c: i32, word: &[char], k: usize) -> bool {
        if k == word.len() { return true; }
        if r < 0 || c < 0 || r >= board.len() as i32 || c >= board[0].len() as i32 {
            return false;
        }
        let (ru, cu) = (r as usize, c as usize);
        if board[ru][cu] != word[k] { return false; }
        let saved = board[ru][cu];
        board[ru][cu] = '#';                     // mark visited
        let found = dfs(board, r + 1, c, word, k + 1)
            || dfs(board, r - 1, c, word, k + 1)
            || dfs(board, r, c + 1, word, k + 1)
            || dfs(board, r, c - 1, word, k + 1);
        board[ru][cu] = saved;                   // restore (un-choose)
        found
    }
    for r in 0..rows {
        for c in 0..cols {
            if dfs(&mut board, r as i32, c as i32, &word, 0) { return true; }
        }
    }
    false
}
```

**Complexity:** exponential by nature — subsets O(2ⁿ · n), permutations O(n! · n). Pruning is what makes it tractable in practice; always mention the worst case honestly.

**Where it shows up:** Subsets, Permutations, Combination Sum, Word Search, Generate Parentheses, N-Queens, Palindrome Partitioning, Sudoku Solver.

## 4.12 Heap / top-K

**Recognize it:** "k largest / smallest / most frequent," "merge k sorted lists," "median of a stream," or any time you repeatedly need the current extreme of a changing set.

**The idea:** a heap gives the min or max in O(1) and pop/insert in O(log n). For top-K, keep a heap of size k: O(n log k), better than sorting everything (O(n log n)) when k ≪ n.

**Template — top K frequent elements:**

```rust
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for n in nums { *freq.entry(n).or_insert(0) += 1; }

    // min-heap of (count, value), capped at size k
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    for (&val, &count) in &freq {
        heap.push(Reverse((count, val)));
        if heap.len() > k as usize { heap.pop(); }   // drop the smallest count
    }
    heap.into_iter().map(|Reverse((_, val))| val).collect()
}
```

(Note: this specific problem also has an O(n) bucket-sort solution — buckets indexed by frequency — worth mentioning as the optimal alternative.)

**Complexity:** O(n log k) with the size-k heap; O(1) peek.

**Where it shows up:** Top K Frequent, Kth Largest Element, Merge K Sorted Lists, Find Median from Data Stream (two heaps), Task Scheduler, K Closest Points to Origin.

## 4.13 Greedy

**Recognize it:** at each step there's a locally optimal choice that you can argue leads to a global optimum — usually after sorting, or with a single sweep tracking a running best.

**The idea:** commit to the best immediate move and never reconsider. The hard part is _proving_ greed works (an exchange argument: show that swapping toward the greedy choice never hurts). When you can't justify it, the problem is probably DP instead.

**Examples woven through other patterns:** Merge Intervals (sort, then greedily extend), Jump Game (track the farthest reachable index), Maximum Subarray / Kadane (extend or restart the running sum), Gas Station, Assign Cookies.

**Kadane's algorithm — maximum subarray:**

```rust
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut best = nums[0];
    let mut current = nums[0];
    for &n in &nums[1..] {
        current = n.max(current + n);   // extend the run, or restart at n
        best = best.max(current);
    }
    best
}
```

(Your practice file solves this with a divide-and-conquer recurrence instead — left half, right half, and the best crossing sum — which is a great way to _talk about_ the three cases, but Kadane's O(n) single pass is the answer to reach for under time pressure. Know both; lead with Kadane.)

**Complexity:** typically O(n) or O(n log n) (if a sort is needed). Space O(1).

**Where it shows up:** Maximum Subarray, Jump Game I/II, Gas Station, Partition Labels, Non-overlapping Intervals, Task Scheduler.

## 4.14 Dynamic programming

The pattern people fear most, but it's mechanical once you see the structure. DP applies when a problem has **optimal substructure** (the answer is built from answers to smaller subproblems) and **overlapping subproblems** (the same subproblem recurs, so caching pays off).

**The universal recipe:**

1. **Define the state.** What does `dp[i]` (or `dp[i][j]`) _mean_? This is 80% of the work. Be precise: "the minimum coins to make amount `i`," not "the answer for `i`."
2. **Write the transition.** How does a state depend on smaller states?
3. **Set the base cases.**
4. **Choose an order** so every state's dependencies are computed before it.
5. **Read off the answer** from the right cell.

Below are the archetypes, ordered easy to hard.

### 1D DP — House Robber

State: `dp[i]` = max money robbable from the first `i` houses. Transition: skip house `i` (`dp[i-1]`) or rob it (`dp[i-2] + nums[i]`). Rolling two variables removes the array:

```rust
fn rob(nums: Vec<i32>) -> i32 {
    let (mut prev, mut curr) = (0, 0);    // prev = dp[i-2], curr = dp[i-1]
    for n in nums {
        let next = curr.max(prev + n);
        prev = curr;
        curr = next;
    }
    curr
}
```

### Knapsack-style DP — Coin Change

State: `dp[a]` = fewest coins to make amount `a`. Transition: for each coin `c`, `dp[a] = min(dp[a], dp[a-c] + 1)`. This is the _unbounded_ knapsack (coins reusable):

```rust
fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;                                   // base: 0 coins make amount 0
    for a in 1..=amount {
        for &c in &coins {
            let c = c as usize;
            if c <= a && dp[a - c] != i32::MAX {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }
    if dp[amount] == i32::MAX { -1 } else { dp[amount] }
}
```

### Subsequence DP — Longest Increasing Subsequence

State: `dp[i]` = length of the longest increasing subsequence _ending at_ `i`. O(n²):

```rust
fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    let mut best = 1;
    for i in 0..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        best = best.max(dp[i]);
    }
    best as i32
}
```

There's an O(n log n) refinement: maintain a `tails` array where `tails[k]` is the smallest possible tail of an increasing subsequence of length `k+1`, and binary-search each element's insertion point (`partition_point`). Mention you know it; code the O(n²) first unless asked.

### 2D string DP — Edit Distance

State: `dp[i][j]` = min edits to turn the first `i` chars of `a` into the first `j` of `b`. If the chars match, no cost (`dp[i-1][j-1]`); otherwise 1 + min(delete, insert, replace):

```rust
fn min_distance(word1: String, word2: String) -> i32 {
    let (a, b) = (word1.as_bytes(), word2.as_bytes());
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }            // delete all of a's prefix
    for j in 0..=n { dp[0][j] = j; }            // insert all of b's prefix
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j]      // delete
                    .min(dp[i][j - 1])           // insert
                    .min(dp[i - 1][j - 1]);      // replace
            }
        }
    }
    dp[m][n] as i32
}
```

### String DP with a dictionary — Word Break

State: `dp[i]` = can the first `i` chars be segmented into dictionary words? Transition: true if some `dp[j]` is true and `s[j..i]` is a word:

```rust
fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let words: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && words.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}
```

**Top-down alternative (memoized recursion):** when the transition is easier to express recursively, write the recursion and cache results in a `HashMap` or `Vec<Option<_>>`. Same complexity, sometimes clearer; mention both directions exist.

**Complexity:** the number of states times the cost per transition. 1D linear-scan DP is O(n²) or O(n·k); 2D string DP is O(m·n). Space often reducible from 2D to 1D by keeping only the previous row.

**Where it shows up:** House Robber I/II, Coin Change, LIS, Edit Distance, Word Break, Longest Common Subsequence, Unique Paths, Decode Ways, Partition Equal Subset Sum, Best Time to Buy/Sell Stock variants.

## 4.15 Trie (prefix tree)

**Recognize it:** many words plus repeated prefix queries — autocomplete, spell-check, "does any word start with…", word-search dictionaries.

**The idea:** a tree where each edge is a character; a path from the root spells a prefix, and a flag marks where a complete word ends. Lookups cost O(word length), independent of dictionary size.

**Template:** using a `HashMap<u8, Trie>` for children keeps the Rust clean (no `Box`/`Option` juggling — a `HashMap` is heap-backed, so the recursive type is sized):

```rust
#[derive(Default)]
struct Trie {
    children: HashMap<u8, Trie>,
    is_word: bool,
}

impl Trie {
    fn new() -> Self { Default::default() }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children.entry(b).or_default();
        }
        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        self.find(word.as_bytes()).map_or(false, |n| n.is_word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix.as_bytes()).is_some()
    }

    fn find(&self, s: &[u8]) -> Option<&Trie> {
        let mut node = self;
        for b in s {
            node = node.children.get(b)?;
        }
        Some(node)
    }
}
```

For a fixed lowercase alphabet, a `[Option<Box<Trie>>; 26]` array of children is the textbook (slightly faster, more memory) variant — but the `HashMap` version is easier to write correctly under time pressure.

**Complexity:** insert/search/prefix in O(L) where L is the word length; space O(total characters across all words).

**Where it shows up:** Implement Trie, Word Search II, Add and Search Word (with `.` wildcards → DFS over children), Replace Words, Design Search Autocomplete.

---

# Part 5 — Algorithms worth knowing by name

These are named algorithms an interviewer might invoke or expect you to reach for.

## Sorting

Rust's `sort` is a stable O(n log n) merge-style sort; `sort_unstable` is a faster introsort. You almost never implement a sort, but you should be able to explain:

- **Merge sort** — divide in half, sort each, merge. O(n log n) time, O(n) space, stable. The recurrence T(n)=2T(n/2)+O(n).
- **Quicksort** — partition around a pivot, recurse. O(n log n) average, O(n²) worst (bad pivots), O(log n) stack. In place.
- **Heap sort** — build a heap, pop n times. O(n log n), O(1) space, not stable.
- **Counting / bucket / radix sort** — O(n + k) when values are bounded integers; this is how you beat n log n (e.g. the bucket-sort solution to Top K Frequent).

## Binary search

Covered in 4.4 — O(log n) on sorted data, and the parametric "search on the answer" form.

## Breadth-first / depth-first search

Covered in 4.8–4.9 — O(V + E). BFS finds shortest paths in _unweighted_ graphs (each edge cost 1).

## Dijkstra's algorithm

Shortest paths from a source in a graph with **non-negative** edge weights, using a min-heap. O((V + E) log V). This is exactly the engine behind Network Delay Time (the answer is the max over all shortest distances from the source; if any node is unreachable, return −1).

```rust
fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, src: usize) -> Vec<i64> {
    let n = adj.len();
    let mut dist = vec![i64::MAX; n];
    dist[src] = 0;
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, src)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }                 // stale entry, skip
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                heap.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}
```

Know that **Bellman-Ford** (O(V·E)) handles negative edges and detects negative cycles, and that for "shortest path with at most K stops" (Cheapest Flights) you run a bounded-iteration Bellman-Ford or a layered BFS. These are above the L4 core — recognize them, don't grind them.

## Topological sort & Union-Find

Covered in 4.10 and Part 3 — O(V + E) and near-O(1) per operation respectively. Note the two ways to count connected components: **Union-Find** (the natural fit for Number of Provinces, where you union connected city pairs and count distinct roots) and plain **graph DFS/BFS** over an adjacency list (the natural fit for Number of Connected Components, where you sweep nodes and flood each unvisited one). Both are correct on either problem — knowing why you'd pick one is the signal.

## Floyd's cycle detection (tortoise and hare)

Two pointers at different speeds detect a cycle in a linked list (or a functional graph) in O(n) time, O(1) space — see 4.7.

---

# Part 6 — Pattern recognition cheat sheet

When you read a problem, scan for these signals:

| Signal in the problem                                | Reach for                        |
| ---------------------------------------------------- | -------------------------------- |
| Sorted array, find a pair/triple                     | Two pointers                     |
| "Longest/shortest contiguous … subarray/substring"   | Sliding window                   |
| Many range-sum queries, or "subarray summing to k"   | Prefix sums (+ hashmap)          |
| Sorted data, O(log n) expected                       | Binary search                    |
| "Minimize/maximize X such that feasible(X)"          | Binary search on the answer      |
| "Next greater/smaller element," histogram            | Monotonic stack                  |
| List of `[start, end]` ranges                        | Intervals (sort, sweep, or heap) |
| Linked list reverse/middle/cycle                     | Fast & slow pointers             |
| Tree, visit by level                                 | BFS (queue)                      |
| Tree, combine results from children                  | DFS recursion                    |
| Grid of connected cells / regions                    | Grid DFS or BFS (flood fill)     |
| Spreads from many origins at once                    | Multi-source BFS                 |
| Dependencies / "must come before"                    | Topological sort                 |
| Connectivity / connected components                  | Union-Find or graph DFS          |
| "Generate/find ALL combinations/permutations"        | Backtracking                     |
| "K largest/smallest/most frequent," stream median    | Heap                             |
| "Max/min total," count of ways, optimal substructure | Dynamic programming              |
| Many words, prefix queries                           | Trie                             |
| Shortest path, unweighted                            | BFS                              |
| Shortest path, non-negative weights                  | Dijkstra                         |

---

# Part 7 — Complexity quick reference

| Operation / approach                        | Time            | Space           |
| ------------------------------------------- | --------------- | --------------- |
| Hash map/set lookup, insert                 | O(1) avg        | O(n)            |
| Binary search                               | O(log n)        | O(1)            |
| Single pass / two pointers / sliding window | O(n)            | O(1)–O(n)       |
| Heap push/pop                               | O(log n)        | O(n)            |
| Sorting                                     | O(n log n)      | O(n) / O(log n) |
| Tree/graph traversal (BFS/DFS)              | O(V + E)        | O(V)            |
| Topological sort                            | O(V + E)        | O(V)            |
| Dijkstra (heap)                             | O((V+E) log V)  | O(V)            |
| Union-Find op (with optimizations)          | ~O(α(n)) ≈ O(1) | O(n)            |
| 1D DP                                       | O(n) or O(n·k)  | O(n) → O(1)     |
| 2D DP                                       | O(m·n)          | O(m·n) → O(n)   |
| Subsets (backtracking)                      | O(2ⁿ·n)         | O(n)            |
| Permutations (backtracking)                 | O(n!·n)         | O(n)            |
| Trie insert/search                          | O(L)            | O(total chars)  |

---

# Part 8 — What to say while coding (the L4 script)

At L4, _how_ you solve is graded as heavily as _whether_ you solve. Run this loop out loud, every time:

**1. Clarify (1–2 min).** Restate the problem in your words. Ask about input size (drives the target complexity), value ranges (negatives? overflow? duplicates?), empty/single-element inputs, and what to return on no answer. "Can the array be empty? Are values within i32? Can there be duplicates?"

**2. State an approach before coding.** "The brute force is checking all pairs, O(n²). But since I only need to know if a complement exists, a hash map gets it to O(n) time, O(n) space. I'll go with that." Naming the brute force first shows you see the space; naming the improvement shows you can optimize.

**3. State the complexity up front,** then confirm it again at the end. Interviewers almost always ask — beat them to it.

**4. Narrate while you write.** "I'll iterate, and for each element check if its complement is already in the map; if not, insert it." Silence reads as being stuck.

**5. Handle edge cases explicitly.** Point to where empty input, a single element, or all-duplicates is handled. If you skip one to save time, say so: "I'm assuming non-empty per our discussion."

**6. Test on a small example.** Trace your code by hand on a 3–4 element input, out loud. This catches off-by-one and boundary bugs before the interviewer does, which is a strong signal.

**7. Offer the trade-off.** Close with "the alternative is X, which trades time for space" — it shows range even when your first answer was right.

**The meta-point:** a clean O(n log n) solution you explain crisply, test, and reason about beats a fragile optimal solution delivered in silence. Recover gracefully from mistakes — "let me re-check this boundary" is a positive signal, not a negative one. Communication and structured thinking are what separate a hire from a no-hire when two candidates write the same code.

---

# Part 9 — Study sequence (likelihood-ranked, topic-interleaved)

This sequence drills your 25-file practice set. One file, `testing.rs`, is an exact duplicate of `subarray_sum_equals_k`, so it's dropped — leaving **24 unique problems**.

Two rules govern the order:

1. **Ranked by likelihood** of appearing in a Google L4 round, most likely first. (This is a judgment call weighted toward Google's known lean on graphs, trees, and array/string patterns — treat clusters as roughly interchangeable, not as a precise leaderboard.)
2. **Interleaved by topic** so that no two consecutive problems share a technique family. You can't coast by guessing "the next one is also BFS" — you have to re-recognize the pattern cold each time, which is the skill that's actually tested.

The bracketed number is each problem's raw likelihood rank _before_ interleaving, so you can still see the priority.

| #   | Problem (file)                                   | Pattern                     | Likelihood |
| --- | ------------------------------------------------ | --------------------------- | ---------- |
| 1   | `number_of_islands`                              | grid flood fill (DFS)       | [1]        |
| 2   | `course_schedule_2`                              | topological sort            | [2]        |
| 3   | `lca`                                            | tree recursion (postorder)  | [3]        |
| 4   | `longest_substring_without_repeating_characters` | sliding window              | [4]        |
| 5   | `merge_intervals`                                | intervals (sort + sweep)    | [5]        |
| 6   | `3sum`                                           | two pointers                | [6]        |
| 7   | `trie`                                           | prefix tree                 | [8]        |
| 8   | `subarray_sum_equals_k`                          | prefix sum + hashmap        | [7]        |
| 9   | `top_k_frequent`                                 | heap / bucket sort          | [9]        |
| 10  | `rotting_oranges`                                | multi-source BFS            | [10]       |
| 11  | `coin_change`                                    | DP (unbounded knapsack)     | [11]       |
| 12  | `validate_bst`                                   | tree recursion (bounds)     | [12]       |
| 13  | `search_rotated_array`                           | binary search               | [13]       |
| 14  | `network_delay_time`                             | Dijkstra                    | [14]       |
| 15  | `binary_tree_level_order_traversal`              | tree BFS                    | [15]       |
| 16  | `koko_eating_bananas`                            | binary search on the answer | [16]       |
| 17  | `meeting_rooms_2`                                | heap + intervals            | [17]       |
| 18  | `product_except_self`                            | prefix / suffix products    | [18]       |
| 19  | `number_of_connected_components`                 | graph DFS                   | [19]       |
| 20  | `edit_distance`                                  | 2D string DP                | [21]       |
| 21  | `number_of_provinces`                            | union-find                  | [20]       |
| 22  | `subsets`                                        | backtracking                | [22]       |
| 23  | `daily_temperatures`                             | monotonic stack             | [23]       |
| 24  | `max_subarray`                                   | divide & conquer / Kadane   | [24]       |
| 25  | `reorder_list`                                   | linked list pointer surgery | [25]       |

(Only four positions deviate from pure likelihood order — `trie`↔`subarray_sum_equals_k` and `edit_distance`↔`number_of_provinces` were swapped to break up adjacent same-family problems — so the sequence stays very close to the underlying priority. The two grid problems sit nine apart, the binary-search pair is split, and no two adjacent items are both tree/graph/grid traversals.)

## Split across three days

Work straight down the list; each day stays interleaved, so you keep re-recognizing patterns within the session too.

- **Day 1 — problems 1–9** (islands, course schedule II, LCA, longest substring, merge intervals, 3Sum, trie, subarray sum = k, top K frequent).
- **Day 2 — problems 10–17** (rotting oranges, coin change, validate BST, search rotated array, network delay time, level order, Koko, meeting rooms II).
- **Day 3 — problems 18–25** (product except self, connected components, edit distance, provinces, subsets, daily temperatures, max subarray, reorder list).

## How to drill each problem

For each: solve once without help; if stuck past ~30 minutes, read the solution, understand it, and re-solve from scratch the next day. State approach and complexity before coding, every time — out loud, following the Part 8 script. Because the list is shuffled by topic, force yourself to do the recognition step first ("what signal tells me which pattern this is?") instead of assuming it matches the previous problem.

## Gaps worth filling

Your practice set is graph/tree/array heavy and light in a few areas the patterns sections above cover but your files don't yet drill. If you have spare time, add at least one of each: a **pure backtracking** beyond subsets (Permutations or Combination Sum), a **1D DP warm-up** (House Robber), a **fast/slow linked-list** problem (Reverse Linked List or Linked List Cycle, since `reorder_list` is the only list problem you have), and **Word Search** (grid + backtracking + trie, a very Google-flavored combination). These round out the recognition table in Part 6.

---

_Master the patterns, not the problems. When a new question appears, the win condition is recognizing which of the ~18 techniques above it reduces to — then the template is muscle memory and your attention is free for the communication that actually gets you the offer._
