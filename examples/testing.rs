// Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

// A subarray is a contiguous non-empty sequence of elements within an array.

use std::collections::HashMap;
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut frequency = HashMap::new();
    frequency.insert(0,1);
    let mut result = 0;

    let mut prefix = 0;

    // k = prefix(end) - prefix(start)
    for num in nums {
        prefix += num;

        if let Some(&count) = frequency.get(&(prefix - k)) {
            result += count;
        }

        *frequency.entry(prefix).or_insert(0) += 1;
    }

    result
}

fn main() {
    let nums = vec![1, 2, -2, -1, 3];
    println!("Result: {}", subarray_sum(nums, 3));
}