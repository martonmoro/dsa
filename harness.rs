//! Google L4 — Rust Interview Harness
//! =====================================
//! Every core data-structure idiom you need to write by hand, on a plain doc,
//! with no autocomplete. std-only, no external crates.
//!
//! HOW TO USE
//!   1. `cargo new harness && cd harness`
//!   2. Replace `src/main.rs` with this file.
//!   3. `cargo test`  -> proves every idiom compiles and is correct.
//!   4. `cargo run`   -> prints a confirmation line.
//!
//! DRILL METHOD: retype each section from memory into an empty file, then run
//! the tests. The borrow checker — not the algorithm — is what costs Rust
//! candidates time. Build the muscle memory now.
//!
//! CONTENTS
//!   1.  Strings & chars (UTF-8 trap)        9.  Linked list (Box) + reversal
//!   2.  Vec init (1D / 2D)                 10.  Binary search (bounds + on-answer)
//!   3.  HashMap / HashSet                  11.  Two pointers / sliding window
//!   4.  Sorting & dedup                    12.  Backtracking (subsets / perms)
//!   5.  BinaryHeap (min / max / tuples)    13.  DP (1D coin change, 2D edit dist)
//!   6.  BFS (grid + graph)                 14.  Trie
//!   7.  Graph build + DFS + topo sort      15.  Misc essentials (overflow, iters)
//!   8.  Binary tree (Box + Rc<RefCell>)

#![allow(dead_code, unused_variables, unused_imports, unused_mut, clippy::all)]

use std::cell::RefCell;
use std::cmp::{Ordering, Reverse, max, min};
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;

// ============================================================================
// 1. STRINGS & CHARS  —  s[i] does NOT compile (strings are UTF-8 byte slices)
// ============================================================================
fn string_idioms() {
    let s = "héllo";
    let chars: Vec<char> = s.chars().collect(); // index THIS instead: chars[0]
    let n = chars.len();
    let bytes = s.as_bytes(); // ASCII problems: bytes[0], b'a'
    let rebuilt: String = chars.iter().collect(); // Vec<char> -> String
    let joined: String = ["a", "b", "c"].join("-");

    let d: u32 = '7'.to_digit(10).unwrap(); // char digit -> 7
    let c: char = std::char::from_digit(7, 10).unwrap(); // 7 -> '7'
    let alpha_index = (b'c' - b'a') as usize; // letter -> 0..25  (==2)

    let _ = (n, bytes, rebuilt, joined, d, c, alpha_index);
}

// ============================================================================
// 2. VEC INIT  —  use vec![..; n], NOT array literals, for runtime sizes
// ============================================================================
fn vec_idioms() {
    let a = vec![0i32; 5]; // 1D length 5
    let mut grid = vec![vec![0i32; 3]; 4]; // 2D: 4 rows x 3 cols
    grid[1][2] = 9;
    let range: Vec<i32> = (0..5).collect(); // [0,1,2,3,4]
    let _ = (a, grid, range);
}

// ============================================================================
// 3. HASHMAP / HASHSET  —  the entry API is your counting workhorse
// ============================================================================
fn map_idioms(words: &[&str]) {
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for &w in words {
        *freq.entry(w).or_insert(0) += 1; // or .or_default()
    }
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for &w in words {
        groups.entry(w.len()).or_default().push(w); // grouped by length
    }
    if let Some(&count) = freq.get("a") { /* present */ }
    let seen: HashSet<&str> = words.iter().copied().collect();
    let _ = (freq, groups, seen);
}

// ============================================================================
// 4. SORTING & DEDUP
// ============================================================================
fn sort_idioms() {
    let mut v = vec![3, 1, 2, 2];
    v.sort(); // ascending, stable
    v.sort_unstable(); // faster, no stability guarantee
    v.sort_by(|a, b| b.cmp(a)); // descending
    v.dedup(); // drops CONSECUTIVE dups (sort first)

    let mut pairs = vec![(1, 'b'), (1, 'a'), (0, 'c')];
    pairs.sort_by_key(|&(k, _)| k); // by first field
    pairs.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1))); // tie-break

    let mut floats = vec![3.0, 1.0, 2.0];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap()); // floats: no total Ord
    let _ = (v, pairs, floats);
}

// ============================================================================
// 5. BINARYHEAP  —  max-heap by default; wrap in Reverse for a min-heap
// ============================================================================
fn heap_idioms() {
    let mut maxh = BinaryHeap::new();
    maxh.push(3);
    maxh.push(1);
    let top = maxh.pop(); // Some(3)

    let mut minh: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    minh.push(Reverse(3));
    minh.push(Reverse(1));
    if let Some(Reverse(x)) = minh.pop() { /* x == 1 */ }

    // Dijkstra-style: min by (distance, node)
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, 0usize)));
    while let Some(Reverse((dist, node))) = pq.pop() {
        // relax neighbors of `node` using `dist`...
        break;
    }
    let _ = (maxh, top);
}

// ============================================================================
// 6. BFS  —  VecDeque is the queue. Grid + graph templates.
// ============================================================================
fn grid_bfs(grid: &Vec<Vec<i32>>, start: (usize, usize)) -> Vec<Vec<i32>> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dist = vec![vec![-1; cols]; rows];
    let mut q = VecDeque::new();
    dist[start.0][start.1] = 0;
    q.push_back(start);
    let dirs = [(-1i32, 0), (1, 0), (0, -1), (0, 1)];
    while let Some((r, c)) = q.pop_front() {
        for &(dr, dc) in &dirs {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if dist[nr][nc] == -1 {
                // && grid[nr][nc] is passable
                dist[nr][nc] = dist[r][c] + 1;
                q.push_back((nr, nc));
            }
        }
    }
    dist
}

fn graph_bfs(adj: &Vec<Vec<usize>>, src: usize) -> Vec<i32> {
    let mut dist = vec![-1; adj.len()];
    let mut q = VecDeque::new();
    dist[src] = 0;
    q.push_back(src);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }
    dist
}

// ============================================================================
// 7. GRAPH  —  ALWAYS index-based (Vec<Vec<usize>>). Never Rc<RefCell> nodes.
// ============================================================================
fn build_adj(n: usize, edges: &[(usize, usize)], directed: bool) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
        if !directed {
            adj[v].push(u);
        }
    }
    adj
}

fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[u] = true;
    for &v in &adj[u] {
        if !visited[v] {
            dfs(v, adj, visited);
        }
    }
}

fn dfs_iter(src: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; adj.len()];
    let mut order = Vec::new();
    let mut stack = vec![src];
    while let Some(u) = stack.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        order.push(u);
        for &v in &adj[u] {
            if !visited[v] {
                stack.push(v);
            }
        }
    }
    order
}

/// Kahn's algorithm. Returns None if the graph has a cycle.
fn topo_sort(adj: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut indeg = vec![0; n];
    for u in 0..n {
        for &v in &adj[u] {
            indeg[v] += 1;
        }
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&u| indeg[u] == 0).collect();
    let mut order = Vec::new();
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if order.len() == n { Some(order) } else { None }
}

// ============================================================================
// 8. UNION-FIND (DSU)  —  path compression + union by size
// ============================================================================
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]); // path compression
            self.parent[x] = root;
        }
        self.parent[x]
    }
    /// Returns false if a and b were already connected.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        let (big, small) = if self.size[ra] < self.size[rb] {
            (rb, ra)
        } else {
            (ra, rb)
        };
        self.parent[small] = big;
        self.size[big] += self.size[small];
        true
    }
}

// ============================================================================
// 9. BINARY TREE
//    Owned tree (Option<Box>) — PREFER THIS in interviews; you define the type.
// ============================================================================
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}
impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
fn leaf(v: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(v)))
}
fn inorder(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder(&n.left, out);
        out.push(n.val);
        inorder(&n.right, out);
    }
}
fn height(node: &Option<Box<TreeNode>>) -> i32 {
    match node {
        None => 0,
        Some(n) => 1 + max(height(&n.left), height(&n.right)),
    }
}

// LeetCode's GIVEN signature is Option<Rc<RefCell<TreeNode>>>. Traverse like so:
type RcLink = Option<Rc<RefCell<RcTreeNode>>>;
struct RcTreeNode {
    val: i32,
    left: RcLink,
    right: RcLink,
}
fn inorder_rc(node: &RcLink, out: &mut Vec<i32>) {
    if let Some(rc) = node {
        let n = rc.borrow(); // shared borrow; .borrow_mut() to mutate
        inorder_rc(&n.left, out);
        out.push(n.val);
        inorder_rc(&n.right, out);
        // clone a child HANDLE (cheap refcount bump): rc.borrow().left.clone()
    }
}

// ============================================================================
// 10. LINKED LIST  —  Option<Box<ListNode>>; .take() detaches the tail
// ============================================================================
#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}
fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take(); // grab the rest before re-pointing
        node.next = prev;
        prev = Some(node);
    }
    prev
}
fn list_to_vec(mut head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(n) = head {
        out.push(n.val);
        head = &n.next;
    }
    out
}

// ============================================================================
// 11. BINARY SEARCH
// ============================================================================
/// First index i with a[i] >= target. (a must be sorted)
fn lower_bound(a: &[i32], target: i32) -> usize {
    let (mut lo, mut hi) = (0, a.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if a[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
fn std_binary_search(a: &[i32], target: i32) {
    let _exact = a.binary_search(&target); // Ok(idx) | Err(insert_pos)
    let _lb = a.partition_point(|&x| x < target); // == lower_bound
}
/// Binary search on the answer: smallest k in [lo, hi) where feasible(k) is true.
fn search_answer(lo: i64, hi: i64, feasible: impl Fn(i64) -> bool) -> i64 {
    let (mut lo, mut hi) = (lo, hi);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if feasible(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// ============================================================================
// 12. TWO POINTERS / SLIDING WINDOW
// ============================================================================
fn longest_unique(s: &str) -> usize {
    let b = s.as_bytes();
    let mut last: HashMap<u8, usize> = HashMap::new();
    let (mut left, mut best) = (0usize, 0usize);
    for (right, &ch) in b.iter().enumerate() {
        if let Some(&p) = last.get(&ch) {
            left = left.max(p + 1);
        }
        last.insert(ch, right);
        best = best.max(right - left + 1);
    }
    best
}
fn two_sum_sorted(a: &[i32], target: i32) -> Option<(usize, usize)> {
    if a.is_empty() {
        return None;
    }
    let (mut i, mut j) = (0usize, a.len() - 1);
    while i < j {
        match (a[i] + a[j]).cmp(&target) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }
    None
}

// ============================================================================
// 13. BACKTRACKING  —  push, recurse, pop. (nested fn captures nothing)
// ============================================================================
fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    fn go(start: usize, nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(path.clone());
        for i in start..nums.len() {
            path.push(nums[i]);
            go(i + 1, nums, path, res);
            path.pop();
        }
    }
    let mut res = Vec::new();
    go(0, nums, &mut Vec::new(), &mut res);
    res
}
fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    fn go(nums: &[i32], used: &mut Vec<bool>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            go(nums, used, path, res);
            path.pop();
            used[i] = false;
        }
    }
    let mut res = Vec::new();
    go(
        nums,
        &mut vec![false; nums.len()],
        &mut Vec::new(),
        &mut res,
    );
    res
}

// ============================================================================
// 14. DYNAMIC PROGRAMMING
// ============================================================================
/// Min coins to make `amount` (unbounded). Returns -1 if impossible.
fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;
    for a in 1..=amount {
        for &c in coins {
            let c = c as usize;
            if c <= a && dp[a - c] != i32::MAX {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }
    if dp[amount] == i32::MAX {
        -1
    } else {
        dp[amount]
    }
}
/// Levenshtein edit distance — almost literally this role's job (code translation).
fn edit_distance(s: &str, t: &str) -> usize {
    let a: Vec<char> = s.chars().collect();
    let b: Vec<char> = t.chars().collect();
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1])
            };
        }
    }
    dp[n][m]
}

// ============================================================================
// 15. TRIE  —  HashMap children; the `let mut node = self;` walk is the trick
// ============================================================================
#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}
impl Trie {
    fn new() -> Self {
        Trie::default()
    }
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true;
    }
    fn search(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_end
    }
}

// ============================================================================
// 16. MISC ESSENTIALS  —  overflow safety + iterator power tools
// ============================================================================
fn misc_essentials() {
    // overflow-safe midpoint (don't write (lo + hi) / 2)
    let (lo, hi) = (1_000_000_000i64, 2_000_000_000i64);
    let mid = lo + (hi - lo) / 2;

    // widen before multiplying to avoid i32 overflow (panics in debug builds)
    let (a, b) = (100_000i32, 100_000i32);
    let product = (a as i64) * (b as i64);

    let v = vec![1, 2, 3, 4];
    let sum: i32 = v.iter().sum();
    let evens: Vec<i32> = v.iter().copied().filter(|x| x % 2 == 0).collect();
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let largest = v.iter().max(); // Option<&i32>
    let (arg_idx, &arg_val) = v.iter().enumerate().max_by_key(|&(_, &x)| x).unwrap();
    let zipped: Vec<(i32, char)> = v.iter().copied().zip("abcd".chars()).collect();
    let folded = v.iter().fold(0, |acc, x| acc + x);

    let opt: Option<i32> = Some(5);
    let _got = opt.unwrap_or(0);
    let _mapped = opt.map(|x| x + 1);

    let _ = (
        mid, product, sum, evens, doubled, largest, arg_idx, arg_val, zipped, folded,
    );
}

fn main() {
    println!("Rust interview harness — run `cargo test` to verify all idioms.");
}

// ============================================================================
// SELF-CHECK: cargo test
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_all() {
        string_idioms();
        vec_idioms();
        map_idioms(&["a", "bb", "a", "cc"]);
        sort_idioms();
        heap_idioms();
        misc_essentials();
    }

    #[test]
    fn graphs() {
        let adj = build_adj(4, &[(0, 1), (0, 2), (2, 3)], true);
        assert_eq!(graph_bfs(&adj, 0), vec![0, 1, 1, 2]);
        let mut vis = vec![false; 4];
        dfs(0, &adj, &mut vis);
        assert!(vis.iter().all(|&b| b));
        assert_eq!(dfs_iter(0, &adj).len(), 4);
        assert_eq!(topo_sort(&adj).map(|o| o.len()), Some(4));

        let grid = vec![vec![0; 3]; 3];
        assert_eq!(grid_bfs(&grid, (0, 0))[2][2], 4);

        let mut dsu = Dsu::new(5);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(1, 2));
        assert!(!dsu.union(0, 2)); // already connected
        assert_eq!(dsu.find(0), dsu.find(2));
    }

    #[test]
    fn tree_and_list() {
        let root = Some(Box::new(TreeNode {
            val: 2,
            left: leaf(1),
            right: leaf(3),
        }));
        let mut io = Vec::new();
        inorder(&root, &mut io);
        assert_eq!(io, vec![1, 2, 3]);
        assert_eq!(height(&root), 2);

        let list = build_list(&[1, 2, 3]);
        assert_eq!(list_to_vec(&list), vec![1, 2, 3]);
        let rev = reverse_list(list);
        assert_eq!(list_to_vec(&rev), vec![3, 2, 1]);
    }

    #[test]
    fn search_and_windows() {
        let a = vec![1, 3, 3, 5, 7];
        assert_eq!(lower_bound(&a, 3), 1);
        std_binary_search(&a, 3);
        assert_eq!(search_answer(0, 100, |k| k * k >= 26), 6);
        assert_eq!(longest_unique("abcabcbb"), 3);
        assert_eq!(two_sum_sorted(&[1, 2, 4, 7], 6), Some((1, 2)));
    }

    #[test]
    fn backtracking_and_dp() {
        assert_eq!(subsets(&[1, 2]).len(), 4);
        assert_eq!(permutations(&[1, 2, 3]).len(), 6);
        assert_eq!(coin_change(&[1, 2, 5], 11), 3);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn trie() {
        let mut t = Trie::new();
        t.insert("car");
        t.insert("cat");
        assert!(t.search("car"));
        assert!(!t.search("ca")); // prefix, not a full word
        assert!(!t.search("dog"));
    }
}
