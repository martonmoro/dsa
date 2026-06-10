// Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

// A subarray is a contiguous non-empty sequence of elements within an array.

use std::collections::HashMap;

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;

    // prefix_sum -> frequency
    let mut map = HashMap::new();

    // empty prefix occurs once
    map.insert(0, 1);

    for num in nums {
        prefix_sum += num;

        if let Some(freq) = map.get(&(prefix_sum - k)){
            count += freq;
        }

        *map.entry(prefix_sum).or_insert(0) += 1;
    }

    count
}

fn main() {
    let nums = vec![1, 2, 3];
    println!("Result: {}", subarray_sum(nums, 3));
}