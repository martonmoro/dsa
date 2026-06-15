// Given an integer array nums and an integer k, return the k most 
// frequent elements. You may return the answer in any order.
use std::collections::{HashMap, BinaryHeap};
fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Count frequencies
        let mut frequencies = HashMap::new();

        for num in nums {
            *frequencies.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();

        for(num, frequency) in frequencies {
            heap.push((frequency, num));
        }

        let mut result = Vec::new();

        for _ in 0..k {
            if let Some((_, num)) = heap.pop() {
                result.push(num)
            } else {
                return result;
            }
        }

        result
}

fn top_k_frequent_bucket(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();

    // 1. Count frequencies
    for n in nums {
        *freq.entry(n).or_insert(0) += 1;
    }

    // 2. Create buckets: index = frequency
    let mut buckets = vec![Vec::new(); freq.len() + 1];

    for (num, count) in freq {
        buckets[count].push(num);
    }

    // 3. Collect top k from high frequency to low
    let mut result = Vec::new();

    for f in (0..buckets.len()).rev() {
        for &num in &buckets[f] {
            result.push(num);

            if result.len() == k as usize {
                return result;
            }
        }
    }

    result
}