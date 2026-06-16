// Given an integer array nums and an integer k, return the k most 
// frequent elements. You may return the answer in any order.

use std::collections::{HashMap, BinaryHeap};
fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut frequency: HashMap<i32, i32> = HashMap::new();

    // Create num -> frequency mapping
    for &num in &nums {
        *frequency.entry(num).or_insert(0) += 1;
    }

    // Push them into binary heap
    let mut heap = BinaryHeap::new();
    for (num, freq) in frequency {
        heap.push((freq, num));
    }

    // Return first k elements (or full heap)
    let mut result = Vec::new();

    for _ in 0..k {
        if let Some((_, num)) = heap.pop() {
            result.push(num);
        } else {
            return result;
        }
    }

    result
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut frequency = HashMap::new();

    // Count frequency
    for &num in &nums {
        *frequency.entry(num).or_insert(0) += 1;
    }

    // Create buckets, indexed by frequency (1..=n)
    let mut buckets = vec![vec![]; n + 1];
    for (num, freq) in frequency {
        buckets[freq].push(num);
    }

    // Collect top k
    let mut result = Vec::new();
    for i in (0..buckets.len()).rev() {
        for num in buckets[i].drain(..) {
            result.push(num);
            if result.len() == k {
                return result;
            }
        }
    }

    result
}

///////////////////////////////////////////////////////////////

// helper for testing (order-independent comparison)
fn contains_same_elements(mut a: Vec<i32>, mut b: Vec<i32>) -> bool {
    a.sort();
    b.sort();
    a == b
}

fn main() {
    // -------------------------
    // Test Case 1: basic case
    // -------------------------
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = top_k_frequent(nums, k);
    println!("Test 1 result: {:?}", result);
    assert!(contains_same_elements(result, vec![1, 2]));

    // -------------------------
    // Test Case 2: single element
    // -------------------------
    let nums = vec![5];
    let k = 1;
    let result = top_k_frequent(nums, k);
    println!("Test 2 result: {:?}", result);
    assert_eq!(result, vec![5]);

    // -------------------------
    // Test Case 3: all same elements
    // -------------------------
    let nums = vec![7, 7, 7, 7];
    let k = 1;
    let result = top_k_frequent(nums, k);
    println!("Test 3 result: {:?}", result);
    assert_eq!(result, vec![7]);

    // -------------------------
    // Test Case 4: ties in frequency
    // -------------------------
    let nums = vec![4, 4, 6, 6, 1, 2];
    let k = 2;
    let result = top_k_frequent(nums, k);
    println!("Test 4 result: {:?}", result);
    assert!(contains_same_elements(result, vec![4, 6]));

    // -------------------------
    // Test Case 5: k equals number of unique elements
    // -------------------------
    let nums = vec![10, 20, 30];
    let k = 3;
    let result = top_k_frequent(nums, k);
    println!("Test 5 result: {:?}", result);
    assert!(contains_same_elements(result, vec![10, 20, 30]));

    // -------------------------
    // Test Case 6: larger input
    // -------------------------
    let nums = vec![
        1,1,1,1,2,2,3,3,3,4,4,5,5,5,5,5
    ];
    let k = 3;
    let result = top_k_frequent(nums, k);
    println!("Test 6 result: {:?}", result);
    assert!(contains_same_elements(result, vec![1, 3, 5]));

    println!("All test cases passed ✅");
}